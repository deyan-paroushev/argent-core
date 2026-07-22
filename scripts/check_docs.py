#!/usr/bin/env python3
"""
Documentation conformance check.

Fails the build when documentation drifts from the contract. This exists because
a set of documents that name contract functions is only as trustworthy as the
last time anyone checked them, and four substantive errors reached review before
this script existed:

  1. Documents claimed there was no cure window. The contract implements one
     (`issue_default_notice` -> `cure_deadline_ledger` -> `CurePeriodNotExpired`).
  2. Documents stated the collateral-adjustment order as owner -> bank -> custodian.
     The contract enforces owner -> custodian -> bank.
  3. Documents listed post-adjustment coverage as a candidate control. It is
     enforced (`AdjustmentUndercovered`).
  4. Documents described line suspension as purely manual. A margin call
     suspends the line automatically.

Every one of those was checkable against the source in seconds. None of them was
checked, because nothing forced the check. This script forces it.

Usage:
    python3 scripts/check_docs.py            # check
    python3 scripts/check_docs.py --verbose  # show every assertion

Exit code 0 = clean, 1 = failures.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
DOCS = REPO / "docs"
CONTRACTS = REPO / "contracts"
CANONICAL_DOCS = (
    REPO / "README.md",
    REPO / "PRODUCT.md",
    REPO / "IMPLEMENTATION_STATUS.md",
    REPO / "PILOT_PROFILE.md",
    REPO / "SECURITY_AND_PRIVACY.md",
    REPO / "LEGAL_PILOT_CHECKLIST.md",
)

FAILURES: list[str] = []
CHECKS_RUN = 0

# Known, accepted failures. A ratchet: the check can be adopted without a
# blocking cleanup, and the baseline is burned down over time. A line here is a
# debt, not a permission — every entry should have an owner and a plan.
#
# Format: one "check name :: substring of the detail" per line. Blank lines and
# lines starting with # are ignored.
BASELINE_FILE = REPO / "scripts" / "docs_baseline.txt"


def load_baseline() -> list[str]:
    if not BASELINE_FILE.exists():
        return []
    out = []
    for line in BASELINE_FILE.read_text().splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            out.append(line)
    return out


BASELINE = load_baseline()


def fail(check: str, detail: str) -> None:
    entry = f"{check}\n      {detail}"
    for waived in BASELINE:
        name, _, frag = waived.partition("::")
        if name.strip() == check and frag.strip() and frag.strip() in detail:
            return  # known and accepted; do not fail the build
    FAILURES.append(entry)


def ok(verbose: bool, msg: str) -> None:
    global CHECKS_RUN
    CHECKS_RUN += 1
    if verbose:
        print(f"  pass  {msg}")


# ---------------------------------------------------------------------------
# Load sources
# ---------------------------------------------------------------------------

def load_contract_source() -> str:
    parts = []
    for rs in sorted(CONTRACTS.rglob("*.rs")):
        if "/target/" in str(rs):
            continue
        parts.append(rs.read_text(errors="replace"))
    if not parts:
        fail("contract source", f"no .rs files found under {CONTRACTS}")
    return "\n".join(parts)


def load_docs() -> dict[pathlib.Path, str]:
    if not DOCS.is_dir():
        fail("docs directory", f"{DOCS} does not exist")
        return {}
    paths = list(DOCS.rglob("*.md"))
    paths.extend(path for path in CANONICAL_DOCS if path.exists())
    return {p: p.read_text(errors="replace") for p in sorted(set(paths))}


# ---------------------------------------------------------------------------
# Check 1 — every backticked contract function named in docs must exist
# ---------------------------------------------------------------------------

def check_function_names(src: str, docs: dict, verbose: bool) -> None:
    """A doc that names a function the contract does not have is a lie a bank
    can detect in one grep. Proposed-but-unbuilt names must be declared below."""
    existing = set(re.findall(r"pub fn (\w+)", src))

    # Names that appear in docs as deliberate proposals, not claims of existence.
    # Each MUST be described in its document as not-yet-implemented.
    PROPOSED = {
        "expire_cure_window",   # credit-control-extension-points.md, design option
        "register_instrument",  # referenced generically in the reference architecture
    }

    # Local variables and struct fields discussed in prose. These are real code
    # identifiers but are not `pub fn`, so they must not be flagged as missing.
    NOT_FUNCTIONS = {
        "warning_band", "action_band", "borrowing_base", "available_limit",
        "drawn_balance", "raw_value", "uniqueness_hash", "manifest_hash",
        "location_hash", "quality_cert_hash", "quantity_cert_hash", "grade_hash",
        "legal_terms_hash", "control_agreement_hash", "eligibility_hash",
        "margin_policy_hash", "valuation_ref", "payoff_letter_hash",
        "notice_hash", "cure_deadline_ledger", "cure_expiry_ledger",
        "max_age_secs", "conf_tol_bps", "haircut_bps", "warning_bps",
        "maintenance_bps", "suspend_ltv_bps", "warning_ltv_bps",
        "consent_hash", "lien_search_hash", "rights_opinion_hash",
        # Crate / contract names, not functions.
        "settlement_vault", "credit_ledger", "rewards_ledger",
    }

    for path, text in docs.items():
        # Catch both `name` and `name()` — the parenthesised form is the most
        # natural way to write a function in Markdown, and an earlier version of
        # this check missed it entirely.
        candidates = set(re.findall(r"`([a-z][a-z0-9_]{6,})\(\)`", text))          # `fn()`
        bare = set(re.findall(r"`([a-z][a-z0-9_]{6,})`", text))                     # `fn`
        for name in candidates | bare:
            if name in existing or name in PROPOSED or name in NOT_FUNCTIONS:
                continue
            # A parenthesised name is unambiguously a function call: always flag.
            # A bare name might be a field or a variable, so require a verb that
            # asserts behaviour before flagging it.
            if name not in candidates and not re.search(
                rf"`{re.escape(name)}`\s*(?:\(|—|is (?:signed|called|enforced)|fails|rejects|takes|sets)", text
            ):
                continue
            fail(
                "nonexistent contract function",
                f"{path.name} names `{name}()` — not found in contracts/ and not "
                f"declared as a proposal. Either it exists (fix the docs), or add "
                f"it to PROPOSED in this script and mark it unimplemented in the text.",
            )
    ok(verbose, "all named contract functions exist or are declared proposals")


# ---------------------------------------------------------------------------
# Check 2 — lifecycle sequences must match the contract's status guards
# ---------------------------------------------------------------------------

def check_lifecycle_sequences(src: str, docs: dict, verbose: bool) -> None:
    """The adjustment order was wrong in five documents. The contract's status
    guards are the ground truth; assert the docs agree with them."""

    # Ground truth, derived from the guards themselves.
    custodian_second = "AdjustmentStatus::Requested" in src and re.search(
        r"pub fn custodian_confirm_adjustment.*?AdjustmentStatus::Requested", src, re.S
    )
    bank_third = re.search(
        r"pub fn bank_approve_adjustment.*?AdjustmentStatus::CustodianConfirmed", src, re.S
    )

    if not (custodian_second and bank_third):
        fail(
            "lifecycle ground truth",
            "could not confirm the adjustment guards in the contract. If the "
            "contract changed, this script must change with it.",
        )
        return

    # Any doc asserting the reverse order is wrong.
    REVERSED = [
        r"owner\s*(?:requests)?\s*(?:→|->)\s*bank\s*(?:approves)?\s*(?:→|->)\s*custodian",
        r"bank\s+approves.{0,40}?custodian\s+confirms",
        r"the bank signs `bank_approve_adjustment`;? the custodian signs `custodian_confirm_adjustment`",
    ]
    for path, text in docs.items():
        low = text.lower()
        for pat in REVERSED:
            if re.search(pat, low):
                fail(
                    "reversed lifecycle sequence",
                    f"{path.name} states the adjustment order as owner → bank → custodian. "
                    f"The contract enforces owner → CUSTODIAN → BANK "
                    f"(custodian_confirm_adjustment requires status Requested; "
                    f"bank_approve_adjustment requires CustodianConfirmed).",
                )
                break
    ok(verbose, "adjustment sequence: docs agree with contract guards (owner → custodian → bank)")


# ---------------------------------------------------------------------------
# Check 3 — implemented controls must not be described as missing
# ---------------------------------------------------------------------------

def check_implemented_not_denied(src: str, docs: dict, verbose: bool) -> None:
    """The expensive class of error: telling a bank you cannot do something the
    contract already does. Each entry is (feature, code evidence, forbidden claims)."""

    RULES = [
        (
            "cure window / enforcement standstill",
            ["cure_deadline_ledger", "cure_expiry_ledger", "CurePeriodNotExpired"],
            [
                r"no time-bound cure",
                r"no cure deadline",
                r"no deadline,? no expiry",
                r"there is no time on this path",
                r"cure window.{0,30}not implemented",
            ],
        ),
        (
            "post-adjustment coverage enforcement",
            ["AdjustmentUndercovered"],
            [
                r"coverage.{0,40}could be enforced",
                r"coverage test.{0,30}candidate",
            ],
        ),
        (
            "automatic suspension on margin call",
            ["LineStatus::Suspended"],
            [
                r"suspension is (?:only )?a (?:purely )?manual",
                r"no threshold drives it",
                r"purely manual,? bank-signed act",
            ],
        ),
        (
            "dual-control release",
            ["ReleaseAuthorized"],
            [],
        ),
    ]

    for feature, evidence, forbidden in RULES:
        implemented = all(e in src for e in evidence)
        if not implemented:
            # The feature is genuinely absent — docs are free to say so.
            ok(verbose, f"{feature}: not in contract, no claim to police")
            continue
        for path, text in docs.items():
            low = text.lower()
            for pat in forbidden:
                if re.search(pat, low):
                    fail(
                        "implemented feature described as missing",
                        f"{path.name} denies '{feature}', but the contract implements it "
                        f"(evidence: {', '.join(evidence)}). This understates the system "
                        f"to a reader who will check.",
                    )
                    break
        ok(verbose, f"{feature}: implemented, and no doc denies it")


# ---------------------------------------------------------------------------
# Check 4 — placeholders must not reach the repository
# ---------------------------------------------------------------------------

def check_placeholders(docs: dict, verbose: bool) -> None:
    BAD = {
        "<COMMIT>": "unresolved commit placeholder in the status header",
        "verify before external use": "a public repository IS external use",
        "TODO": "unresolved TODO",
        "TKTK": "unresolved placeholder",
    }
    for path, text in docs.items():
        low = text.lower()
        for token, why in BAD.items():
            if token.lower() in low:
                fail("placeholder in published docs", f"{path.name} contains '{token}' — {why}")
    ok(verbose, "no unresolved placeholders")


# ---------------------------------------------------------------------------
# Check 5 — relative links must resolve
# ---------------------------------------------------------------------------

def check_links(docs: dict, verbose: bool) -> None:
    for path, text in docs.items():
        refs: set[str] = set()

        # Markdown targets, optionally followed by a heading fragment.
        for target in re.findall(r"\]\(([^)]+)\)", text):
            target = target.strip().strip("<>").split(maxsplit=1)[0]
            if target.split("#", 1)[0].endswith((".md", ".pdf")):
                refs.add(target)

        # Bare backticked document paths used in architecture prose.
        refs |= set(
            re.findall(
                r"`((?:(?:\.\.?/|docs/)[\w./-]+|[\w-]+)\.(?:md|pdf))(?:#[\w-]+)?`(?!\])",
                text,
            )
        )

        for ref in refs:
            if re.match(r"^[a-z][a-z0-9+.-]*://", ref, re.I):
                continue
            clean = ref.split("#", 1)[0]
            target = (REPO / clean) if clean.startswith("docs/") else (path.parent / clean)
            if not target.resolve().exists():
                fail(
                    "broken doc reference",
                    f"{path.relative_to(REPO)} references '{ref}', which does not resolve from that document.",
                )
    ok(verbose, "all inter-document references resolve")


# ---------------------------------------------------------------------------
# Check 6 — the Apache-2.0 claim must be backed
# ---------------------------------------------------------------------------

def check_licence(docs: dict, verbose: bool) -> None:
    claims = [p.name for p, t in docs.items() if "Apache-2.0" in t]
    if not claims:
        ok(verbose, "no licence claim to back")
        return

    licence = REPO / "LICENSE"
    if not licence.exists():
        fail(
            "unbacked licence claim",
            f"{len(claims)} document(s) claim Apache-2.0 but there is no /LICENSE file. "
            f"This is checkable by anyone in seconds. Add the licence or drop the claim.",
        )
        return
    if "Apache License" not in licence.read_text(errors="replace"):
        fail("unbacked licence claim", "/LICENSE exists but does not contain the Apache licence text")
        return

    for manifest in CONTRACTS.rglob("Cargo.toml"):
        if "/target/" in str(manifest):
            continue
        text = manifest.read_text(errors="replace")
        if "[package]" in text and "license" not in text:
            fail(
                "licence not declared in manifest",
                f"{manifest.relative_to(REPO)} has a [package] section but no "
                f'license = "Apache-2.0"',
            )
    ok(verbose, f"Apache-2.0 claim backed by /LICENSE and declared in manifests ({len(claims)} docs claim it)")


# ---------------------------------------------------------------------------

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--verbose", "-v", action="store_true", help="show each passing assertion")
    args = ap.parse_args()

    print("Checking documentation against contract source...\n")

    src = load_contract_source()
    docs = load_docs()

    if not docs:
        print("No documents found. Nothing to check.")
        return 1

    check_function_names(src, docs, args.verbose)
    check_lifecycle_sequences(src, docs, args.verbose)
    check_implemented_not_denied(src, docs, args.verbose)
    check_placeholders(docs, args.verbose)
    check_links(docs, args.verbose)
    check_licence(docs, args.verbose)

    print()
    if FAILURES:
        print(f"FAILED — {len(FAILURES)} problem(s) across {len(docs)} document(s):\n")
        for i, f in enumerate(FAILURES, 1):
            print(f"  {i}. {f}\n")
        print("Documentation that misstates the contract is worse than no documentation:")
        print("it invites a reader to trust a claim, and rewards them for checking.\n")
        return 1

    msg = f"PASSED — {CHECKS_RUN} check(s) across {len(docs)} document(s). Docs agree with the contract."
    if BASELINE:
        msg += f"\n         ({len(BASELINE)} known failure(s) waived via scripts/docs_baseline.txt — burn these down.)"
    print(msg)
    return 0


if __name__ == "__main__":
    sys.exit(main())

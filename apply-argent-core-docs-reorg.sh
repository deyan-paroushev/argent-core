#!/usr/bin/env bash
#
# Argent public core (argent-core) — documentation reorganization + five
# canonical front-door documents. Contracts are NOT touched.
#
# WHAT THIS DOES
#   1. Adds five root front-door docs: PRODUCT, IMPLEMENTATION_STATUS,
#      PILOT_PROFILE, SECURITY_AND_PRIVACY, LEGAL_PILOT_CHECKLIST. These state
#      built-vs-funded-next honestly and are the public reviewer front door.
#   2. Moves the existing docs/*.md design material into docs/reference/ (kept
#      as labelled design/reference background, not current product claims),
#      using `git mv` so history is preserved.
#   3. Moves scripts/patches/*.sh into docs/reference/legacy-patches/.
#   4. Rewrites README.md and docs/README.md to point at the five front-door
#      docs and the reference/ tree.
#   5. Updates the docs CI: .github/workflows/docs.yml triggers on the five root
#      docs, and scripts/check_docs.py now recurses docs/ (rglob), includes the
#      root docs, and resolves links relative to each document.
#
# WHAT THIS DOES NOT TOUCH
#   - contracts/ is byte-for-byte unchanged (verified before packaging).
#   - scripts/docs_baseline.txt is unchanged.
#   - No source outside docs/, scripts/check_docs.py, the workflow, and the
#     two READMEs.
#
# HOW TO RUN
#   Put this script and argent-core-docs-reorg.tar.gz in your argent-core repo
#   root (the folder containing contracts/, docs/, scripts/), then:
#       chmod +x apply-argent-core-docs-reorg.sh
#       ./apply-argent-core-docs-reorg.sh
#
set -euo pipefail

TARBALL="argent-core-docs-reorg.tar.gz"

# --- preconditions ----------------------------------------------------------
if [[ ! -d contracts || ! -d docs || ! -f scripts/check_docs.py ]]; then
  echo "ERROR: run this from the argent-core repo root (needs contracts/, docs/, scripts/check_docs.py)." >&2
  echo "       Current dir: $(pwd)" >&2
  exit 1
fi
if [[ ! -f "$TARBALL" ]]; then
  echo "ERROR: $TARBALL not found in $(pwd). Put it next to this script." >&2
  exit 1
fi
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "ERROR: not inside a git work tree. This script uses 'git mv' to preserve history." >&2
  exit 1
fi
if [[ -n "$(git status --porcelain)" ]]; then
  echo "WARNING: working tree is not clean. Commit or stash first so this reorg is its own reviewable commit." >&2
  echo "         Continuing in 3s (Ctrl-C to abort)..." >&2
  sleep 3
fi

mkdir -p docs/reference/legacy-patches

# helper: git mv only if the source still exists at the old path
gmv() {
  local src="$1" dst="$2"
  if [[ -f "$src" ]]; then
    mkdir -p "$(dirname "$dst")"
    git mv -f "$src" "$dst"
    echo "    moved $src -> $dst"
  elif [[ -f "$dst" ]]; then
    echo "    (already at $dst)"
  else
    echo "    WARN: neither $src nor $dst present; skipping" >&2
  fi
}

echo "==> Moving docs/*.md into docs/reference/ (history preserved)"
for base in \
  REVIEWER_QUICKSTART.md argent-architecture.md argent-core-v5-summary.pdf \
  argent-dfns-signing-sequence.md auto-collateralisation-layer.md \
  bank-integration-and-adapter-strategy.md bullion-collateral-reference-architecture.md \
  bullion-collateral-system-design.md capacity-reservation-and-deliverability.md \
  collateral-as-locked-value.md collateral-control.md \
  collateral-eligibility-and-rights-model.md collateral-eligibility-and-risk-policy.md \
  collateral-failure-modes.md commodity-finance-positioning.md \
  confidential-control-and-public-integrity.md credit-control-extension-points.md \
  custodian-as-security-infrastructure.md design-partners.md gold-market-notes.md \
  integration-and-interoperability.md obligation-facility-profile.md \
  physical-collateral-and-trade-finance.md protocol.md \
  reserve-obligation-infrastructure.md selective-disclosure-and-institutional-privacy.md \
  shared-gold-infrastructure-and-argent.md threat-model-and-security-boundaries.md \
  why-gold-secured-operational-credit.md \
  DOCUMENT_STATUS_MATRIX.md TEST_SURFACE_MATRIX.md deployment-and-runbook.md \
  evidence-pack-index.md product-roadmap.md
do
  gmv "docs/$base" "docs/reference/$base"
done

echo "==> Moving POSITIONING_UPDATE.md from repo root into docs/reference/"
gmv "POSITIONING_UPDATE.md" "docs/reference/POSITIONING_UPDATE.md"

echo "==> Moving scripts/patches/*.sh into docs/reference/legacy-patches/"
for base in \
  apply-argent-core-capacity-privacy.sh apply-argent-core-clarifications.sh \
  apply-argent-core-docs.sh apply-argent-core-shared-gold-infra.sh \
  apply-argent-core-whitepaper-aligned.sh
do
  gmv "scripts/patches/$base" "docs/reference/legacy-patches/$base"
done
# remove the now-empty patches dir if git left it
[[ -d scripts/patches ]] && rmdir scripts/patches 2>/dev/null && echo "    removed empty scripts/patches/" || true

echo "==> Laying down new + edited content (front-door docs, indexes, README, CI, check_docs.py)"
# --strip-components=1 drops the argent-core/ prefix so files land at repo root.
# This overwrites the move+edited docs with their final content and creates new files.
tar -xzf "$TARBALL" --strip-components=1
echo "    content extracted."

echo "==> Sanity checks"
grep -q "deyan-paroushev/argent-core" README.md && echo "    README argent-core clone URL: OK"
if grep -rn "deyan-p/argent-core" . --include=*.md 2>/dev/null | grep -v node_modules; then
  echo "    ERROR: a wrong argent-core handle is present (see above)." >&2; exit 1
fi
# no stray duplicate: old flat docs should be gone
STRAY=$(find docs -maxdepth 1 -name '*.md' ! -name 'README.md' | wc -l | tr -d ' ')
echo "    flat docs/*.md remaining besides README (expect 0): $STRAY"
[[ "$STRAY" == "0" ]] || { echo "    ERROR: stray flat docs remain; a move did not complete." >&2; exit 1; }

echo "==> Contracts untouched?"
if git diff --quiet -- contracts && git diff --cached --quiet -- contracts; then
  echo "    contracts/: unchanged (good)"
else
  echo "    ERROR: contracts/ shows changes; this patch must not touch contracts." >&2; exit 1
fi

echo "==> Doc-check gate"
python3 scripts/check_docs.py --verbose
echo "    doc-check: PASSED"

cat <<'GIT'

==> Done. Review, then commit and push:

    git add -A
    git status        # should show renames (R) for the moved docs, new front-door
                      # docs, and modifications to README/docs.yml/check_docs.py
    git commit -m "Reorganize docs into reference/; add five canonical front-door documents"
    git push

The docs CI workflow runs check_docs.py on push; it passed locally above, so
CI should pass too. Contracts are untouched, so no Rust rebuild is required.
GIT

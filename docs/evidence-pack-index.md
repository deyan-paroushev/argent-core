# Evidence Pack Index

**Argent Core V5. Reviewer and design-partner evidence index.**

Status: evidence checklist and index. Fill the placeholders before final submission or public release. This document exists so a reviewer can verify the prototype without relying on the deck narrative.

Argent’s claim is not “trust us.” The claim is: the contracts, tests, testnet transactions, event trail and certificates can be opened and checked.

---

## 1. Current baseline

| Item | Current value |
|---|---|
| Network | Stellar testnet |
| Soroban SDK | `23.5.3` |
| Contract suite | `credit_ledger`, `settlement_vault`, `rewards_ledger` |
| Test count | 224 passing tests: 162 credit ledger, 45 rewards ledger, 17 settlement vault |
| Current reference implementation | Argent Core V5, public Argent Protocol v0.1 |
| Event schemas | `CollateralEventV1`, `GovernanceEventV1` |
| Core boundary | Physical asset remains off-chain and in custody; chain records signed control state, not title transfer. |

---

## 2. Public links

Replace or confirm before final publication.

| Artifact | Link |
|---|---|
| Public repository | `TODO: https://github.com/deyan-paroushev/argent-core` |
| Live demo | `TODO: https://argent-production-4a3f.up.railway.app/demo` |
| Overview deck (if shared) | `TODO` |
| Evidence certificate sample | `TODO` |
| StellarExpert explorer | `https://stellar.expert/explorer/testnet` |

---

## 3. Source and build evidence

| Evidence | Required content | Status |
|---|---|---|
| Git commit | Commit hash matching the tested source | `TODO` |
| Git tag | Release tag if used, e.g. `v5.0.0-testnet` | `TODO` |
| Rust version | `rustc --version` output | `TODO` |
| Stellar CLI version | `stellar --version` output | `TODO` |
| Cargo lockfile | `Cargo.lock` committed or build dependencies otherwise pinned | `TODO` |
| Build command | Exact command used for WASM build/optimize | `TODO` |
| Optimized WASM hashes | SHA-256 for each deployed optimized WASM | `TODO` |
| Contract spec check | `stellar contract inspect` or equivalent output | `TODO` |

Suggested command block:

```bash
cd contracts
rustc --version
stellar --version
cargo build --target wasm32v1-none --release -p credit_ledger
cargo build --target wasm32v1-none --release -p settlement_vault
cargo build --target wasm32v1-none --release -p rewards_ledger
cargo test --workspace
```

---

## 4. Contract deployment evidence

| Contract | Contract ID | What to verify |
|---|---|---|
| `credit_ledger` | `CA5PIUK6ZQZD5CRLKHWUWWFWK6LZATVWUVWR4B6HR3CNAFENZK6JE4GZ` | initialized admin, settlement vault binding, role approvals, lifecycle state. |
| `settlement_vault` | `CB45EGGKMQPINDHAFQRDSBAT4MSFNVSTQODBAGMGUPQH6CHIHCI4WZI5` | bound credit ledger and settlement token. |
| `rewards_ledger` | `TODO` | optional rewards surface, separate from collateral-control invariant. |
| Settlement token | `TODO` | Stellar/Soroban asset used for testnet repayment. |

For a fresh deployment, replace all IDs and regenerate the reference lifecycle.

---

## 5. Reference lifecycle evidence

The reference lifecycle should be run from a clean deployment, with every action linked to a transaction hash.

| Step | Contract | Method | Tx hash | What it proves |
|---:|---|---|---|---|
| 1 | `credit_ledger` | `initialize` | `TODO` | Admin and settlement vault binding established. |
| 2 | `credit_ledger` | `approve_party` | `TODO` | Bank/custodian/processor/valuation/vault roles approved. |
| 3 | `credit_ledger` | `register_instrument` | `TODO` | Reusable instrument registered. |
| 4 | `credit_ledger` | `register_framework` | `TODO` | Legal/control framework created. |
| 5 | `credit_ledger` | `admit_instrument` | `TODO` | Instrument admitted under haircut, max LTV, maintenance treatment. |
| 6 | `credit_ledger` | `register_position` | `TODO` | Lot evidence registered against framework and instrument. |
| 7 | `credit_ledger` | `select_lot_for_collateral` | `TODO` | Owner selects exact lot for collateral use. |
| 8 | `credit_ledger` | `confirm_and_immobilize` | `TODO` | Custodian confirms control / immobilization. |
| 9 | `credit_ledger` | `activate_pledge` | `TODO` | Bank activates pledge. |
| 10 | `credit_ledger` | `open_credit_line` | `TODO` | Borrowing base, haircut, max LTV and valuation reference enforced. |
| 11 | `credit_ledger` | `record_drawdown` | `TODO` | Drawn exposure recorded without exceeding capacity. |
| 12 | `settlement_vault` | `settle_repayment` | `TODO` | Settlement asset moves and exposure reduces atomically. |
| 13 | `credit_ledger` | `bank_authorize_release` | `TODO` | Bank authorizes release only after release conditions. |
| 14 | `credit_ledger` | `custodian_confirm_release` | `TODO` | Custodian confirms release; lot lock returns to free/reusable state. |

Minimum evidence standard: each row must include a transaction hash, ledger number, and the decoded event summary.

---

## 6. Canonical event evidence

Argent uses two event families:

| Event family | Scope | Required proof |
|---|---|---|
| `CollateralEventV1` | Deal lifecycle: framework, position, pledge, line, drawdown, repayment, release, default, enforcement. | Topic marker, map-shaped data, per-framework sequence, event payload, state fold. |
| `GovernanceEventV1` | Authority lifecycle: admin, party approval/revocation, instrument registration/retirement/admission. | Global governance sequence, actor, action, evidence hash, payload. |

Evidence to capture:

```text
TODO: transaction containing CollateralEventV1 #1
TODO: transaction containing CollateralEventV1 repayment event
TODO: transaction containing CollateralEventV1 release-confirmed event
TODO: transaction containing GovernanceEventV1 instrument-registered event
TODO: transaction containing GovernanceEventV1 instrument-admitted event
```

Reviewer check:

- [ ] Sequences are gap-free for the reference lifecycle.
- [ ] Event actor and role match the signer required by the method.
- [ ] Evidence hashes and valuation references are non-zero where required.
- [ ] Events correspond to committed state changes.

---

## 7. Test evidence

Expected output:

```text
credit_ledger:    162 passed
rewards_ledger:    45 passed
settlement_vault:  17 passed
Total:            224 passed, 0 failed
```

Key tests to surface:

| Test | Why it matters |
|---|---|
| `register_position_refuses_instrument_not_admitted` | Instrument eligibility cannot be bypassed. |
| `open_credit_line_refused_when_ltv_exceeds_instrument_ceiling` | Framework treatment controls credit terms. |
| `refuses_double_pledge_of_same_bars` / lot uniqueness equivalent | Same physical lot cannot back two active pledges. |
| `unapproved_vault_cannot_apply_repayment` | Only bound/approved vault can reduce exposure. |
| `refuses_confirm_release_before_bank_authorizes` | Custodian cannot release alone. |
| `repayment_does_not_release_collateral` | Repayment and release remain distinct control acts. |
| `replay_fold_rebuilds_framework_position_pledge_line` | Event stream supports projection/reconstruction. |
| `contract_spec_contains_governance_event_v1` | Governance events are visible in contract spec. |

Attach or link:

```text
TODO: test-summary.txt
TODO: test-summary.pdf
TODO: CI run URL, if available
```

---

## 8. Certificate evidence

The evidence certificate should identify:

- network;
- contract IDs;
- commit hash;
- relevant transaction hashes;
- owner/bank/custodian public addresses or redacted role labels;
- instrument and lot evidence hashes;
- eligibility treatment hash;
- valuation reference;
- pledge state;
- line state;
- repayment state;
- release or enforcement state;
- generation time;
- disclaimer that the certificate records signed control facts, not physical truth.

Certificate samples:

| Certificate | Link | Status |
|---|---|---|
| Certificate of Collateral Evidence | `TODO` | `TODO` |
| Enforcement readiness certificate | `TODO` | `TODO` |
| Settlement repayment proof | `TODO` | `TODO` |

---

## 9. DFNS evidence (current build)

For the DFNS-governed signing layer the current build introduces, capture:

| Evidence | Description | Status |
|---|---|---|
| Role-wallet topology | Bank, custodian, processor/verifier, sponsor, operator, owner path. | `TODO` |
| Method-to-role map | Which Soroban method each DFNS wallet may authorize. | `TODO` |
| Approval trace | DFNS approval ID linked to Soroban tx hash. | `TODO` |
| Release approval | Bank approval and custodian confirmation through DFNS. | `TODO` |
| Enforcement approval | Quorum or stricter policy on enforcement path. | `TODO` |
| Webhook trace | Pending, approved, signed, broadcast, confirmed states. | `TODO` |

This section should remain empty until the DFNS integration is actually implemented. Do not imply current completion.

---

## 10. What must not be included

Never include:

- private keys;
- seed phrases;
- DFNS API keys;
- Railway or service secrets;
- unredacted KYC or client data;
- card PANs or payment credentials;
- unredacted custody agreements or legal documents;
- non-public bank/custodian emails;
- production credentials for RPC or indexer services.

Use role labels and evidence hashes where confidentiality matters.

---

## 11. Pre-submission checklist

Before publishing the repository or sharing with a reviewer or design partner:

- [ ] All contract IDs are current.
- [ ] Test count matches the current repository.
- [ ] Every lifecycle table row has a valid tx hash or is explicitly marked `TODO`.
- [ ] The evidence certificate was regenerated after the latest deployment.
- [ ] Demo URL points to the same network and contract IDs as this document.
- [ ] Public docs distinguish live functionality from roadmap features.
- [ ] No secrets or client-confidential files are linked.
- [ ] Physical truth boundary is stated clearly.

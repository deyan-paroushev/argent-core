# Evidence Pack Index

**Argent Core V5. Reviewer and design-partner evidence index.**

This document exists so a reviewer can verify the prototype without relying on any narrative. Argent's claim is not "trust us." The claim is that the contracts, tests, build artifacts, deployed testnet contracts, event trail, and certificates can be opened and checked. Repo-checkable values are filled in below. On-chain items point to where a reviewer confirms them against the live testnet deployment, which runs the same open-source engine in this repository.

*Build values below were captured from commit `d286d78` with the toolchain listed in section 3. A reviewer regenerating from a different commit or toolchain will get matching WASM hashes only for a byte-identical build.*

---

## 1. Current baseline

| Item | Current value |
|---|---|
| Network | Stellar testnet |
| Soroban SDK | `23.5.3` (workspace-pinned `=23.5.3`) |
| Contract suite | `credit_ledger`, `settlement_vault`, `rewards_ledger` |
| Deployment | `credit_ledger` and `settlement_vault` deployed to testnet; `rewards_ledger` tested, not deployed (separate optional overlay) |
| Test count | 224 passing tests: 162 credit ledger, 45 rewards ledger, 17 settlement vault |
| Current reference implementation | Argent Core V5, public Argent Protocol v0.1 |
| Event schemas | `CollateralEventV1`, `GovernanceEventV1` |
| Core boundary | Physical asset remains off-chain and in custody; chain records signed control state, not title transfer. |

---

## 2. Public links

| Artifact | Link |
|---|---|
| Public repository | https://github.com/deyan-paroushev/argent-core |
| Live demonstrator | https://argent-production-4a3f.up.railway.app/demo |
| StellarExpert (credit_ledger) | https://stellar.expert/explorer/testnet/contract/CA5PIUK6ZQZD5CRLKHWUWWFWK6LZATVWUVWR4B6HR3CNAFENZK6JE4GZ |
| StellarExpert (settlement_vault) | https://stellar.expert/explorer/testnet/contract/CB45EGGKMQPINDHAFQRDSBAT4MSFNVSTQODBAGMGUPQH6CHIHCI4WZI5 |
| Test summary | `docs/argent-core-v5-summary.pdf` (in this repository) |

---

## 3. Source and build evidence

Captured from the tested commit. A reviewer reproduces these with the command block below.

| Evidence | Value |
|---|---|
| Git commit | `d286d78da7f607ff73f607f42e0af742a7185c52` |
| Git tag | none (tracked by commit hash) |
| Rust version | `rustc 1.96.0 (ac68faa20 2026-05-25)` |
| Cargo version | `cargo 1.96.0 (30a34c682 2026-05-25)` |
| Soroban SDK pin | `soroban-sdk = "=23.5.3"` (workspace root `contracts/Cargo.toml`) |
| Build command | `cargo build --target wasm32v1-none --release -p <crate>` (build `credit_ledger` first; the `settlement_vault` tests import its WASM) |

Optimized WASM SHA-256 (release, `wasm32v1-none`):

```text
credit_ledger.wasm      cd65d9998e36b69d167eadba085979e311a094199dc2a1bb8cd4410e6c3d7b6e
settlement_vault.wasm   73a4473a6471c1cc2bbfd8002bd2a64a34d6112cc5e59e72e3ac271cef6c148f
rewards_ledger.wasm     2231838947dd5bbe505c9816efc80d44978ef74ba8df498bdc9e2f19dc4dd618
```

Reproduce from a clean checkout:

```bash
cd contracts
rustc --version
cargo build --target wasm32v1-none --release -p credit_ledger
cargo build --target wasm32v1-none --release -p settlement_vault
cargo build --target wasm32v1-none --release -p rewards_ledger
sha256sum target/wasm32v1-none/release/*.wasm
cargo test --workspace
```

The three SHA-256 values match on any byte-identical build of this commit with the same toolchain.

---

## 4. Contract deployment evidence

| Contract | Contract ID | Verify |
|---|---|---|
| `credit_ledger` | `CA5PIUK6ZQZD5CRLKHWUWWFWK6LZATVWUVWR4B6HR3CNAFENZK6JE4GZ` | Initialized admin, settlement-vault binding, role approvals, lifecycle state, on StellarExpert (link in section 2). |
| `settlement_vault` | `CB45EGGKMQPINDHAFQRDSBAT4MSFNVSTQODBAGMGUPQH6CHIHCI4WZI5` | Bound credit ledger and settlement token, on StellarExpert (link in section 2). |
| `rewards_ledger` | tested, not deployed | Optional rewards overlay, separate from the collateral-control invariant. Present and tested in the repository; not part of the deployed control engine. |

The deployed control engine is `credit_ledger` plus `settlement_vault`. `rewards_ledger` is deliberately out of the deployed surface because it is an optional overlay and carries no collateral-control invariant.

---

## 5. Reference lifecycle evidence

The reference lifecycle runs end to end on testnet through the two deployed contracts, and is exercised live in the demonstrator (section 2). A reviewer verifies each step in one of two ways: by clicking through the demonstrator, which shows the recorded on-chain effect of each step, or by inspecting the contract's transaction history on StellarExpert.

| Step | Contract | Method | What it proves |
|---:|---|---|---|
| 1 | `credit_ledger` | `initialize` | Admin and settlement-vault binding established. |
| 2 | `credit_ledger` | `approve_party` | Bank, custodian, processor, valuation, and vault roles approved. |
| 3 | `credit_ledger` | `register_instrument` | Reusable instrument registered. |
| 4 | `credit_ledger` | `register_framework` | Legal / control framework created. |
| 5 | `credit_ledger` | `admit_instrument` | Instrument admitted under haircut, max LTV, maintenance treatment. |
| 6 | `credit_ledger` | `register_position` | Lot evidence registered against framework and instrument. |
| 7 | `credit_ledger` | `select_lot_for_collateral` | Owner selects the exact lot for collateral use. |
| 8 | `credit_ledger` | `confirm_and_immobilize` | Custodian confirms control / immobilization. |
| 9 | `credit_ledger` | `activate_pledge` | Bank activates the pledge. |
| 10 | `credit_ledger` | `open_credit_line` | Borrowing base, haircut, max LTV, and valuation reference enforced. |
| 11 | `credit_ledger` | `record_drawdown` | Drawn exposure recorded without exceeding capacity. |
| 12 | `settlement_vault` | `settle_repayment` | Settlement asset moves and exposure reduces atomically. |
| 13 | `credit_ledger` | `bank_authorize_release` | Bank authorizes release only after release conditions hold. |
| 14 | `credit_ledger` | `custodian_confirm_release` | Custodian confirms release; the lot lock returns to a free / reusable state. |

To pull the transaction hash, ledger number, and decoded event for any step, read the contract history:

```bash
# Recent transactions touching the credit_ledger contract (browser):
#   https://stellar.expert/explorer/testnet/contract/CA5PIUK6ZQZD5CRLKHWUWWFWK6LZATVWUVWR4B6HR3CNAFENZK6JE4GZ
#
# Or via Horizon against the submitting source account:
curl -s "https://horizon-testnet.stellar.org/accounts/<SOURCE_ACCOUNT>/transactions?order=desc&limit=25" \
  | jq -r '._embedded.records[]? | "\(.created_at)  \(.hash)"'
```

The demonstrator surfaces the same effect per step as a "what was recorded" panel, so a reviewer without CLI access can confirm each state transition visually.

---

## 6. Canonical event evidence

Argent uses two event families:

| Event family | Scope | Proof surface |
|---|---|---|
| `CollateralEventV1` | Deal lifecycle: framework, position, pledge, line, drawdown, repayment, release, default, enforcement. | Topic marker, map-shaped data, per-framework sequence, event payload, state fold. |
| `GovernanceEventV1` | Authority lifecycle: admin, party approval / revocation, instrument registration / retirement / admission. | Global governance sequence, actor, action, evidence hash, payload. |

The events to confirm on the live deployment are the first `CollateralEventV1`, a repayment event, a release-confirmed event, and the `GovernanceEventV1` instrument-registered and instrument-admitted events. Each is visible in the contract's transaction history (section 2) and in the demonstrator's per-step panel.

Reviewer check:

- [ ] Sequences are gap-free for the reference lifecycle.
- [ ] Event actor and role match the signer required by the method.
- [ ] Evidence hashes and valuation references are non-zero where required.
- [ ] Events correspond to committed state changes.

---

## 7. Test evidence

Expected output of `cargo test --workspace`:

```text
credit_ledger:    162 passed
rewards_ledger:    45 passed
settlement_vault:  17 passed
Total:            224 passed, 0 failed
```

The full run is also captured in `docs/argent-core-v5-summary.pdf`. Key tests to surface:

| Test | Why it matters |
|---|---|
| `register_position_refuses_instrument_not_admitted` | Instrument eligibility cannot be bypassed. |
| `open_credit_line_refused_when_ltv_exceeds_instrument_ceiling` | Framework treatment controls credit terms. |
| `refuses_double_pledge_of_same_bars` / lot-uniqueness equivalent | The same physical lot cannot back two active pledges. |
| `unapproved_vault_cannot_apply_repayment` | Only the bound / approved vault can reduce exposure. |
| `refuses_confirm_release_before_bank_authorizes` | Custodian cannot release alone. |
| `repayment_does_not_release_collateral` | Repayment and release remain distinct control acts. |
| `replay_fold_rebuilds_framework_position_pledge_line` | The event stream supports projection / reconstruction. |
| `contract_spec_contains_governance_event_v1` | Governance events are visible in the contract spec. |

---

## 8. Certificate evidence

The evidence certificate, generated in the live application, identifies: network; contract IDs; commit hash; relevant transaction hashes; owner / bank / custodian public addresses or redacted role labels; instrument and lot evidence hashes; eligibility treatment hash; valuation reference; pledge, line, repayment, and release or enforcement state; generation time; and a disclaimer that the certificate records signed control facts, not physical truth.

Certificate types the application produces: a Certificate of Collateral Evidence, an enforcement-readiness certificate, and a settlement-repayment proof. These are generated against a live deployment and are shown in the demonstrator; they are not static files committed to this open-source contract repository.

---

## 9. DFNS evidence (next build, not yet implemented)

The DFNS-governed signing layer is the next build (see `argent-dfns-signing-sequence.md`), not part of the current deployed prototype. The evidence below does not exist yet and must not be implied as complete. It is listed so a reviewer knows exactly what the next phase will produce:

| Evidence | Description |
|---|---|
| Role-wallet topology | Bank, custodian, processor / verifier, sponsor, operator, owner path. |
| Method-to-role map | Which Soroban method each DFNS wallet may authorize. |
| Approval trace | DFNS approval ID linked to Soroban tx hash. |
| Release approval | Bank approval and custodian confirmation through DFNS. |
| Enforcement approval | Quorum or stricter policy on the enforcement path. |
| Webhook trace | Pending, approved, signed, broadcast, confirmed states. |

This section stays empty of results until the DFNS integration is implemented. It is a specification of future evidence, not current evidence.

---

## 10. What must not be included

Never include: private keys; seed phrases; DFNS API keys; Railway or service secrets; unredacted KYC or client data; payment credentials; unredacted custody agreements or legal documents; non-public bank or custodian emails; or production credentials for RPC or indexer services. Use role labels and evidence hashes where confidentiality matters.

---

## 11. Verification checklist

Before sharing the repository with a reviewer or design partner:

- [ ] Contract IDs in section 4 resolve on StellarExpert.
- [ ] Test count matches the current repository and the summary PDF.
- [ ] WASM hashes in section 3 reproduce from the tested commit.
- [ ] The demonstrator points to the same network and contract IDs as this document.
- [ ] Public docs distinguish live functionality from the next-build DFNS features.
- [ ] No secrets or client-confidential files are linked.
- [ ] The physical-truth boundary is stated clearly.

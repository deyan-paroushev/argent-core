# Reviewer quickstart

This repository contains the current Argent Core V5 Soroban engine: three contracts that enforce and evidence control state around physically held collateral.

The **product direction** is broader than the current contract names. Argent is evolving into gold-backed obligation infrastructure: one controlled reserve supporting multiple purpose-bound bank obligations, with no unrestricted customer cash draw. The code in this repository is the tested **secured-credit reference branch** that proves the shared collateral foundation.

Read the product thesis at `reserve-obligation-infrastructure.md` and the target domain model at `obligation-facility-profile.md`. Use this document to verify what is actually implemented today.

> **Do not infer typed guarantees, documentary credits, treasury obligations, or production confidentiality from the current contracts.** Those are target profiles, not a relabeling of shipped code. The current storage and replayable events are publicly inspectable and must use synthetic data.

---

## What V5 proves

1. An asset class is registered once as a reusable instrument and admitted to a framework under explicit eligibility treatment.
2. A position cannot be registered against an unregistered or unadmitted instrument.
3. The same supplied 32-byte `uniqueness_hash` cannot be active under two positions. The current contract does not derive a private bar identity and does not detect the same bar submitted under a different value.
4. A credit line cannot open above the bank-approved instrument ceiling or without margin headroom.
5. Only the bound settlement vault can reduce drawn exposure.
6. Repayment moves settlement value and updates credit exposure atomically.
7. Release requires bank authorization followed by custodian confirmation.
8. Default, cure, enforcement readiness, and enforcement recording follow an enforced order.
9. Deal acts emit canonical `CollateralEventV1` events.
10. Authority acts emit parallel `GovernanceEventV1` events.
11. Contract state can be rebuilt from the event stream.

Items 9-11 describe the transparent reference profile. Their reconstructability is useful verification evidence and also the reason they are not the confidential production event model.

These properties are reusable for the obligation facility because any guarantee, documentary credit, supplier undertaking, or treasury exposure still depends on identified collateral, exclusive allocation, governed exposure, controlled release, and an enforceable adverse path.

---

## What V5 does not yet prove

The current contracts do not yet implement:

- a generic master obligation facility;
- product and subsidiary sublimits;
- beneficiary-specific capacity reservations;
- typed guarantees, documentary credits, accepted instruments, or treasury exposures;
- contingent, pending-claim, and crystallized exposure classes;
- issue, amend, cancel, expire, present, claim, and discharge states;
- the target no-unrestricted-cash-draw invariant;
- electronic trade-document or bank trade-finance adapters;
- available-versus-issuable capacity preflight;
- provisional and committed reservation states;
- idempotent issue callbacks and ambiguous-outcome reconciliation;
- custodian-controlled canonical bar identity and deterministic nullifier derivation;
- confidential state and nullifier-set roots;
- minimized batch anchoring, common relay, uniform event, cadence, padding, and leakage tests;
- role-specific projections, encrypted evidence access, or advanced selective-disclosure proofs.

The repository states these gaps explicitly so reviewers can distinguish demonstrated engineering from the commercial product extension.

---

## Run the suite

The settlement vault imports the compiled `credit_ledger.wasm`, so the credit ledger must be built to WASM before the settlement-vault integration tests compile.

```bash
cd contracts
cargo build --target wasm32v1-none --release -p credit_ledger
cargo test --workspace
```

Expected:

```text
credit_ledger:    162 passed
rewards_ledger:    45 passed
settlement_vault:  17 passed
```

**224 tests, 0 failed**, on soroban-sdk 23.5.3.

If the `wasm32v1-none` target is missing:

```bash
rustup target add wasm32v1-none
```

Then rebuild and rerun the suite.

---

## Run the documentation conformance check

```bash
python3 scripts/check_docs.py --verbose
```

The checker prevents documentation from naming nonexistent contract functions, reversing enforced lifecycle order, describing implemented controls as missing, or linking to absent local documents.

---

## Key tests to inspect

### Eligibility and capacity

- `register_position_refuses_instrument_not_admitted`
- `open_credit_line_refused_when_ltv_exceeds_instrument_ceiling`
- `open_credit_line_refuses_ltv_not_below_maintenance`

### Exclusivity and release

- `refuses_double_pledge_of_same_bars`
- `refuses_second_credit_line_against_same_pledge`
- `refuses_confirm_release_before_bank_authorizes`
- `repayment_does_not_release_collateral`
- `refuses_release_with_outstanding_balance`

### Settlement and exposure

- `unapproved_vault_cannot_apply_repayment`
- `duplicate_payment_ref_does_not_move_tokens_twice`

### Default and enforcement

- `cure_restores_line`
- `refuses_enforce_before_cure_expiry`
- `enforcement_cannot_be_recorded_twice`

### Events and replay

- `batch3_collateral_event_v1_topic_marker_is_pinned`
- `batch4_spec_matches_wire_contract`
- `replay_fold_rebuilds_framework_position_pledge_line`
- `contract_spec_contains_governance_event_v1`

---

## Live V5 contracts - Stellar testnet

- Credit ledger: `CA5PIUK6ZQZD5CRLKHWUWWFWK6LZATVWUVWR4B6HR3CNAFENZK6JE4GZ`
- Settlement vault: `CB45EGGKMQPINDHAFQRDSBAT4MSFNVSTQODBAGMGUPQH6CHIHCI4WZI5`

The public demonstrator and explorer evidence are linked from the root README and `evidence-pack-index.md`.

---

## Why the existing branch matters to the new product

The mature facility changes the commercial use of capacity, not the physical-collateral control requirements.

| Current secured-credit proof | Reuse in obligation facility |
|---|---|
| instrument and lot identity | reserve eligibility and identity |
| identical supplied pledge-key refusal | no duplicate obligation allocation after the target canonical custodian-nullifier control is added |
| borrowing base | approved reserve capacity |
| available limit | free obligation capacity |
| draw utilization | capacity consumed by a bank obligation |
| repayment | reimbursement after bank payment |
| dual-control release | release after all obligations are discharged |
| default and enforcement | adverse path after unpaid reimbursement |
| canonical transparent event stream | reference evidence pattern; production uses confidential projections plus minimized batch anchors |

The next design step preserves the authorization, release, and enforcement foundation while adding canonical custodian identity, confidential state anchoring, and obligation lifecycles. It must not reuse the transparent event projection with real facility data.

---

## Where to read next

- `DOCUMENT_STATUS_MATRIX.md` - the scope and status of every document.
- `reserve-obligation-infrastructure.md` - the current product thesis.
- `obligation-facility-profile.md` - the target technical model.
- `capacity-reservation-and-deliverability.md` - the target reservation, issuability, callback, and reconciliation model.
- `selective-disclosure-and-institutional-privacy.md` - the target data-visibility and evidence-disclosure model.
- `confidential-control-and-public-integrity.md` - the production public/private state boundary, nullifier profile, batch anchor, and leakage gates.
- `argent-architecture.md` - the full architecture and boundary.
- `argent-dfns-signing-sequence.md` - the institutional signing model.
- `protocol.md` - the protocol specification and current reference profile.
- `TEST_SURFACE_MATRIX.md` - test coverage by risk surface.

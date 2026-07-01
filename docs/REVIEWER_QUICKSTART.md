# Reviewer quickstart

This repository is the Argent Core V5 contract engine: three Soroban contracts
that enforce the control state around physically-held collateral. The gold stays
in the vault. The control over it goes on chain.

Argent Core V5 is the current Soroban reference implementation of the public Argent Protocol v0.1 (see `protocol.md`). It uses `CollateralEventV1` and `GovernanceEventV1` as the first stable event schemas.

## What V5 proves

1. An asset class is registered once as a reusable instrument, then admitted to a framework as eligible collateral under an explicit treatment (haircut, maximum advance rate, maintenance threshold). A position cannot be registered against an unregistered or unadmitted instrument.
2. The same lot cannot back two active pledges.
3. Only the bound settlement vault can reduce a drawn balance.
4. Repayment moves token settlement and updates credit exposure together.
5. Release requires bank authorization and then custodian confirmation.
6. Deal acts emit a canonical, replayable CollateralEventV1; authority acts (instrument registration, admission, party and admin changes) emit a CollateralEventV1-parallel GovernanceEventV1.
7. Both event schemas are inspectable through the contract spec.

## Run the suite

The settlement vault imports the compiled `credit_ledger.wasm`, so the credit
ledger must be built to WASM before the settlement-vault integration tests can
compile. Build it once, then run the workspace suite:

```bash
cd contracts
cargo build --target wasm32v1-none --release -p credit_ledger
cargo test --workspace
```

Expected:

```
credit_ledger:    162 passed
rewards_ledger:    45 passed
settlement_vault:  17 passed
```

224 tests, 0 failed, on soroban-sdk 23.5.3.

If the `wasm32v1-none` target is missing, add it with
`rustup target add wasm32v1-none` and rebuild.

## Key tests to inspect

The guarantees above are each pinned by a test. To read the proof directly:

- `register_position_refuses_instrument_not_admitted`
- `open_credit_line_refused_when_ltv_exceeds_instrument_ceiling`
- `refuses_double_pledge_of_same_bars`
- `unapproved_vault_cannot_apply_repayment`
- `refuses_confirm_release_before_bank_authorizes`
- `repayment_does_not_release_collateral`
- `batch3_collateral_event_v1_topic_marker_is_pinned`
- `batch4_spec_matches_wire_contract`
- `replay_fold_rebuilds_framework_position_pledge_line`
- `contract_spec_contains_governance_event_v1`

The first two are the V5 instrument-eligibility gate. The next four are the
institutional impossibilities. The last four show that the canonical event
streams are well-formed and that contract state can be rebuilt from events alone.

## Live V5 contracts (Stellar testnet)

The engine in this repository is deployed and exercised end to end on testnet:

- Credit ledger: `CA5PIUK6ZQZD5CRLKHWUWWFWK6LZATVWUVWR4B6HR3CNAFENZK6JE4GZ`
- Settlement vault: `CB45EGGKMQPINDHAFQRDSBAT4MSFNVSTQODBAGMGUPQH6CHIHCI4WZI5`

Any act can be verified on the public explorer at stellar.expert.

## Where to read next

- `docs/TEST_SURFACE_MATRIX.md` — what each surface covers and the risk it addresses.
- `docs/argent-architecture.md` — how the three contracts fit together.
- `docs/argent-dfns-signing-sequence.md` — the production signing model: each party signs with its own authority; the contract enforces roles, the signer enforces who can sign.
- `docs/protocol.md` — the protocol specification.

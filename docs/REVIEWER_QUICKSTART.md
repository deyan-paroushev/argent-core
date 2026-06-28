# Reviewer quickstart

This repository is the Argent Core V4 contract engine: three Soroban contracts
that enforce the control state around physically-held collateral. The gold stays
in the vault. The control over it goes on chain.

## What V4 proves

1. The same bars cannot back two active pledges.
2. Only the bound settlement vault can reduce a drawn balance.
3. Repayment moves token settlement and updates credit exposure together.
4. Release requires bank authorization and then custodian confirmation.
5. Every lifecycle act emits a canonical, replayable CollateralEventV1.
6. The event schema is inspectable through the contract spec.

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
credit_ledger:    125 passed
rewards_ledger:    45 passed
settlement_vault:  17 passed
```

187 tests, 0 failed, on soroban-sdk 23.5.3.

If the `wasm32v1-none` target is missing, add it with
`rustup target add wasm32v1-none` and rebuild.

## Key tests to inspect

The guarantees above are each pinned by a test. To read the proof directly:

- `refuses_double_pledge_of_same_bars`
- `unapproved_vault_cannot_apply_repayment`
- `refuses_confirm_release_before_bank_authorizes`
- `repayment_does_not_release_collateral`
- `batch3_collateral_event_v1_topic_marker_is_pinned`
- `batch4_spec_matches_wire_contract`
- `replay_fold_rebuilds_framework_position_pledge_line`

The first four are the institutional impossibilities. The last three show that
the canonical event stream is well-formed and that contract state can be rebuilt
from events alone.

## Live V4 contracts (Stellar testnet)

The engine in this repository is deployed and exercised end to end on testnet:

- Credit ledger: `CD7RTIYSKMKOCTLQ2QXE5JVOPUITWA2K25ZZDJ6KO746RJZUCJX5BPZD`
- Settlement vault: `CB45EGGKMQPINDHAFQRDSBAT4MSFNVSTQODBAGMGUPQH6CHIHCI4WZI5`

Any act can be verified on the public explorer at stellar.expert.

## Where to read next

- `docs/TEST_SURFACE_MATRIX.md` — what each surface covers and the risk it addresses.
- `docs/argent-architecture.md` — how the three contracts fit together.
- `docs/argent-dfns-signing-sequence.md` — the production signing model: each party signs with its own authority; the contract enforces roles, the signer enforces who can sign.
- `docs/protocol.md` — the protocol specification.

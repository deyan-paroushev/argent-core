# Argent Core — asset-agnostic collateral-control contracts for Soroban

Open-source Soroban smart contracts for governing **physically-backed collateral**
as on-chain, role-signed state. This is the reusable core extracted from
[Argent](https://argent-production-4a3f.up.railway.app), a tri-party secured-credit
application on Stellar.

The asset stays in the vault. Only the **control and authorization** over it
become programmable on Soroban. Nothing here tokenizes, custodies, or moves the
physical asset.

## What this is

A holder pledges a custodied physical asset; a bank opens a secured credit line
against it; the asset is never sold or moved. The pledge, the borrowing base,
utilization, repayment, release, and the enforcement workflow are recorded on
Soroban as **role-signed state transitions** between parties who do not fully
trust each other — owner, bank, custodian.

The contracts are **asset-agnostic by design**. The governing structure carries
no commodity. Argent runs on allocated gold; the same core binds copper, iron,
or lead at the leaf fields, with no change to the structure.

## Contracts

| Crate | Role |
|-------|------|
| `credit_ledger` | The tri-party control framework, bar-set uniqueness, position state, credit line, margin, release, default, and enforcement lifecycle. Signer-agnostic. |
| `settlement_vault` | Atomic repay-and-reduce: the borrower repays in a Stellar settlement asset and the credit exposure reduces in a single transaction. |
| `rewards_ledger` | Sponsor-funded, non-transferable rewards tied to eligible posted spend and verified claims. Separate from pledged collateral. |

The core data structure — `ControlFramework` in `credit_ledger` — binds three
parties and six document hashes (facility agreement, pledge agreement, custody
agreement, eligible-collateral schedule, margin policy, enforcement waterfall).
There is no commodity field in it. That is what makes it reusable across
physical-collateral use cases.

## Status

These contracts are **deployed and tested on Stellar testnet**, with a passing
test suite across all three crates. They are real Soroban contracts — `require_auth`,
emitted events, and one atomic value transfer in `settlement_vault` — not
arbitrary on-chain storage.

**What is funded, not yet built:** the institutional **DFNS authorization layer**
that sits on top of these contracts — DFNS role wallets, deny-by-default approval
policies, the Soroban signer adapter, and the pending-state / reconciliation
machinery — is the deliverable of an in-progress Stellar Community Fund
Integration Track build, alongside a mainnet launch. The design for that layer is
documented in [`docs/argent-dfns-signing-sequence.md`](docs/argent-dfns-signing-sequence.md).
The contracts here are deliberately **signer-agnostic** so any institutional
signing layer — DFNS or otherwise — can govern them.

## Build and test

```bash
cd contracts
cargo test --workspace        # run the contract test suite
cargo build --release         # build the wasm (opt-level z, panic=abort)
```

Requires the Rust toolchain and the `wasm32-unknown-unknown` target. The
contracts target `soroban-sdk` 22.

## Documentation

- [`docs/argent-architecture.md`](docs/argent-architecture.md) — the full system
  architecture: contracts, lifecycle, the tri-party model, and why Soroban.
- [`docs/argent-dfns-signing-sequence.md`](docs/argent-dfns-signing-sequence.md) —
  the planned DFNS authorization layer and the Soroban signing sequence.

## What is not here

This repository is the reusable contract core only. The proprietary application
around it — the TypeScript collateral service, the React cockpit, deployment
configuration, and any institutional integration credentials — is not part of
this open-source core and is not required to build, test, or reuse the contracts.

## License

Apache License 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

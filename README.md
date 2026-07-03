# Argent Core: asset-agnostic collateral-control contracts for Soroban

Open-source Soroban smart contracts for governing **physically-backed collateral**
as on-chain, role-signed state. This is the reusable core extracted from
[Argent](https://argent-production-4a3f.up.railway.app), a tri-party secured-credit
application on Stellar.

The asset stays in the vault. Only the **control and authorization** over it
become programmable on Soroban. Nothing here tokenizes, custodies, or moves the
physical asset.

Argent started with allocated gold because it is the cleanest institutional
proof case. The core is not a gold product. It is a commodity-collateral control
layer for physical assets that remain in professional custody: eligibility,
pledge, borrowing base, utilization, repayment, release, default, and
enforcement. The commodity-finance positioning is set out in
[`docs/commodity-finance-positioning.md`](docs/commodity-finance-positioning.md).

These contracts are the first reference implementation of the
[Argent Protocol](docs/protocol.md), an open specification for expressing
physical-collateral control as a signed, ordered, verifiable event chain. The
durable contribution is the protocol; this Soroban implementation is the first
proof of it.

## What this is

A holder pledges a custodied physical asset; a bank opens a secured credit line
against it; the asset is never sold or moved. The pledge, the borrowing base,
utilization, repayment, release, and the enforcement workflow are recorded on
Soroban as **role-signed state transitions** between parties who do not fully
trust each other, owner, bank, custodian.

The contracts are **asset-agnostic by design**. The governing structure carries
no commodity. Argent runs on allocated gold; the same core binds other
custody-stable assets (base metals, critical minerals, warehouse-held
commodities) at the leaf fields, with no change to the structure.

The instrument and eligibility model is shaped by the **ISDA Common Domain
Model (CDM)** collateral-criteria / treatment taxonomy, and informed by Daml
Finance's holding decomposition. An instrument is registered once as a reusable
asset class; a framework then admits it as eligible collateral under an explicit
treatment, the CDM "GC basket" pattern, recording the haircut, maximum advance
rate, and maintenance threshold the bank applies. Modeling the instrument layer
on a recognized financial-industry taxonomy is what makes the asset-agnosticism
principled rather than ad hoc: the same eligibility and treatment shape carries
across commodities, which is the basis for interoperability with systems that
already speak CDM.

## Contracts

| Crate | Role |
|-------|------|
| `credit_ledger` | The tri-party control framework, asset-lot uniqueness, position state, credit line, margin, release, default, and enforcement lifecycle. Signer-agnostic. |
| `settlement_vault` | Atomic repay-and-reduce: the borrower repays in a Stellar settlement asset and the credit exposure reduces in a single transaction. |
| `rewards_ledger` | Sponsor-funded, non-transferable rewards tied to eligible posted spend and verified claims. Separate from pledged collateral. |

The core data structure, `ControlFramework` in `credit_ledger`, binds three
parties and six document hashes (facility agreement, pledge agreement, custody
agreement, eligible-collateral schedule, margin policy, enforcement waterfall).
There is no commodity field in it. That is what makes it reusable across
physical-collateral use cases.

## Status

These contracts are **deployed and tested on Stellar testnet**, with **224
passing tests** across all three crates (`credit_ledger` 162, `rewards_ledger`
45, `settlement_vault` 17). They are real Soroban contracts: `require_auth`, a
typed canonical `CollateralEventV1` event stream for deal acts and a
`GovernanceEventV1` stream for authority acts (instrument registration,
eligibility admission, party and admin changes), and one atomic value transfer
in `settlement_vault`. Not arbitrary on-chain storage.

The full run is captured in
[`docs/argent-core-v5-summary.pdf`](docs/argent-core-v5-summary.pdf): 224
passing, 0 failed, with the V5 instrument-registry, governance-event, and
admit-eligibility guarantees listed by test name. It is reproducible from the
build-and-test steps below.

**The next build, not yet built:** the institutional **DFNS authorization layer**
that sits on top of these contracts, DFNS role wallets, deny-by-default approval
policies, the Soroban signer adapter, and the pending-state / reconciliation
machinery, is the focus of the next integration build toward a mainnet
launch. The design for that layer is
documented in [`docs/argent-dfns-signing-sequence.md`](docs/argent-dfns-signing-sequence.md).
The contracts here are deliberately **signer-agnostic** so any institutional
signing layer, DFNS or otherwise, can govern them.

In this demo, one operator holds local keys for all parties so the lifecycle
runs end to end. In production each party (bank, custodian, owner, processor)
holds its own signing authority through DFNS under its own approval policies;
Argent assembles the transaction but holds no party's keys and can sign for no
one. The contract enforces which role may perform each act; DFNS enforces that
only the real party can produce that act's signature. Same lifecycle, same
contract, decentralized authority.

## Build and test

```bash
cd contracts
cargo build --target wasm32v1-none --release -p credit_ledger  # build first: settlement_vault tests import this wasm
cargo test --workspace                                         # run the contract test suite
```

Expected: `credit_ledger` 162, `rewards_ledger` 45, `settlement_vault` 17, for
224 passing, 0 failed.

Requires the Rust toolchain and the `wasm32v1-none` target
(`rustup target add wasm32v1-none`). The contracts target `soroban-sdk` 23.5.3.
The `settlement_vault` integration tests import the compiled `credit_ledger`
wasm, so the credit ledger must be built before the workspace test run.

New reviewers should start with
[`docs/REVIEWER_QUICKSTART.md`](docs/REVIEWER_QUICKSTART.md): what V5 proves, the
key tests to inspect, and the live testnet contract ids.

## Documentation

The `docs/` folder is organized so you can read in the order that matches what
you want to know. A suggested path:

**Start here to understand what Argent is and why it exists**

- [`docs/collateral-control.md`](docs/collateral-control.md): the collateral-control
  thesis, the bank pain points it addresses, and the institutional grounding. The
  clearest single entry point to the idea.
- [`docs/custodian-as-security-infrastructure.md`](docs/custodian-as-security-infrastructure.md):
  why the custodian is the physical root of trust the whole control record anchors
  to, what allocated custody and the bar list actually provide, how the structure
  mirrors triparty repo, and where the boundary sits between what the custodian
  attests and what Argent records. Sourced notes, not a pitch.
- [`docs/physical-collateral-and-trade-finance.md`](docs/physical-collateral-and-trade-finance.md):
  a neutral primer on the market this sits in, who is affected, how physical
  collateral is financed, and where the category is heading. Background reading,
  not a pitch.
- [`docs/collateral-as-locked-value.md`](docs/collateral-as-locked-value.md):
  an evidence-led reading of the market signals, why the binding constraint in
  financing physical commodities is the missing control instrument rather than
  asset quality, why the market reaches for tokenization and why that is the
  wrong tool for physical goods, and why the timing is now. The market-signal
  case for why Argent exists, applicable across commodities, not just gold.
- [`docs/commodity-finance-positioning.md`](docs/commodity-finance-positioning.md):
  how the current engine reads as a commodity-collateral control layer rather
  than a gold-only product, anchored in the international legal and standards
  direction (MLETR, the 2024 UNCITRAL-UNIDROIT Model Law on Warehouse Receipts,
  ICC DSI). Why not only gold, why not tokenization, which workflows the engine
  already maps to, and which commodities come next.

**Verify that it works**

- [`docs/REVIEWER_QUICKSTART.md`](docs/REVIEWER_QUICKSTART.md): what V5 proves, the
  key tests to inspect, and the live testnet contract ids. The fastest way to
  confirm the claims are real.
- [`docs/TEST_SURFACE_MATRIX.md`](docs/TEST_SURFACE_MATRIX.md): what each tested
  surface covers and the risk it addresses.
- [`docs/evidence-pack-index.md`](docs/evidence-pack-index.md): a single index of
  everything a reviewer can independently verify, contracts, tests, transactions,
  events, and certificates.

**Understand how it is built**

- [`docs/protocol.md`](docs/protocol.md): the Argent Protocol public draft, the
  open, event-sourced specification for controlling and proving the lifecycle of
  physical collateral that remains in custody. Start here for the conceptual
  model, the event and evidence design, the state machines, and the
  allocated-gold adapter.
- [`docs/argent-architecture.md`](docs/argent-architecture.md): the full system
  architecture, contracts, lifecycle, the tri-party model, and why Soroban.
- [`docs/argent-dfns-signing-sequence.md`](docs/argent-dfns-signing-sequence.md):
  the DFNS authorization layer and the Soroban signing sequence.

**Assess safety and operations**

- [`docs/threat-model-and-security-boundaries.md`](docs/threat-model-and-security-boundaries.md):
  what Argent protects against, what it deliberately does not, and the trust
  boundaries. Read this to understand the edges of what the contracts enforce.
- [`docs/deployment-and-runbook.md`](docs/deployment-and-runbook.md): how the
  contracts are built, deployed, initialized, verified, and exercised end to end.

**Direction and engagement**

- [`docs/go-to-market.md`](docs/go-to-market.md): the business context around the
  engine, who the product is built for, why allocated gold is the first asset,
  where the first commercial conversations are most credible, and how the engine
  expands. Read this to understand the market and the customer, not the code.
- [`docs/gold-market-notes.md`](docs/gold-market-notes.md): background research on
  the gold market the engine works in, how much gold exists and who holds it,
  whether lending against it is already common, and where institutional collateral
  infrastructure is heading. Sourced notes, not a pitch.
- [`docs/product-roadmap.md`](docs/product-roadmap.md): the product direction
  beyond the current build, framed so the open core points further than any single
  application without enlarging what ships today.
- [`docs/design-partners.md`](docs/design-partners.md): an invitation to
  institutions that want to shape the product against their real operational
  needs, and how to start a conversation.

## What is not here

This repository is the reusable contract core only. The proprietary application
around it, the TypeScript collateral service, the React cockpit, deployment
configuration, and any institutional integration credentials, is not part of
this open-source core and is not required to build, test, or reuse the contracts.

## License

Apache License 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

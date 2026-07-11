# Argent Core

**Asset-agnostic collateral-control contracts for Soroban.**

The asset stays in the vault. Only the **control** over it becomes programmable. Nothing here tokenizes, custodies, or moves the physical asset.

A holder pledges a custodied physical asset. A bank opens a secured credit line against it. The asset is never sold or moved. Pledge, borrowing base, drawdown, repayment, release, default, and enforcement are recorded on Soroban as **role-signed state transitions between parties who do not fully trust each other** — owner, bank, custodian.

Argent runs on allocated gold because it is the cleanest institutional proof case. **The core is not a gold product.** The governing structure carries no commodity; the same contracts bind other custody-stable assets (base metals, critical minerals, warehouse-held commodities) at the leaf fields, with no change to the structure.

These contracts are the first reference implementation of the [Argent Protocol](docs/protocol.md), an open specification for expressing physical-collateral control as a signed, ordered, verifiable event chain. **The durable contribution is the protocol; this Soroban implementation is the first proof of it.**

---

## Verify it in five minutes

Every claim below is one you can check yourself. That is the point of the repository.

```bash
git clone https://github.com/deyan-paroushev/argent-core
cd argent-core
cargo test --manifest-path contracts/Cargo.toml     # 224 tests
python3 scripts/check_docs.py                       # docs match the contract
```

### Four properties worth checking first

Each is an **invariant enforced in code**, not a procedure that could be skipped under pressure. Each is one grep away.

| Property | Where | If you try to violate it |
|---|---|---|
| **The custodian cannot release metal before the bank authorises it.** | `custodian_confirm_release` | The transaction is **rejected** unless the pledge is already in `ReleaseAuthorized`. Not "should not" — *cannot*. |
| **The same bars cannot back two pledges.** | `uniqueness_hash` on the position | Uniqueness is keyed at the **lot**, not the position. This is the warehouse-receipt duplicate-financing control. |
| **A credit line cannot open at its own margin-call level.** | `open_credit_line` | The contract enforces `ltv_bps < maintenance_bps`. Every facility starts with headroom by construction. |
| **Repayment and exposure reduction are atomic.** | `settlement_vault` | The settlement asset moves and the drawn balance falls in one transaction, or neither does. |

Then read **[`docs/REVIEWER_QUICKSTART.md`](docs/REVIEWER_QUICKSTART.md)**.

---

## What is on chain, and what is not

The boundary is the design, and it is deliberately narrow.

**On chain:** the control state. Who pledged, who immobilised, who authorised release, who confirmed it, when the line was revalued and against what price reference, who declared default.

**Off chain:** the metal, the title, the credit exposure, the bar serials, the KYC, the legal documents. Sensitive detail is referenced by hash, never stored.

**What these contracts do not do:** create the security interest (the pledge agreement does that), convey ownership, value the collateral independently of the lender, or execute enforcement. They record and enforce the *contractually authorised control state*. Where the record and the legal documents disagree, **the documents govern.**

---

## Why Stellar, specifically

Three protocol features do work no off-chain database can, which is the test that matters:

1. **Multi-party authorisation.** `require_auth` binds each state transition to the party that performed it. Owner, bank, and custodian each sign only their own act. The record is *evidential*, not merely informational.
2. **Atomic settlement (SEP-41).** Repayment and exposure reduction occur in one transaction. A conventional payment rail cannot bind a value transfer to a collateral-state change atomically; Soroban can.
3. **Shared, verifiable state.** The parties and an auditor read one role-signed control record, instead of reconciling four private systems by hand.

Without Stellar, Argent is another private collateral database.

---

## Contracts

| Contract | Role |
|---|---|
| `credit_ledger` | The control core. Instrument registry, eligibility, pledge lifecycle, borrowing base, margin, adjustment, default, enforcement. |
| `settlement_vault` | Atomic repayment. The settlement asset and the drawn balance move together, or not at all. |
| `rewards_ledger` | Optional overlay. Not required by the control model. |

The instrument and eligibility model is shaped by the **ISDA Common Domain Model (CDM)** collateral taxonomy. An instrument is registered once as a reusable definition; a position references it. Full function map and actor model in [`docs/argent-architecture.md`](docs/argent-architecture.md).

---

## Documentation

**Three documents matter.** Everything else is reference, kept for anyone who needs to dig.

| | |
|---|---|
| **[`REVIEWER_QUICKSTART.md`](docs/REVIEWER_QUICKSTART.md)** | **Start here.** What the contracts prove, and how to check each claim. |
| **[`argent-architecture.md`](docs/argent-architecture.md)** | Architecture, actor model, function map, trust boundaries. **Authoritative** wherever it and any other document disagree. |
| **[`protocol.md`](docs/protocol.md)** | The Argent Protocol specification. |

<details>
<summary><b>Reference documentation</b> — domain model, risk policy, integration, roadmap</summary>

<br>

**Domain model**

- [`bullion-collateral-reference-architecture.md`](docs/bullion-collateral-reference-architecture.md) — twelve requirements that bullion makes of *any* system controlling it as collateral. Vendor-neutral, with a conformance checklist.
- [`bullion-collateral-system-design.md`](docs/bullion-collateral-system-design.md) — representation taxonomy, lifecycle state machines, integration architecture.
- [`collateral-eligibility-and-rights-model.md`](docs/collateral-eligibility-and-rights-model.md) — **not all gold is collateral.** Why a holding's legal rights must be classified before it can enter a borrowing base. *Specifies a gap; see "What is not built".*
- [`collateral-control.md`](docs/collateral-control.md) · [`collateral-as-locked-value.md`](docs/collateral-as-locked-value.md) · [`custodian-as-security-infrastructure.md`](docs/custodian-as-security-infrastructure.md)

**Credit policy and risk**

- [`credit-control-extension-points.md`](docs/credit-control-extension-points.md) — controls a lender may ask for that the contract does **not** enforce, what each would take, and which should **never** be built.
- [`collateral-eligibility-and-risk-policy.md`](docs/collateral-eligibility-and-risk-policy.md) · [`collateral-failure-modes.md`](docs/collateral-failure-modes.md) · [`threat-model-and-security-boundaries.md`](docs/threat-model-and-security-boundaries.md)

**Positioning**

- [`why-gold-secured-operational-credit.md`](docs/why-gold-secured-operational-credit.md) · [`commodity-finance-positioning.md`](docs/commodity-finance-positioning.md) · [`physical-collateral-and-trade-finance.md`](docs/physical-collateral-and-trade-finance.md) · [`gold-market-notes.md`](docs/gold-market-notes.md)

**Integration and operations**

- [`bank-integration-and-adapter-strategy.md`](docs/bank-integration-and-adapter-strategy.md) · [`argent-dfns-signing-sequence.md`](docs/argent-dfns-signing-sequence.md) · [`integration-and-interoperability.md`](docs/integration-and-interoperability.md) · [`auto-collateralisation-layer.md`](docs/auto-collateralisation-layer.md) · [`deployment-and-runbook.md`](docs/deployment-and-runbook.md) · [`evidence-pack-index.md`](docs/evidence-pack-index.md)

**Testing and roadmap**

- [`TEST_SURFACE_MATRIX.md`](docs/TEST_SURFACE_MATRIX.md) · [`design-partners.md`](docs/design-partners.md) · [`product-roadmap.md`](docs/product-roadmap.md)

</details>

---

## What is not built

Stated on the front page rather than buried, because a reviewer will find it anyway and it is better to hear it from us.

- **The rights gate.** The contract cannot yet reject a holding that is unallocated, non-transferable, or consent-gated — it would accept a *claim on a bank* as though it were metal. Specified in [`collateral-eligibility-and-rights-model.md`](docs/collateral-eligibility-and-rights-model.md) §9. **This is the highest-value item on the collateral-model roadmap**, because it gates every control downstream of it.
- **Pre-call draw suspension.** Suspension *on* a margin call is automatic. A distinct threshold *before* the call is not implemented.
- **A named commercial triangle.** The engine is proven. A pilot with a real borrower, a real custodian, and a real lender is not yet signed. That is the honest state.

---

## Documentation cannot drift

`scripts/check_docs.py` runs in CI on every push. It **fails the build** when a document names a contract function that does not exist, states a lifecycle sequence the contract does not enforce, describes an implemented control as missing, or claims a licence the repository does not carry.

It exists because it was needed. Documents in this repository once described a cure window as unimplemented when the contract enforces one. **Documentation that misstates the contract is worse than none** — it invites a reader to trust a claim, and rewards them for checking.

```bash
python3 scripts/check_docs.py --verbose
```

---

## Status

Stellar testnet. **224 tests.** Three contracts, two deployed. Full lifecycle exercised end to end in a [live demonstrator](https://argent-production-4a3f.up.railway.app).

Apache-2.0 — see [`LICENSE`](LICENSE).

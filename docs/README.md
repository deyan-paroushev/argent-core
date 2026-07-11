# Documentation

Engineering and domain documentation for Argent Core.

**Start at the [root README](../README.md)** for the five-minute verification path. This index is for readers who need depth.

> **Implementation baseline.** Documents that make claims about contract behaviour record the commit they were verified against in their status header. Contract code evolves; a document naming a function is only as good as the commit it was checked at. `scripts/check_docs.py` enforces this in CI.

---

## Core

| Document | Purpose |
|---|---|
| [`REVIEWER_QUICKSTART.md`](REVIEWER_QUICKSTART.md) | What the contracts prove, and how to check each claim yourself. |
| [`argent-architecture.md`](argent-architecture.md) | Architecture, actor model, function map, on-chain/off-chain boundary. **Authoritative** wherever it and another document disagree. |
| [`protocol.md`](protocol.md) | The Argent Protocol: physical-collateral control as a signed, ordered, verifiable event chain. |
| [`TEST_SURFACE_MATRIX.md`](TEST_SURFACE_MATRIX.md) | What the 224 tests cover. |

## Domain model

| Document | Purpose |
|---|---|
| [`bullion-collateral-reference-architecture.md`](bullion-collateral-reference-architecture.md) | **Specification.** Twelve requirements bullion makes of *any* system controlling it as collateral, vendor-neutral, with a conformance checklist. Answers: what does correctness mean here? |
| [`bullion-collateral-system-design.md`](bullion-collateral-system-design.md) | **Build plan.** Representation taxonomy, product profiles, lifecycle state machines, integration architecture, roadmap. Answers: what should be constructed, in what order? |
| [`collateral-eligibility-and-rights-model.md`](collateral-eligibility-and-rights-model.md) | The rights gate. **Not all gold is collateral** — a holding's legal rights must be classified before it can enter a borrowing base. **Specifies a gap, not shipped code** (§9). |
| [`collateral-control.md`](collateral-control.md), [`collateral-as-locked-value.md`](collateral-as-locked-value.md), [`custodian-as-security-infrastructure.md`](custodian-as-security-infrastructure.md) | The control model and the custodian's role. |

## Credit policy and risk

| Document | Purpose |
|---|---|
| [`credit-control-extension-points.md`](credit-control-extension-points.md) | Controls a lender may ask for that the contract does **not** enforce today, what each would take, and which should **never** be built because they are determinations a bank makes rather than computations a contract can perform. |
| [`collateral-eligibility-and-risk-policy.md`](collateral-eligibility-and-risk-policy.md) | Eligibility and risk treatment. |
| [`collateral-failure-modes.md`](collateral-failure-modes.md) | How this fails, and what the contract does about it. |
| [`threat-model-and-security-boundaries.md`](threat-model-and-security-boundaries.md) | What the ledger proves, what it does not, and what it rests on. |

## Positioning

| Document | Purpose |
|---|---|
| [`why-gold-secured-operational-credit.md`](why-gold-secured-operational-credit.md) | Why borrowing against allocated gold rather than selling it is an established institutional market, and what the operating-infrastructure gap is. |
| [`commodity-finance-positioning.md`](commodity-finance-positioning.md) | The core is asset-agnostic. Where else it binds. |
| [`physical-collateral-and-trade-finance.md`](physical-collateral-and-trade-finance.md) | Relationship to warehouse-receipt and trade finance. |
| [`gold-market-notes.md`](gold-market-notes.md) | Market reference. |

## Integration and operations

| Document | Purpose |
|---|---|
| [`bank-integration-and-adapter-strategy.md`](bank-integration-and-adapter-strategy.md) | How a bank connects to this. |
| [`argent-dfns-signing-sequence.md`](argent-dfns-signing-sequence.md) | Institutional key governance. |
| [`integration-and-interoperability.md`](integration-and-interoperability.md), [`auto-collateralisation-layer.md`](auto-collateralisation-layer.md) | Interop surfaces. |
| [`deployment-and-runbook.md`](deployment-and-runbook.md) | Deployment. |
| [`evidence-pack-index.md`](evidence-pack-index.md) | Evidence and audit artefacts. |
| [`design-partners.md`](design-partners.md), [`product-roadmap.md`](product-roadmap.md) | Where this is going. |

---

## The boundary, across every document

Custody stays with the custodian. Ownership stays with the owner. Credit exposure stays with the lender. Argent records and enforces the authorised control state — it does not create the security interest, does not tokenize the metal, does not value the collateral independently of the lender, and does not execute enforcement.

Where any document and [`argent-architecture.md`](argent-architecture.md) touch, **the architecture document is authoritative.**

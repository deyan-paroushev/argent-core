# Document status matrix

This matrix prevents a reader from confusing three different things:

1. **what the current Soroban contracts implement;**
2. **what the mature Argent product is intended to become;**
3. **what later research or extension paths may add.**

The repository deliberately keeps the tested secured-credit reference branch because it proves the shared collateral engine. The current product direction is broader: a non-cash-drawable reserve obligation facility. Documents are classified below so the two are not presented as the same implementation state.

## Status labels

| Label | Meaning |
|---|---|
| **Canonical direction** | Current product and protocol positioning. Use this language externally. |
| **Implemented reference** | Describes behavior present in the current contracts or testnet system. |
| **Target profile** | Designed extension built on the implemented reference; not yet shipped. |
| **Domain specification** | Asset, rights, risk, custody, or evidence requirements independent of one product profile. |
| **Integration strategy** | How the protocol sits beside bank, custodian, signing, and document systems. |
| **Research extension** | Later possibility; not the current product commitment. |
| **Legacy filename, updated content** | Filename retained for link stability; content is aligned to the current direction. |

---

## Canonical product and protocol direction

| Document | Status | What it establishes |
|---|---|---|
| `reserve-obligation-infrastructure.md` | **Canonical direction** | One controlled reserve supports many purpose-bound bank obligations while operating cash remains available. |
| `obligation-facility-profile.md` | **Target profile** | Domain objects, states, invariants, no-cash-draw rule, and mapping to the current contracts. |
| `argent-architecture.md` | **Canonical direction + implemented reference** | Full system architecture and the boundary between the live credit branch and target obligation profile. |
| `protocol.md` | **Protocol specification + implemented reference** | Event-sourced physical-reserve control protocol; V0.1 currently implements the secured-credit profile. |
| `product-roadmap.md` | **Canonical direction** | Sequenced path from tested contracts to institutional signing, typed obligations, interoperability, and selective disclosure. |

---

## Verification and current implementation

| Document | Status | What it establishes |
|---|---|---|
| `REVIEWER_QUICKSTART.md` | **Implemented reference** | Fast verification of the 224-test contract engine and live testnet deployment. |
| `TEST_SURFACE_MATRIX.md` | **Implemented reference** | Test coverage for the secured-credit reference branch. |
| `deployment-and-runbook.md` | **Implemented reference** | Deployment and operating procedure for the current contracts. |
| `evidence-pack-index.md` | **Implemented reference** | Available evidence, certificates, source, and verification artifacts. |
| `argent-core-v5-summary.pdf` | **Implemented reference** | Test-result summary; not a product-positioning document. |

---

## Institutional signing and interoperability

| Document | Status | What it establishes |
|---|---|---|
| `argent-dfns-signing-sequence.md` | **Integration strategy** | DFNS permissions, policies, approvals, asynchronous signing, role separation, and reconciliation. |
| `bank-integration-and-adapter-strategy.md` | **Integration strategy** | Bank sidecar and adapter model across limits, trade finance, treasury, custody, accounting, and evidence. |
| `integration-and-interoperability.md` | **Integration strategy** | Ledger-neutral coexistence with authoritative institutional systems. |
| `auto-collateralisation-layer.md` | **Research extension** | Funded just-in-time liquidity extension; not the primary obligation-first product. |

---

## Asset, custody, rights, and risk

| Document | Status | What it establishes |
|---|---|---|
| `bullion-collateral-reference-architecture.md` | **Domain specification** | Vendor-neutral correctness requirements for bullion under collateral control. |
| `bullion-collateral-system-design.md` | **Domain specification + target profiles** | System build plan; secured credit is the current reference profile, obligation capacity is the primary target profile. |
| `collateral-eligibility-and-rights-model.md` | **Domain specification** | Legal-rights gate before valuation and capacity. |
| `collateral-eligibility-and-risk-policy.md` | **Domain specification** | Eligibility, valuation, haircuts, concentration, exposure, margin, substitution, and escalation. |
| `collateral-control.md` | **Domain specification** | Programmable control patterns for reserve release, substitution, sale, settlement, and enforcement. |
| `collateral-as-locked-value.md` | **Domain and positioning** | Why physical reserves need instrumentation before they can support bank products. |
| `custodian-as-security-infrastructure.md` | **Domain specification** | Custodian control as the physical root of trust. |
| `collateral-failure-modes.md` | **Domain and risk evidence** | Documented failure record and the invariants that address it. |
| `threat-model-and-security-boundaries.md` | **Implemented reference + target boundary** | Assets, actors, trust assumptions, and protocol limitations. |

---

## Trade finance and commercial positioning

| Document | Status | What it establishes |
|---|---|---|
| `commodity-finance-positioning.md` | **Canonical direction** | Market category and differentiated value proposition for reserve-backed obligations. |
| `physical-collateral-and-trade-finance.md` | **Domain and market analysis** | How reserves, bank undertakings, documents, and settlement interact in trade finance. |
| `why-gold-secured-operational-credit.md` | **Legacy filename, updated content** | Why gold is useful as a parallel assurance reserve; funded credit is treated as one reference branch, not the primary product. |
| `gold-market-notes.md` | **Market reference** | Gold-market context; not the product definition. |
| `design-partners.md` | **Canonical direction** | Priority bank, custodian, bullion, trade-finance, and corporate design-partner profiles. |

---

## Extension and engineering planning

| Document | Status | What it establishes |
|---|---|---|
| `credit-control-extension-points.md` | **Legacy filename, updated content** | Controls for the current funded branch and broader obligation facility, with shipped versus proposed status. |
| `evidence-pack-index.md` | **Implemented reference** | Evidence artifacts and missing production evidence. |
| `product-roadmap.md` | **Canonical direction** | Product sequencing and explicit non-goals. |

---

## Reading order by audience

### Investor, accelerator, or ecosystem reviewer

1. `../README.md`
2. `reserve-obligation-infrastructure.md`
3. `argent-architecture.md`
4. `product-roadmap.md`
5. `REVIEWER_QUICKSTART.md`

### Bank, trade-finance, or treasury reviewer

1. `reserve-obligation-infrastructure.md`
2. `obligation-facility-profile.md`
3. `bank-integration-and-adapter-strategy.md`
4. `collateral-eligibility-and-risk-policy.md`
5. `physical-collateral-and-trade-finance.md`

### Custodian or bullion operator

1. `bullion-collateral-reference-architecture.md`
2. `custodian-as-security-infrastructure.md`
3. `collateral-control.md`
4. `obligation-facility-profile.md`

### Technical reviewer

1. `REVIEWER_QUICKSTART.md`
2. `argent-architecture.md`
3. `protocol.md`
4. `argent-dfns-signing-sequence.md`
5. `TEST_SURFACE_MATRIX.md`

---

## Governing interpretation

Where product-positioning language differs, apply this order:

1. `reserve-obligation-infrastructure.md` for the market and product direction;
2. `obligation-facility-profile.md` for the target facility model;
3. `argent-architecture.md` for system boundaries and implementation mapping;
4. `protocol.md` for current protocol and contract behavior;
5. the contract source and tests for what is actually implemented.

No document may convert a target design into a claim of shipped functionality. The source code and tests remain the implementation ground truth.

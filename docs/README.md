# Documentation

Engineering, protocol, market, risk, and integration documentation for Argent Core.

**Start at the [root README](../README.md).** The repository now distinguishes clearly between:

- the **mature product direction** - reserve-backed, purpose-bound bank obligations;
- the **implemented reference branch** - the tested secured-credit lifecycle;
- later **research and interoperability extensions**.

Read [`DOCUMENT_STATUS_MATRIX.md`](DOCUMENT_STATUS_MATRIX.md) before using a document as evidence of shipped functionality.

> **Implementation baseline.** Documents that make claims about current contract behavior are checked against the source. `scripts/check_docs.py` fails when named functions, lifecycle order, implemented controls, references, or licensing claims drift from the repository.

---

## Start here

| Document | Purpose |
|---|---|
| [`reserve-obligation-infrastructure.md`](reserve-obligation-infrastructure.md) | **Canonical product direction.** One reserve, many bank obligations, one authoritative capacity state. |
| [`obligation-facility-profile.md`](obligation-facility-profile.md) | **Target facility specification.** Non-cash-drawable capacity, typed obligations, sublimits, claims, reimbursement, and release. |
| [`capacity-reservation-and-deliverability.md`](capacity-reservation-and-deliverability.md) | **Capacity orchestration.** Available versus issuable capacity, atomic reservation, preflight, callbacks, and reconciliation. |
| [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md) | **Institutional privacy.** Role-specific views, minimized ledger state, encrypted evidence, and selective disclosure. |
| [`DOCUMENT_STATUS_MATRIX.md`](DOCUMENT_STATUS_MATRIX.md) | **Scope map.** Distinguishes live implementation, target profile, domain specification, and research. |
| [`REVIEWER_QUICKSTART.md`](REVIEWER_QUICKSTART.md) | **Implementation proof.** Verify the 224-test Soroban reference branch. |
| [`argent-architecture.md`](argent-architecture.md) | **Architecture.** Actors, trust boundaries, contract map, Stellar and DFNS integration, and product evolution. |
| [`protocol.md`](protocol.md) | **Protocol specification.** Event-sourced control of physical reserves, with the secured-credit profile implemented today. |

---

## Product and market direction

| Document | Purpose |
|---|---|
| [`commodity-finance-positioning.md`](commodity-finance-positioning.md) | Positioning Argent as gold-backed obligation infrastructure rather than a general cash-credit product. |
| [`physical-collateral-and-trade-finance.md`](physical-collateral-and-trade-finance.md) | How reserve control, bank undertakings, trade documents, and settlement fit together. |
| [`why-gold-secured-operational-credit.md`](why-gold-secured-operational-credit.md) | Legacy filename retained for link stability; now explains why gold can support obligations while fiat remains available. |
| [`design-partners.md`](design-partners.md) | Priority design partners, pilot profiles, and what each participant must validate. |
| [`gold-market-notes.md`](gold-market-notes.md) | Market reference only; not the product definition. |

---

## Architecture, domain, and protocol

| Document | Purpose |
|---|---|
| [`capacity-reservation-and-deliverability.md`](capacity-reservation-and-deliverability.md) | Reservation and deliverability model for converting eligible reserve value into usable bank capacity. |
| [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md) | Data classification, evidence privacy, role projections, and disclosure products. |
| [`bullion-collateral-reference-architecture.md`](bullion-collateral-reference-architecture.md) | Vendor-neutral correctness requirements for bullion under collateral control. |
| [`bullion-collateral-system-design.md`](bullion-collateral-system-design.md) | Representation, lifecycle, product profiles, and build plan. |
| [`collateral-eligibility-and-rights-model.md`](collateral-eligibility-and-rights-model.md) | Rights classification before a holding may enter a bank capacity calculation. |
| [`collateral-control.md`](collateral-control.md) | Control patterns for pledge, substitution, sale, settlement, release, and enforcement. |
| [`collateral-as-locked-value.md`](collateral-as-locked-value.md) | Why physical reserves require instrumentation before they can support bank products. |
| [`custodian-as-security-infrastructure.md`](custodian-as-security-infrastructure.md) | Custodian control as the physical root of trust. |

---

## Risk and security

| Document | Purpose |
|---|---|
| [`collateral-eligibility-and-risk-policy.md`](collateral-eligibility-and-risk-policy.md) | Eligibility, valuation, haircut, concentration, exposure, margin, substitution, and escalation. |
| [`collateral-failure-modes.md`](collateral-failure-modes.md) | Documented physical-collateral losses and the control invariants they imply. |
| [`threat-model-and-security-boundaries.md`](threat-model-and-security-boundaries.md) | Protected assets, adversaries, trust assumptions, and explicit non-goals. |
| [`credit-control-extension-points.md`](credit-control-extension-points.md) | Legacy filename; distinguishes current funded controls from target obligation controls and later candidates. |

---

## Institutional signing, bank integration, and interoperability

| Document | Purpose |
|---|---|
| [`argent-dfns-signing-sequence.md`](argent-dfns-signing-sequence.md) | DFNS permissions, policy gates, approval quorums, asynchronous signing, and reconciliation. |
| [`bank-integration-and-adapter-strategy.md`](bank-integration-and-adapter-strategy.md) | Sidecar integration with bank limits, trade-finance, treasury, custody, accounting, and evidence systems. |
| [`integration-and-interoperability.md`](integration-and-interoperability.md) | Ledger-neutral coexistence and authoritative-system boundaries. |
| [`auto-collateralisation-layer.md`](auto-collateralisation-layer.md) | Later funded-liquidity extension; not the primary obligation-first product. |

---

## Verification, evidence, deployment, and roadmap

| Document | Purpose |
|---|---|
| [`TEST_SURFACE_MATRIX.md`](TEST_SURFACE_MATRIX.md) | Test counts and security surfaces. |
| [`evidence-pack-index.md`](evidence-pack-index.md) | Source, testnet, certificate, production-evidence, privacy-classification, and disclosure-evidence map. |
| [`deployment-and-runbook.md`](deployment-and-runbook.md) | Current contract deployment plus clearly marked target reservation, reconciliation, privacy, and service-level operations. |
| [`product-roadmap.md`](product-roadmap.md) | Sequenced evolution from the live reference branch to governed obligations and interoperability. |

---

## Governing rule

For current behavior, the source code and tests govern. For product direction, `reserve-obligation-infrastructure.md` governs. For the target facility model, `obligation-facility-profile.md` governs. For reservation and issuability, `capacity-reservation-and-deliverability.md` governs. For data visibility and disclosure, `selective-disclosure-and-institutional-privacy.md` governs. No document should be read as claiming that typed bank obligations, deliverability orchestration, or advanced privacy proofs are already implemented.

# Argent positioning update

**Date:** 19 July 2026

Argent has matured from a credit-line-first presentation into **gold-backed obligation infrastructure**.

> **One reserve. Many obligations. One authoritative capacity state.**

The target product is a bank-operated, non-cash-drawable master facility under which customer-owned allocated bullion supports purpose-bound guarantees, documentary credits, supplier undertakings, accepted obligations, treasury exposures, and related commercial instruments. Operating fiat remains available for operations, reimbursement, fees, margin, and final settlement.

The code in this repository remains a valid and tested secured-credit reference branch. It proves the shared collateral substrate: identity, custody confirmation, exclusive pledge, eligibility, valuation, capacity, authorization, settlement-linked exposure reduction, controlled release, default, cure, and enforcement evidence.

This repository update therefore does three things:

1. makes the mature product direction canonical;
2. labels the current contracts honestly as the implemented reference profile;
3. specifies the target obligation-facility extension without claiming it is already built.

The repository remains fund-neutral and may be shared with banks, custodians, accelerators, investors, implementation partners, and open-source reviewers.

## July 2026 refinement - capacity orchestration and institutional privacy

The product direction was refined again without changing the implemented reference branch. The core now distinguishes **available capacity** from **issuable and operationally deliverable capacity**. A target obligation request must pass product, beneficiary, jurisdiction, evidence, approval, and external-system checks; reserve capacity atomically; and reconcile the authoritative bank-product result before capacity is released or reused.

The repository also makes institutional privacy a first-class control surface. Shared protocol state is minimized, restricted evidence remains encrypted and role-bound, and any selective-disclosure mechanism must state what it proves and what remains authoritative off-chain. The new canonical specifications are:

- `docs/capacity-reservation-and-deliverability.md`;
- `docs/selective-disclosure-and-institutional-privacy.md`.

These refinements strengthen Argent as a reserve-capacity orchestration layer beside existing bank and custody systems. They are target-profile specifications, not claims that the current Soroban contracts already implement typed obligations, callbacks, or advanced privacy proofs.


## July 2026 refinement - shared gold infrastructure boundary

Independent review of the World Gold Council's Gold247 programme, Wholesale Digital Gold and Pooled Gold Interests, the LBMA Gold Bar Integrity rollout, and the March 2026 Gold as a Service proposal confirms a layered market direction:

- provenance and bar integrity;
- legal ownership and entitlement;
- shared custody, issuance, reconciliation, assurance, and redemption infrastructure;
- bank-specific encumbrance, capacity reservation, and obligation lifecycle.

Argent occupies the final layer. It should consume authoritative reserve assertions from upstream gold infrastructure while remaining authoritative for facility capacity and obligation state. It must not create a duplicate gold token, ownership record, or shadow bar registry.

The repository now adds `docs/shared-gold-infrastructure-and-argent.md` and aligns architecture, reserve profiles, interoperability, privacy, eligibility, capacity, market, custodian, bank-adapter, and roadmap documents. The first profile remains allocated, individually identified bullion. Pooled and digital-gold interests are later candidate profiles requiring separate bank, legal, custody, operator, insolvency, and enforcement analysis.

No World Gold Council, LBMA, Gold Bar Integrity, Pooled Gold Interests, Wholesale Digital Gold, Standard Gold Unit, or Gold as a Service affiliation or integration is claimed. No current contract or mainnet-delivery commitment has changed.


## July 2026 refinement - full Gold as a Service assurance boundary

Review of the complete World Gold Council and BCG white paper adds a narrower and more useful boundary than the press release alone:

- Gold as a Service is intended as an upstream, issuer-facing platform rather than a consumer product;
- issuers retain the customer proposition, commercial terms, pricing, distribution, interface, and implementation;
- the platform's stated assurance relates to physical gold and legal entitlements;
- market-level fungibility depends on economic, legal, and operational equivalence;
- the platform is intended to enable third-party products and services above the common foundation;
- the paper's funded collateral example validates the current secured-credit reference branch but does not supply a bank-obligation lifecycle.

The core now preserves four explicit gates: reserve verified, legally pledgeable, operationally controllable, and facility issuable. It also separates potentially fungible reserve units from specific, purpose-bound and non-transferable facility rights.

Canonical documents, adapters, privacy rules, runbooks, threat controls, evidence requirements, market notes, and the roadmap have been aligned. No current contract, mainnet-delivery commitment, World Gold Council relationship, or production integration is claimed.

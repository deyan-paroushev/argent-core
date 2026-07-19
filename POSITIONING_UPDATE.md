# Argent positioning update

**Date:** 18 July 2026

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

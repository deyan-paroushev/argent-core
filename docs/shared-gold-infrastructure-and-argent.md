# Shared Gold Infrastructure and Argent

**Status:** Market and interoperability analysis. This document describes how Argent can coexist with emerging gold-market infrastructure. It does not claim a partnership, endorsement, production integration, or current support for pooled or tokenised gold.

---

## 1. Purpose

The gold market is developing shared infrastructure for provenance, ownership, custody coordination, assurance, reconciliation, and digital product operation. Argent should not duplicate those functions. It should consume authoritative reserve information from them and remain authoritative only for the bank-facility state they do not provide:

- security-interest and control status;
- bank-approved eligibility treatment;
- available and reserved capacity;
- product, applicant, beneficiary, and purpose restrictions;
- bank-obligation lifecycle;
- claims, reimbursement, release, default, and enforcement evidence.

> **Gold infrastructure proves the reserve. Argent governs what the bank may issue against it.**

---

## 2. Independently verified market direction

### 2.1 Gold Bar Integrity - provenance and market data

The LBMA and World Gold Council Gold Bar Integrity programme is building structured infrastructure for bar provenance, refiner assurance, and vault reporting. The GBI Database has onboarded all Good Delivery refiners. Voluntary periodic Country of Origin reporting launched in April 2026, mandatory monthly reporting is planned for 2027, and London custodians are expected to begin aggregated vault-holdings reporting by December 2026. LBMA states an intent to move toward bar-level custodian reporting after that.

This strengthens the upstream evidence environment. It does not establish a bank pledge, prove that a specific customer owns an unencumbered position, or calculate facility capacity.

### 2.2 Wholesale Digital Gold and Pooled Gold Interests - ownership and transfer

The World Gold Council, Linklaters, and Hilltop Walk Consulting have proposed Pooled Gold Interests as a third wholesale holding structure beside allocated and unallocated gold. The concept is intended to combine beneficial ownership of pooled vaulted gold with fractional transferability and broader collateral use.

PGI is a proposed legal and market structure, not an allocated-bar position and not automatically equivalent to the first Argent profile. A bank would need a separate eligibility, control, insolvency, valuation, and enforcement analysis before admitting it.

### 2.3 Gold as a Service - shared digital-gold operating infrastructure

The World Gold Council and Boston Consulting Group have proposed Gold as a Service as shared infrastructure connecting physical gold operations with digital product issuance and lifecycle management. The issuer would remain responsible for product terms, pricing, distribution, brand, and customer relationships. The shared platform is intended to coordinate physical custody and movement, digital issuance and transfers, reconciliation, compliance controls, assurance, liquidity, and redemption.

The design is organised around three layers:

1. a physical layer for sourcing, vaulting, inventory, insurance, audit, logistics, and redemption;
2. a digital layer for product issuance, balances or tokens, transfers, reporting, and lifecycle management;
3. a connecting layer for synchronisation, reconciliation, control, and assurance between physical and digital records.

The World Gold Council describes the platform as being developed with industry input. Its published current scope is separate from Pooled Gold Interests, although the paper notes that a later evolution could leverage PGI. It is therefore an emerging upstream infrastructure initiative, not a live Argent dependency.

### 2.4 Argent - encumbrance, capacity, and bank obligations

Argent sits above the reserve system and below the bank-product system:

```text
Authoritative gold record
    -> ownership, custody, provenance, backing, quantity

Argent
    -> security interest, eligibility treatment, capacity, reservation,
       obligation allocation, reimbursement, release, enforcement

Bank product system
    -> guarantee, documentary credit, undertaking, treasury exposure
```

Argent does not need to become a digital-gold issuer to benefit from better gold infrastructure.

---

## 3. Functional map

| Layer | Primary question | Candidate authority | Argent treatment |
|---|---|---|---|
| Provenance | Where did the gold come from, and does it satisfy market-integrity rules? | Refiner, assurance provider, LBMA GBI, or another approved source | Ingest signed evidence or a reference; do not recreate global provenance. |
| Ownership and entitlement | What legal interest does the customer hold? | Custodian, account provider, title register, PGI or digital-gold operator, legal documents | Classify under a bank-approved `ReserveProfile`; reject unsupported rights. |
| Physical backing and custody | Does the gold exist, where is it, and who controls release? | Custodian, vault, approved shared-gold platform | Treat the physical authority as superior to the ledger mirror. |
| Digital product operation | What digital units, balances, transfers, or redemption rights exist? | Gold as a Service operator or product issuer | Reference only when relevant; do not duplicate units. |
| Security interest | Is the reserve validly encumbered for the bank? | Legal agreement, control agent, bank, custodian | Record the approved control state and evidence references. |
| Facility capacity | How much value may support bank products? | Bank policy and risk systems | Record and enforce the bank-approved treatment. |
| Reservation | Which proposed obligation has consumed capacity? | Argent shared state | Argent is authoritative for protocol reservation state. |
| Bank obligation | What has the bank issued, amended, claimed, paid, or discharged? | Bank trade-finance or treasury system | Reconcile authoritative callbacks and maintain the shared lifecycle. |

No single ledger should claim authority over every row.

---

## 4. Reserve profiles

The first implementation remains allocated, individually identified bullion. The domain model should nevertheless distinguish the source and legal character of a reserve.

```text
ReserveProfile
- ALLOCATED_BAR_SET
- POOLED_GOLD_INTEREST
- DIGITAL_GOLD_ENTITLEMENT
```

### 4.1 `ALLOCATED_BAR_SET`

Required characteristics:

- specific bars or a specific segregated lot;
- direct ownership or another bank-approved proprietary right;
- authoritative bar list and custody account;
- custodian control over release;
- uniqueness and existing-encumbrance checks;
- enforceable security-interest path.

This is the first Argent profile and the only profile proven by the current reference implementation.

### 4.2 `POOLED_GOLD_INTEREST`

Candidate characteristics:

- beneficial ownership in an identified pool;
- clear insolvency treatment and proprietary rights;
- authoritative unit and pool records;
- enforceable control or account-blocking mechanism;
- allocation, redemption, transfer, and realisation rules;
- no hidden double issuance or conflicting security interest.

PGI or similar structures may eventually fit this profile. They are not accepted merely because they are described as physically backed.

### 4.3 `DIGITAL_GOLD_ENTITLEMENT`

Candidate characteristics:

- a token, account balance, certificate, or comparable digital entitlement;
- legal mapping to physical gold and a defined redemption or realisation right;
- authoritative supply, ownership, custody, and backing records;
- transfer, freeze, pledge, and enforcement controls appropriate to the jurisdiction;
- continuous reconciliation within stated tolerances;
- bank-approved technology, operator, and insolvency risk.

This profile must distinguish a proprietary gold interest from an unsecured claim on an issuer.

### 4.4 Common fields

A future reserve adapter should return at least:

```text
reserve_profile
legal_owner_or_entitlement_holder
authoritative_record_id
custodian_or_account_provider
quantity_and_unit
purity_or_economic_gold_equivalent
location_or_governing_market
backing_or_allocation_status
redemption_or_realisation_rights
existing_encumbrance_status
evidence_timestamp
policy_eligibility_status
```

The adapter result is evidence for a bank decision. It does not itself create a pledge.

---

## 5. Shared-gold infrastructure adapter

A future adapter should be read-first and authority-preserving.

### 5.1 Inputs

- reserve identifier and profile;
- owner or entitlement-holder identifier;
- custody or product-account identifier;
- quantity, quality, and valuation inputs;
- provenance and assurance references;
- physical-backing or allocation status;
- redemption, transfer, freeze, and control capabilities;
- existing liens, holds, or encumbrance status where available;
- evidence version and timestamp.

### 5.2 Outcomes

```text
VERIFIED
VERIFIED_WITH_LIMITATIONS
STALE
DISCREPANCY
UNSUPPORTED_PROFILE
UNSUPPORTED_OPERATOR
OWNERSHIP_UNCONFIRMED
BACKING_UNCONFIRMED
CONTROL_UNAVAILABLE
ENCUMBRANCE_UNKNOWN
```

### 5.3 Rules

1. A `VERIFIED` reserve may still be ineligible under bank policy.
2. A stale, discrepant, or unsupported reserve fails closed for new reservation and issuance.
3. An upstream discrepancy does not automatically release collateral or erase an active obligation.
4. Active exposures move into exception, margin, cure, substitution, or enforcement handling according to the facility documents and bank instruction.
5. Argent stores the minimum evidence needed to prove the decision and correlation, not a duplicate full bar list or customer ledger.
6. A token or digital account may not be mirrored onto Stellar as a second competing asset representation.

---

## 6. Continuous reconciliation

Argent should reconcile a different chain of facts from a digital-gold product operator:

```text
authoritative reserve quantity
<-> bank-eligible quantity
<-> pledged or controlled quantity
<-> approved facility capacity
<-> provisional and committed reservations
<-> active obligations
<-> crystallised exposure
```

| Class | Comparison | Failure treatment |
|---|---|---|
| Custody | Authoritative reserve record versus Argent reserve reference | Stop new use; open discrepancy; require custodian resolution. |
| Rights | Owner, entitlement, lien, and control evidence versus approved profile | Stop new use; legal or control review. |
| Valuation | Price, quantity, haircut, and freshness versus current policy | Recompute capacity; margin, cure, or suspend as required. |
| Capacity | Facility limit versus reservations and active exposure | Reject over-allocation; investigate replay or adapter error. |
| Bank product | Argent obligation state versus bank-product status | Preserve reservation during ambiguity; reconcile before release. |
| Approval | Institutional approval versus Soroban action and evidence | Quarantine unmatched or unauthorised actions. |

A mismatch is an event to manage, not an excuse to let one system silently overwrite another.

---

## 7. Privacy and disclosure

Shared infrastructure does not mean public infrastructure. Bar lists, ownership records, reserve values, customer balances, beneficiary identities, and control documents may be visible to the relevant operator, bank, custodian, auditor, or supervisor while remaining unavailable to other facility participants and the public.

Argent should prefer:

- signed reserve assertions over copied source data;
- purpose-bound access to detailed evidence;
- commitments and versioned references on the shared ledger;
- role-specific projections;
- sufficiency and control statements that reveal no more than the recipient needs;
- disclosure logs showing who received what and why.

A shared bar list inside a trusted gold platform is not a public bar list and should not be treated as one.

---

## 8. Product and market implications

### 8.1 Complement, do not compete

Gold as a Service is intended to let issuers launch and operate digital gold products. Argent is the bank-facility layer that can use a bank-approved reserve to support guarantees, documentary credits, supplier undertakings, or treasury exposures.

### 8.2 Stronger product boundary

> **Gold-market infrastructure establishes the reserve. Argent establishes the bank-usable capacity and obligation state.**

### 8.3 Potential distribution path

Future gold-product operators, bullion banks, custodians, and digital commodity infrastructure providers may become reserve-data providers, custody or assurance partners, design partners, distribution channels, or users of the Argent bank-obligation control profile.

### 8.4 Competitive risk

If upstream platforms expand into lending, encumbrance, capacity, and obligation management, basic reserve evidence may become commoditised. Argent must therefore own the harder bank-specific layer:

- facility and sublimit policy;
- purpose-bound reservation;
- beneficiary and product controls;
- claims and reimbursement;
- release and enforcement;
- bank, custodian, and corporate role governance;
- deterministic reconciliation with bank systems.

---

## 9. Implementation sequence

### Phase 1 - explicit reserve-source metadata

Add profile and authoritative-source fields to the target model and private read model. No new asset type is admitted.

### Phase 2 - signed upstream reserve assertion

Consume one signed custodian or gold-platform assertion in the preflight and evidence flow.

### Phase 3 - reconciliation and discrepancy handling

Implement freshness, tolerance, discrepancy, exception, and fail-closed controls.

### Phase 4 - one non-bar reserve profile

Only after bank and legal validation, support either a pooled proprietary interest or a qualifying digital entitlement as a separate profile.

### Phase 5 - ecosystem adapter

Build a production adapter only when an upstream platform exposes a stable interface, authority model, legal basis, and design partner.

These phases are outside the current secured-credit reference implementation and current mainnet-delivery commitment unless separately approved.

---

## 10. Communication guidance

### Customer-facing

> **Keep the gold. Let the bank issue the promise.**

### Institutional

> Argent converts verified reserve information into controlled, reserved, and reusable bank capacity without creating another gold token.

### Ecosystem

> Gold infrastructure proves the reserve. Argent governs what the bank may issue against it.

### Required disclaimer

References to World Gold Council, LBMA, Gold Bar Integrity, Wholesale Digital Gold, Pooled Gold Interests, Standard Gold Unit, or Gold as a Service are independent market references. No affiliation, integration, endorsement, or production dependency is implied.

---

## 11. Non-goals

Argent does not:

- issue digital gold;
- define a standard unit of gold;
- create PGI or another ownership structure;
- replace GBI provenance or responsible-sourcing systems;
- operate vault, insurance, logistics, liquidity, or redemption networks;
- promise fungibility between legally different gold products;
- treat every physically backed token or account as equivalent collateral;
- make an upstream platform authoritative for bank underwriting or Argent obligation state;
- claim an integration before a stable interface and design partner exist.

---

## 12. References

1. World Gold Council and Boston Consulting Group, **Digital Gold: The Case for a Shared Infrastructure**, 19 March 2026. https://www.gold.org/goldhub/research/digital-gold-case-shared-infrastructure
2. World Gold Council, **Gold as a Service: A proposition for the industry**, 19 March 2026. https://www.gold.org/goldhub/research/digital-gold-case-shared-infrastructure/gold-as-a-service
3. World Gold Council, **Vision: The future of digital gold**, 19 March 2026. https://www.gold.org/goldhub/research/digital-gold-case-shared-infrastructure/vision
4. World Gold Council, **World Gold Council to Develop Shared Infrastructure for Digital Gold**, 19 March 2026. https://www.gold.org/news-and-events/press-releases/world-gold-council-develop-shared-infrastructure-digital-gold
5. World Gold Council, Linklaters, and Hilltop Walk Consulting, **Pooled Gold Interests and Wholesale Digital Gold - A New Vision for the Gold Market**, 2025. https://www.gold.org/what-we-do/gold247/linklaters-white-paper
6. World Gold Council, **Gold247**, including Gold Bar Integrity, Wholesale Digital Gold, and Standard Gold Unit. https://www.gold.org/what-we-do/gold247
7. LBMA, **Gold Bar Integrity Ecosystem**. https://www.lbma.org.uk/gold-bar-integrity-ecosystem
8. LBMA, **Gold Bar Integrity: Periodic Reporting Launches**, 14 April 2026. https://www.lbma.org.uk/articles/gold-bar-integrity-periodic-reporting-launches

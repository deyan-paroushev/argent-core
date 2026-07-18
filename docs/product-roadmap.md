# Argent: Product Roadmap

**From tested secured-credit reference branch to reserve-backed obligation infrastructure.**

**Status:** product sequencing document; only Stage 0 is fully implemented in this repository  
**Primary target:** non-cash-drawable master facility for bank-issued obligations  
**First reserve asset:** allocated physical gold  

> **One reserve. Many obligations. One authoritative capacity state.**

---

## 1. How to read this roadmap

This roadmap separates three layers:

1. **protocol foundation** - physical reserve identity, control, capacity, release, and adverse-path evidence;
2. **institutional governance** - permissions, policies, approvals, signing, and reconciliation;
3. **bank obligation products** - guarantees, documentary credits, undertakings, treasury exposure, claims, reimbursement, and discharge.

The current contracts prove the first layer through a secured-credit profile. They do not yet implement the complete third layer.

The roadmap is ordered to preserve evidence, safety, and reviewability. It does not begin by renaming `credit_ledger` or adding many product types. It first institutionalizes the signing and evidence path, then generalizes the domain model, then adds obligations one family at a time.

---

## 2. Product destination

The destination is a bank-operated master facility under which:

- the customer owns allocated gold held by an approved custodian;
- the bank holds a valid security and control arrangement;
- the bank applies eligibility, valuation, haircut, concentration, and product policy;
- the facility exposes free, reserved, contingent, crystallized, and released capacity;
- every use is tied to an approved product, purpose, amount, beneficiary, and term;
- unused capacity cannot be drawn as unrestricted customer cash;
- the bank issues the actual guarantee, documentary credit, undertaking, or treasury product;
- Argent records the shared capacity, authorization, evidence, reimbursement, release, and enforcement state.

The public market category is:

# **Gold-backed obligation infrastructure**

The broader protocol category is:

# **Reserve orchestration infrastructure**

---

## 3. Stage 0 - implemented reference foundation

**Status: built and testable.**

### Contracts

- `credit_ledger`;
- `settlement_vault`;
- `rewards_ledger` as a separate optional overlay.

### Proven controls

- instrument registration and framework admission;
- position and lot identity;
- lot-level uniqueness;
- custodian immobilization;
- exclusive pledge;
- borrowing base and available capacity;
- utilization and reversal;
- price freshness and margin state;
- suspension and resumption;
- owner-custodian-bank adjustment order;
- atomic repayment;
- bank authorization and custodian confirmation for release;
- default, cure, enforcement readiness, and enforcement recording;
- canonical collateral and governance events;
- replay reconstruction;
- 224 contract tests.

### Commercial interpretation

This stage proves the reserve-control engine through a funded-credit branch. It is not the final market positioning.

### Gate to leave Stage 0

- current source, tests, documentation, and testnet evidence remain reproducible;
- target obligation claims remain explicitly marked as unimplemented.

---

## 4. Stage 1 - institutional signing and governance

**Objective:** replace development signing with an institutionally governed intent and approval layer.

### Deliverables

- DFNS organization-controlled wallets for bank, custodian, verifier, sponsor, and operator roles;
- defined owner-wallet treatment based on whether owner actions require organization policy;
- permissions and role assignment;
- policy gates for signing;
- approval groups and quorums;
- pending, approved, denied, expired, submitted, and confirmed states;
- webhook or polling reconciliation;
- Soroban authorization-entry signing;
- link between intent, approval, signature, transaction, event, and resulting state;
- mainnet-ready runbook and evidence model.

### Security requirements

- policies are deliberately configured; unconfigured wallets are not treated as deny-by-default;
- delegated wallets are not used for acts that require institutional policy control;
- operator wallets cannot authorize bank or custodian business acts;
- release and enforcement authorities are separated;
- approval expiry and idempotency are enforced.

### Reusable ecosystem output

- Soroban-aware DFNS signer adapter;
- method and argument decoder for policy review;
- role topology and policy templates;
- approval-to-transaction reconciliation schema;
- reference evidence certificate.

### Exit criteria

- at least one complete lifecycle path is executed with real governed role approvals;
- wrong-role and unapproved actions are refused before signature or by the contract;
- every approval is traceable to the resulting Stellar transaction and canonical event.

---

## 5. Stage 2 - facility and capacity generalization

**Objective:** separate the reserve pool and facility from the current credit-line object.

### New protocol concepts

- MasterFacility;
- ProductSublimit;
- FacilityParticipant;
- CapacityReservation;
- generic ExposureState;
- external product-system reference.

### Required behavior

- expose gross, eligible, approved, reserved, crystallized, and free capacity;
- support group and product sublimits;
- reserve capacity before an external bank instrument is issued;
- automatically expire unused reservations;
- prevent concurrent allocation of the same capacity;
- preserve policy version and evidence context;
- maintain backward compatibility with the current credit branch.

### Migration principle

The existing line and draw state should be represented as one facility profile rather than deleted. Existing event history and identifiers must remain replayable.

### Exit criteria

- one existing credit position can be read through the generalized facility view;
- a reservation can be created, committed, expired, and released without moving value;
- aggregate and sublimit constraints are enforced;
- no current test or event replay is silently broken.

---

## 6. Stage 3 - no-cash-draw facility profile

**Objective:** enforce the mature product boundary.

### Core invariant

```text
customer-directed unrestricted cash withdrawal = prohibited
```

### Required utilization fields

- product type;
- applicant;
- beneficiary;
- commercial purpose;
- amount and currency;
- issue date;
- expiry or maturity;
- external bank-system reference;
- governing rules or legal-context reference;
- reserved capacity;
- evidence package.

### Permitted value movement

Value may move only through:

- payment to a named beneficiary under an approved instrument;
- reimbursement to the bank;
- valid claim or maturity settlement;
- approved regulatory or market settlement;
- enforcement realization.

### Exit criteria

- no API, contract, or user path converts free facility capacity into an unrestricted customer balance;
- each bank payment is attributable to an approved obligation and beneficiary;
- release remains blocked until all active, pending, and crystallized exposure is resolved.

---

## 7. Stage 4 - first typed obligation family: guarantees

**Why first:** guarantees are beneficiary-specific, non-cash-drawable, familiar to banks, and commercially relevant across construction, trade, customs, licences, utilities, and regulated operations.

### Initial types

- bid guarantee;
- performance guarantee;
- advance-payment guarantee;
- retention guarantee;
- warranty guarantee;
- customs or regulatory guarantee.

### Lifecycle

```text
request
-> reserve capacity
-> bank approve
-> issue
-> amend or remain active
-> expire, cancel, claim, or pay
-> reimburse or enforce
-> discharge and release capacity
```

### Required controls

- beneficiary identity;
- amount and expiry;
- claim conditions reference;
- amendment authority;
- claim and bank-decision evidence;
- paid versus contingent exposure;
- reimbursement due;
- discharge evidence.

### Exit criteria

- one complete guarantee lifecycle runs end to end;
- valid expiry restores capacity;
- a claim path creates reimbursement exposure without releasing collateral;
- the bank remains the authoritative issuer and claim decision-maker.

---

## 8. Stage 5 - documentary-credit profile

**Objective:** connect reserve capacity to payment against compliant documents.

### Initial types

- sight documentary credit;
- deferred-payment documentary credit.

### Required objects

- applicant and beneficiary;
- issue and expiry;
- amount and currency;
- document requirements reference;
- presentation;
- bank compliance decision;
- bank payment;
- reimbursement;
- discharge.

### Integration boundary

Argent does not determine documentary compliance. The bank trade-finance platform remains authoritative. Argent records:

- capacity reservation;
- issue evidence;
- presentation status;
- bank-authorized decision;
- payment and reimbursement references;
- capacity release.

### Electronic-document extension

Adapters may later verify the authoritative state of:

- electronic bills of lading;
- warehouse receipts;
- invoices;
- inspection certificates;
- other electronic transferable records.

A document hash alone is not treated as legal possession or control.

### Exit criteria

- issue, presentation, bank decision, payment, reimbursement, and discharge are traceable;
- bank and document systems remain authoritative;
- no duplicate settlement or capacity release occurs.

---

## 9. Stage 6 - supplier and maturity obligations

### Product candidates

- supplier payment undertaking;
- financial standby letter of credit;
- bank-accepted draft;
- avalised bill or promissory note;
- forfaiting-ready payment obligation.

### Specific risks

- direct credit-substitute treatment;
- transferability and holder identity;
- amendment and cancellation restrictions;
- maturity settlement;
- duplicate presentation;
- discounting and assignment evidence;
- sanctions and jurisdiction exposure.

### Exit criteria

- the system distinguishes applicant, original beneficiary, permitted holder, maturity, payment, and reimbursement;
- legal transferability is determined by the authoritative instrument and law, not the protocol alone.

---

## 10. Stage 7 - treasury and margin exposure

### Product candidates

- FX forwards;
- precious-metals hedges;
- commodity derivatives;
- interest-rate hedges;
- bank-supported margin;
- collateral transformation.

### Additional requirements

- current and potential future exposure;
- mark-to-market and threshold state;
- netting-set reference;
- collateral-call and cure state;
- close-out and settlement reference;
- product-specific exposure consumption.

### Boundary

Argent does not replace a treasury, derivatives, margin, or netting platform. It records the approved reserve allocation and reconciles authoritative exposure inputs.

---

## 11. Stage 8 - group treasury and subsidiary sublimits

### Use case

A holding company or treasury vehicle owns the reserve. Approved subsidiaries receive non-transferable, purpose-bound sublimits.

### Controls

- legal owner and pledgor identity;
- third-party security and reimbursement agreements;
- corporate benefit and approvals;
- entity, product, currency, beneficiary, and tenor sublimits;
- no cross-subsidiary capacity leakage;
- central visibility of aggregate exposure;
- revocation and wind-down.

### Exit criteria

- one central reserve supports at least two entity sublimits without duplicate allocation;
- every obligation remains attributable to its applicant and reimbursement source;
- release considers all group exposure.

---

## 12. Stage 9 - selective disclosure and privacy

### Objective

Prove capacity sufficiency and instrument authenticity without exposing:

- full bar serials;
- total reserve size;
- unrelated obligations;
- bank risk policy;
- private beneficiary terms.

### Evidence products

- reserve-capacity certificate;
- instrument verification receipt;
- approval and authority receipt;
- release certificate;
- default and enforcement evidence pack.

### Principle

The protocol should disclose the minimum fact required by the verifier, while preserving bank, customer, and beneficiary confidentiality.

---

## 13. Stage 10 - multi-collateral architecture

Gold remains the market-entry reserve. The architecture may later support other bank-approved reserve assets through separate profiles:

- tokenized government securities or money-market instruments;
- electronic warehouse receipts;
- qualifying receivables;
- other allocated metals;
- cash deposits.

Each profile requires its own:

- rights model;
- custody or control model;
- valuation and haircut policy;
- concentration limits;
- liquidity and enforcement route;
- evidence schema.

No asset becomes eligible merely because it is tokenized or recorded on a ledger.

---

## 14. Stage 11 - automation and conditional orchestration

### Candidates

- milestone-based guarantee reduction;
- automatic reservation expiry;
- price and evidence freshness circuit breakers;
- controlled collateral substitution;
- document-triggered settlement readiness;
- ERP or treasury-system requests;
- strictly bounded AI-agent requests.

### Agent boundary

An automated agent may request an approved instrument within defined limits. It must not:

- approve its own request;
- override bank policy;
- draw unrestricted cash;
- change custody or legal evidence;
- release collateral;
- initiate enforcement without governed approval.

---

## 15. Research extension - funded auto-collateralisation

The repository contains research on self-triggered, just-in-time funded credit. That work remains relevant for settlement and liquidity use cases, but it is not the primary product direction.

It should be considered only after:

- the non-cash-draw obligation facility is operational;
- institutional signing and policy are production-grade;
- settlement and reimbursement controls are proven;
- a bank explicitly requires a funded use case.

See `auto-collateralisation-layer.md`.

---

## 16. Commercial pilot sequence

### Pilot A - precious-metals company

- owned allocated bullion;
- approved custodian;
- recurring import or guarantee need;
- one bank;
- one documentary credit or guarantee profile.

### Pilot B - contractor or diversified group

- central gold reserve;
- one operating subsidiary;
- bid, performance, or advance-payment guarantee;
- product sublimit and group reimbursement agreement.

### Pilot C - corporate treasury reserve holder

- formal reserve policy;
- occasional bank-obligation need;
- no unrestricted cash draw;
- selective capacity evidence.

---

## 17. What should not be built

Argent should not build:

- a gold stablecoin;
- a public capacity token;
- a bearer claim against the reserve;
- direct consumer lending;
- operator-controlled bank authority;
- on-chain legal judgment;
- public disclosure of complete reserve and beneficiary data;
- duplicate gold representations across chains;
- a generic RWA marketplace before the first bank obligation works.

---

## 18. Roadmap success measure

The protocol is mature when a reviewer can follow one complete obligation from:

```text
identified reserve
-> bank policy
-> governed approval
-> capacity reservation
-> bank issue
-> beneficiary reliance
-> expiry, payment, or claim
-> reimbursement or enforcement
-> capacity release
```

and independently verify:

- who acted;
- under what authority;
- against which reserve and policy;
- how much capacity was consumed;
- what external system remained authoritative;
- why release or enforcement became valid.

That is the product destination.

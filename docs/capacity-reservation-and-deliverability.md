# Capacity Reservation and Deliverability

**A target Argent Protocol specification for converting eligible physical-reserve value into reserved, issuable, and operationally deliverable bank capacity.**

**Status:** canonical design specification; not yet the implemented contract surface  
**Applies to:** the reserve-obligation facility profile and future bank, custodian, trade-finance, treasury, and settlement adapters  
**Primary rule:** an asset can be valuable and uncommitted without being usable for the requested obligation at the requested time

---

## 1. Purpose

Argent needs to answer more than one collateral question.

The current secured-credit reference branch can answer questions such as:

- is this reserve position admitted;
- is it uniquely identified;
- is it pledged;
- what is its borrowing base;
- how much exposure is outstanding;
- may it be released or enforced.

A production obligation facility must answer a harder operational question:

> **Can this bank issue this product, for this applicant, to this beneficiary, under these terms, against this reserve, now?**

That question cannot be answered by reserve value alone. A bank instrument may be economically covered but still not be issuable because:

- the applicant is not approved for the product;
- the beneficiary or jurisdiction is restricted;
- the product sublimit is unavailable;
- the requested wording or tenor is outside policy;
- required evidence is missing or stale;
- the bank product system is unavailable;
- the custodian has not confirmed control;
- settlement or reimbursement arrangements are incomplete;
- another request has already reserved the same capacity.

This document defines the capacity states, reservation mechanics, deliverability checks, external-system outcomes, and failure controls needed to close that gap.

---

## 2. The central distinction

### 2.1 Reserve value

The observed or appraised value of the physical reserve before bank treatment.

### 2.2 Eligible reserve value

The portion of reserve value that survives asset, custody, legal, concentration, and valuation policy.

### 2.3 Approved facility capacity

The maximum amount the bank permits the facility to support after its own credit and risk decision.

### 2.4 Available capacity

Approved capacity that is not already reserved, committed, crystallized, or held as a required buffer.

### 2.5 Reservable capacity

Available capacity that may be provisionally locked for a specific request without breaching aggregate, product, entity, currency, tenor, concentration, or policy constraints.

### 2.6 Issuable capacity

Reserved capacity for which all bank, applicant, beneficiary, product, legal, evidence, and authorization conditions are satisfied.

### 2.7 Deliverable capacity

Issuable capacity for which the bank can complete the required operational act and return a definitive outcome to the originating system within the relevant business window.

### 2.8 Active capacity

Capacity supporting an issued and unresolved obligation.

### 2.9 Releasable capacity

Capacity that may return to the free pool because the obligation has expired, been cancelled, reimbursed, discharged, or otherwise resolved, and all release conditions have been satisfied.

The target progression is:

```text
reserve value
-> eligible reserve value
-> approved facility capacity
-> available capacity
-> reserved capacity
-> issuable capacity
-> deliverable capacity
-> active obligation capacity
-> releasable capacity
-> available capacity
```

Argent must not collapse these states into one number.

---

## 3. Why this matters

A theoretical capacity calculation can be correct while the proposed use still fails operationally.

Examples:

- The facility has $5 million free, but only $1 million remains under the documentary-credit sublimit.
- The reserve covers the request, but the beneficiary is in a prohibited jurisdiction.
- The bank approves the applicant, but the guarantee wording is open-ended and outside policy.
- The bank intends to issue, but another request reserves the same capacity before the transaction is completed.
- The bank product system issues the instrument, but the callback is lost and Argent still shows the reservation as pending.
- The bank declines the request, but an expired reservation continues blocking capacity.

The protocol must therefore treat reservation and deliverability as first-class control states rather than UI convenience.

---

## 4. Systems of authority

Argent coordinates systems that remain authoritative for different facts.

| Domain | Authoritative source | Argent responsibility |
|---|---|---|
| Physical existence and custody | Custodian or vault system | record signed custody and control evidence; detect disagreement |
| Shared-gold assurance | Bank-approved gold platform, product operator, or assurance provider | record the assurance scope, equivalence class, reconciliation status, and expiry; do not expand the source's claim |
| Ownership and security documents | Legal and bank files; applicable registry | bind evidence references and agreed control state |
| Eligibility and limits | Bank risk, credit, collateral, and limits systems | enforce the bank-approved result and policy version |
| Product issuance | Bank guarantee, trade-finance, treasury, or facility system | reserve capacity before issue; reconcile issue outcome |
| Beneficiary and sanctions status | Bank KYC, AML, sanctions, and product controls | require current decision reference; do not duplicate screening logic |
| Trade documents | Bank documentary platform or recognised electronic-document system | bind document identity and bank-authorized outcome |
| Settlement and reimbursement | Bank core, payment rail, or compatible on-chain asset | record finality reference and resulting exposure state |
| Institutional authorization | DFNS or equivalent policy-governed signer | reconcile approval activity to the exact Soroban authorization and transaction |
| Shared control state | Argent contracts and read model | reject invalid transitions and maintain one replayable capacity history |

A ledger record does not override a bank product system, custody book, legal registry, or settlement rail. It coordinates their authorized facts.

---

### 4.1 Upstream reserve authority and profile

Capacity calculation may begin with an allocated bar set or, in a later profile, an approved pooled or digital-gold entitlement. The source record must be explicit.

```text
ReserveProfile
- ALLOCATED_BAR_SET
- POOLED_GOLD_INTEREST
- DIGITAL_GOLD_ENTITLEMENT
```

The current implementation proves only `ALLOCATED_BAR_SET`.

A reserve adapter must identify the authoritative reserve record, owner or entitlement holder, custody or account provider, quantity, quality, backing, location, legal and redemption character, evidence freshness, discrepancy state, and any hold or encumbrance information the source can provide.

An upstream `VERIFIED` result means the source facts were verified within that system's scope. It does not mean the bank has accepted the reserve, that a security interest exists, or that capacity is issuable.

If the upstream source becomes stale or reports a discrepancy:

1. reject new reservations and issuance;
2. preserve active reservation and obligation state;
3. open a reconciliation exception;
4. apply bank-directed margin, cure, substitution, or enforcement procedures;
5. never release capacity solely because the upstream record disappeared or changed.

See [`shared-gold-infrastructure-and-argent.md`](shared-gold-infrastructure-and-argent.md).

### 4.2 Four gates from reserve assurance to facility issuability

A bank should not move directly from an upstream `VERIFIED` flag to an issuable obligation. The target profile separates four gates:

1. **Reserve verified** - the upstream source confirms backing, custody, quantity, and ownership or entitlement within a stated assurance scope.
2. **Legally pledgeable** - the customer's rights can support the required security interest, control arrangement, and enforcement path.
3. **Operationally controllable** - the custodian or account operator can block, freeze, earmark, substitute, release, and realise the interest as required.
4. **Facility issuable** - the bank accepts the reserve for this applicant, product, beneficiary, amount, currency, jurisdiction, tenor, evidence package, and operating route.

```text
reserve verified
-> legally pledgeable
-> operationally controllable
-> facility issuable
```

Failure at any gate prevents new issuance. Passing one gate never implies that a later gate has passed. Economic, legal, or operational equivalence at an upstream gold-product layer does not make a bank facility reservation transferable or fungible.

---

## 5. Capacity equations

### 5.1 Eligible reserve value

```text
observed reserve value
- asset haircut
- liquidity adjustment
- concentration adjustment
- custody or legal adjustment
- volatility buffer
= eligible reserve value
```

### 5.2 Approved facility capacity

```text
min(bank-approved facility limit, eligible reserve value)
= approved facility capacity
```

### 5.3 Available capacity

```text
approved facility capacity
- provisional reservations
- committed reservations
- active obligation allocations
- pending-presentation or pending-claim reserve
- crystallized reimbursement exposure
- required margin buffer
= available capacity
```

### 5.4 Reservable capacity

```text
available capacity
constrained by:
- aggregate facility limit
- product sublimit
- applicant or subsidiary sublimit
- beneficiary and jurisdiction limit
- currency and tenor limit
- concentration and portfolio rules
= reservable capacity for the request
```

### 5.5 Issuable capacity

```text
reserved capacity
+ all mandatory policy decisions current
+ all required evidence present and fresh
+ all required institutional approvals complete
+ no blocking legal, sanctions, beneficiary, or operational condition
= issuable capacity
```

The face amount of an obligation and the amount of capacity it consumes may differ. The bank supplies the approved conversion or exposure treatment. Argent stores and enforces that result; it does not invent regulatory or credit factors.

---

## 6. Request and reservation objects

### 6.1 ObligationRequest

A request should identify at minimum:

- request ID;
- originating-system reference;
- idempotency key;
- facility ID;
- applicant and requesting role;
- proposed product type;
- beneficiary identity or approved beneficiary category;
- face amount and currency;
- requested issue date;
- expiry, maturity, or maximum tenor;
- commercial purpose and reference;
- governing rules or template reference;
- evidence package reference;
- reimbursement source or arrangement reference;
- requested capacity amount, if pre-calculated by the bank;
- request creation and expiry time.

The request is not an obligation and does not create a bank promise.

### 6.2 CapacityReservation

A reservation should identify:

- reservation ID;
- request ID;
- facility ID;
- policy version;
- bank-recognized capacity amount;
- product and entity sublimits consumed;
- selected reserve positions or pool allocation reference;
- reservation creation time;
- reservation expiry time;
- current state;
- approving role references;
- external bank-system status;
- evidence commitment;
- reason code for refusal, expiry, cancellation, or release.

A reservation is purpose-bound, non-transferable, time-limited, and revocable only according to bank policy and lifecycle state.

### 6.3 DeliverabilityDecision

A deliverability decision should contain:

- decision ID;
- request and reservation IDs;
- decision timestamp;
- policy and rule-set versions;
- each required check and result;
- approved capacity amount;
- permitted issue window;
- required remaining actions;
- final decision status;
- deterministic reason code;
- institutional approval references;
- evidence reference.

The decision should be reproducible from the recorded inputs and policy version, even when the underlying policy logic remains private to the bank.

---

## 7. Reservation lifecycle

### 7.1 Target state machine

```text
REQUESTED
-> PRECHECKED
-> PROVISIONALLY_RESERVED
-> BANK_APPROVED
-> ISSUABLE
-> ISSUE_SUBMITTED
-> ISSUED
-> ACTIVE
```

Alternative paths:

```text
REQUESTED -> REFUSED
PRECHECKED -> REFUSED
PROVISIONALLY_RESERVED -> EXPIRED
PROVISIONALLY_RESERVED -> CANCELLED
BANK_APPROVED -> APPROVAL_EXPIRED
ISSUABLE -> ISSUE_FAILED
ISSUE_SUBMITTED -> RECONCILIATION_REQUIRED
```

Resolution paths:

```text
ACTIVE -> EXPIRED -> DISCHARGED -> RELEASED
ACTIVE -> CANCELLED -> DISCHARGED -> RELEASED
ACTIVE -> PRESENTED_OR_CLAIMED -> BANK_DECIDED
BANK_DECIDED -> NO_PAYMENT_DUE -> DISCHARGED -> RELEASED
BANK_DECIDED -> BANK_PAID -> REIMBURSEMENT_DUE
REIMBURSEMENT_DUE -> REIMBURSED -> DISCHARGED -> RELEASED
REIMBURSEMENT_DUE -> DEFAULT -> CURE -> ENFORCEMENT
```

### 7.2 Reservation commitment is policy-defined

The request lifecycle and the reservation's commitment status are related but not identical. A reservation begins as provisional and becomes committed at the bank-defined commit point. That point may occur before issue submission, when an issuance message is dispatched, or only when the product system reports `ISSUED`. The protocol must record the configured rule and current commitment status rather than assume one universal ordering.

A committed reservation remains capacity-consuming through an ambiguous issue outcome until the authoritative product system is reconciled.

### 7.3 Reservation timing

Every provisional reservation must have:

- a start time;
- an expiry time;
- an owner or responsible bank workflow;
- a defined renewal rule;
- a deterministic release path;
- a reconciliation status.

No abandoned application may block capacity indefinitely.

### 7.4 Commit point

The bank must define the point at which a provisional reservation becomes committed. Representative commit points include:

- final internal product approval;
- external instrument number assigned;
- signed bank undertaking created;
- authenticated issuance message dispatched;
- bank product system reports `ISSUED`.

Argent records the bank-selected commit rule. It does not define when the legal instrument comes into existence.

---

## 8. Deliverability checks

A production preflight should evaluate the following dimensions.

### 8.1 Reserve checks

- reserve exists and remains admitted;
- upstream assurance scope covers the facts the bank relies on;
- economic, legal, and operational equivalence classes are accepted for the requested profile;
- source reconciliation remains within its stated tolerance and the assurance has not expired;
- custody attestation is current;
- required bar, lot, or pool identity is valid;
- security interest remains active;
- valuation is current;
- free capacity is sufficient;
- no incompatible reservation or allocation exists;
- substitution or release is not pending.

### 8.2 Facility checks

- facility is active and not suspended or winding down;
- aggregate capacity remains available;
- product sublimit remains available;
- applicant or subsidiary sublimit remains available;
- policy version is current;
- covenant, margin, and cure status permit new issuance.

### 8.3 Product checks

- product type is permitted;
- face amount, currency, and tenor are within limits;
- proposed wording or template is permitted;
- claim, presentation, expiry, and governing-rule references are complete;
- transaction-specific capacity treatment is approved.

### 8.4 Applicant and beneficiary checks

- applicant is entitled to use the facility;
- applicant authority is current;
- beneficiary is identified or falls within an approved category;
- jurisdiction and sanctions decisions are current;
- related-party or group-use conditions are satisfied;
- corporate-benefit and authority evidence exists where required.

### 8.5 Documentary checks

- required commercial reference exists;
- required agreements and approvals are current;
- required trade or project evidence is present;
- any external electronic document is authentic and in the required state;
- evidence references resolve to the expected version.

### 8.6 Operational checks

- issuing system is available;
- institution signing path is available;
- required approvers can act within the issue window;
- callback or status-query route is available;
- settlement and reimbursement route is configured;
- required manual review is assigned;
- no incident or operational hold blocks issue.

A capacity amount should not be described as deliverable when a mandatory operational check is unresolved.

---

## 9. Decision outcomes and reason codes

The service should return deterministic, machine-readable outcomes.

Representative approval outcomes:

- `APPROVED_AND_RESERVED`
- `APPROVED_PENDING_INSTITUTIONAL_SIGNATURE`
- `ISSUABLE`
- `ISSUE_SUBMITTED`
- `ISSUED`

Representative refusal or hold outcomes:

- `INSUFFICIENT_AVAILABLE_CAPACITY`
- `PRODUCT_SUBLIMIT_EXCEEDED`
- `APPLICANT_SUBLIMIT_EXCEEDED`
- `BENEFICIARY_NOT_PERMITTED`
- `JURISDICTION_NOT_PERMITTED`
- `PRODUCT_NOT_PERMITTED`
- `TENOR_OUTSIDE_POLICY`
- `VALUATION_STALE`
- `CUSTODY_ATTESTATION_STALE`
- `EVIDENCE_INCOMPLETE`
- `FACILITY_SUSPENDED`
- `MARGIN_OR_CURE_HOLD`
- `POLICY_VERSION_CHANGED`
- `APPROVAL_REQUIRED`
- `APPROVAL_EXPIRED`
- `DUPLICATE_REQUEST`
- `RESERVATION_CONFLICT`
- `ISSUING_SYSTEM_UNAVAILABLE`
- `ISSUE_STATUS_AMBIGUOUS`
- `RECONCILIATION_REQUIRED`
- `ASSURANCE_SCOPE_INSUFFICIENT`
- `LEGAL_RIGHTS_NOT_PLEDGEABLE`
- `OPERATIONAL_CONTROL_UNAVAILABLE`
- `SOURCE_ASSURANCE_EXPIRED`
- `SOURCE_TOLERANCE_BREACHED`
- `EQUIVALENCE_CLASS_NOT_ACCEPTED`

Reason codes should identify the controlling domain without disclosing private risk logic to unauthorized recipients.

---

## 10. Concurrency and double-allocation control

### 10.1 The problem

Two valid requests may arrive against the same free capacity at nearly the same time. A read-then-write implementation can approve both if it does not reserve capacity atomically.

### 10.2 Required property

> **The same legal collateral capacity must not support incompatible concurrent reservations or obligations.**

### 10.3 Control pattern

A reservation transaction should:

1. load the current facility and relevant sublimit state;
2. validate the request and policy version;
3. calculate or receive the bank-approved capacity consumption;
4. verify sufficient free capacity;
5. create the reservation;
6. reduce available capacity;
7. emit the reservation event;
8. commit all state changes atomically.

If any step fails, no reservation is created and capacity remains unchanged.

### 10.4 Physical-position selection

Where the bank requires identified-bar or identified-lot allocation, selection should be deterministic or evidenced, and selected positions should move through explicit states:

```text
FREE
-> SELECTED
-> PROVISIONALLY_RESERVED
-> COMMITTED
-> RELEASED
```

Selection strategy may minimize over-collateralization, concentration, substitution cost, or expected liquidation friction, but the strategy must remain bank-governed and policy-versioned.

Daml Finance allocation and settlement instructions, Corda token-selection claims, and Quant's inventory-reservation model are useful design precedents. Argent applies the pattern to physical-reserve capacity rather than transferable token holdings.

---

## 11. Idempotency, retries, and duplicate handling

Every inbound command from a bank, custodian, or product system must carry:

- an immutable originating-system reference;
- an idempotency key;
- an expected facility or obligation version where optimistic concurrency is used;
- a request timestamp and expiry;
- the authenticated source identity.

Required behavior:

- replay of the same request returns the prior result;
- reuse of an idempotency key with different payload data is rejected;
- duplicate callbacks do not create duplicate events;
- a timeout does not imply failure;
- ambiguous transaction status triggers reconciliation before resubmission;
- an expired request cannot be revived silently;
- amendments create a new version or explicit amendment event rather than overwriting history.

The service must store DFNS approval state, Stellar transaction state, bank-product state, and external settlement state separately, then reconcile them into one read model.

---

## 12. External-system finality and reconciliation

### 12.1 Finality is domain-specific

Different systems provide different kinds of finality:

- Soroban finality confirms a contract state transition;
- DFNS approval confirms an institutional approval process reached a result;
- the bank product system confirms issuance, amendment, cancellation, or claim status;
- the custodian confirms physical control or release;
- a payment system confirms settlement;
- legal finality depends on the governing instrument and law.

No single reference should be presented as proof of every domain.

### 12.2 Required correlation record

A production correlation record should include:

- originating request ID;
- idempotency key;
- reservation ID;
- obligation ID;
- DFNS activity and approval IDs;
- Soroban transaction hash and ledger sequence;
- bank product-system reference;
- custody instruction and confirmation reference;
- settlement or reimbursement reference;
- current reconciliation status;
- last verified timestamp.

### 12.3 Ambiguous issue outcome

If the bank product system times out after issue submission:

1. keep the reservation committed;
2. mark the issue state `AMBIGUOUS` or `RECONCILIATION_REQUIRED`;
3. query the authoritative bank system using the stable request reference;
4. do not submit a second issue request blindly;
5. resolve to `ISSUED`, `FAILED`, or a manual exception state;
6. release capacity only after authoritative non-issuance is established.

### 12.4 Disagreement

If Argent and an authoritative external system disagree:

- stop new use of affected capacity where safety requires;
- preserve both observations and timestamps;
- identify the authoritative source for the disputed fact;
- open a reconciliation case;
- require role-approved correction or compensating event;
- never rewrite prior events silently.

---

## 13. Adapter contract

### 13.1 Preflight request

A bank or originating system should be able to request:

```text
Can facility F support product P
for applicant A
benefiting B
for amount X in currency C
until date T
under evidence package E?
```

### 13.2 Preflight response

The response should include:

- decision status;
- approved capacity amount;
- reservation ID if created;
- reservation expiry;
- required next actions;
- reason codes;
- policy version;
- evidence and approval references;
- whether issue submission is permitted;
- reserve-verification, legal-pledgeability, operational-control, and facility-issuability gate status;
- upstream assurance scope, provider, expiry, and tolerance status when an external gold platform is used.

### 13.3 Issue callback

The bank product system should return one of:

- issued;
- refused;
- cancelled;
- expired before issue;
- ambiguous;
- manual review required.

### 13.4 Lifecycle callbacks

The same integration should support:

- amendment;
- reduction;
- cancellation;
- expiry;
- presentation;
- claim;
- bank decision;
- payment;
- reimbursement;
- default;
- discharge.

Argent should integrate through one controlled adapter contract rather than infer product state from informal messages.

---

## 14. Event model extension

Representative target events:

- `ObligationRequestReceived`
- `CapacityPrecheckCompleted`
- `CapacityProvisionallyReserved`
- `CapacityReservationRenewed`
- `CapacityReservationExpired`
- `CapacityReservationCancelled`
- `DeliverabilityDecisionRecorded`
- `ObligationIssueSubmitted`
- `ObligationIssued`
- `ObligationIssueFailed`
- `ExternalStatusReconciled`
- `ObligationAmended`
- `ObligationReduced`
- `ObligationExpired`
- `PresentationRecorded`
- `ClaimRecorded`
- `BankDecisionRecorded`
- `BankPaymentRecorded`
- `ReimbursementDueRecorded`
- `ReimbursementRecorded`
- `CapacityReleased`

Each event should identify:

- facility and obligation or request;
- actor and authority;
- prior and resulting state;
- capacity delta;
- policy version;
- evidence reference;
- external-system correlation reference;
- sequence and timestamp.

These are target protocol events, not claims about current contract event names.

---

## 15. Privacy boundary

Capacity deliverability can reveal commercially sensitive information, including:

- total reserve size;
- available headroom;
- bank risk appetite;
- customer liquidity pressure;
- beneficiary and transaction identity;
- trade corridor;
- product pricing or policy;
- pending claims or defaults.

The shared ledger should therefore record the minimum state required to enforce capacity and prove lifecycle order. Detailed evidence, bank logic, identities, and commercial documents should remain encrypted and role-restricted.

The privacy model is specified in [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md).

---

## 16. Threats and controls

| Threat | Required control | Residual risk |
|---|---|---|
| Concurrent requests over-allocate capacity | atomic reservation and version checks | capacity pledged outside Argent remains external risk |
| Abandoned reservations block the facility | expiry, renewal policy, reconciliation, and automatic release | incorrect expiry policy may release too early or too late |
| Duplicate bank request creates duplicate obligation | idempotency key and authoritative issue reference | bad external identifiers may still require manual repair |
| Issue succeeds but callback is lost | committed reservation and status reconciliation | product-system outage delays certainty |
| Policy changes after reservation | policy version pinning and revalidation before issue | urgent policy override needs explicit governance |
| Stale custody or valuation evidence | freshness gates and issue hold | false source data remains source risk |
| Beneficiary becomes restricted after approval | time-of-issue recheck and amendment/hold policy | post-issue legal obligations may persist |
| Operator releases capacity after timeout | release only after authoritative non-issuance or discharge | manual exception process remains necessary |
| Reason codes reveal bank policy | role-scoped responses and coarse external codes | timing and repeated probes can still leak information |
| Reservation starvation or denial of service | quotas, expiry, authenticated requests, and monitoring | malicious approved users may still consume review capacity |

---

## 17. Mapping to the current implementation

### Already proven by the secured-credit reference branch

- unique collateral position identity;
- prevention of a second active pledge over the same recorded lot;
- bank-approved eligibility and treatment;
- deterministic capacity calculation;
- refusal above available capacity;
- role-specific authorization;
- ordered events;
- atomic settlement-asset repayment and exposure reduction;
- two-act release;
- default, cure, and enforcement sequencing.

### Required target extensions

- generic `MasterFacility`;
- product and applicant sublimits;
- `ObligationRequest`;
- provisional and committed `CapacityReservation`;
- applicant, beneficiary, product, currency, jurisdiction, and tenor checks;
- `DeliverabilityDecision`;
- typed `BankObligation`;
- issue callbacks and external finality reconciliation;
- reservation expiry, cancellation, renewal, and amendment;
- contingent, pending-claim, crystallized, and discharged exposure classes;
- selective disclosure and role-specific views;
- a `SharedGoldAssuranceSnapshot` or equivalent source-correlation object that records assurance scope without duplicating the upstream gold record.

This is a domain-model and integration extension, not a replacement of the collateral-control foundation.

---

## 18. Implementation sequence

### Phase 1 - read-model preflight

- expose current eligible and available capacity;
- accept authenticated, idempotent preflight requests;
- return deterministic reason codes;
- do not reserve or write back to the bank system.

### Phase 2 - provisional reservation

- add atomic provisional reservation;
- add expiry, cancellation, and reconciliation;
- prove concurrent requests cannot over-allocate capacity.

### Phase 3 - one bank product adapter

- connect one guarantee or documentary-credit system;
- reconcile issue submission and definitive issue outcome;
- convert reservation to active obligation.

### Phase 4 - lifecycle completion

- amendment, reduction, expiry, claim, payment, reimbursement, and discharge;
- role-approved correction and exception handling.

### Phase 5 - portfolio deliverability

- multiple product, applicant, currency, and jurisdiction sublimits;
- deterministic selection or pool allocation;
- portfolio concentration and operational-routing controls.

---

## 19. Conformance checklist

A conforming implementation should demonstrate:

- [ ] available capacity and issuable capacity are distinct;
- [ ] every reservation has purpose, amount, policy version, and expiry;
- [ ] reservation creation and capacity reduction are atomic;
- [ ] two concurrent requests cannot consume the same capacity;
- [ ] replay of an identical request returns the prior result;
- [ ] reuse of an idempotency key with different payload is rejected;
- [ ] ambiguous issue status does not trigger blind resubmission;
- [ ] capacity is not released until non-issuance or discharge is authoritative;
- [ ] external system references reconcile to Soroban and institutional approvals;
- [ ] policy changes are versioned and revalidated at the required control point;
- [ ] reason codes do not expose unauthorized bank policy data;
- [ ] corrections use new events rather than rewriting history;
- [ ] upstream verification, legal pledgeability, operational control, and facility issuability are separate gates;
- [ ] an upstream assurance assertion states its scope, provider, expiry, and tolerance status;
- [ ] upstream fungibility never makes a facility reservation transferable or reusable outside bank-approved state transitions;
- [ ] the originating system receives a definitive machine-readable outcome;
- [ ] privacy controls satisfy the companion privacy specification.

---

## 20. Non-goals

This specification does not:

- define the bank's credit or regulatory exposure model;
- decide whether a beneficiary or jurisdiction is permitted;
- determine documentary compliance or claim validity;
- create a bank guarantee or documentary credit by itself;
- provide legal finality for an external instrument;
- guarantee that external systems are available continuously;
- make physical gold transferable on-chain;
- create a public market for facility capacity;
- permit unrestricted customer cash withdrawal.

---

## 21. Design precedents and references

- Quant, *Unlocking collateral mobility: How tokenisation transforms settlement infrastructure* (2026): https://quant.network/perspectives/unlocking-collateral-mobility-how-tokenisation-transforms-settlement-infrastructure/
- Daml Finance asset model: https://docs.daml.com/daml-finance/concepts/asset-model.html
- Daml Finance settlement and allocation: https://docs.daml.com/daml-finance/concepts/settlement.html
- Daml Finance lifecycling: https://docs.daml.com/daml-finance/concepts/lifecycling.html
- Daml Finance glossary, including locking: https://docs.daml.com/daml-finance/reference/glossary.html
- Corda UTXO token selection: https://docs.r3.com/en/platform/corda/5.2/developing-applications/api/ledger/utxo-ledger/token-selection.html
- Stellar smart-contract authorization: https://developers.stellar.org/docs/build/guides/auth/contract-authorization
- Stellar contract events: https://developers.stellar.org/docs/build/guides/events
- DFNS policies and approvals: https://docs.dfns.co/core-concepts/policies
- DFNS policy approvals: https://docs.dfns.co/api-reference/policy-approvals

These references are design precedents. They do not imply partnership, endorsement, legal equivalence, or production compatibility.

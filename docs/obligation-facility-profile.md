# Reserve Obligation Facility Profile

**A target Argent Protocol profile for purpose-bound, bank-issued obligations backed by controlled physical reserves.**

**Status:** design specification; not yet the implemented contract surface  
**Compatibility:** extends the current secured-credit reference branch without changing its proven collateral identity, pledge, valuation, release, and enforcement invariants  
**Primary rule:** unused facility capacity is not customer-drawable cash  

---

## 1. Purpose

This profile defines how a bank may use one controlled reserve pool to support multiple approved obligations while maintaining one authoritative capacity state.

The profile exists to separate four concepts that the current credit-oriented implementation combines:

1. the reserve asset;
2. the legal and custody control over it;
3. the bank facility and available capacity;
4. each individual obligation issued under that facility.

The separation follows the same institutional modeling logic found in financial contract frameworks such as Daml Finance, where instruments, holdings, accounts, settlement instructions, and lifecycle events are distinct objects rather than one token or balance.

---

## 2. Profile statement

```text
One controlled reserve pool
    supports
many bank-approved obligations
    subject to
one authoritative capacity state
```

The reserve remains physically stationary. What changes is the allocation of its bank-recognized capacity.

The profile is non-cash-drawable by the reserve owner. It may still produce payment by the bank to an approved beneficiary when an obligation is presented, claimed, matures, or settles according to its terms.

---

## 3. Parties

| Party | Economic role | Protocol authority |
|---|---|---|
| Reserve owner / applicant | Owns the reserve and requests approved instruments | requests facility use, provides reimbursement, requests permitted substitution or release |
| Issuing bank | Underwrites and issues the obligation | approves facility, policy, sublimits, obligations, claims, release, default, and enforcement |
| Custodian / control agent | Holds the reserve and restricts disposition | attests custody, immobilizes, confirms substitution, release, and realization |
| Beneficiary | Receives the bank obligation | presents documents or claims where the instrument permits; has no claim on the reserve through Argent |
| Verifier / document source | Supplies authoritative commercial or risk evidence | attests defined evidence only |
| Valuation source | Supplies bank-approved price or valuation data | publishes or signs valuation evidence |
| Operator | Runs software and submits transactions | cannot exercise bank, custodian, owner, or beneficiary authority |
| Auditor / supervisor | Reviews evidence | read-only or selectively disclosed access |

The same legal entity may occupy more than one commercial role only where the bank's governance and conflict controls expressly permit it. Protocol role separation should remain explicit even where an institution performs several functions.

---

## 4. Domain model

### 4.1 ReserveAsset

Defines the reusable asset class or instrument definition.

Representative fields:

- asset class;
- form and grade;
- unit convention;
- accepted identity scheme;
- permitted custody modes;
- valuation method;
- liquidation class;
- active status.

### 4.2 CustodyHolding

Represents a specific owner position under a specific custodian.

Representative fields:

- owner;
- custodian;
- asset definition;
- lot or bar commitment;
- quantity;
- location and custody evidence references;
- status;
- uniqueness commitment.

### 4.3 SecurityInterest

Represents the agreed control relationship between the owner, bank, and custodian.

Representative fields:

- governing facility and security documents;
- secured parties;
- maximum secured amount or obligation scope;
- control and release conditions;
- perfection or registration evidence reference;
- enforcement route;
- active status.

Argent records evidence and agreed control state. It does not create the security interest by itself.

### 4.4 EligibilityPolicy

Contains the bank-approved treatment for a reserve asset and facility.

Representative fields:

- eligible asset classes and forms;
- approved custodians and jurisdictions;
- haircut and advance limits;
- valuation freshness;
- concentration limits;
- legal and evidence requirements;
- product eligibility;
- substitution rules;
- margin and cure policy;
- policy version.

### 4.5 MasterFacility

The master facility is the relationship under which the bank may issue approved obligations against the controlled reserve.

Representative fields:

- facility parties;
- reserve pool;
- maximum capacity;
- permitted currencies;
- permitted product families;
- aggregate and product sublimits;
- facility status;
- policy version;
- reimbursement and enforcement references.

### 4.6 ProductSublimit

A bank-defined limit for one obligation family, entity, beneficiary category, jurisdiction, tenor, or currency.

Examples:

- documentary credits;
- contract guarantees;
- regulatory guarantees;
- supplier undertakings;
- treasury exposure;
- one approved subsidiary.

A sublimit is not a transferable asset and not a promise by Argent. It is a bank-approved constraint on how facility capacity may be used.

### 4.7 CapacityReservation

A temporary or active allocation of free facility capacity to a proposed or issued obligation.

Representative fields:

- facility;
- obligation reference;
- requested amount;
- bank-recognized capacity amount;
- selected collateral or pool allocation;
- policy version;
- reservation expiry;
- state;
- evidence reference.

### 4.8 BankObligation

The bank-issued commercial instrument supported by the reservation.

Representative fields:

- product type;
- applicant;
- beneficiary;
- face amount and currency;
- bank-recognized exposure amount;
- issue date;
- expiry or maturity;
- governing rules and law references;
- claim or presentation conditions reference;
- commercial purpose;
- external bank-system identifier;
- state.

The bank obligation remains authoritative in the bank's product system and legal documentation. Argent stores a control representation and evidence references, not a substitute instrument unless the participating bank and applicable law expressly define it otherwise.

### 4.9 Presentation, Claim, and SettlementInstruction

These objects represent events capable of changing contingent exposure into a payment or reimbursement obligation.

They must identify:

- the instrument;
- presenter or claimant;
- evidence package;
- amount;
- date and status;
- bank decision;
- settlement reference where payment occurs.

Argent should not independently decide documentary compliance or claim validity. It records the bank-authorized outcome and the evidence identity.

### 4.10 Reimbursement

Represents the reserve owner's obligation to restore value after the bank has paid or funded an approved obligation.

Representative fields:

- paid amount;
- due date;
- settlement asset or off-chain payment reference;
- amount reimbursed;
- cure status;
- default status.

---

## 5. Capacity model

### 5.1 Reserve capacity

```text
raw reserve value
- asset haircut
- liquidity adjustment
- concentration adjustment
- legal or custody adjustment
- volatility buffer
= eligible reserve value
```

### 5.2 Facility capacity

```text
min(bank-approved facility limit, eligible reserve value)
= approved facility capacity
```

### 5.3 Available capacity

```text
approved facility capacity
- active contingent allocations
- pending presentations or claims
- crystallized reimbursement exposure
- required margin reserve
= free capacity
```

### 5.4 Product-specific consumption

The face amount of an obligation and the amount of facility capacity it consumes may differ.

The bank may apply:

- product conversion factors;
- tenor multipliers;
- beneficiary or jurisdiction adjustments;
- currency mismatch adjustments;
- margin requirements;
- portfolio concentration rules;
- transaction-specific overlays.

Argent stores the approved result and its policy version. It does not calculate regulatory exposure unless the bank supplies and governs that calculation.

---

## 6. State machines

### 6.1 Facility state

```text
DRAFT
-> APPROVED
-> ACTIVE
-> SUSPENDED
-> WIND_DOWN
-> CLOSED
```

A facility may be suspended without invalidating already issued obligations.

### 6.2 Reservation state

```text
REQUESTED
-> POLICY_VALIDATED
-> RESERVED
-> COMMITTED
-> RELEASED
```

Alternative terminal states:

```text
REQUESTED -> REFUSED
RESERVED -> EXPIRED
RESERVED -> CANCELLED
```

A reservation must expire automatically if the related instrument is not issued within the bank-approved window.

### 6.3 Obligation state

```text
DRAFT
-> BANK_APPROVED
-> ISSUED
-> ACTIVE
```

Normal outcomes:

```text
ACTIVE -> EXPIRED -> DISCHARGED
ACTIVE -> CANCELLED -> DISCHARGED
ACTIVE -> AMENDED -> ACTIVE
```

Payment outcomes:

```text
ACTIVE
-> PRESENTED_OR_CLAIMED
-> BANK_DECIDED
-> PAID_OR_SETTLED
-> REIMBURSEMENT_DUE
-> REIMBURSED
-> DISCHARGED
```

Default outcome:

```text
REIMBURSEMENT_DUE
-> DEFAULT_NOTICED
-> CURE_PERIOD
-> ENFORCEMENT_READY
-> REALIZATION_RECORDED
```

### 6.4 Reserve state

```text
FREE
-> SELECTED
-> IMMOBILIZED
-> PLEDGED
-> ALLOCATED
```

Resolution paths:

```text
ALLOCATED -> PLEDGED -> RELEASE_PENDING -> RELEASED
ALLOCATED -> ENFORCEMENT -> REALIZED
```

The precise legal state names may vary by jurisdiction and custody model. The protocol state must map to, not replace, the governing legal terminology.

---

## 7. Mandatory invariants

### 7.1 No unrestricted cash draw

Facility capacity cannot be transferred into the reserve owner's general account merely because it is available.

Permitted value movement must be linked to:

- a named beneficiary;
- an approved instrument;
- a bank-authorized payment or settlement condition;
- an external bank-system reference;
- the relevant reservation.

### 7.2 No double allocation

The same legal collateral capacity cannot support two incompatible active reservations or obligations.

This extends the current lot-level no-double-pledge invariant from pledge creation into quotation, reservation, issuance, and amendment.

### 7.3 Reserve before issue

A bank obligation cannot enter the issued state unless sufficient capacity has already been reserved under the current policy version.

### 7.4 No silent policy substitution

An active obligation remains linked to the policy and evidence under which it was issued. A later policy change cannot retrospectively alter its recorded basis without a bank-authorized amendment process.

### 7.5 No release with unresolved exposure

Reserve release must be blocked while any of the following remain:

- active obligation allocations;
- pending presentation or claim;
- unpaid bank settlement;
- reimbursement exposure;
- cure or enforcement state;
- unresolved reconciliation exception.

### 7.6 Substitution without unsecured interval

Replacement collateral must become eligible and controlled before the outgoing collateral is released.

### 7.7 Actor separation

The operator cannot impersonate the bank, custodian, owner, beneficiary, or verifier. Each act must carry the authority of the role that owns it.

### 7.8 External authority remains external

Argent cannot mark a bank instrument as legally issued, a claim as valid, a document as compliant, or a security interest as perfected without an authorized source from the relevant authoritative system or institution.

---

## 8. Product selection waterfall

The facility should choose the least costly and least risk-intensive bank instrument that fully satisfies the external requirement.

```text
Evidence sufficient?
-> capacity certificate

Assurance against non-performance required?
-> transaction-related guarantee

Payment against documents required?
-> documentary credit

Future bank payment certainty required?
-> deferred-payment undertaking

Transferable or discountable future claim required?
-> accepted or avalised instrument

Immediate funded payment unavoidable?
-> bank pays the named beneficiary under an approved settlement path
```

The profile does not assume that every customer need should be solved with an accepted draft or a financial guarantee. Product choice remains with the bank and customer.

---

## 9. Privacy and evidence

### 9.1 Shared minimum state

The shared protocol state should include only what is necessary to prove:

- facility and obligation identity;
- role authority;
- capacity sufficiency;
- state transition;
- evidence version;
- time and sequence;
- settlement or release outcome.

### 9.2 Private evidence

Private or restricted evidence may include:

- bar serial numbers and detailed bar list;
- custody statements;
- legal agreements and opinions;
- KYC and sanctions records;
- beneficiary terms;
- trade documents;
- pricing and bank risk models;
- claims and dispute material;
- group exposure details.

### 9.3 Selective disclosure

A beneficiary or external reviewer may need to verify that an instrument is authentic and sufficiently supported without seeing the owner's total reserve or unrelated obligations.

The protocol should support evidence statements such as:

> At the recorded issuance time, the bank-approved facility had sufficient eligible and unallocated capacity for this obligation.

That statement is not a guarantee from Argent and does not transfer rights in the reserve.

---

## 10. External system adapters

### Bank facility and limits system

Authoritative for:

- customer and group approval;
- facility and sublimits;
- product eligibility;
- pricing and fees;
- accounting and exposure.

### Trade-finance or guarantee platform

Authoritative for:

- instrument text;
- issue, amendment, cancellation, presentation, claim, and expiry;
- applicable ICC rules or local product rules;
- external messages and beneficiary communications.

### Custodian system

Authoritative for:

- physical possession;
- allocation and segregation;
- movement restrictions;
- release and realization.

### Electronic trade-document system

Authoritative for:

- electronic bill of lading, warehouse receipt, promissory note, or other transferable-record control where legally recognized.

### Settlement system

Authoritative for:

- bank payment;
- tokenized deposit, stablecoin, commercial-bank money, or central-bank settlement where applicable;
- reimbursement and payment finality.

### Institutional signer

Authoritative for:

- permissions;
- policy evaluations;
- approvals;
- signature generation;
- approval and signing audit trail.

Argent reconciles these authorities; it does not merge them into one legal system.

---

## 11. Mapping to the current implementation

| Target concept | Current reference branch | Reuse assessment |
|---|---|---|
| ReserveAsset | instrument registry | direct conceptual reuse |
| CustodyHolding | position and lot evidence | direct reuse |
| SecurityInterest | pledge and control framework | direct reuse with broader obligation scope |
| EligibilityPolicy | instrument treatment and framework policy | direct reuse plus product eligibility |
| MasterFacility | credit line and framework combination | requires generalization |
| ProductSublimit | not implemented | new |
| CapacityReservation | utilization / available capacity | requires pre-issuance reservation state |
| BankObligation | credit line exposure only | new typed object |
| Presentation / Claim | not implemented | new, bank-authorized outcome only |
| Reimbursement | repayment path | direct conceptual reuse |
| Release | two-step release | direct reuse with broader unresolved-exposure test |
| Default / Cure / Enforcement | implemented reference path | direct reuse with obligation triggers |
| Canonical evidence events | CollateralEventV1 and GovernanceEventV1 | extend rather than replace |

The current implementation therefore proves the hard collateral foundation but not the complete obligation lifecycle.

---

## 12. Event model extension

The existing canonical event pattern should be extended with obligation events such as:

- FacilityActivated;
- ProductSublimitSet;
- CapacityRequested;
- CapacityReserved;
- ReservationExpired;
- ObligationApproved;
- ObligationIssued;
- ObligationAmended;
- PresentationRecorded;
- ClaimRecorded;
- BankDecisionRecorded;
- BankPaymentRecorded;
- ReimbursementDue;
- ReimbursementSettled;
- ObligationDischarged;
- CapacityReleased.

These event names describe the target profile. They do not claim current contract emission.

Each event should identify:

- facility;
- obligation;
- actor and authority;
- product type;
- amount and currency;
- policy version;
- capacity delta;
- evidence reference;
- external authoritative-system reference;
- sequence and time.

---

## 13. DFNS governance profile

A representative organization-wallet topology is:

### Bank

- facility authority;
- product-issuance authority;
- release authority;
- claims and payment authority;
- enforcement authority.

### Custodian

- custody-attestation authority;
- immobilization authority;
- release-confirmation authority;
- realization authority.

### Owner / applicant

- facility-request authority;
- obligation-request authority;
- reimbursement authority;
- substitution-request authority.

### Operator and verifier

- transaction submitter;
- evidence verifier;
- document adapter;
- no bank or custodian discretionary authority.

Policies must be explicitly configured. DFNS policies add restrictions; they do not create a whitelist automatically. Organization-controlled wallets should be used for roles that require policy governance.

---

## 14. Minimal implementation sequence

### Phase 1 - current reference baseline

- preserve the tested contracts and event replay;
- add DFNS-governed signing and approval reconciliation;
- launch and document the current reference lifecycle.

### Phase 2 - facility generalization

- introduce a generic facility identifier and capacity view;
- distinguish reserve capacity from funded exposure;
- preserve backward compatibility for current credit-line state.

### Phase 3 - typed obligations

- add product type, beneficiary, purpose, expiry or maturity, and external instrument reference;
- add pre-issuance reservation;
- add product and group sublimits;
- enforce no unrestricted cash draw.

### Phase 4 - lifecycle completion

- issue, amend, cancel, expire, present, claim, pay, reimburse, and discharge;
- extend release checks to all unresolved obligation states;
- add obligation-specific evidence certificates.

### Phase 5 - interoperability

- bank product adapter;
- custodian adapter;
- trade-document adapter;
- settlement and reimbursement adapter;
- selective-disclosure capacity proof.

---

## 15. Conformance checklist

A deployment conforms to this profile only if:

- [ ] the reserve asset remains legally and operationally separate from any protocol token;
- [ ] a bank or licensed provider remains the obligation issuer;
- [ ] unused capacity cannot be drawn as unrestricted cash by the reserve owner;
- [ ] every obligation has an approved type, purpose, amount, beneficiary, and term;
- [ ] capacity is reserved before issue;
- [ ] active and crystallized exposure are distinguishable;
- [ ] the same reserve capacity cannot be double allocated;
- [ ] the custodian controls physical release;
- [ ] release is blocked while unresolved obligations or claims remain;
- [ ] settlement and reimbursement are attributable to the relevant obligation;
- [ ] legal documents and authoritative external systems remain authoritative;
- [ ] operator, bank, custodian, and owner authorities are separated;
- [ ] evidence is versioned and replayable;
- [ ] private commercial detail is not exposed beyond the approved disclosure model.

---

## 16. Non-goals

This profile does not define:

- a gold stablecoin;
- a private CBDC;
- a bearer capacity token;
- a public DeFi lending pool;
- automated legal judgment on documentary compliance;
- on-chain perfection of a security interest;
- public liquidation of physical gold;
- replacement of bank trade-finance or treasury platforms;
- universal regulatory capital treatment.

---

## 17. Design precedents

- Daml Finance asset model: separate instruments, holdings, and accounts.
- Daml Finance settlement and lifecycling: separate contractual effects from settlement instructions and execute coordinated lifecycle events.
- ICC UCP 600 and URDG 758: bank instruments have defined independent documentary and guarantee frameworks.
- UNCITRAL MLETR: electronic transferable records require authoritative control and integrity, not merely a document hash.
- Stellar authorization: independently supplied authorization entries support role-specific acts.
- DFNS policies: approvals, quorums, and signing governance occur before the institutional signature is produced.

These are design references, not claims of legal equivalence or interoperability.

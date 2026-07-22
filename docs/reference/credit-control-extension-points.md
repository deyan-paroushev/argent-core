# Obligation and Credit Control Extension Points

**Legacy filename retained for link stability.**

This document distinguishes:

- controls already enforced by the current secured-credit reference branch;
- controls required by the target reserve obligation facility;
- later controls that a bank may request;
- controls that Argent should never attempt to own.

**Status:** engineering design and extension register, not a claim that all listed controls are implemented.

---

## 0. Product context

The current contracts implement collateral identity, eligibility, pledge, borrowing base, utilization, repayment, release, default, cure, and enforcement.

The mature product direction is a non-cash-drawable master facility for bank-issued obligations. That profile requires additional controls around:

- beneficiaries;
- product type and purpose;
- capacity reservation;
- product and group sublimits;
- contingent, pending, and crystallized exposure;
- claims and presentations;
- reimbursement;
- discharge;
- no unrestricted customer cash draw.

The current code should not be relabeled as if those controls already exist.

---

## 1. Controls enforced today

### 1.1 Instrument and framework eligibility

A position cannot enter the active lifecycle unless its instrument is registered and admitted to the framework under bank-approved treatment.

### 1.2 Lot uniqueness and exclusive pledge

The current contract refuses two active positions using the identical supplied 32-byte `uniqueness_hash`. It does not prove that differently generated values do not describe the same physical lot. The target production profile adds versioned canonical identity and a custodian-controlled deterministic nullifier, scoped to the governed Argent/custodian domain.

### 1.3 Initial risk headroom

A credit line cannot open at or above its maintenance threshold.

### 1.4 Capacity limit

Utilization cannot exceed current available secured capacity.

### 1.5 Valuation freshness and margin state

Revaluation and margin logic refuse stale or invalid inputs and can suspend the line on a call.

### 1.6 Controlled collateral adjustment

The enforced order is:

```text
owner requests
-> custodian confirms
-> bank approves
```

Post-adjustment coverage is checked.

### 1.7 Atomic repayment

Only the bound settlement vault can apply repayment, and value movement and exposure reduction occur atomically.

### 1.8 Dual-control release

The bank authorizes release and the custodian confirms it. Repayment alone does not release the reserve.

### 1.9 Default, cure, and enforcement order

The contract enforces a cure window and blocks premature or duplicate enforcement recording.

---

## 2. Controls required for the reserve obligation facility

### 2.1 Generic master facility

**Need:** separate the reserve pool and facility from one credit-line object.

**Required state:**

- approved aggregate capacity;
- free capacity;
- reserved capacity;
- active contingent exposure;
- pending claim or presentation;
- crystallized reimbursement exposure;
- margin reserve;
- release eligibility.

**Status:** proposed.

### 2.2 Product sublimits

**Need:** a bank must constrain use by product, entity, currency, jurisdiction, beneficiary class, tenor, or group member.

Examples:

- guarantees only;
- documentary credits only;
- one subsidiary;
- one currency;
- one beneficiary class;
- maximum long-dated exposure.

**Status:** proposed.

### 2.3 Pre-issuance capacity reservation

**Need:** capacity must be held before a bank obligation becomes effective.

Reservation states should include:

```text
requested
-> validated
-> reserved
-> committed
-> released / expired / refused
```

The reservation must have an expiry so abandoned applications do not block capacity indefinitely.

**Status:** proposed.

### 2.4 No unrestricted cash draw

**Need:** target product safety and economic boundary.

Free facility capacity must not become a general customer balance or transferable capacity token.

Permitted payment must identify:

- bank-approved obligation;
- beneficiary;
- amount;
- purpose;
- settlement condition;
- external product reference;
- capacity reservation.

**Status:** proposed and highest-priority product invariant after facility generalization.

### 2.5 Beneficiary control

**Need:** every obligation must be linked to an approved named beneficiary or authoritative beneficiary class.

Controls:

- identity and sanctions status;
- no customer self-payment unless explicitly allowed by the product and bank;
- amendment of beneficiary only by bank-authorized process;
- settlement only to the authoritative beneficiary or permitted holder.

**Status:** proposed.

### 2.6 Product-purpose binding

**Need:** capacity cannot be used for an unspecified purpose.

Representative fields:

- product type;
- commercial contract or transaction reference;
- issue date;
- expiry or maturity;
- governing rules or legal context;
- reimbursement source;
- evidence package.

**Status:** proposed.

### 2.7 Contingent versus crystallized exposure

**Need:** a guarantee not yet called is not the same state as a bank-paid claim.

The facility should distinguish:

- contingent notional;
- bank-recognized capacity consumption;
- pending claim;
- paid amount;
- reimbursement due;
- overdue reimbursement;
- default and enforcement exposure.

**Status:** proposed.

### 2.8 Claim and presentation state

**Need:** bank obligations may change state through an external presentation, demand, maturity, or settlement event.

Argent must not determine legal validity itself. It records:

- presentation or claim identity;
- evidence reference;
- bank-authorized decision;
- payment or refusal reference;
- capacity and reimbursement effect.

**Status:** proposed.

### 2.9 Obligation amendment control

**Need:** amendments may change amount, expiry, beneficiary, conditions, or capacity.

Required order:

```text
amendment requested
-> bank risk and capacity check
-> additional capacity reserved if required
-> authoritative instrument amended
-> protocol state updated
```

An amendment must not reduce collateral or release capacity before the amended instrument is effective and correctly represented.

**Status:** proposed.

### 2.10 Discharge and release control

**Need:** reserve capacity should return only after the bank confirms that the obligation is discharged.

Discharge evidence may include:

- expiry without valid claim;
- return or cancellation;
- beneficiary release;
- full reimbursement;
- bank system closure;
- final settlement.

**Status:** proposed.

---

## 3. Bank-owned configuration controls

These controls belong to bank policy and should be versioned and evidenced rather than hard-coded as universal law.

### 3.1 Product capacity factors

A bank may consume different internal capacity for equal face amounts depending on:

- product type;
- tenor;
- currency;
- beneficiary;
- jurisdiction;
- claim probability;
- collateral volatility;
- regulatory treatment.

### 3.2 Concentration

Limits may apply by:

- owner or group;
- custodian;
- vault;
- jurisdiction;
- refiner;
- product type;
- maturity bucket;
- currency;
- beneficiary;
- valuation source.

### 3.3 Warning and action bands

The bank may define:

- warning threshold;
- new-issue suspension threshold;
- margin-call threshold;
- cure period;
- forced-reduction threshold;
- enforcement threshold.

### 3.4 Evidence freshness

Different evidence may have different maximum ages:

- valuation;
- custody statement;
- insurance;
- legal opinion;
- sanctions screening;
- beneficial ownership;
- assay or inspection.

### 3.5 Role and approval policy

Each action may require:

- initiator role;
- approver groups;
- quorum;
- amount threshold;
- auto-reject timeout;
- independent release or enforcement authority;
- bank and custodian separation.

---

## 4. High-value later controls

### 4.1 Dynamic guarantee reduction

Capacity may reduce as verified project milestones are completed.

Risk:

- milestone evidence may be disputed;
- the beneficiary and bank must authorize reduction;
- protocol evidence cannot replace legal amendment.

### 4.2 Controlled collateral optimization

Select the lowest-cost eligible collateral combination based on:

- haircut;
- liquidity;
- concentration;
- vault;
- currency;
- tenor;
- substitution cost.

Risk:

- optimization must not create a legal or operational unsecured interval.

### 4.3 Selective-disclosure capacity proof

Prove sufficient bank-approved free capacity without exposing:

- total reserve;
- bar serials;
- unrelated obligations;
- bank risk policy.

Risk:

- the proof must be bound to a current policy, valuation, and facility state.

### 4.4 Electronic trade-document trigger

Use authoritative electronic bill-of-lading or warehouse-receipt state as an input to a bank decision.

Risk:

- control, title, integrity, and legal recognition belong to the document system and law;
- a document hash alone is insufficient.

### 4.5 Group sublimits

Allow a parent reserve to support approved subsidiaries.

Risk:

- corporate benefit;
- third-party security;
- transfer pricing;
- reimbursement allocation;
- insolvency and priority;
- group concentration.

### 4.6 Multi-bank portability

Move a facility from one bank to another through ordered release and re-pledge.

Risk:

- never allow simultaneous competing security;
- intercreditor and priority complexity;
- bank and custodian coordination.

---

## 5. Controls that should remain outside Argent

Argent should not decide:

- whether to approve the customer;
- whether to issue a guarantee or documentary credit;
- whether a claim or presentation is legally valid;
- regulatory capital or liquidity treatment;
- accounting classification;
- sanctions or AML disposition;
- asset market value independently of an approved source;
- legal perfection;
- corporate benefit;
- physical truth beyond signed source evidence;
- final enforcement rights.

The protocol may require authoritative evidence of those decisions. It should not replace the authority that owns them.

---

## 6. Controls that should never be built

### 6.1 Freely transferable capacity token

Why not:

- could resemble private money or an unauthorized financial claim;
- separates capacity from bank underwriting and beneficiary purpose;
- creates transfer, holder, insolvency, and securities questions;
- undermines the no-cash-draw boundary.

### 6.2 Operator override of institutional roles

Why not:

- destroys separation of duties;
- makes the shared evidence operator-controlled;
- creates unacceptable custody and bank risk.

### 6.3 Automatic physical enforcement by smart contract

Why not:

- the contract cannot possess or sell physical gold;
- enforcement is governed by law, documents, custody, and human authority;
- disputes and insolvency can alter the path.

### 6.4 Public disclosure of full reserve data

Why not:

- commercial sensitivity;
- security and theft risk;
- client confidentiality;
- bank secrecy and data protection;
- unnecessary for most verification.

### 6.5 Cross-chain duplication of the reserve claim

Why not:

- duplicate state and bridge risk;
- conflicting encumbrance representations;
- unclear authority;
- weakens the one-authoritative-state principle.

---

## 7. Prioritization

### Immediate

1. DFNS institutional signing and reconciliation.
2. Generic facility and capacity view.
3. Pre-issuance reservation.
4. Product and group sublimits.
5. No unrestricted cash draw.

### First obligation product

6. Guarantee lifecycle.
7. Claim, payment, reimbursement, and discharge.

### Next

8. Documentary credit lifecycle and bank-system adapter.
9. Selective-disclosure evidence.
10. Group treasury profile.

### Later

11. Treasury exposure.
12. electronic trade documents;
13. optimization and automation;
14. funded auto-collateralisation where explicitly required.

---

## 8. Status rule

Every proposed control must be labeled as one of:

- implemented in current contracts;
- implemented in service only;
- specified but not built;
- external bank or custodian control;
- research only;
- prohibited by product design.

This prevents product positioning from outrunning the implementation.

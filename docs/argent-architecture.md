# Argent: Technical Architecture

**Gold-backed obligation infrastructure with a tested Soroban collateral-control reference branch.**

**Architecture version:** 2.0  
**Implementation status:** Stellar testnet, 224 contract tests  
**Institutional signing direction:** DFNS-governed organization wallets, policies, approvals, and reconciliation  
**Canonical product direction:** `reserve-obligation-infrastructure.md`  
**Target domain profile:** `obligation-facility-profile.md`  

This document is self-contained. It distinguishes what is implemented from what is designed next. Contract source and tests remain authoritative for current behavior.

---

## 1. Executive summary

Argent is a Soroban-first control protocol for physical reserves that remain in professional custody. The first reference asset is allocated gold because it is individually identifiable, high-value, professionally vaulted, independently valued, and already used in secured-finance structures.

The mature product direction is a **corporate reserve obligation facility**:

> One controlled reserve supports many purpose-bound, bank-issued obligations while one authoritative state prevents over-allocation, premature release, and unauthorized action.

The bank may issue approved guarantees, documentary credits, supplier undertakings, regulatory security, treasury exposures, and other beneficiary-specific instruments. The reserve owner cannot withdraw unused facility capacity as unrestricted cash. The gold remains in custody and ownership remains with the company subject to the security interest, unless a valid adverse event and enforcement process results in realization.

The current open-source contracts implement a narrower but foundational **secured-credit reference profile**. They prove:

- instrument eligibility;
- position and lot identity;
- exclusive pledge;
- borrowing-base and capacity calculation;
- utilization and reversal;
- atomic repayment;
- collateral adjustment;
- margin, suspension, default, cure, and enforcement ordering;
- dual-control release;
- canonical event emission and replay.

The product extension generalizes the facility and exposure objects. It does not replace those controls.

The core architectural sentence is:

> **Soroban records and enforces the shared control state. DFNS governs which institution may authorize each act. Bank, custodian, legal, trade-document, and settlement systems remain authoritative for their own domains.**

---

## 2. Product direction and implementation boundary

### 2.1 Target product profile

```text
Allocated reserve
    -> bank-controlled pledge
    -> approved master capacity
    -> purpose-bound reservation
    -> bank-issued obligation
    -> expiry, reimbursement, claim, or enforcement
    -> capacity restored or collateral realized
```

Target obligation families include:

- documentary credits;
- bid, performance, advance-payment, retention, warranty, and regulatory guarantees;
- supplier payment undertakings;
- accepted or avalised maturity instruments;
- treasury and hedging exposure;
- other approved bank obligations.

### 2.2 Implemented reference profile

```text
Position registered
    -> exact lots selected
    -> custodian immobilizes
    -> pledge activated
    -> credit line opened
    -> utilization recorded
    -> repayment settled
    -> bank authorizes release
    -> custodian confirms release
```

The implemented branch is not presented as a guarantee or documentary-credit engine. It is the first executable profile of the common reserve-control protocol.

### 2.3 Why the relationship is credible

A bank obligation and a funded credit line both require:

- eligible collateral;
- exclusive control;
- an approved capacity limit;
- correct actor authority;
- exposure tracking;
- controlled amendment or substitution;
- no release while exposure remains;
- reimbursement after payment;
- default and enforcement evidence.

The next phase adds typed obligations, beneficiaries, sublimits, reservation states, claims, and discharge. It does not require a new physical-collateral identity or enforcement foundation.

---

## 3. Product boundary

### 3.1 What Argent is

- a shared control record for identified physical reserves;
- a protocol for role-authorized lifecycle transitions;
- an authoritative capacity and allocation state;
- an evidence and reconciliation layer between institutions;
- a sidecar to bank, custody, trade-finance, treasury, document, and settlement systems.

### 3.2 What Argent is not

- not tokenized gold;
- not a bank, lender, guarantor, or documentary-credit issuer;
- not a custodian;
- not an accounting system;
- not a private currency, CBDC, stablecoin, or freely transferable capacity token;
- not a card processor;
- not a legal-enforcement engine;
- not a substitute for KYC, sanctions screening, underwriting, legal opinions, or bank product documentation.

### 3.3 Legal and operational authority

| Domain | Authority |
|---|---|
| Gold title and entitlement | legal documents and owner records |
| Physical possession, segregation, movement, and release | custodian |
| Security interest and enforcement rights | governing agreements and applicable law |
| Product issue, amendment, claim, and payment | bank product system and bank authority |
| Trade-document control and presentation | authoritative document system and bank process |
| Accounting and regulatory exposure | bank and company systems |
| Shared control state and evidence ordering | Argent / Soroban |
| Institutional permission and approval | DFNS or equivalent governed signing layer |

Where protocol state conflicts with authoritative legal documentation, the legal documentation governs. The conflict should be recorded and escalated rather than hidden.

---

## 4. Actors and responsibilities

| Actor | Real-world responsibility | Current reference actions | Target obligation actions |
|---|---|---|---|
| Reserve owner / applicant | owns the reserve; requests bank use | register framework, select collateral, request adjustment, repay | request instrument, reimburse, request substitution or release |
| Custodian | holds and controls the physical asset | confirm and immobilize, confirm adjustment, confirm release, confirm realization | attest reserve, maintain control, confirm substitution, release, and realization |
| Bank / issuer | owns the credit and product decision | admit instrument, activate pledge, open line, suspend, release, default, enforce | approve facility, sublimits, reservations, obligations, claims, payment, release, and enforcement |
| Beneficiary | receives the bank obligation | not represented in current branch | receives instrument; may present or claim under its terms |
| Verifier | supplies defined evidence | eligible-use or utilization evidence where configured | trade, milestone, invoice, or document evidence only |
| Valuation source | supplies price or valuation | valuation reference input | reserve and exposure valuation input |
| Sponsor | optional rewards overlay | campaign and claims | outside core obligation profile |
| Operator | builds, submits, indexes, and renders | service and transaction orchestration | same, with no discretionary bank or custodian authority |
| Auditor / supervisor | reviews evidence | read-only | selective, read-only, or regulator-authorized view |

The operator never signs as the bank, custodian, or owner.

---

## 5. High-level architecture

```text
Owner / applicant       Bank       Custodian       Beneficiary / verifier
       |                   |             |                    |
       | requests, approvals, attestations, presentations    |
       +-------------------+-------------+--------------------+
                                   |
                                   v
                     Argent application and adapters
        domain validation | policy preflight | evidence binding
        approval queue | reconciliation | event indexer | certificates
                                   |
                    +--------------+--------------+
                    |                             |
                    v                             v
             DFNS governance                Authoritative systems
      permissions | policies | approvals    bank product | custody
      MPC/HSM signing | webhooks             documents | accounting
                    |                             |
                    +--------------+--------------+
                                   |
                                   v
                           Stellar / Soroban
              credit_ledger | settlement_vault | rewards_ledger
              role auth | deterministic refusal | canonical events
                                   |
                                   v
                             Evidence layer
           transaction | event | approval | policy | document reference
```

The architecture is intentionally modular. A production bank does not have to move its complete trade-finance or collateral system onto Stellar. Argent supplies one shared control layer and bank-readable evidence.

---

## 6. Current contract architecture

### 6.1 `credit_ledger`

The control core. It implements:

- framework and party governance;
- instrument registry and eligibility admission;
- position and lot registration;
- custody confirmation and immobilization;
- pledge activation;
- credit-line creation;
- utilization and reversal;
- valuation and margin checks;
- suspension and resumption;
- collateral adjustment;
- repayment application through the approved vault;
- release;
- default, cure, enforcement readiness, and enforcement recording;
- canonical collateral and governance events.

### 6.2 `settlement_vault`

The only current value-moving contract. It transfers the configured Stellar settlement asset and applies repayment to the bound credit ledger atomically.

It does not release collateral automatically. Release remains a separate bank-and-custodian control process.

### 6.3 `rewards_ledger`

An optional sponsor-funded rewards overlay. It is separated from pledged collateral and is not required for the obligation infrastructure thesis.

---

## 7. Current function map

### Governance and eligibility

- `initialize`
- `register_framework`
- `approve_party`
- `revoke_party`
- `register_instrument`
- `admit_instrument`
- `retire_instrument`

### Position and pledge

- `register_position`
- `select_lot_for_collateral`
- `confirm_and_immobilize`
- `activate_pledge`

### Secured-credit reference branch

- `open_credit_line`
- `record_drawdown`
- `reverse_drawdown`
- `revalue_and_check`
- `bank_suspend_line`
- `bank_resume_line`
- `apply_repayment`

### Adjustment and release

- `request_collateral_adjustment`
- `custodian_confirm_adjustment`
- `bank_approve_adjustment`
- `bank_authorize_release`
- `custodian_confirm_release`

### Default and enforcement

- `issue_default_notice`
- `cure_default`
- `open_enforcement_readiness`
- `populate_enforcement_readiness`
- `expire_enforcement_readiness`
- `record_enforcement`

### Atomic settlement

- `settle_repayment`

### Read and replay

The contracts expose getters for framework, instrument, position, selection, pledge, line, valuation, drawdown, repayment, adjustment, custody control, and enforcement readiness, plus sequence getters for canonical replay.

---

## 8. Current lifecycle and enforced order

### 8.1 Origination

```text
framework registered
-> parties approved
-> instrument registered and admitted
-> position registered
-> lot selected
-> custodian confirms and immobilizes
-> bank activates pledge
-> bank opens credit line
```

### 8.2 Utilization and repayment

```text
utilization recorded
-> valuation and margin monitored
-> settlement vault transfers value
-> credit ledger applies repayment
```

### 8.3 Collateral adjustment

The enforced order is:

```text
owner requests
-> custodian confirms
-> bank approves
```

The bank does not approve before the custodian confirms the replacement state.

### 8.4 Release

```text
exposure cleared
-> bank authorizes release
-> custodian confirms release
```

Repayment alone never releases the collateral.

### 8.5 Adverse path

```text
default notice
-> cure period
-> cure or enforcement readiness
-> enforcement evidence populated
-> enforcement outcome recorded
```

This ordering is reusable for obligations after a bank payment creates a reimbursement exposure.

---

## 9. Target obligation architecture

The next profile introduces a facility layer above the current collateral core.

### 9.1 New target objects

- MasterFacility;
- ProductSublimit;
- FacilityParticipant;
- CapacityReservation;
- BankObligation;
- Beneficiary;
- Presentation;
- Claim;
- SettlementInstruction;
- Reimbursement;
- LegalContext;
- EvidencePackage.

These names are design objects, not claims of current contract types.

### 9.2 Target lifecycle

```text
obligation requested
-> policy validated
-> capacity reserved
-> bank approved
-> instrument issued
-> active
-> expired, cancelled, presented, claimed, or matured
-> reimbursement or discharge
-> capacity released
```

### 9.3 Target no-cash-draw invariant

The facility owner cannot withdraw free capacity into a general account. Value movement must be linked to a named beneficiary and a bank-authorized obligation or settlement instruction.

### 9.4 Exposure classes

The target state distinguishes:

- active contingent allocation;
- pending presentation or claim;
- bank-paid or funded exposure;
- reimbursement due;
- default and enforcement exposure;
- released capacity.

### 9.5 Backward compatibility

The existing credit line becomes one implemented facility profile. A migration or adapter should preserve current identifiers, event replay, and state while exposing a generalized facility view.

---

## 10. Why Stellar

### 10.1 Role-specific authorization

Soroban `require_auth` binds each state transition to the address whose authority is required. Independent authorization entries allow separate institutions to authorize their own acts without one party holding a shared private key.

### 10.2 Deterministic refusal

The contracts make certain states impossible:

- unadmitted instrument use;
- duplicate pledge;
- unsafe initial LTV;
- unauthorized repayment application;
- premature release;
- invalid enforcement order.

The mature profile extends that refusal model to duplicate reservation, issue without capacity, unauthorized beneficiary, and unrestricted cash draw.

### 10.3 Atomic settlement where needed

The current vault proves that settlement value and exposure state can move together. The obligation profile can use the same pattern for reimbursement or beneficiary settlement where the participating bank selects an on-chain settlement asset.

The protocol does not require every bank payment to occur on Stellar. Off-chain settlement may be referenced and reconciled instead.

### 10.4 Shared event evidence

The event stream allows institutions to reconstruct the agreed control state without treating the operator's database as the source of truth.

---

## 11. DFNS governance architecture

The contracts are signer-agnostic. The production signer layer must support institutional controls.

### 11.1 DFNS responsibilities

- organization-controlled wallets;
- permissions and role assignment;
- policy evaluation before signing;
- approval groups and quorums;
- pending, approved, denied, and expired approval states;
- MPC or supported HSM-backed signing;
- webhooks and audit evidence.

### 11.2 Precision about policies

DFNS policies add restrictions; an unconfigured organization is not automatically deny-by-default. Delegated wallets bypass organization policy. Argent must therefore construct a deliberate policy topology for each governed role.

### 11.3 Representative role topology

#### Bank

- facility authority;
- credit or product authority;
- release authority;
- claims and payment authority;
- enforcement authority.

#### Custodian

- custody and immobilization authority;
- adjustment or substitution authority;
- release and realization authority.

#### Operator

- fee sponsorship and submission where approved;
- no business authorization for another party.

### 11.4 Reusable integration output

The reusable contribution is not only a wallet connection. It is:

- a Soroban-aware signing adapter;
- decoded authorization intent;
- policy-to-method and role mapping;
- approval-to-auth-entry and transaction reconciliation;
- evidence that links institutional approval, signature, Stellar transaction, and resulting event.

---

## 12. Data and privacy boundary

### On-chain or shared state

- identifiers and role addresses;
- policy version;
- committed evidence hashes;
- eligibility and capacity values needed for shared control;
- lifecycle state;
- canonical events;
- transaction and sequence evidence.

### Off-chain or restricted state

- bar serials and full bar list;
- KYC, sanctions, and personal data;
- complete legal agreements and opinions;
- beneficiary commercial terms;
- bank credit model and pricing;
- trade documents;
- claims, disputes, and enforcement files.

A production deployment should support selective disclosure. A beneficiary may verify instrument authenticity and recorded capacity sufficiency without seeing the owner's total reserve or unrelated obligations.

---

## 13. External integration architecture

### Bank systems

- credit and facility approval;
- limits and collateral management;
- trade-finance and guarantee platform;
- treasury and derivatives platform;
- accounting and general ledger;
- regulatory reporting.

### Custody and valuation

- vault or warehouse records;
- assay and inspection evidence;
- valuation and pricing;
- insurance and location evidence;
- release and realization.

### Trade documents

- electronic bill of lading;
- warehouse receipt;
- promissory note or bill;
- invoice and inspection evidence;
- other authoritative transferable-record systems.

### Settlement

- commercial-bank money;
- stablecoin or tokenized deposit where approved;
- central-bank or CBDC rail where available;
- off-chain payment reference.

Argent uses adapters and evidence commitments. It does not duplicate the same legal asset claim across ledgers.

---

## 14. Evidence and audit model

Every accepted transition should be reconstructable from:

- intent;
- actor and role;
- institutional permission;
- policy evaluation;
- approval decisions;
- signed authorization entry;
- transaction hash;
- canonical event;
- evidence package reference;
- resulting state.

Evidence certificates should state both what they prove and what they do not prove.

They may prove:

- the protocol recorded a custodian-authorized immobilization;
- the bank-authorized pledge or release state;
- sufficient recorded capacity under a named policy version;
- the event sequence and transaction evidence.

They do not independently prove:

- physical truth beyond the source attestation;
- legal perfection;
- documentary compliance;
- bank solvency;
- claim validity;
- accounting or regulatory treatment.

---

## 15. Security model

Primary protected properties include:

- no duplicate use of the same collateral lot;
- no use of unapproved collateral;
- no exposure above approved capacity;
- no state change by the wrong role;
- no repayment by an unbound vault;
- no release before bank authorization;
- no release while exposure remains;
- no enforcement before required cure conditions;
- replayable and attributable evidence.

Target-profile additions include:

- no issue without prior reservation;
- no duplicate capacity allocation;
- no unrestricted customer cash draw;
- no release with pending claims or unpaid reimbursement;
- no silent policy or document substitution;
- no operator-created bank obligation.

See `threat-model-and-security-boundaries.md` for the detailed adversary and trust analysis.

---

## 16. Testing and verification

The current repository contains **224 passing contract tests**:

- `credit_ledger`: 162;
- `rewards_ledger`: 45;
- `settlement_vault`: 17.

Coverage includes:

- role authorization and party revocation;
- instrument eligibility;
- double-pledge refusal;
- borrowing-base and LTV constraints;
- stale and future-dated valuation refusal;
- repayment and duplicate-reference safety;
- no release with outstanding exposure;
- default, cure, and enforcement order;
- collateral adjustment order and post-adjustment coverage;
- canonical event schema and replay.

Typed obligations and the no-cash-draw profile require a separate future test matrix. They are not included in the 224 count.

---

## 17. Delivery sequence

### Stage 1 - institutionalize the current proof

- DFNS role wallets;
- permission and policy topology;
- approval queue and webhooks;
- Soroban authorization signing;
- approval-to-transaction reconciliation;
- mainnet reference deployment and runbook.

### Stage 2 - generalize facility and capacity

- generic master facility view;
- product and group sublimits;
- pre-issuance reservation;
- contingent versus crystallized exposure.

### Stage 3 - implement typed obligations

- guarantees;
- documentary credits;
- supplier undertakings;
- treasury exposure;
- issue, amend, expire, claim, pay, reimburse, and discharge.

### Stage 4 - connect authoritative systems

- bank product adapter;
- custodian adapter;
- electronic trade-document adapter;
- settlement and reimbursement adapter;
- selective-disclosure evidence.

---

## 18. Ecosystem contribution

The open contribution is broader than one gold facility:

1. a canonical event model for physical-reserve control;
2. a Soroban-aware DFNS institutional authorization adapter;
3. a role and approval blueprint for multi-party financial workflows;
4. an evidence reconciliation pattern linking intent, approval, signature, transaction, event, and state;
5. a target obligation-capacity profile that other regulated Stellar applications can reuse or adapt.

This makes Stellar useful not only for token issuance or payments, but for institutionally governed rights and obligations around off-chain assets that should remain in custody.

---

## 19. Non-goals

Argent does not aim to:

- tokenize title to the gold;
- issue a private currency;
- create public DeFi liquidation of physical collateral;
- make a bank product legally effective solely through an on-chain event;
- replace trade-finance, custody, accounting, or regulatory systems;
- expose private bar and facility data publicly;
- claim that all target obligation types are already implemented.

---

## 20. Summary

Argent is becoming a reserve-obligation control protocol, not a general-purpose gold-backed cash line.

The current contracts already prove the shared physical-collateral foundation. The next product profile adds purpose-bound bank obligations, capacity reservations, beneficiaries, sublimits, claims, reimbursement, and a no-unrestricted-cash-draw rule.

> **One reserve. Many obligations. One authoritative capacity state.**

---

## References

- Stellar authorization: https://developers.stellar.org/docs/learn/fundamentals/contract-development/authorization
- Stellar contract-invocation signing: https://developers.stellar.org/docs/build/guides/transactions/signing-soroban-invocations
- DFNS policies: https://docs.dfns.co/core-concepts/policies
- DFNS Stellar signing: https://docs.dfns.co/api-reference/sign/stellar
- Daml Finance asset model: https://docs.daml.com/daml-finance/concepts/asset-model.html
- Daml Finance settlement: https://docs.daml.com/daml-finance/concepts/settlement.html
- Daml Finance lifecycling: https://docs.daml.com/daml-finance/concepts/lifecycling.html
- ICC UCP 600: https://library.iccwbo.org/content/tfb/RULES/tfb-ucp600-rules.htm
- UNCITRAL MLETR: https://uncitral.un.org/en/texts/ecommerce/modellaw/electronic_transferable_records

No source reference implies partnership, endorsement, legal equivalence, or production compatibility.

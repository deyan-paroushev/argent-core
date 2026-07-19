# Integration and Interoperability: How Argent Sits Beside Existing Institutional Systems

**Status:** architecture and integration strategy. Current Soroban contracts implement the secured-credit reference branch; the reservation, bank-obligation, and advanced privacy flows described here are target-profile capabilities unless explicitly identified as current.

---

## 1. The governing principle

Argent should **integrate, not replace**.

A bank, custodian, trade-finance platform, treasury system, accounting ledger, document network, and settlement rail each remain authoritative for the domain they already operate. Argent provides the bounded shared-control layer between them:

```text
identify the reserve
-> verify current control and eligibility
-> calculate available capacity
-> preflight the requested bank product
-> reserve capacity without double use
-> reconcile institutional approvals
-> bind the authoritative issue outcome
-> track exposure, reimbursement, release, or enforcement
```

The value is not a second core-banking system. It is one authoritative capacity and control state that existing systems can consume and reconcile.

> **Move instructions, proofs, and capacity state. Do not create duplicate legal claims over the reserve.**

---

## 2. Systems of authority

Argent must name the authoritative system for every material fact.

| Domain | Authoritative source | Argent responsibility |
|---|---|---|
| Legal ownership and security | governing legal documents and applicable registry or control arrangement | commit evidence identity and record authorized lifecycle acts |
| Physical existence and custody | custodian, vault, warehouse, or collateral manager | record signed custody, immobilization, substitution, release, and realization events |
| Facility and product approval | bank credit, limits, trade-finance, guarantee, or treasury system | enforce the approved policy version and capacity allocation |
| Product issuance and wording | bank product system | correlate request and reservation to the definitive issue or rejection result |
| Trade-document control | legally recognized document or transferable-record system | reference the authoritative state and evidence; never infer title from a hash alone |
| Valuation and eligibility | bank-approved price and collateral-policy sources | enforce freshness, treatment, haircut, sublimit, and refusal rules |
| Payment and reimbursement | bank core, payment rail, or selected on-chain settlement asset | bind settlement reference to exposure state and reconcile final outcome |
| Institutional signing | DFNS or equivalent governed signer | require the correct Soroban role authorization and preserve approval evidence |
| Shared protocol state | Soroban contracts and canonical events | provide deterministic state transitions, refusals, replay, and evidence references |

When authorities disagree, Argent must expose the disagreement and block unsafe progression. It must not silently choose whichever source is most convenient.

---

## 3. Available capacity is not necessarily usable capacity

A reserve may be present, eligible, valued, and apparently unallocated, yet still be unusable for a proposed obligation.

Argent should distinguish:

```text
gross reserve value
-> eligible collateral value
-> approved facility capacity
-> available capacity
-> reservable capacity
-> reserved capacity
-> issuable capacity
-> deliverable capacity
-> active obligation capacity
-> releasable capacity
```

A request becomes **issuable** only after the relevant applicant, beneficiary, product, tenor, currency, jurisdiction, evidence, sublimit, and approval checks pass.

It becomes **deliverable** only when the required operational route is available: the bank product system can accept and decide the request, required signers and callbacks are available, mandatory evidence is current, and the originating system can receive a definitive result.

The detailed state and reason-code model is defined in [`capacity-reservation-and-deliverability.md`](capacity-reservation-and-deliverability.md).

---

## 4. The orchestration transaction

A target production request should follow one correlated transaction envelope.

### 4.1 Request

The originating system supplies:

- request ID and idempotency key;
- facility, applicant, and requested product;
- beneficiary and commercial purpose;
- amount, currency, tenor, expiry, or maturity;
- required evidence references;
- requested response deadline;
- originating system and callback route.

### 4.2 Preflight

Argent evaluates or obtains authoritative decisions for:

- reserve and facility status;
- current eligible and available capacity;
- product and entity sublimits;
- applicant and beneficiary permission;
- product, currency, jurisdiction, tenor, and concentration rules;
- custody, valuation, document, and policy freshness;
- institutional approval route;
- bank product-system readiness;
- settlement and reimbursement path.

### 4.3 Reservation

A positive preflight may create a purpose-bound provisional reservation. Creation and capacity reduction must be atomic. The reservation includes its amount, policy version, purpose, applicant, beneficiary, product, expiry, and correlation identifiers.

### 4.4 Institutional approval

DFNS or an equivalent signer governs whether the bank, custodian, verifier, or operator role may sign. Soroban independently checks whether the signed transition is permitted by contract state.

### 4.5 Product-system issue

The bank product system remains authoritative for issuance. Argent sends or references the approved instruction and waits for a correlated result:

- issued;
- definitively rejected;
- still pending;
- ambiguous and requiring status reconciliation.

A timeout is not a rejection. Argent must retain committed capacity until the authoritative product system establishes the outcome.

### 4.6 Definitive response

The originating system receives a machine-readable response containing:

- final or current status;
- request, reservation, facility, and obligation IDs;
- reason codes;
- policy and evidence version;
- DFNS approval reference where applicable;
- Soroban transaction and event reference where applicable;
- authoritative bank product reference;
- next required action.

---

## 5. Adapter architecture

### 5.1 Bank facility and limits adapter

Responsibilities:

- customer and group eligibility;
- facility and sublimit state;
- collateral treatment and capacity-consumption rules;
- product eligibility;
- hold, decline, and exception decisions;
- exposure and accounting references.

### 5.2 Bank trade-finance and guarantee adapter

Responsibilities:

- product preflight;
- request and reservation correlation;
- issue, amendment, cancellation, expiry, presentation, claim, payment, and discharge callbacks;
- instrument and beneficiary references;
- governing-rule and document requirements;
- definitive product-system status.

Argent does not author instrument wording or decide documentary compliance.

### 5.3 Custodian, vault, or warehouse adapter

Responsibilities:

- position identity and owner;
- allocation or segregation state;
- location, quantity, quality, and control restrictions;
- immobilization, substitution, release, legal hold, and realization;
- statement freshness and evidence hash;
- signed discrepancy and exception status.

The custodian remains the physical root of truth.

### 5.4 `SharedGoldAssuranceAdapter`

This optional upstream adapter consumes authoritative reserve assertions from a custodian, provenance system, pooled-gold register, digital-gold product operator, or future shared gold platform.

The adapter should produce a source-correlation object such as:

```text
SharedGoldAssuranceSnapshot
- reserve_profile
- authoritative_record_id
- source_product_id
- owner_or_entitlement_holder
- custodian_or_product_operator
- quantity_and_unit
- purity_or_economic_gold_equivalent
- backing_and_allocation_status
- assurance_provider
- assurance_scope
- assurance_timestamp
- assurance_expiry
- source_tolerance_status
- economic_equivalence_class
- legal_equivalence_class
- operational_equivalence_class
- redemption_or_realisation_pathway
- known_hold_or_encumbrance_status
- discrepancy_status
```

Required boundary:

- the upstream system remains authoritative for physical backing, custody, ownership or entitlement, reconciliation, and redemption within its published scope;
- participating issuers remain responsible for their product interface, commercial terms, distribution, and implementation;
- Argent remains authoritative for facility encumbrance, bank eligibility treatment, reservation, obligation allocation, and release state;
- a verified upstream record does not create a pledge, prove perfection, or force bank eligibility;
- asset-layer fungibility does not make a facility reservation transferable or reusable;
- no upstream token, balance, or pooled interest is duplicated as a competing asset representation on Stellar;
- stale, expired, tolerance-breached, or discrepant data stops new reservation and issuance but does not silently erase active exposure.

See [`shared-gold-infrastructure-and-argent.md`](shared-gold-infrastructure-and-argent.md).

### 5.5 Valuation and collateral-policy adapter

Responsibilities:

- price and timestamp;
- eligible quantity;
- haircut and advance rate;
- concentration and wrong-way-risk treatment;
- maintenance, margin, and cure state;
- policy version and exception authority.

### 5.6 Trade-document adapter

Responsibilities:

- authoritative document identifier;
- issuer, holder, beneficiary, or controller where applicable;
- operative status;
- issue, endorsement, transfer, surrender, cancellation, or presentation state;
- document-system evidence reference.

A document hash proves integrity against a supplied document. It does not by itself prove legal possession, control, or transfer.

### 5.7 Settlement and reimbursement adapter

Responsibilities:

- beneficiary payment reference;
- reimbursement source and status;
- crystallized exposure;
- settlement finality according to the selected rail;
- duplicate-payment prevention;
- reconciliation and exception state.

Soroban finality proves a Soroban transaction. It does not automatically prove finality in a bank ledger, payment system, court process, or custody book.

### 5.8 DFNS and signer adapter

Responsibilities:

- organization and role-wallet mapping;
- permission and policy evaluation;
- approval groups and quorums;
- payload decoding and human-readable intent;
- signature generation;
- pending, approved, denied, expired, submitted, and confirmed states;
- webhook or polling reconciliation;
- approval-to-auth-entry and transaction evidence.

### 5.9 Soroban event and state adapter

Responsibilities:

- canonical event ingestion;
- ledger cursor and gap detection;
- state replay;
- transaction, event, and evidence correlation;
- divergence alerts;
- durable archive outside recent-ledger query windows.

---

## 6. Interoperability identifiers

Every participating system should preserve stable identifiers for:

- legal entity and role;
- facility and sublimit;
- reserve asset, position, and pledge;
- request and idempotency key;
- reservation;
- bank obligation;
- beneficiary;
- trade document;
- presentation or claim;
- payment and reimbursement;
- DFNS approval;
- Soroban transaction and event;
- evidence package and policy version.

Identifiers should be namespaced and source-qualified. An identifier must not be silently recycled for a new commercial object.

---

## 7. Idempotency, retries, and ambiguous outcomes

Institutional systems retry. Networks time out. Webhooks are duplicated or arrive out of order. The adapter layer must treat these as normal operating conditions.

Required controls:

- stable idempotency key per originating command;
- canonical request digest;
- rejection when the same key is reused with different data;
- duplicate callback suppression;
- monotonic lifecycle version or authoritative timestamp;
- status-query fallback after lost callbacks;
- no blind resubmission after ambiguous issue or payment status;
- no release while product or settlement status is unresolved;
- manual repair workflow with full audit evidence.

---

## 8. Privacy and minimum disclosure

Interoperability must not turn Argent into a central store of every participant's confidential data.

### Shared minimum

- protocol and facility identifiers;
- role authority;
- policy and evidence commitments;
- capacity or sufficiency facts required for control;
- lifecycle status;
- sequence and transaction evidence.

### Restricted information

- bar serials and complete reserve book;
- customer KYC and sanctions files;
- bank credit model, pricing, and internal rating;
- beneficiary terms and trade documents;
- claims, disputes, legal opinions, and enforcement files;
- group-company exposure and reimbursement arrangements.

Each adapter and view should disclose only what the receiving role needs. Hashes are integrity commitments, not universal confidentiality controls. Stable identifiers, exact values, and event timing can create correlation leakage.

The canonical model is [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md).

---

## 9. Incremental adoption without re-platforming

Argent should be introduced in stages.

### Mode 1 - evidence-only mirror

- ingest approved sample or production evidence;
- generate hashes, state views, and certificates;
- no decision authority and no write-back.

### Mode 2 - shadow state

- replay capacity and lifecycle beside the bank and custodian systems;
- compare results;
- measure divergence and operational latency;
- no production block or release decision.

### Mode 3 - controlled decision gate

- use Argent preflight and refusal results for one bounded action;
- retain human approval and existing product-system issuance;
- fail closed on stale, inconsistent, or unavailable state.

### Mode 4 - limited write-back

- send one approved reservation, release, or evidence update to an existing system;
- require correlation, idempotency, and reconciliation;
- preserve manual fallback.

### Mode 5 - governed production integration

- operate one product, one bank, one custodian, and one reserve pool end to end;
- monitor service levels, reconciliation, privacy, and exception handling;
- expand only after the first route is stable.

This progression lets an institution begin with a single venue or product and expand without replacing its core systems.

---

## 10. Multi-ledger and multi-rail boundary

Argent may reference or coordinate systems on several ledgers or payment rails. It must not mint duplicate representations of the same bullion claim merely to make each ledger aware of the reserve.

The safe pattern is:

```text
one legal reserve and security state
+ one authoritative Argent capacity state
+ adapters carrying proofs, instructions, and outcomes
+ settlement references on the selected rail
```

A bridge or token wrapper cannot establish physical truth, legal perfection, bank approval, or custody control. Cross-system interoperability is therefore a reconciliation and authority problem before it is a token-transfer problem.

---

## 11. Operational monitoring

A production integration should monitor:

- adapter availability and latency;
- stale custody, valuation, policy, and document state;
- reservation age and expiry;
- issue and callback latency;
- unmatched or duplicated requests;
- Soroban indexer gaps;
- DFNS pending and expired approvals;
- disagreement between contract, bank, custodian, and evidence state;
- unauthorized disclosure attempts;
- age of the oldest unresolved exception.

The system should expose clear ownership and escalation for every exception class.

---

## 12. What this model is not

- not a replacement for core banking, limits, accounting, trade-finance, treasury, custody, or document systems;
- not a claim that every bank product can settle on-chain;
- not legal finality for an external instrument;
- not proof that the physical reserve exists beyond the authoritative attestation;
- not an automatic permission to issue when collateral value is sufficient;
- not a universal data lake for bank and customer documents;
- not a cross-chain duplication of the reserve;
- not a promise of straight-through production use before operational testing.

---

## 13. Recommended first integration

```text
one bank
+ one custodian
+ one reserve owner
+ one allocated bullion pool
+ one product family
+ one originating system
+ one preflight and reservation route
+ one authoritative issue callback
+ one reimbursement and release route
```

The purpose of the first integration is to prove authoritative boundaries, operational deliverability, privacy, and reconciliation. It is not to maximize the number of connected systems.

---

## 14. References

- Quant, "Unlocking collateral mobility: How tokenisation transforms settlement infrastructure," 2026: https://quant.network/perspectives/unlocking-collateral-mobility-how-tokenisation-transforms-settlement-infrastructure/
- Daml Finance asset model: https://docs.daml.com/daml-finance/concepts/asset-model.html
- Daml Finance settlement: https://docs.daml.com/daml-finance/concepts/settlement.html
- Corda UTXO token selection: https://docs.r3.com/en/platform/corda/5.2/developing-applications/api/ledger/utxo-ledger/token-selection.html
- Stellar authorization: https://developers.stellar.org/docs/learn/fundamentals/contract-development/authorization
- Stellar events: https://developers.stellar.org/docs/learn/fundamentals/contract-development/events
- DFNS policies: https://docs.dfns.co/core-concepts/policies
- DFNS policy approvals: https://docs.dfns.co/api-reference/policy-approvals
- DFNS webhooks: https://docs.dfns.co/developers/webhooks
- ISO 20022: https://www.iso20022.org/iso-20022
- ICC Digital Standards Initiative: https://www.dsi.iccwbo.org/
- UNCITRAL Model Law on Electronic Transferable Records: https://uncitral.un.org/en/texts/ecommerce/modellaw/electronic_transferable_records
- Digital Asset, Canton ledger privacy model: https://docs.digitalasset.com/overview/3.5/explanations/ledger-model/ledger-privacy.html

No source implies partnership, certification, endorsement, or production compatibility.

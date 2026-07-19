# Reserve-backed obligation infrastructure

**Canonical product direction for Argent.**

**Status:** product and protocol thesis; the complete obligation profile is not yet implemented in the current contracts  
**Implemented foundation:** physical-collateral identity, eligibility, pledge, capacity, utilization, repayment, release, default, cure, enforcement, canonical events  
**Target profile:** non-cash-drawable master facility for bank-issued obligations  
**First reserve asset:** allocated physical gold in professional custody  
**Infrastructure rail:** Stellar/Soroban, with institutional authorization governed through DFNS or an equivalent policy-controlled signer  

> **One reserve. Many obligations. One authoritative capacity state.**
>
> **Use gold to secure the promises. Keep cash for the payments.**

---

## 1. The product in one sentence

Argent enables a bank to convert identifiable, customer-owned allocated bullion into reusable, purpose-bound capacity for guarantees, documentary credits, supplier undertakings, regulatory security, treasury exposures, and other approved obligations while the gold remains in professional custody and unused capacity cannot be withdrawn as unrestricted cash.

This is not a claim that gold can do something cash is legally incapable of doing. Cash is normally the strongest and simplest collateral. The commercial distinction is that cash pledged or blocked as collateral is no longer available to run the business. A separately owned gold reserve can perform the assurance function while operating fiat remains available for payroll, taxes, ordinary expenses, reimbursement, margin, and final settlement.

The value proposition is therefore not "gold instead of money." It is:

```text
Gold reserve
    -> assurance and bank-obligation capacity

Operating fiat
    -> operations and final settlement
```

The two resources become complementary rather than competing uses of the same cash balance.

---

## 2. The problem

Companies repeatedly need more than payment liquidity. They need accepted promises.

Examples include:

- a bid bond before being allowed to tender;
- a performance guarantee before a contract becomes effective;
- an advance-payment guarantee before the customer releases project funds;
- a documentary credit before a supplier ships;
- a customs or regulatory guarantee before goods or a licence are released;
- a supplier payment undertaking before open-account terms are granted;
- an accepted future-payment instrument before a supplier extends time;
- a treasury or hedging limit before a bank accepts market exposure.

The conventional answer is often one of four things:

1. block cash;
2. consume an ordinary credit limit;
3. sell another asset and deposit the proceeds;
4. negotiate a separate collateral package for each instrument.

All four approaches fragment liquidity and duplicate work. A company may hold a substantial strategic reserve, yet still maintain several separate cash margins and bank facilities because the reserve is not represented as one controlled, reusable capacity pool.

Argent addresses that gap.

---

## 3. The reframe: from collateralized borrowing to obligation capacity

A conventional secured-credit product asks:

> How much cash may the company borrow against this asset?

Argent's mature product asks:

> Which bank-issued obligation is required to unlock this transaction, and how much reserve capacity must support it?

The distinction changes product behavior.

### Credit-first model

```text
Pledge reserve
-> draw cash
-> interest begins
-> customer decides where to spend it
-> repayment restores credit
```

### Obligation-first model

```text
Pledge reserve
-> approve master capacity
-> request a specific bank instrument
-> reserve capacity for a named purpose and beneficiary
-> bank issues or confirms the obligation
-> expiry, reimbursement, claim, or settlement resolves it
-> capacity is restored or enforcement begins
```

The customer does not receive a general-purpose balance. The bank supplies the precise institutional promise required by the external transaction.

---

## 4. Product boundary

### 4.1 Permitted product families

The master facility may support bank-approved instruments such as:

#### Trade and supplier instruments

- sight documentary credits;
- deferred-payment documentary credits;
- revolving, transferable, or back-to-back credits where approved;
- supplier payment undertakings;
- standby letters of credit;
- accepted drafts, avalised bills, or similar maturity obligations;
- forfaiting-ready bank-supported claims.

#### Contract assurance

- bid guarantees;
- performance guarantees;
- advance-payment guarantees;
- retention guarantees;
- warranty or defects-liability guarantees;
- delivery and completion guarantees.

#### Regulatory and operating security

- customs guarantees;
- tax, excise, warehouse, licence, utility, labour, or lease guarantees where accepted by the relevant beneficiary and permitted by law;
- other beneficiary-specific security instruments.

#### Treasury and market exposure

- FX forward limits;
- precious-metals and commodity hedging limits;
- interest-rate hedging limits;
- margin or collateral-transformation support;
- other approved contingent market exposures.

### 4.2 Prohibited customer behavior

The target product does not allow:

- withdrawal into the customer's general operating account;
- an unrestricted overdraft or revolving cash balance;
- cash-equivalent redemption of unused capacity;
- transfer of facility capacity to an unknown beneficiary;
- a freely transferable capacity token;
- anonymous bearer use;
- duplicate allocation of the same reserve capacity;
- movement or release of pledged bars without the required bank and custodian acts.

### 4.3 What may still create funded exposure

"Non-cash-drawable" does not mean the bank can never pay.

A funded reimbursement exposure may arise when:

- compliant documents are presented under a documentary credit;
- an accepted instrument reaches maturity;
- a valid guarantee claim is paid;
- a treasury position settles against the customer;
- another approved bank undertaking crystallizes.

The bank then has a reimbursement claim against the customer. Failure to reimburse may trigger cure and enforcement under the governing agreements.

---

## 5. The company value proposition

### 5.1 Preserve operating liquidity

The company does not immobilize the same cash it needs for payroll, taxes, ordinary expenses, emergency liquidity, or final settlement.

### 5.2 Retain the reserve asset

The company remains the legal owner subject to the security interest and control arrangement. It retains the residual economic value of the gold unless a valid default and enforcement process results in realization.

Ownership is not unrestricted while pledged. The company cannot withdraw, sell, move, or pledge the same bars elsewhere without satisfying the agreed release or substitution process.

### 5.3 Avoid unnecessary funded carry

A guarantee or documentary undertaking may require fees and bank capital, but it does not necessarily create the same funded-interest cost as drawing a cash loan from day one.

### 5.4 Unlock third-party resources

The bank obligation may release value from another party:

- a customer advance backed by an advance-payment guarantee;
- supplier time backed by a payment undertaking;
- a shipment under a documentary credit;
- retained project cash replaced by a retention guarantee;
- tender or regulatory access backed by an accepted bank promise.

The most efficient transaction may therefore obtain funding or access from the customer, supplier, project owner, or market rather than from a bank cash advance.

### 5.5 Reuse one reserve sequentially

When an obligation expires, is cancelled, reimbursed, or otherwise discharged, its reserved capacity becomes available for another approved use.

This is collateral velocity, not simultaneous double use.

### 5.6 Consolidate group treasury

Subject to legal, corporate-benefit, related-party, and bank approval, a holding company reserve may support controlled sublimits for approved subsidiaries. The reserve remains centrally governed instead of being fragmented into separate blocked deposits across operating entities.

---

## 6. Value for each institution

### 6.1 Bank

The bank gains:

- one controlled reserve relationship beneath several fee-generating products;
- clearer eligibility, valuation, capacity, and encumbrance evidence;
- product and group sublimits;
- purpose-bound utilization instead of unrestricted customer cash leakage;
- stronger reconciliation between credit, collateral, trade-finance, treasury, custody, and signing systems;
- evidence for approvals, amendments, release, default, and enforcement;
- deeper corporate, private-banking, bullion, trade-finance, and treasury relationships.

Argent does not decide the bank's credit policy or regulatory capital treatment. The bank owns the underwriting, product, pricing, haircut, concentration, and legal decisions.

### 6.2 Custodian

The custodian becomes an active security-infrastructure participant by providing signed evidence of:

- bar identity and allocation;
- custody and segregation;
- immobilization;
- release restrictions;
- substitution;
- realization and enforcement outcomes.

It retains physical control and does not become a lender or product issuer.

### 6.3 Beneficiary

The beneficiary receives a conventional bank obligation, not a claim on the gold. It need not evaluate the reserve, custody arrangement, haircut, or enforcement path. It relies on the bank instrument and its governing terms.

### 6.4 Reserve owner

The owner receives commercial access while preserving fiat and retaining the reserve subject to encumbrance.

### 6.5 Auditor, risk function, and supervisor

They receive one ordered evidence path showing:

- what reserve was eligible;
- which policy version applied;
- who approved each act;
- which obligation consumed capacity;
- whether the exposure remained covered;
- how the obligation was discharged or crystallized;
- when release or enforcement became valid.

---

## 7. Why allocated gold is the first asset

Gold is not selected because it always outperforms other collateral. It is selected because it combines several useful properties:

- globally observable market pricing;
- standardized wholesale forms and purity conventions;
- individual bar identity;
- professional vaulting and audit practices;
- high value density;
- no corporate issuer credit risk in the metal itself;
- an established sale and liquidation market;
- existing use in secured finance and collateral arrangements.

A cash balance is identifiable by account, not by durable constituent units. Allocated bullion can be bound to specific bars, evidence, custody, eligibility, and encumbrance states. That makes it suitable for an object-level control protocol.

Gold still has disadvantages:

- no contractual yield;
- price volatility;
- custody, insurance, assay, and control costs;
- margin-call risk;
- enforcement and liquidation latency;
- possible concentration and jurisdiction risk.

The protocol must make those limitations visible rather than market gold as risk-free.

---

### 7.1 Relationship to emerging shared gold infrastructure

The gold market is developing upstream infrastructure that can make reserve information more structured, trusted, and interoperable:

- LBMA Gold Bar Integrity for provenance, refiner assurance, and progressively richer vault reporting;
- Wholesale Digital Gold and Pooled Gold Interests for a proposed proprietary, transferable pooled-gold structure;
- the World Gold Council and BCG proposal for Gold as a Service, connecting physical custody with digital product issuance, reconciliation, assurance, and redemption.

These initiatives validate the need for shared operating infrastructure but do not replace Argent's bank-specific state.

```text
Gold-market infrastructure
    -> provenance, ownership, custody, backing, digital product records

Argent
    -> security interest, bank eligibility, capacity, reservation,
       obligation allocation, claims, reimbursement, release, enforcement
```

> **Trusted gold infrastructure can establish and continuously assure the reserve within its defined scope. Argent governs what the bank may issue against it.**

Argent should consume signed assertions or references from bank-approved upstream authorities. It should not copy or mint a second representation of a gold token, account balance, pooled interest, or bar record. The full Gold as a Service paper makes the relevant boundary explicit: upstream assurance may cover physical gold and legal entitlements, while product implementation and customer proposition remain with the issuer. A verified upstream position still needs separate bank approval for legal pledgeability, operational control, valuation, concentration, product use, and enforcement.

The first profile remains allocated, individually identified bullion. Pooled beneficial interests and qualifying digital-gold entitlements are later candidate profiles, each requiring its own legal and risk treatment. See [`shared-gold-infrastructure-and-argent.md`](shared-gold-infrastructure-and-argent.md).

### 7.2 Fungible reserve units, specific facility rights

Shared gold infrastructure may make qualifying units interchangeable within an accepted economic, legal, and operational equivalence class. Argent does not make facility rights fungible.

A security interest, capacity reservation, guarantee, documentary credit, or treasury exposure remains specific to the customer, bank, product, amount, beneficiary, purpose, and expiry. No upstream transferability or product fungibility authorizes re-pledge, reassignment, or cross-facility reuse.

The four gates are:

```text
reserve verified
-> legally pledgeable
-> operationally controllable
-> facility issuable
```

---

## 8. One authoritative capacity and deliverability state

The central system object is not a tokenized bar and not a customer cash balance. It is the facility's authoritative capacity state, together with the evidence needed to determine whether that capacity is usable for the requested bank instrument.

```text
Gross reserve value
- valuation haircuts
- liquidity, concentration, custody, and legal adjustments
- required volatility buffer
= eligible reserve value

min(eligible reserve value, bank-approved facility limit)
= approved facility capacity

Approved facility capacity
- provisional and committed reservations
- active obligation allocations
- crystallized reimbursement exposure
- pending claims and settlement
- required margin buffer
= available capacity
```

Available capacity is not automatically issuable capacity. A proposed use must also pass:

- product and applicant sublimits;
- beneficiary, jurisdiction, currency, and tenor rules;
- custody, valuation, legal, and documentary freshness checks;
- institutional approvals;
- bank-product and settlement-route readiness.

The target progression is:

```text
eligible
-> available
-> reservable
-> reserved
-> issuable
-> deliverable
-> active
-> releasable
```

A bank may apply different capacity treatments by product. A performance guarantee, documentary credit, financial guarantee, and treasury exposure do not necessarily consume identical internal risk capacity merely because their face values match.

Argent records and enforces the bank-approved capacity and deliverability decision. It does not invent the bank's exposure model, beneficiary decision, documentary decision, or operating calendar.

The canonical reservation and deliverability specification is [`capacity-reservation-and-deliverability.md`](capacity-reservation-and-deliverability.md).

---

## 9. Core domain objects

The mature protocol separates the reserve from the obligation.

### Reserve domain

- `ReserveAsset`
- `AssetIdentity`
- `CustodyHolding`
- `SecurityInterest`
- `EligibilityPolicy`
- `ValuationSnapshot`

### Facility domain

- `MasterFacility`
- `ProductSublimit`
- `FacilityParticipant`
- `CapacityReservation`

### Obligation domain

- `BankObligation`
- `Beneficiary`
- `TradeDocumentReference`
- `Presentation`
- `Claim`
- `SettlementInstruction`
- `Reimbursement`

### Control domain

- `Substitution`
- `Release`
- `Default`
- `Cure`
- `Enforcement`
- `LifecycleEvent`
- `LegalContext`
- `EvidencePackage`

These names describe the target domain. They are not all implemented contract types in the current repository.

---

## 10. Lifecycle

A typical obligation follows:

```text
REQUESTED
-> PRECHECKED
-> PROVISIONALLY_RESERVED
-> BANK_APPROVED
-> ISSUABLE
-> ISSUE_SUBMITTED
-> INSTRUMENT_ISSUED
-> ACTIVE
```

If issue status is ambiguous, the reservation remains committed until the authoritative bank product system is reconciled. The service must not release capacity or submit a duplicate instrument merely because a callback or transaction response timed out.

Normal expiry:

```text
ACTIVE
-> EXPIRED_OR_CANCELLED
-> CAPACITY_RELEASED
```

Payment or maturity:

```text
ACTIVE
-> PRESENTED_OR_MATURED
-> BANK_PAID
-> REIMBURSEMENT_DUE
-> REIMBURSED
-> CAPACITY_RELEASED
```

Default:

```text
REIMBURSEMENT_DUE
-> DEFAULT_NOTICE
-> CURE_PERIOD
-> ENFORCEMENT_READY
-> REALIZATION_RECORDED
```

At every stage, the authoritative state must distinguish:

- contingent exposure;
- funded or crystallized exposure;
- pending claims;
- free capacity;
- release eligibility;
- enforcement status.

---

## 11. Why Stellar and DFNS fit the model

### Stellar/Soroban

Soroban supplies:

- role-specific authorization through `require_auth`;
- deterministic contract invariants;
- ordered and replayable events;
- atomic coordination where settlement value and state must change together;
- public verifiability without publishing private commercial documents.

### DFNS or equivalent institutional signer

The signer layer supplies:

- organization-controlled wallets;
- permissions and separation of duties;
- approval groups and quorums;
- policy gating before signing;
- pending, approved, denied, and expired approval states;
- audit and webhook evidence;
- MPC or HSM-backed signing arrangements.

The integration principle is:

> Soroban decides whether an authorized state transition is valid. The institutional signer decides whether the institution is allowed to authorize it.

DFNS documentation notes that policies add restrictions rather than providing an automatic whitelist, and delegated wallets bypass organization policy. Argent must therefore construct a deliberate deny-by-default topology for bank, custodian, verifier, and operator roles rather than assume governance exists automatically.

---

## 12. Interoperability

Argent should sit beside, not replace:

- bank credit and limits systems;
- trade-finance platforms;
- treasury and collateral systems;
- custodian and vault records;
- electronic trade-document systems;
- KYC, sanctions, and identity systems;
- accounting and regulatory reporting;
- payment, stablecoin, tokenized-deposit, or CBDC settlement rails.

The protocol should move instructions, proofs, and control state. It should not create duplicate legal claims over the same gold on multiple ledgers. It should act as an orchestration layer between existing systems: preflight the proposed use, reserve capacity without double allocation, reconcile institutional approvals, receive the bank's issue outcome, and return one definitive status to the originating system without requiring the institution to re-platform.

A future documentary-credit path may coordinate:

```text
bank approval
+ capacity reservation
+ custodian control
+ electronic trade-document state
+ beneficiary settlement instruction
```

The legal document system, bank product system, custodian, and settlement rail remain authoritative for their respective domains.

---

## 13. Commercial entry points

### Precious-metals businesses

Bullion dealers, refiners, jewellery manufacturers, wholesalers, importers, exporters, and re-exporters may already own eligible metal and repeatedly need trade, customs, supplier, processing, and hedging instruments.

### Contractors and diversified groups

Construction, logistics, hospitality, trading, recruitment, property, and other operating companies often require recurring guarantees. A centrally owned reserve may support approved subsidiary sublimits where law, governance, and bank policy permit.

### Corporate treasury reserve holders

Companies maintaining allocated gold as a strategic reserve may use it as a complementary assurance layer while keeping operating cash available.

### Banks and custodians

The bank and custodian may adopt Argent as a product-enablement and evidence layer even where the end customer never interacts with a blockchain interface.

---

## 14. What the product must refuse

The protocol should make refusal a product feature.

It must refuse or block:

- unidentified or ineligible reserve assets;
- stale valuation or expired evidence;
- capacity above bank-approved limits;
- concurrent allocation of the same capacity;
- unauthorized beneficiaries or product types;
- release while active or crystallized obligations remain;
- substitution that creates a moment of under-collateralization;
- customer-directed unrestricted cash withdrawal;
- operator action on behalf of a bank, custodian, or owner;
- public disclosure of private bar, facility, or beneficiary data beyond the agreed evidence model.

---

## 15. Current implementation relationship

The current contracts implement a secured-credit lifecycle rather than the complete obligation profile.

What is already reusable:

- instrument registration and admission;
- lot-level identity and uniqueness;
- custody and immobilization acts;
- pledge activation;
- valuation and borrowing-base math;
- exposure and available-capacity state;
- controlled adjustment;
- atomic repayment;
- dual-control release;
- default, cure, enforcement readiness, and enforcement recording;
- canonical events and replay.

What must be added:

- generic master facility;
- typed bank obligations;
- beneficiaries and product sublimits;
- contingent, pending, and crystallized exposure classes;
- provisional and committed capacity reservation before issuance;
- available, issuable, deliverable, and releasable capacity states;
- deterministic preflight outcomes, idempotency, callbacks, and external-system reconciliation;
- issue, amend, expire, present, claim, pay, and reimburse states;
- target no-cash-draw invariant;
- trade-document and beneficiary-settlement adapters;
- role-specific views and selective disclosure of reserve sufficiency, custody control, and instrument status.

The correct claim is therefore:

> The current engine proves the collateral-control foundation. The obligation profile is the product generalization built on top of it.

---

## 16. Economic test

The structure creates value when:

```text
cash collateral avoided
+ contract or market access enabled
+ supplier time obtained
+ customer advances or retentions released
+ reserve sale and repurchase avoided
+ group liquidity consolidated
> bank fees
+ custody and control costs
+ margin and encumbrance costs
+ enforcement and operational risk
```

It is not automatically efficient for every company. A company should not buy gold solely to manufacture borrowing or guarantee capacity unless the reserve has an independent commercial or treasury rationale.

---

## 17. Legal and accounting boundary

Argent does not determine:

- whether a security interest is valid or perfected;
- whether a bank instrument is enforceable;
- whether a subsidiary guarantee provides corporate benefit;
- how the gold and obligations are classified in financial statements;
- how capital, liquidity, tax, sanctions, or consumer rules apply;
- whether an electronic document has legal possession or title effect;
- whether a claim is valid under the governing instrument.

Those questions remain with the bank, custodian, counsel, accountant, and relevant authorities. The protocol records the agreed state and evidence; it does not convert software state into legal rights by assertion.

---

## 18. References and design precedents

- Stellar smart-contract authorization: https://developers.stellar.org/docs/learn/fundamentals/contract-development/authorization
- DFNS policies and approvals: https://docs.dfns.co/core-concepts/policies
- Daml Finance asset model: https://docs.daml.com/daml-finance/concepts/asset-model.html
- Daml Finance lifecycling: https://docs.daml.com/daml-finance/concepts/lifecycling.html
- Daml Finance settlement: https://docs.daml.com/daml-finance/concepts/settlement.html
- ICC UCP 600: https://library.iccwbo.org/content/tfb/RULES/tfb-ucp600-rules.htm
- ICC URDG 758 overview: https://academy.iccwbo.org/trade-finance/e-books/urdg-758/
- UNCITRAL Model Law on Electronic Transferable Records: https://uncitral.un.org/en/texts/ecommerce/modellaw/electronic_transferable_records
- DTCC, *Collateral Infrastructure for Tokenized Capital Markets* (2026): https://www.dtcc.com/news/2026/may/13/tokenized-collateral-could-unlock-billions-in-capital-and-transform-liquidity-management
- Quant, *Unlocking collateral mobility: How tokenisation transforms settlement infrastructure* (2026): https://quant.network/perspectives/unlocking-collateral-mobility-how-tokenisation-transforms-settlement-infrastructure/
- Canton ledger privacy model: https://docs.digitalasset.com/overview/3.5/explanations/ledger-model/ledger-privacy.html
- W3C Verifiable Credentials Data Model 2.0: https://www.w3.org/TR/vc-data-model-2.0/
- OpenID for Verifiable Presentations 1.0: https://openid.net/specs/openid-4-verifiable-presentations-1_0.html

No reference implies partnership, endorsement, legal equivalence, or production compatibility.

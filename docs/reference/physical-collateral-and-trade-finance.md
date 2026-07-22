# Physical Reserves and the Financing of Trade

**How identified physical reserves can support bank obligations, trade documents, and settlement without becoming tokens or unrestricted cash.**

**Status:** market and domain analysis  
**Product direction:** reserve-backed obligation infrastructure  
**Implemented reference:** secured-credit collateral-control branch  

---

## 1. The central distinction

Trade finance does not always require a bank to advance cash to the applicant.

Many transactions require the bank to provide:

- payment certainty;
- performance assurance;
- documentary payment;
- deferred settlement;
- a maturity promise;
- regulatory or customs security;
- treasury exposure capacity.

These are obligations and undertakings. They may remain contingent until documents are presented, a claim is made, an instrument matures, or a market position settles.

Argent's product direction is therefore not simply physical-asset lending. It is:

> **Use a controlled physical reserve to support the bank-issued obligation required by the trade, while preserving fiat for operations, reimbursement, and final settlement.**

---

## 2. Why physical collateral matters

A large share of real-economy value exists in physical form:

- metals;
- agricultural commodities;
- energy inventory;
- goods in warehouses;
- goods in processing;
- goods in transit;
- equipment and finished inventory.

The financing problem is not only price. A bank must determine:

- what the asset is;
- who owns it;
- whether it is allocated or pooled;
- where it is held;
- who controls movement;
- whether it is already pledged;
- how quality and quantity are verified;
- how it is valued and liquidated;
- what document represents or evidences it;
- how release and enforcement work.

Allocated gold is a strong first reserve because the wholesale market already uses bar-level identity, allocation records, professional custody, assay conventions, and observable market pricing.

---

## 3. Trade finance contains several different economic functions

### 3.1 Payment

Cash or bank money settles an obligation immediately.

### 3.2 Conditional payment

A documentary credit commits the bank to honor when complying documents are presented under the instrument's terms.

### 3.3 Assurance

A demand guarantee or standby instrument protects a beneficiary against specified non-performance or non-payment.

### 3.4 Deferred payment

A bank undertaking, accepted draft, or avalised instrument gives the supplier a bank-supported future payment claim.

### 3.5 Financing

A bank or another financier advances value before the commercial cycle produces cash.

### 3.6 Document and title control

Bills of lading, warehouse receipts, and other transferable records may control rights to claim performance or delivery.

The product must not collapse all six functions into "credit." A reserve may support any of them, but the bank should use the least costly and least risk-intensive instrument that satisfies the commercial requirement.

---

## 4. The four synchronized states

A robust trade-finance transaction may depend on four separate states.

### Reserve state

- asset identity;
- custody;
- eligibility;
- pledge;
- capacity;
- release status.

### Bank-obligation state

- application;
- approval;
- issue;
- amendment;
- presentation or claim;
- payment;
- reimbursement;
- discharge.

### Trade-document state

- issuer;
- holder or controller;
- integrity;
- presentation;
- transfer;
- surrender or cancellation.

### Settlement state

- payment instruction;
- value movement;
- finality;
- reimbursement;
- accounting reference.

Argent should synchronize evidence across these states without claiming that one ledger is legally authoritative for all of them.

---

### Operational deliverability across the four states

Synchronizing the four states requires more than enough collateral value. Before issuance, Argent must establish that the requested product is permitted, the beneficiary and transaction are approved, required documents and approvals are current, capacity is reserved, and the bank product system can return a definitive outcome.

```text
available reserve capacity
+ product and counterparty eligibility
+ documentary readiness
+ institutional approval
+ operational route
= issuable and deliverable capacity
```

The canonical preflight and reservation model is [`capacity-reservation-and-deliverability.md`](capacity-reservation-and-deliverability.md).

## 5. Reference flow: import documentary credit

```text
1. Importer owns eligible allocated gold.
2. Bank and custodian establish the reserve-control facility.
3. Importer requests a documentary credit for a named supplier.
4. Bank validates product, beneficiary, amount, tenor, and facility capacity.
5. Argent reserves the bank-approved capacity.
6. Bank trade-finance system issues the credit.
7. Supplier ships and presents the required documents.
8. Bank determines compliance under the instrument and governing rules.
9. Bank pays or accepts a deferred-payment obligation.
10. Importer reimburses the bank.
11. Argent releases the obligation reservation.
12. Gold remains pledged for reuse or is released under the facility terms.
```

Argent does not decide whether documents comply. It records the bank-authorized decision and the authoritative document reference.

---

## 6. Reference flow: performance guarantee

```text
1. Contractor requests a guarantee for a named project owner.
2. Bank validates the underlying contract and guarantee wording.
3. Argent reserves capacity against the gold pool.
4. Bank issues the guarantee.
5. The contractor performs.
6. The guarantee expires or is returned.
7. Bank records discharge.
8. Argent restores capacity.
```

Claim path:

```text
beneficiary presents claim
-> bank validates under the guarantee
-> bank pays where required
-> reimbursement becomes due
-> company reimburses or enters default and enforcement
```

A claim does not give the beneficiary direct rights in the gold through Argent. The beneficiary's claim remains against the bank instrument.

---

## 7. Reference flow: advance-payment guarantee

This is one of the highest-value structures because the gold need not finance the project directly.

```text
Gold supports bank guarantee
-> guarantee protects project owner
-> project owner releases advance payment
-> advance funds project execution
-> contractor performs and guarantee reduces or expires
-> reserve capacity is restored
```

The external customer, rather than a bank cash loan, supplies working capital. The gold provides assurance.

---

## 8. Reference flow: accepted maturity obligation

```text
Company purchases goods on deferred terms
-> bank accepts or supports the future-payment instrument
-> supplier holds, transfers, or discounts it where permitted
-> company receives goods before final payment
-> bank pays at maturity if required
-> company reimburses bank
-> capacity is restored
```

This can turn the reserve owner's gold into supplier liquidity while keeping the supplier's claim against the bank rather than the physical reserve.

Transferability, discounting, holder status, and legal effect remain governed by the instrument and applicable law.

---

## 9. Warehouse-receipt finance lessons

Warehouse-receipt systems show that goods can remain in storage while a record supports financing or transfer.

The important lessons for Argent are:

- the receipt is not enough without reliable warehouse control;
- uniqueness and cancellation matter;
- title, possession, holdership, and security may be different concepts;
- quality and quantity evidence must remain current;
- duplicate financing is a system-level risk;
- release must be linked to payment or lender consent;
- the legal recognition of an electronic record cannot be assumed from a hash.

Argent should integrate with authoritative warehouse or electronic-document systems rather than manufacture a competing receipt.

---

## 10. Receivables-finance lessons

Receivables finance contributes a dynamic portfolio model:

- eligible versus ineligible assets;
- concentration limits;
- ageing and disputes;
- dilution and offsets;
- borrowing-base recalculation;
- reserve accounts;
- trigger events;
- ongoing reporting.

The obligation facility should adopt the same discipline for reserve capacity:

- bank-approved eligibility;
- product and group sublimits;
- current valuation;
- pending claims;
- crystallized exposure;
- available capacity;
- exceptions and cure.

---

## 11. Securities-collateral lessons

Institutional securities collateral systems contribute:

- haircuts;
- margining;
- concentration and wrong-way risk;
- substitution;
- recall;
- optimization;
- triparty control;
- net exposure and settlement coordination.

The central insight is that collateral value comes from the ability to allocate the right eligible asset to the right exposure at the right time.

For physical gold, the safer version is often:

> **Move the control function and capacity allocation, not the bars.**

---

## 12. Electronic trade documents

The UNCITRAL Model Law on Electronic Transferable Records provides a useful legal design principle: an electronic transferable record must satisfy functional requirements around integrity and control; it is not enough to scan a document or publish a hash.

Argent should treat an electronic bill of lading, warehouse receipt, or promissory note as an external authoritative object with:

- system identifier;
- issuer;
- current controller or holder where applicable;
- integrity status;
- transfer or presentation status;
- legal-context reference.

Argent may use that state as an input to a bank-authorized obligation transition. It should not claim to create possession or title through a collateral event alone.

---

## 13. The no-cash-draw advantage

A general secured line may create three weaknesses:

- unnecessary interest carry;
- customer discretion over use;
- financing of persistent losses rather than a defined transaction.

The obligation-first facility reduces those weaknesses because each use must identify:

- product;
- beneficiary;
- purpose;
- amount;
- term;
- evidence;
- reimbursement source.

This makes the facility closer to transaction infrastructure than a general liquidity account.

---

## 14. Why cash remains necessary

The company still needs fiat for:

- payroll;
- taxes and fees;
- ordinary expenses;
- reimbursement after a bank payment;
- margin calls;
- obligations for which no bank undertaking is accepted;
- final settlement.

The product is not a way to avoid payment. It is a way to avoid using cash prematurely for assurance or collateral where a bank obligation is acceptable.

---

## 15. Priority commercial profiles

### Precious-metals company

- reserve already exists;
- trade and hedging needs are recurring;
- custody and valuation are familiar;
- strongest first design-partner profile.

### Contractor or diversified group

- recurring guarantees;
- high cost of blocked cash and retention;
- potential central reserve and subsidiary sublimits;
- more complex group legal and reimbursement structure.

### Corporate treasury reserve holder

- strongest reserve philosophy;
- may require board, accounting, and treasury-policy development;
- best after the first operating proof.

---

## 16. Argent's role in the trade stack

```text
Commercial contract
        |
        v
Bank product system
LC / guarantee / undertaking / treasury product
        |
        v
Argent reserve-control layer
eligibility / capacity / reservation / reimbursement / release
        |
        v
Custodian and reserve evidence
        |
        v
Trade-document and settlement systems
```

Argent is the reserve-capacity control layer. It should not replace the systems above or below it.

---

### Privacy across the trade stack

The bank, custodian, applicant, beneficiary, document provider, auditor, and supervisor do not need identical data. A beneficiary may verify the bank instrument and its status without receiving the full bar list or group exposure. A custodian may receive bar-control instructions without receiving the complete credit file. Trade documents and claims remain in their authoritative systems or encrypted evidence stores.

See [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md).

## 17. Current implementation and next extension

The current contracts implement:

- reserve identity and eligibility;
- pledge and capacity;
- utilization and repayment;
- release and adverse path.

The trade-finance extension requires:

- generic master facility;
- beneficiary;
- product type;
- reservation before issue;
- issue, amend, expire, present, claim, pay, reimburse, and discharge;
- document and bank-system references;
- no unrestricted customer cash draw.

See `obligation-facility-profile.md` and `product-roadmap.md`.

---

## 18. Product conclusion

The strongest trade-finance proposition is not:

> Lend cash against gold.

It is:

> **Use gold to secure the bank obligation that unlocks the trade, while preserving cash for the operating cycle and final settlement.**

That is the role of Argent.

---

## References

- ICC UCP 600: https://library.iccwbo.org/content/tfb/RULES/tfb-ucp600-rules.htm
- ICC URDG 758 overview: https://academy.iccwbo.org/trade-finance/e-books/urdg-758/
- UNCITRAL MLETR: https://uncitral.un.org/en/texts/ecommerce/modellaw/electronic_transferable_records
- Daml Finance asset model: https://docs.daml.com/daml-finance/concepts/asset-model.html
- Daml Finance settlement: https://docs.daml.com/daml-finance/concepts/settlement.html

No source reference implies legal equivalence, partnership, endorsement, or production interoperability.

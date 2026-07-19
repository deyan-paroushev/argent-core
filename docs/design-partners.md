# Working with Argent: an invitation to design partners

**Design-partner brief for gold-backed obligation infrastructure.**

Argent is developing a bank-operated, non-cash-drawable master facility under which customer-owned allocated bullion supports purpose-bound bank obligations. The bank issues the guarantee, documentary credit, undertaking, or treasury product. The custodian keeps physical control. Argent maintains one authoritative capacity, authorization, evidence, reimbursement, release, and enforcement state.

> **One reserve. Many obligations. One authoritative capacity state.**

This document is not an offer of credit, custody, guarantees, or investment products. It identifies the institutions and operating questions required to design and validate a real facility.

---

## Who this is for

### Banks and regulated product issuers

Teams in:

- corporate and commercial banking;
- trade finance;
- guarantees and standby instruments;
- collateral and limits management;
- treasury and derivatives;
- private banking and family-office coverage;
- transaction banking;
- innovation and digital assets;
- credit, legal, risk, compliance, and operations.

### Custodians, vaults, and bullion operators

Institutions able to provide:

- allocated bar identity;
- segregation and custody evidence;
- immobilization and release controls;
- substitution;
- valuation, assay, insurance, and location evidence;
- realization support.

### Reserve-owning companies

Priority profiles:

1. precious-metals businesses;
2. contractors and diversified family groups;
3. corporate treasury reserve holders.

### Trade-document and infrastructure providers

Providers of:

- electronic bills of lading;
- warehouse receipts;
- trade-finance platforms;
- bank integration;
- institutional signing;
- valuation and proof-of-reserve data;
- shared gold infrastructure, provenance, pooled-gold, or digital-gold operators;
- accounting and evidence systems.

---

### Shared gold infrastructure and reserve-data providers

Potential partners include custodians, provenance systems, pooled-gold registers, digital-gold product operators, and future shared gold platforms that can provide an authoritative reserve assertion without asking Argent to duplicate their ownership or product ledger.

A useful design-partner conversation must establish:

- what facts the provider is authoritative for;
- whether the customer holds allocated bars, a proprietary pooled interest, or an issuer claim;
- how ownership, backing, custody, and redemption are reconciled;
- whether a pledge, block, freeze, or control instruction is supported;
- how discrepancies, outages, and stale records are handled;
- what minimum assertion can be shared with the bank and Argent;
- whether a stable production interface exists.

This is a future integration category. No World Gold Council, LBMA, Gold Bar Integrity, Pooled Gold Interests, Wholesale Digital Gold, Standard Gold Unit, or Gold as a Service partnership or integration is claimed.

## What Argent is

Argent is a control and evidence layer beneath bank-issued obligations.

It coordinates:

- reserve identity;
- custody and immobilization;
- bank eligibility and valuation treatment;
- pledge and available capacity;
- purpose-bound capacity reservation;
- institutional authorization;
- obligation, reimbursement, release, default, and enforcement evidence.

## What Argent is not

- not tokenized gold;
- not a bank or guarantor;
- not a custodian;
- not a public DeFi lender;
- not a trade-finance platform replacement;
- not a private currency;
- not a customer cash-withdrawal product;
- not a substitute for legal documentation or bank underwriting.

---

## The design question

The central question is not:

> Can a company borrow cash against gold?

It is:

> Can one controlled bullion reserve support a portfolio of bank-approved obligations without forcing the company to sell the gold or block the same cash it needs to operate?

A production design partner helps answer:

- which obligations create the strongest first use case;
- what reserve forms and custody arrangements are acceptable;
- what legal security and control are required;
- how the bank calculates capacity by product;
- which evidence must be shared and which must remain private;
- how issue, amendment, claim, payment, reimbursement, and release are reconciled;
- how the bank and custodian govern signing authority;
- which existing systems remain authoritative.

---

## Priority customer profiles

### 1. Precious-metals businesses

Potential users:

- bullion dealers;
- refiners;
- jewellery manufacturers;
- wholesalers;
- importers and re-exporters;
- gold logistics or processing businesses that own eligible reserve metal.

Recurring needs:

- import documentary credits;
- supplier payment undertakings;
- customs and regulatory guarantees;
- processing and storage guarantees;
- FX and precious-metals hedging limits;
- performance or delivery guarantees.

The strongest pilot uses ring-fenced, company-owned, fully assayed allocated bullion rather than customer metal, work in progress, or constantly circulating inventory.

### 2. Contractors and diversified groups

Potential uses:

- bid bonds;
- performance guarantees;
- advance-payment guarantees;
- retention and warranty guarantees;
- lease, utility, customs, labour, licence, and supplier instruments;
- trade and treasury sublimits.

A holding company or treasury entity may own the reserve while approved subsidiaries use product-specific sublimits, subject to corporate-benefit, related-party, authority, reimbursement, and bank group-exposure rules.

### 3. Corporate treasury reserve holders

Companies that maintain allocated gold as a strategic reserve may use it as a complementary assurance layer while keeping operating cash available.

This is the clearest philosophical match but may require more education and formal reserve-policy work than a bullion-sector pilot.

---

## Priority institutional partners

### Bank

The bank validates:

- customer and group eligibility;
- accepted reserve form;
- haircut and capacity calculation;
- product and beneficiary eligibility;
- sublimits;
- legal structure;
- issue and claim process;
- reimbursement and enforcement;
- accounting, capital, liquidity, and operational treatment.

The bank remains the product issuer and risk owner.

### Custodian

The custodian validates:

- bar identity and allocation;
- custody and segregation;
- control agreement;
- immobilization;
- release and substitution;
- evidence frequency;
- discrepancies and incident handling;
- realization process.

### Reserve owner

The owner validates:

- reserve policy and commercial purpose;
- ownership and authority;
- permitted obligation types;
- reimbursement source;
- cash-preservation value;
- acceptable encumbrance and margin risk;
- operational usability.

### Beneficiary or trade counterparty

The beneficiary validates:

- authenticity and status verification;
- bank instrument form;
- presentation or claim channel;
- privacy requirements;
- whether any protocol evidence is useful beyond the bank instrument itself.

---

## Recommended first pilot

A credible first pilot should be narrow:

- one bank;
- one custodian;
- one reserve owner;
- one allocated bullion pool;
- one jurisdiction and governing-law package;
- one product family;
- one beneficiary or beneficiary class;
- one reimbursement route;
- one enforcement route.

Recommended first product options:

1. performance or advance-payment guarantee;
2. import documentary credit;
3. customs or regulatory guarantee.

The pilot should not begin with multi-bank collateral, several jurisdictions, public tokenization, autonomous agents, or unrestricted funded credit.

---

## What the current implementation can demonstrate

The current Soroban reference branch already demonstrates:

- exact collateral identity;
- instrument eligibility;
- custodian immobilization;
- exclusive pledge;
- capacity computation;
- utilization and repayment;
- dual-control release;
- default, cure, and enforcement evidence;
- canonical event replay;
- refusal of unsafe or unauthorized actions.

For a design partner, this is the executable collateral foundation.

It does not yet demonstrate:

- typed guarantees or documentary credits;
- product sublimits;
- beneficiary-specific reservation;
- claim and presentation states;
- the target no-cash-draw profile;
- a production bank or custodian integration.

---

## What a design engagement should produce

### Product output

- selected use case;
- instrument lifecycle;
- facility and sublimit rules;
- capacity-consumption method;
- claim, payment, reimbursement, and discharge process;
- user and operator workflows.

### Legal and control output

- reserve ownership and eligibility requirements;
- security and control structure;
- required approvals;
- governing documents;
- release and enforcement process;
- evidence and record hierarchy.

### Technical output

- bank, custodian, and signer integration map;
- authoritative-system boundaries;
- data fields and identifiers;
- event and evidence schema;
- reconciliation and exception handling;
- preflight, reservation expiry, idempotency, callback, and ambiguous-outcome rules;
- privacy, role-view, retention, and selective-disclosure requirements.

### Pilot output

- agreed test cases;
- acceptance criteria;
- failure and refusal scenarios;
- testnet or ring-fenced mainnet demonstration;
- post-pilot production decision.

---

## Questions for a bank design partner

1. Which non-funded products most often require cash margin or separate collateral?
2. Which reserve forms are already acceptable or potentially acceptable?
3. Is capacity calculated at facility, product, customer, or group level?
4. What product-specific conversion, tenor, currency, and concentration factors apply?
5. Which systems hold facility, collateral, product, claim, payment, and accounting authority?
6. What must be approved before issue, amendment, release, claim payment, or enforcement?
7. Which actions require separate authorities or quorum approval?
8. What evidence may be placed on a shared ledger and what must remain private?
9. What would make a first pilot commercially worthwhile?
10. Which risk or control would cause the bank to reject the model?
11. At what point does an application or quote reserve capacity, and for how long?
12. Which system is authoritative if an issue request times out or a callback is lost?
13. Which statuses and reason codes must be returned to the originating system?
14. Which reserve, beneficiary, and obligation fields may each role see?
15. What daily reconciliation and manual exception process would operations require?

---

## Questions for a custodian design partner

1. Which custody mode creates a sufficiently clear owner and control state?
2. How are bar identity, allocation, segregation, and movement restrictions represented?
3. Can the custodian provide signed, machine-readable attestations?
4. What events require manual intervention?
5. How are substitution, assay discrepancy, insurance change, and location change handled?
6. What constitutes valid bank release authority?
7. How would realization be instructed and evidenced?
8. What privacy limitations apply to serial numbers and client positions?
9. Which control facts may be shared without exposing the complete bar list or customer book?
10. How should a disagreement between the custody book and Argent be escalated and resolved?

---

## Questions for a reserve owner

1. Why is the gold held independently of the facility?
2. Which recurring bank obligations currently consume cash or ordinary credit limits?
3. What is the economic cost of blocked cash, advance payment, retention, or lost tender access?
4. What reimbursement source exists if an instrument is paid or called?
5. What encumbrance, margin, and price risk is acceptable?
6. Which entities in the group need capacity?
7. Which obligations must never be supported by the reserve?

---

## Engagement boundary

A design-partner relationship is not:

- a promise that a bank will accept gold;
- a credit approval;
- a guarantee issuance;
- a custody appointment;
- legal, accounting, tax, or regulatory advice;
- a claim that on-chain evidence creates legal rights.

It is a structured process for determining whether one reserve-backed obligation facility can be made lawful, operational, commercially useful, and technically verifiable for named institutions.

---

## Contact proposition

> **Keep the reserve. Preserve the cash. Let the bank issue the instrument.**

Argent is seeking banks, custodians, bullion operators, and reserve-owning companies willing to define and test the first complete obligation lifecycle on top of the existing collateral-control engine.

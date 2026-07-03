# Commodity Finance Positioning for Argent

**Purpose:** sharpen Argent's value proposition for investor and design-partner conversations by positioning the current Soroban engine as a control layer for commodity finance broadly, with allocated gold as the first clean proof, not as a gold-only lending application.

**Status:** strategy and positioning document. It does not expand the current engine scope, and it recommends no new contract surface. It translates the existing asset-agnostic collateral-control core into commodity-finance language, and anchors that translation in the international legal and standards direction the field is already moving in. Companion reading: `physical-collateral-and-trade-finance.md` (neutral market primer), `collateral-as-locked-value.md` (the market-signal case), `collateral-control.md` (the product thesis and the four control primitives), and `protocol.md` (the technical specification).

---

## 1. One-line value proposition

**Argent is a shared control-state layer for commodity collateral that stays in professional custody.**

It does not tokenize the commodity. It makes the lender's control over the commodity explicit, signed by the relevant parties, and auditable across the facility lifecycle: eligibility, pledge, borrowing base, drawdown, repayment, release, default, and enforcement. A sharper external sentence:

> Argent helps banks and commodity financiers lend against physical collateral with less reconciliation risk. The goods remain with the warehouse, vault, or custodian. The signed control lifecycle becomes programmable and independently verifiable.

## 2. The positioning in one frame

Three modernizations are happening in trade and commodity finance at once, and they are not the same thing. Keeping them distinct is what makes Argent's position legible.

Trade is digitizing its **documents**. The ICC Digital Standards Initiative's Key Trade Documents and Data Elements work has now analysed 36 key trade documents drawn from the WTO, UNCITRAL, and UN ESCAP Cross-border Paperless Trade Toolkit, with the warehouse receipt among them, so that the same document means the same thing across platforms [1]. In parallel, the legal notion of holding a document has been dematerialized: the UNCITRAL Model Law on Electronic Transferable Records (MLETR, adopted 2017) makes *control* the functional equivalent of *possession* for transferable records including warehouse receipts, on a technology-neutral basis that expressly accommodates distributed ledgers [2]. And the 2024 UNCITRAL-UNIDROIT Model Law on Warehouse Receipts (MLWR) gives the underlying warehouse-receipt system a modern, medium-neutral legal frame in which stored goods may be used as collateral while warehoused [3].

Market infrastructure, separately, is digitizing **settlement**. The Bank of England's Project Meridian Securities tested programmable, conditional settlement, collateral mobilisation, and interoperability between conventional systems and DLT-based platforms [4]. The Eurosystem's Pontes initiative targets settlement in central-bank money for DLT-based transactions [5]. DTCC, the core of US securities settlement, has a path to bring DTC-custodied assets on-chain [6].

Neither of those modernizations, by itself, creates a shared, live **control state** over the physical collateral. A digitized document is still a document. A programmable settlement rail still needs to know, at the moment of settlement, whether the collateral is eligible, sufficient, and released by the right parties. That control state is the missing layer, and it is where Argent sits:

> Trade is digitizing documents. Market infrastructure is digitizing settlement. Argent digitizes the missing control state over physical collateral in custody.

The MLETR framing matters here for a specific reason. The law already accepts that, for a transferable record, *control* can stand in for *possession* [2]. Argent extends the same logic one layer down, from the document to the collateral itself: the lender's protection is exercised not by holding the goods but by holding legible, signed control over them. Argent is not a settlement infrastructure and should never claim to be. It is the physical-collateral control layer that a settlement rail or a document platform can rely on and plug into.

## 3. Why this is a commodity-finance problem, not only a gold problem

Commodity finance is built around physical goods moving through production, storage, shipment, processing, and sale. The bank's real question is rarely only "what is the asset worth." The harder question is:

```text
Can I prove what the collateral is, who controls it, whether it is still eligible,
how much credit it supports today, and what must happen before it can be released?
```

That question applies to gold, but equally to base metals, refined metals, warehouse-held agricultural commodities, energy inventory, and critical materials. Gold is the first clean adapter because it is standardized, liquid, and custody-stable. The broader category is commodity collateral control, and the governing contract structure carries no commodity field, which is what makes the same engine reusable across asset classes rather than tied to any one [see `protocol.md`].

## 4. The market thesis, kept narrow

The investable thesis should not claim Argent addresses all of trade finance. That is too broad and reads as inflated. The stronger thesis is narrower and evidence-backed:

1. Commodity finance is a large, real-economy secured-credit market, reputed above US$3 trillion in lending, and it is transaction-driven: the lender needs line of sight through the transaction chain to settlement and to its protection rights [7].
2. Secured lending already dominates corporate credit. In the euro area roughly 70% of credit amounts are collateralized, and secured loans carry materially better terms, larger committed amounts and lower rates, yet physical movable assets remain among the least-pledged collateral forms relative to their economic weight [8].
3. Movable collateral is not weak because it lacks value. It is weak because control, monitoring, and enforcement are expensive. When European reforms let firms pledge movables without surrendering possession, secured issuance rose, but average loan cost and covenants also rose, because banks still had to price the residual monitoring and misappropriation risk the law alone could not remove [9]. That priced residual is the wedge.
4. The industry is digitizing documents and settlement, but neither creates a shared control state over the physical collateral.
5. Argent occupies that missing layer.

A credible market statement:

> Argent does not need to prove a new appetite for secured lending. Secured lending already dominates corporate credit, and commodity finance already exists at large scale. The gap is that physical movable collateral remains less systematized than securities or real estate. Argent targets the operational control gap inside that market, not the appetite for the market itself.

The fuller market-signal case, trapped working capital, the tokenization mismatch, and the timing convergence, is set out in `collateral-as-locked-value.md` and is not repeated here.

## 5. Why this matters to lenders

For a lender, Argent is not a better database. It is a control discipline, and it maps to named operational pain.

**Fewer uncontrolled releases.** A pledged lot should not be releasable because one operational party sends an instruction. Release should follow the agreed policy: repayment, sale proceeds, margin sufficiency, and the required role signatures. Improper release of collateral is named directly as a concrete risk of today's manual, fragmented processes [10].

**Cleaner borrowing-base evidence.** Borrowing-base finance depends on a recurring calculation:

```text
eligible collateral value
x haircut / advance-rate treatment
- drawn exposure
= available capacity
```

In practice quantity, eligibility, price, FX, haircut, and exposure sit in different systems, and a static, manually maintained base is exactly the spreadsheet-bound process the industry flags as error-prone [10]. Argent turns those steps into a signed, event-sourced control record.

**Better partner coordination.** Commodity credit is multi-party: borrower, lender, custodian or warehouse, collateral manager, valuation source, insurer, broker or offtaker, and sometimes an agent bank. Argent gives these parties one control history rather than several private reconciliations.

**Stronger audit trail.** A bank, auditor, or risk committee should be able to reconstruct which collateral was eligible, who attested to it, which documents governed it, which exposure it secured, whether margin sufficiency held, who approved release, and what happened on default or enforcement, without assembling it from disparate systems after the fact.

## 6. Why this matters to borrowers

The borrower does not want collateral to become operationally frozen. The borrower's value is mobility under control: substitute one eligible lot for another, partially release excess collateral after repayment or price movement, sell pledged inventory through an approved proceeds-capture workflow, and use higher-quality custody evidence to negotiate less manual friction. These are the four control primitives set out in `collateral-control.md`.

The borrower benefit should be framed carefully. Argent should not promise cheaper credit by itself. It can credibly claim to reduce the operational reasons lenders apply conservative procedures, larger buffers, or slow releases.

## 7. Where Argent should start beyond gold

The next asset category should be chosen by operational fit, not by market size alone.

| Priority | Category | Why it fits | Main risk |
|---|---|---|---|
| 1 | Allocated precious metals | Standardized, liquid, vault-held, high value per unit | Narrower market; strong incumbent custody practices |
| 2 | Exchange-grade base metals | Warehouse-held, standardized lots, clear quality specs, existing financing practice | Warehouse, location, and receipt risk; price volatility |
| 3 | Critical minerals and battery metals | Strategic importance; real financing need; high-value inventory | Assay, provenance, ESG, sanction, and offtake complexity |
| 4 | Regulated warehouse-receipt agriculture | Strong development-finance relevance; established warehouse-receipt logic, now with a modern model law [3] | Quality, perishability, jurisdictional enforcement, local regulation |
| 5 | Energy inventory and cargoes | Large financing volumes; proceeds-capture relevance | Sanctions, title-chain, insurance, environmental and operational complexity |

Position gold as the first proof, then name the expansion lane as custody-stable commodities. Do not present all commodities as equally ready.

> Argent expands first where the asset is custody-stable, externally valued, and release-controlled. The order is gold, then warehouse-held metals, then selected regulated warehouse-receipt commodities with credible local infrastructure.

## 8. What not to build yet

The current engine is enough for investor outreach. The next improvement is documentation, proof design, and design-partner targeting, not extra contract surface. Do not add now: commodity-specific contract branches; general warehouse-receipt issuance; tokenized commodity ownership; automated legal enforcement; full trade-document platform functionality; energy-cargo workflows before a serious partner defines them; or agricultural workflows before a credible warehouse-receipt jurisdiction is selected. The right technical posture:

> The core remains asset-agnostic. Commodity specificity enters through eligibility schedules, custody attestations, valuation sources, haircut treatment, and release policy, not through new contract branches per commodity.

## 9. Go-to-market: first institutional clients' positioning

The positioning below is what carries into first-client outreach, calls, and the deck. It leads with the proof and the gap, not the technology:

> Argent started with allocated gold because it is the cleanest institutional proof of physical-collateral control. But the engine is not a gold product. It is a control-state layer for commodity collateral that remains in professional custody. The same lifecycle applies to warehouse-held metals and other custody-stable commodities: eligibility, pledge, borrowing base, drawdown, release, default, and enforcement. The gap is not that banks do not understand commodities. The gap is that control over physical collateral is still fragmented across borrower, bank, warehouse, legal documents, valuation sources, and payment records. Argent turns that lifecycle into a signed, auditable control record.

## 10. What each party must confirm in order to participate

Every party to a financed commodity trade, banks, commodity financiers, warehouses, collateral managers, and custodians, has to satisfy itself on a specific set of points before it will rely on a shared control record. These are the questions that surface those points, and they double as the agenda for a first design-partner conversation:

1. Which collateral types are operationally hardest to finance today: gold, base metals, agricultural goods, inventory in transit, or receivables-linked inventory?
2. Where does record drift actually occur: quantity, quality, eligibility, price, release authority, repayment, insurance, or enforcement?
3. Which releases are most painful: full release, partial release, release after sale, substitution, release after repayment, or enforcement release?
4. What is currently managed in spreadsheets or email despite being critical to risk?
5. How often is the borrowing base recalculated, and from which data sources?
6. What evidence would make a credit committee more comfortable with a facility?
7. Which party should be the first buyer: bank, custodian, warehouse operator, collateral manager, or commodity trader?
8. Which single workflow would justify a pilot even before full production integration?

## 11. Pilot wedge

The cleanest pilot is not "commodity finance on chain." It is narrower:

```text
A bank or commodity financier controls one pledged pool of custody-stable collateral,
with a named custodian or warehouse, a defined borrowing-base formula,
role-signed release conditions, and a replayable evidence pack.
```

Success is measured by time to reconstruct collateral state, number of manual reconciliations removed, release approvals captured in one record, margin sufficiency provable at each release, credit and risk reviewer confidence, and willingness to define a second asset adapter.

## 12. Claims to avoid

Do not claim that Argent finances all trade; that the addressable market equals the entire trade-finance gap; that tokenization is the product; that a blockchain verifies physical truth; that a smart contract can seize goods; that all commodity collateral is ready for the same workflow; or that borrowers automatically get cheaper credit. Use instead: Argent records signed control, not physical truth; custodians remain responsible for physical custody and attestations; legal agreements remain the source of enforceable rights; the contract enforces the agreed control logic inside the shared record; and the commercial value is less reconciliation risk, safer release, and more legible collateral.

## 13. Where Argent fits, stated once

Argent is the shared control instrument that commodity finance currently lacks. The physical asset stays in professional custody and is never tokenized. What becomes legible is the control over it, recorded as a sequence of signed, ordered, verifiable events, and reusable across metals, minerals, and warehouse-held commodities because the governing structure carries no commodity. It is the answer to the field's own direction: documents are being digitized, settlement is being digitized, and the control state over physical collateral is the layer still missing. Whether the instrument proves out at scale is an adoption-and-proof question, to be settled with design partners rather than by argument. The technical design and current status, 224 passing tests across three Soroban contracts with two deployed to testnet, are in `protocol.md`, `argent-architecture.md`, and `collateral-control.md`. The invitation to institutions that live with this gap is in `design-partners.md`.

---

## References

[1] International Chamber of Commerce Digital Standards Initiative, "Key Trade Documents and Data Elements (KTDDE)," complete framework analysing 36 key trade documents from the WTO-UNCITRAL-ESCAP Cross-border Paperless Trade Toolkit, warehouse receipt included, 2023-2025. https://dsi.iccwbo.org/our-work

[2] United Nations Commission on International Trade Law, "Model Law on Electronic Transferable Records (MLETR)," adopted 13 July 2017; Article 11 establishes control as the functional equivalent of possession for transferable records including warehouse receipts, on a technology-neutral basis accommodating distributed ledgers. https://uncitral.un.org/en/texts/ecommerce/modellaw/electronic_transferable_records

[3] UNCITRAL and the International Institute for the Unification of Private Law, "UNCITRAL-UNIDROIT Model Law on Warehouse Receipts (MLWR)," adopted 26 June 2024 (UNIDROIT Governing Council approval 8-10 May 2024); medium- and technology-neutral, supports paper and electronic receipts, goods may be used as collateral while stored. https://uncitral.un.org/en/mlwr

[4] Bank of England and BIS Innovation Hub London Centre, "Project Meridian Securities," Bank of England (programmable conditional settlement, collateral mobilisation, and conventional-to-DLT interoperability).

[5] European Central Bank, "Pontes and the Appia roadmap for a European tokenised financial ecosystem," ECB, 2026.

[6] The Depository Trust & Clearing Corporation and Stellar Development Foundation, "DTC's tokenization service to connect with Stellar public blockchain," DTCC press release, May 2026.

[7] Association for Financial Markets in Europe, "Capital Treatment of Commodity Finance," AFME (commodity finance as transaction-driven, secured, monitored lending; market reputed above US$3 trillion).

[8] H. Degryse, O. De Jonghe, L. Laeven, and T. Zhao, "Collateral and credit," ECB Working Paper Series No. 3095, 2025 (~70% of euro-area credit amounts collateralized; secured loans show larger commitments and lower rates; physical movable assets among the least-pledged forms). https://www.ecb.europa.eu/pub/pdf/scpwps/ecb.wp3095~0e81ee7f34.en.pdf

[9] S. Ongena, W. Saffar, Y. Sun, and L. Wei, "Movables as Collateral and Corporate Credit: Loan-Level Evidence from Legal Reforms across Europe," Swiss Finance Institute Research Paper No. 22-75 (post-reform secured issuance rose, but loan cost and covenants also rose as banks priced residual monitoring and misappropriation risk).

[10] Ernst & Young, "How banks can align collateral functions to a services-based model," and "Transforming collateral management functions for regional banks," EY Insights, Banking & Capital Markets (improper release and valuation misreporting as concrete risks of manual, spreadsheet-bound, fragmented processes).

[11] Deutsche Bank, "A Guide to Trade Finance," 2nd edition, September 2025 (borrowing-base finance, warehouse and inventory financing, collateral revaluation and shortfall remedy, repayment tied to sale or use of goods, trade digitization and MLETR).

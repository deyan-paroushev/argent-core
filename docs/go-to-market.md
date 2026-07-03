# Argent Go-to-Market

**Collateral-control infrastructure for institutions that lend against, hold, or operate custody-held physical assets.**

## What this document is

This document explains the business context around Argent: who the product is built for, why allocated gold is the first asset, where the first commercial conversations are most credible, how the product reaches market, and how the engine can expand.

It is written as a business reference. It explains the market logic behind the current product and can be used consistently across customer, partner, and financing conversations.

---

## 1. Executive thesis

Argent is built for the institutions that make secured credit against physical assets possible: banks, secured-credit providers, custodians, vault operators, bullion platforms, and professional asset holders.

The first asset is allocated physical gold. That choice is deliberate. Gold is liquid, identifiable, non-perishable, already stored by professional custodians, and widely held as a reserve asset. It is one of the cleanest assets against which to prove a collateral-control engine.

The business problem is not that gold lacks value. The problem is that credit against custody-held gold requires a control layer the parties can rely on. A lender needs to know that specific bars are vaulted, pledged, unavailable for a second pledge, subject to an agreed borrowing base, blocked from release while debt remains outstanding, and released only after repayment or through an agreed enforcement path.

In many real-world workflows, those answers sit across custody records, legal documents, emails, spreadsheets, bank systems, manual approvals, and periodic reconciliations. The control is possible, but it is fragmented.

Argent makes the control state explicit.

The asset stays in custody. The asset is not tokenized. The lender does not become the custodian. The borrower does not spend the gold. Argent records the signed control lifecycle around the asset: attestation, pledge, credit line, drawdown, repayment, release, default, cure, and enforcement evidence.

The first business wedge is therefore simple:

> Institutions already understand gold-backed credit. Argent gives them a better control layer for making it operational, auditable, and harder to misuse.

---

## 2. Who this is built for

Argent is not a retail gold card and not a consumer token product. It is infrastructure for the institutions around a secured-credit workflow.

### 2.1 Banks and secured-credit providers

These are the primary commercial buyers.

They need a way to extend credit against custody-held physical collateral without relying only on manual reconciliation between the borrower, custodian, bank operations, and legal files.

Their job is to lend safely.

Argent gives them:

- verified pledge state;
- borrowing-base and loan-to-value discipline;
- drawdown and repayment visibility;
- no double pledge;
- no release before repayment;
- default and enforcement evidence;
- role-governed authorization.

### 2.2 Custodians, vaults, and bullion-market operators

These are the strategic workflow partners.

They already hold the asset. Argent makes their attestation usable inside a credit workflow without asking them to become a lender or take credit risk.

Their job is to make custody status reliable.

Argent gives them:

- a structured way to attest vaulted bars;
- clear status for free, pledged, release-pending, released, or enforcement-related assets;
- a record of what they confirmed and when;
- separation from the bank's credit risk.

### 2.3 Private banks, wealth platforms, and bullion platforms

These are natural channel partners.

They already sit close to clients who hold physical gold as a reserve, hedge, or store of value. They can use Argent to connect custody-held gold with a controlled credit facility.

Their job is to make gold holdings more useful without turning them into a speculative token product.

Argent gives them:

- a differentiated secured-credit product;
- an infrastructure layer they do not need to build internally;
- a way to offer liquidity without selling the client's gold;
- a clearer evidence path for operations and audit.

### 2.4 Gold-holding institutions, businesses, funds, and family offices

These are the demand-side users.

They hold allocated gold and may want liquidity without selling the asset, losing the hedge, or converting the position into token exposure.

Their job is to access working liquidity while keeping the gold position.

Argent gives them:

- a way to pledge allocated bars without selling them;
- a credit line whose use is tracked against verified collateral;
- a release path once the debt is repaid;
- a clearer view of what is pledged and what remains available.

---

## 3. Why gold first

Gold is the beachhead, not the full market.

### 3.1 Gold is the cleanest control asset

Allocated gold is easier to model than most physical commodities. A bar has a serial number, weight, fineness, vault location, and custodian record. It does not decay, expire, spoil, or require continuous quality monitoring.

That lets the product prove the hard part first: the multi-party control state.

### 3.2 Gold already sits idle as a reserve asset

Global gold demand reached record levels in 2025. The World Gold Council reported total annual demand, including OTC, above 5,000 tonnes for the first time, during a year with 53 all-time highs in the gold price. The same market backdrop included continued central-bank buying and strong investor interest. [S1] [S2]

More gold held as a strategic reserve means a larger pool of collateral that is valuable but operationally idle.

Argent exists for that idle reserve.

### 3.3 Gold proves the engine before harder assets

Warehouse receipts, base metals, tank storage, agricultural inventory, and live borrowing-base finance are natural extensions. But they introduce more complexity: quality, location, decay, insurance, price volatility, legal receipt systems, and collateral-manager workflows.

Gold first keeps the proof clean.

The sequence is:

```text
allocated gold
    -> warehouse receipt commodities
    -> broader physical collateral
    -> live borrowing-base infrastructure
```

---

## 4. What Argent is

Argent is a role-signed control record for physical collateral that stays in professional custody.

It records:

1. which asset is being pledged;
2. who attested custody;
3. which bank or secured-credit provider accepted the pledge;
4. what credit line was opened;
5. how much was drawn;
6. what was repaid;
7. whether release is allowed;
8. whether default, cure, or enforcement occurred.

The chain records the control state. It does not replace the legal agreement, custody agreement, bank facility, card processor, or enforcement process.

### Control, not title

Argent does not tokenize the gold. It does not sell the gold. It does not move ownership on-chain.

The legal and economic right remains off-chain, under the relevant custody, pledge, and lending documents. Argent records the signed operational control events that make those rights auditable and harder to misuse.

### Role separation

Each party keeps its role:

| Party | Role |
|---|---|
| Custodian | Holds the metal and confirms custody state. |
| Bank or secured-credit provider | Opens and manages the secured credit line. |
| Asset holder | Owns the allocated gold and uses the line. |
| Argent | Provides the control, evidence, and refusal layer. |

The value is not that one party controls everything. The value is that no single party has to be trusted as the sole record.

---

## 5. What the current product demonstrates

The current Argent implementation demonstrates the gold-backed secured-credit lifecycle:

- custodian attestation of allocated bars;
- pledge to a bank;
- credit line opening at a defined loan-to-value;
- drawdown and repayment;
- release only after repayment;
- default and enforcement recording;
- refusal of unsafe actions.

The most important product message is refusal by design.

Argent refuses:

- a second pledge of the same bars;
- a draw above available capacity;
- a release while debt remains outstanding;
- participation by an unapproved counterparty.

For a secured-credit provider, the system's value is not only what it records. It is what it prevents.

---

## 6. First commercial wedge

The first commercial wedge is not "people who want to spend gold." It is institutions that already hold or service gold and want a controlled way to make that gold usable in a credit workflow.

This correction matters because the asset, gold, can look like a consumer or retail story, when the real buyer is institutional. The demand-side holder feels the pain, but the facility only becomes real through the bank, secured-credit provider, custodian, or bullion-market operator that underwrites, controls, or services the workflow. The sales motion and pilot shape that follow from this are set out in section 8.

A strong first reference relationship is not about volume. It is about proving that independent parties, a lender, a custodian, and a holder, can rely on the same signed control state.

---

## 7. Geographic sequencing

### 7.1 Europe and Switzerland: credibility and design base

Europe and Switzerland are the natural credibility base.

They offer:

- regulated fintech and digital-asset infrastructure conversations;
- private-banking and wealth-management relevance;
- custody and bullion-market expertise;
- institutional investors and venture funds familiar with financial infrastructure;
- a strong setting for design-partner conversations.

Europe is where the product can be explained, funded, reviewed, and professionally framed.

### 7.2 UAE and wider GCC: strongest first commercial market

The UAE is the strongest first commercial market because the underlying behavior already exists.

Two signals matter.

First, UAE banks already lend against gold. National Bank of Fujairah is one of the leading bullion banks in the UAE, offers gold loans secured against gold held as collateral, is a significant corporate lender of gold, and even finances margin calls on gold loans, which is adjacent to Argent's exact territory. [S3]

Second, UAE private-banking channels are already bringing custody-held physical gold to professional investors. First Abu Dhabi Bank Private Banking partnered with Gilded to offer institutional-quality physical gold bars to professional investors, with storage, insurance, and authenticity handled through the service. [S4]

That is the market logic Argent needs:

```text
professional investors already hold physical gold in custody
        -> banks and platforms already service that gold
        -> credit against that custody-held gold is a natural next workflow
        -> Argent provides the control layer
```

The UAE is therefore not just a gold market. It is a market where banks, private investors, bullion infrastructure, and digital financial services already intersect.

### 7.3 Wider GCC and Southeast Asia: the Islamic-finance opportunity

The wider GCC, and Islamic-finance markets more broadly, are strategically important, and the fit is stronger than a generic "gold in the Gulf" observation. It rests on a specific structural alignment between how Argent works and how Islamic law treats gold and collateral.

The market is large and growing. Global Islamic finance assets reached about US$5.98 trillion in 2024, up 21 percent year on year, and are projected to reach roughly US$9.7 trillion by 2029 [S-IF1]. The GCC and Southeast Asia dominate the sector, with Saudi Arabia, the UAE, and Malaysia among the largest markets [S-IF1]. This is not a niche adjacent to Argent's thesis. It is a major pool of exactly the buyer Argent targets: institutions that hold gold and lend against it.

The structural fit is specific. In Islamic finance, the concept closest to what Argent controls is rahn, a pledged asset used as security for an obligation, and the use of gold as collateral (rahn) is recognized as permissible under the standard [S-IF3]. Gold itself is treated under special rules. The AAOIFI Shari'ah Standard No. 57 on Gold, developed with the World Gold Council, requires that gold be traded on a spot basis and that ownership be physical or constructive, and in the case of constructive possession the gold must be fully allocated, evidenced by same-day settlement or a certificate specifying bar ownership [S-IF2b]. Deferred-payment and derivative structures on gold are impermissible [S-IF2b].

This is precisely why Argent's control-not-title framing is an advantage rather than a hurdle. Argent does not sell gold on deferred terms, does not tokenize it, and does not create abstract exposure. The client already owns specific allocated bars held in professional custody, with the ability to take delivery, which is exactly the allocated, constructive-possession arrangement the standard recognizes as valid [S-IF2a]. Argent records the pledge, the control state, the borrowing base, drawdown, repayment, release, and refusal. The financing contract itself is structured by the Islamic bank under its own Shariah framework. That separation is the point: Argent supplies the rahn control layer, not the financing contract.

The regulatory environment reinforces this. The UAE made AAOIFI standards mandatory in 2018 [S-IF2c], so in one of Argent's strongest first markets, allocated, spot-based, non-tokenized gold treatment is not a preference but a requirement. A design built around allocated, custody-held gold may be easier for an Islamic bank and its Shariah board to review than a structure based on synthetic exposure, deferred gold settlement, or unallocated token claims.

### 7.4 How to speak to Islamic-finance institutions, and the hard boundary

The correct positioning to an Islamic bank is as a control layer for rahn-based gold finance, not as a financing product and not as a compliance claim. The language that fits the market: Argent helps an Islamic bank offer gold-backed secured liquidity to business and private-banking clients; the client keeps allocated gold in custody; the bank structures the financing under its Shariah framework; Argent supplies the rahn control layer covering pledge, borrowing base, draw, repayment, release, default, and evidence. In Islamic-banking terms, a card attached to such a line is described as a covered card, not a credit card, and profit or service-fee language replaces interest language. Using the market's own vocabulary signals competence at no cost.

The hard boundary, which must never be crossed: Argent does not claim to be Shariah-compliant. Shariah-compliance is a certified status granted by a bank's Shariah supervisory board after reviewing the actual contracts, profit model, fees, late-payment treatment, custody arrangement, and card terms. Argent is not the certifying party and cannot self-declare this status. In the UAE in particular, where AAOIFI standards are mandatory, an unearned compliance claim would be both a credibility failure and a regulatory misstatement. The only accurate framing is that Argent is designed for Shariah-compliant structuring by a partner Islamic bank, and that the bank's Shariah board approves the actual product. State the structural affinity and the intent to structure properly; never assert the certified status.

A natural product extension, when a real Islamic-bank design partner is engaged, is a Shariah evidence pack: a structured record of allocated ownership, custody attestation, that no double pledge exists, that the gold was never sold or tokenized, valuation and haircut, drawdown history, and the repayment and release trail, produced for the bank's Shariah board, risk team, and auditor. This is a natural extension of the evidence certificates the engine already produces, not a new product, and it should be built only when a partner pulls it, not on speculation.

---

## 8. Sales motion

### 8.1 Start with the institution that can carry the facility

The right first conversation is with the party that can make a facility real:

- a bank;
- a secured-credit provider;
- a private bank;
- a custodian-adjacent platform;
- a bullion-market operator.

A gold holder may feel the pain, but the facility cannot exist without the institution that underwrites, controls, or services the credit workflow.

### 8.2 First-meeting objective

The first objective is a design-partner conversation, not a signed facility.

The critical questions are:

- which party signs the custody attestation;
- what evidence backs the bar list;
- how the bank defines eligible collateral;
- what loan-to-value and haircut policy applies;
- what approval is needed before release;
- how default and enforcement are evidenced;
- what the custodian will and will not confirm;
- what the bank operations team needs to see.

Those answers shape the production product.

### 8.3 First-pilot shape

The first pilot should be narrow:

```text
one asset type: allocated gold
one custodian or vault workflow
one bank or secured-credit provider
one controlled facility structure
one complete lifecycle:
    attest -> pledge -> open line -> draw -> repay -> release
```

This is enough to prove the product category.

### 8.4 What must be validated

Argent is infrastructure, and infrastructure does not by itself resolve the legal, custodial, and structuring questions a production facility depends on. Naming these openly matters, because the boundary between what the software solves and what the institution must decide is part of an honest account of the product. Before a production facility, the following must be validated with the specific counterparties:

- which legal pledge or security document gives the bank enforceable rights over the allocated bars;
- which custodian statement is sufficient for the bank to rely on the bar list, and whether the custodian can operationally block release while the pledge is active;
- how the bank defines eligible collateral, haircut, loan-to-value, margin call, and cure;
- whether the drawdown, card, or settlement rail is conventional or Islamic;
- for Islamic structures, which Shariah contract is used and what the bank's Shariah board requires;
- which evidence pack each function needs: credit, operations, audit, legal, and, where relevant, Shariah review.

None of these is a software problem. Each is a design-partner conversation, and each is why the first reference relationship matters more than breadth of outreach.

---

## 9. Business model

Argent is infrastructure sold to the institution that carries or manages the secured-credit workflow.

Potential revenue streams:

1. **Platform fee** from the bank, secured-credit provider, custodian-adjacent platform, or collateral operator.
2. **Integration and onboarding fee** for connecting custodian records, bank workflows, and evidence outputs.
3. **Per-facility or per-position fee** for active collateral-control records.
4. **Evidence and audit pack fee** for generated certificates, replay outputs, and compliance support.
5. **Optional partner-program fee** for future sponsor or rewards overlays, where relevant.

Argent does not need to take credit risk, hold the asset, issue a card, or originate loans to capture value.

The business is infrastructure-style, not balance-sheet style.

---

## 10. Expansion arc

### Now: allocated gold

This is the current product and the cleanest proof.

### Next: warehouse receipt commodities

Warehouse receipt systems already let goods in storage serve as collateral. UNCITRAL and UNIDROIT adopted the Model Law on Warehouse Receipts to support modern warehouse receipt systems, including electronic and paper-based receipts. UNIDROIT also notes that the model law contemplates electronic platforms, distributed-ledger systems, and other technological mechanisms. [S5] [S6]

This is a natural next lane because the asset is still physical, still custody-held, and already represented by a control document.

### Then: live borrowing base

The advanced product is a live borrowing-base engine for physical collateral.

In that model, verified collateral state becomes current credit availability, availability governs drawdown, and a committed sale or offtake turns the same control state into payoff and release.

That is not the first product to sell. It is the direction the engine can grow into once the gold control layer is credible.

---

## 11. Why this can be defensible

Argent is deliberately narrow.

It is not:

- a lender;
- a custodian;
- a card processor;
- a tokenized-gold issuer;
- a gold ETF;
- a commodity exchange;
- an automated enforcement agent.

Its position is the shared control layer between parties that need to rely on the same collateral state but do not naturally share one database.

A bank's system is the bank's book. A custodian's system is the custodian's book. An asset holder's treasury file is the holder's file.

Argent's role is to become the neutral signed control record that all parties can verify.

The defensibility is the combination of:

- narrow institutional positioning;
- role separation;
- refusal logic;
- evidence discipline;
- integration with real custodians and credit providers;
- becoming the state record that parties coordinate around.

---

## 12. What this is, and what it is not

What Argent is:

- infrastructure for custody-held physical collateral;
- control, not tokenization: the asset stays in custody and title stays with the owner;
- built around gold as the clean first asset;
- secured-credit infrastructure that fits an existing bank and custodian workflow;
- a control layer that refuses a double pledge and refuses release before repayment;
- audit-ready evidence for every pledge, draw, repayment, release, and refusal;
- conservative by design, built for controlled credit rather than maximum leverage;
- extensible to warehouse receipts and broader physical collateral over time.

What Argent is not:

- not a gold token or tokenized commodity;
- not a consumer gold card, and not a way to spend gold;
- not a lender, a custodian, or a payment processor;
- not a replacement for the bank;
- not automated foreclosure, and not fully automated lending;
- not a retail or speculative product.

In one line:

> Argent makes custody-held gold usable as controlled credit collateral without selling it, tokenizing it, or moving it out of custody.

---

## 13. Source map

[S1] World Gold Council, *Gold Demand Trends: Q4 and Full Year 2025*. The report states that total gold demand, including OTC, topped 5,000 tonnes during a year with 53 all-time highs in the gold price.  
https://www.gold.org/goldhub/research/gold-demand-trends/gold-demand-trends-full-year-2025

[S2] World Gold Council, *Gold investment rockets in 2025, setting a new high as uncertainty bites*. The World Gold Council reported continued elevated central-bank demand in 2025, with 863 tonnes added by the official sector.  
https://www.gold.org/news-and-events/press-releases/gold-investment-rockets-2025-setting-new-high-uncertainty-bites

[S3] National Bank of Fujairah, *Precious Metals & Diamond Financing*. NBF is one of the leading bullion banks in the UAE and describes precious-metals financing including gold-loan solutions, secured against gold held as collateral, and financing for margin calls on gold loans.  
https://nbf.ae/en/business/industries/precious-metals-diamonds

[S4] First Abu Dhabi Bank, *FAB and Gilded launch digitally enabled physical gold investment*. FAB states that FAB Private Banking will offer institutional-quality physical gold bars to professional investors through Gilded, with storage, insurance, and authenticity verification handled through the service.  
https://www.bankfab.com/en-ae/about-fab/group/in-the-media/fab-and-gilded-launch-digitally-enabled-physical-gold-investment

[S5] UNCITRAL, *UNCITRAL-UNIDROIT Model Law on Warehouse Receipts*. The model law supports issuance and transfer of electronic and paper-based warehouse receipts.  
https://uncitral.un.org/en/mlwr

[S6] UNIDROIT, *Model Law on Warehouse Receipts*. UNIDROIT notes that the model law contemplates electronic warehouse receipts, including through electronic platforms, distributed ledger technology systems, and other technological mechanisms.  
https://www.unidroit.org/studies/model-law-on-warehouse-receipts/

[S-IF1] LSEG and ICD, *Islamic Finance Development Report 2025*. Global Islamic finance assets reached about US$5.98 trillion in 2024, up 21 percent year on year, and are projected to reach roughly US$9.7 trillion by 2029, with the GCC and Southeast Asia dominating the sector and Saudi Arabia, the UAE, and Malaysia among the largest markets.  
https://www.lseg.com/en/data-analytics/islamic-finance/islamic-market-intelligence/islamic-finance-development-report-2025

[S-IF2a] World Gold Council, *Shariah-compliant gold*. Overview of the AAOIFI gold standard, stating that gold can be owned on a physical or constructive basis and that constructive possession requires full allocation, evidenced by same-day settlement or a certificate specifying bar ownership.  
https://www.gold.org/gold-standards/shariah-gold

[S-IF2b] AAOIFI, *Shari'ah Standard No. 57 on Gold and its Trading Controls* (developed with the World Gold Council). The standard requires gold to be traded on a spot basis, permits physical or constructive ownership, requires full allocation for constructive possession, and prohibits deferred-payment and derivative gold structures.  
https://www.gold.org/download/file/18645/The-Shariah-Standard-on-Gold-English.pdf

[S-IF2c] Adoption note: AAOIFI Shariah standards were made mandatory in the UAE in 2018, as reported in the World Gold Council's Shariah-gold materials. The specific adopting instrument should be confirmed against the AAOIFI or Central Bank of the UAE announcement before use in a formal document.  
https://www.gold.org/gold-standards/shariah-gold

[S-IF3] AAOIFI, *Shari'ah Standard No. 57 on Gold and its Trading Controls*. The standard addresses the use of gold as pledge or collateral (rahn); using gold as rahn is permissible, subject to the financing contract and the possession and control requirements being properly structured under a qualified Shariah framework. This is not a statement that any particular product is Shariah-compliant.  
https://www.gold.org/download/file/18645/The-Shariah-Standard-on-Gold-English.pdf

---

## 14. Summary

Argent is collateral-control infrastructure for custody-held physical assets. It starts with allocated gold because gold is the cleanest asset for proving the control engine and because professional investors, businesses, and institutions already hold gold as a reserve. The first commercial buyer is not the retail borrower. It is the bank, secured-credit provider, custodian-adjacent platform, or bullion-market operator that carries the secured-credit workflow. The UAE and wider GCC are strong commercial markets because gold, private banking, bullion finance, and asset-backed structuring already meet there. The same control engine can later expand toward warehouse receipts and live borrowing-base finance. The business is narrow by design: Argent does not lend, custody, tokenize, or process payments. It provides the neutral, signed control state that lets existing institutions make physical collateral usable in credit.

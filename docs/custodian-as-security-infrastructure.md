# The Custodian as Security Infrastructure

Notes on why the custodian is the piece that makes Argent work. Argent records and enforces a control lifecycle over physical gold, but that record only means something if it is anchored to a real, independent statement that the gold exists, is allocated, and is held. That statement comes from the custodian. This document sets out what a custodian actually is, why it sits at the root of the whole design, and where the boundary lies between what the custodian provides and what Argent provides. Sources are linked at the end.

Related docs: `collateral-control.md`, `threat-model-and-security-boundaries.md`.

---

## What a custodian actually is

A precious-metals custodian runs secure vaults and holds bullion on behalf of others. In the London market, custody underpins the whole system: precious metals held in London vaults by custodians underpin the market's clearing and trading, and the vaults act as the gatekeepers of the market [S1][S2]. The custodians are established, specialist operators, names like Brink's, Loomis, Malca-Amit, JP Morgan, HSBC, ICBC Standard Bank, and the Bank of England, which offers a custody service for gold [S1][S-LBMA-CUST].

The defining feature, for Argent's purposes, is what allocated custody means. When gold is held on an allocated basis, specific physical bars are set aside and segregated for a specific owner, and the vault holder, acting as custodian, provides a detailed list of bar numbers, weights, and assay qualities [S-LPMCL]. That list, the bar list or weight list, records each bar's unique identifier, refiner, gross weight, and fineness [S3][S-BV]. It is the document that proves who owns which specific metal.

Two properties of allocated custody matter enormously here. First, the custodian has no right to use, lend, lease, or otherwise deal with allocated metal; it is segregated from the custodian's own assets and from other clients' assets [S3]. Second, because the owner holds legal title to specific identified bars, that metal is not part of the custodian's or the platform's bankruptcy estate if either fails [S3][S4]. This is not theoretical: when Lehman Brothers collapsed in 2008, clients with allocated metal were unaffected, while clients holding unallocated credits became unsecured creditors in the bankruptcy [S4]. Allocated custody is grounded in property law rather than contract law, which is why it is the standard used by central banks, sovereign wealth funds, and sophisticated institutional investors [S4].

The bar list is not a one-time document. Vault operators send bar lists on an ongoing basis, and independence is the point: a platform whose gold is held by a custodian has no influence over the production of the bar lists against which ownership is proven [S-BV]. On top of this, LBMA-accredited vaults are independently audited, with third-party assayers physically entering the vault to count, weigh, and verify that the metal held matches the records [S3][S-BV].

## Why this is the root of Argent's design

Argent records a signed control lifecycle over an asset: attestation, pledge, drawdown, repayment, release, default, enforcement. Every one of those events is a statement about a specific bar or set of bars. The chain can record the order of those events, enforce that they follow the rules (no double pledge, no release before repayment, no unapproved party), and make the result verifiable. What the chain cannot do, by itself, is know that the bar exists, that it is allocated to this owner, that it is really in the vault, and that it has genuinely been immobilized.

Only the custodian can supply that. The custodian is the bridge between the physical world and the signed record. It is the party that can truthfully attest: these specific bars, by serial number and weight and fineness, are held here, allocated to this owner, and are locked against release. That attestation is the ground truth the entire control lifecycle is built on top of.

This is why the custodian is the piece that makes everything click. Consider what each other party contributes and what it depends on. The bank carries the credit and relies on the pledge being real. The borrower keeps title and relies on the metal being safe and returnable. Argent records and enforces the control state and relies on the attestation being true. All three of those dependencies resolve at the same point: the custodian's statement about the physical metal. If that statement is sound, the pledge is meaningful, the credit is secured, the control record is trustworthy, and the release at the end genuinely frees the metal. If that statement is absent or unreliable, none of the downstream machinery means anything, because it would be enforcing control over an asset no one has confirmed exists.

Put simply: Argent does not create trust in the gold. It records and governs what the custodian already makes trustworthy, and it makes that trust programmable, ordered, and shared across the parties. The custodian supplies the trust; Argent supplies the control lifecycle around it.

## A structure institutions already recognize: triparty

Argent's shape, three parties with clearly separated roles, is not novel in institutional finance. It closely mirrors the triparty repo model, which is a large and established part of the collateral markets. In a triparty repo, collateral is managed by a third-party custodian or agent, typically for a fee, while the counterparty (credit) risk remains bilateral between the cash lender and the collateral provider [S-FSB]. The agent handles collateral selection, settlement, custody, and management during the life of the transaction, but does not participate in the risk of the transaction: if a party defaults, the impact falls on the counterparty, not the agent [S-ICMA]. In Europe the principal triparty agents are institutions like Clearstream, Euroclear, BNY Mellon, and JP Morgan [S-ICMA].

The parallel to Argent is close. The bank keeps the credit risk, the borrower keeps title, the custodian holds and controls the asset, and a neutral layer records and manages the control state, without itself taking on the credit or the asset. The main translation is the asset class: triparty repo is built for securities, which are already dematerialized and centrally settled, whereas Argent applies the same separation-of-roles logic to physical gold, where the equivalent of the agent's control function has to be anchored to a custodian's attestation about real bars. Framing Argent as triparty logic applied to physical collateral gives an institutional reader an immediate, trusted reference point.

This also explains, from the collateral side, why allocated custody specifically is the form that matters. Under UK and EU law, unallocated gold, because it is a credit claim rather than owned metal, generally cannot be treated as eligible collateral, while allocated gold is legally usable as collateral but is operationally complex, since in practice it can require physical transfers, deliveries, and segregation [S-WGCL]. In institutional practice, banks and specialist lenders accept allocated bullion as collateral through triparty or pledge structures, and identified bars, clean title, and custody confirmations reduce haircuts and legal friction compared with pooled claims [S-ALLOC]. So the allocated bar list is not just a nicety; it is closer to the collateral-eligible form, and the operational friction around it, moving, segregating, confirming, is exactly the layer a shared control record can help with.

## The boundary: what the custodian provides, and what Argent provides

Being precise about this boundary is important, because it defines what Argent is and is not, and it is exactly the boundary a bank's risk and operations teams will probe.

The custodian provides the physical security, the segregation of allocated metal, the bar list that identifies specific bars and their owner, the operational ability to immobilize (lock) and release metal, the independent audit, and the insurance. These are the custodian's existing business; Argent does not replace any of them.

Argent provides the shared, signed, ordered record of control events referencing that custodied metal, the enforcement of the rules across that lifecycle, and the verifiable evidence trail that lets the bank, the custodian, and the owner rely on the same state. Argent does not hold metal, does not take title, and does not attest to physical facts. It records and governs attestations made by the party entitled to make them.

The critical dependency, stated plainly: Argent's control record is only as reliable as the custodian attestation it anchors to. A signed on-chain statement that a bar is locked is trustworthy exactly to the degree that the custodian's confirmation behind it is trustworthy. This is why the custodian relationship is not a detail to be arranged later. It is the foundation, and it is the single most important thing to get right in any real deployment. The strength of the whole system rests on the quality and enforceability of the custodian's attestation and its operational ability to actually block release while a pledge is active.

## Where the industry is heading: making custody data trusted and structured

The direction the precious-metals industry is taking supports the same view: the goal is not to remove the custodian but to make custody and bar data more trusted, structured, and usable. The LBMA's Gold Bar Integrity (GBI) Ecosystem is a concrete example. It aims to digitally trace gold through the supply chain, confirming provenance and giving transparency over the value chain, and its GBI Database, built on distributed-ledger technology by the Swiss provider aXedras (appointed in March 2024), is the governance tool that collects London vault-holdings data and refiner data [S-GBI]. As of the start of 2026, all Good Delivery refiners were using the database, and the published timeline sets out custodians onboarding to report aggregated vault holdings from December 2026, an intent to move to bar-level reporting for custodians in 2027, and an ultimate goal of near real-time reporting [S-GBI].

This matters for Argent in two ways. It confirms that the trusted layer the whole market is investing in is custody and bar data, not a replacement for the custodian, which is the same premise Argent rests on. And it points at a future in which custodian attestations about specific bars become more standardized and machine-readable, which is precisely the kind of input a control record like Argent's anchors to. Argent operates at a different layer from GBI, GBI is about provenance and integrity of the bar itself, Argent is about the control lifecycle (pledge, draw, release) around a bar used as collateral, but both depend on, and both strengthen, the same thing: trusted, structured custody data.

## What this means for building and selling Argent

Because the custodian is the root, the custodian conversation is arguably the highest-leverage one in bringing Argent to production, alongside the lender conversation. Several things follow.

A custodian, vault operator, or bullion platform can find value in Argent before a bank does, because Argent gives them a way to turn their custody status, free, pledged, release-pending, released, under enforcement, into a clean, credit-grade control signal that a lender can rely on. That is useful to the custodian's own institutional relationships independently of any single bank.

The questions that must be settled with a custodian in any real deployment are concrete: which custodian statement is sufficient for a bank to rely on the bar list, whether the custodian can operationally block release while a pledge is active, how lock and release status is represented and updated, and what the custodian is and is not willing to attest to. None of these is a software question. Each is a design-partner conversation, and each is why a custodian design partner is as important as a lending one.

The honest summary: gold as collateral works because custody works. Argent's contribution is the control layer on top, but the layer underneath, the custodian, is what the whole structure stands on. One way to state Argent's role precisely, and consistent with how the product is described elsewhere, is that it makes custody status credit-grade: it takes the custodian's status for an asset, free, pledged, release-pending, released, under enforcement, and turns it into a shared, signed, ordered record a lender can rely on, without the custodian becoming a lender or Argent taking the asset. Getting the custodian attestation right is not one task among many. It is the task the rest depends on.

## Sources

[S1] LBMA, *Vaulting*. Precious metals held in London vaults by custodians underpin the market's clearing and trading; there are six custodians offering vaulting services in the Loco London market, and the Bank of England also offers a gold custody service.  
https://www.lbma.org.uk/market-standards/vaulting

[S2] LBMA, *London Vault Data*. Vaults provide secure storage for bullion and act as gatekeepers to the Loco London precious metals market; they are its custodians.  
https://www.lbma.org.uk/prices-and-data/london-vault-data

[S3] Bullion Trading LLC, *Allocated vs Unallocated Gold Storage*, citing the LBMA guide. In an allocated account the custodian has no right to use, lend, lease, or deal with the gold; the metal is segregated from the custodian's own and other clients' assets; each bar appears on a weight list with its unique identifier, refiner, gross weight, and fineness; and if the custodian fails, allocated gold is not part of the bankruptcy estate.  
https://bulliontradingllc.com/blog/allocated-vs-unallocated-gold-storage/

[S4] GBI Direct, *How Professional Gold Vaults Work*. Allocated storage means metal registered in the owner's name at a vault; in the 2008 Lehman collapse, allocated-account clients were unaffected while unallocated-credit clients became unsecured creditors. Professional vaults undergo regular independent audits by third-party assayers who physically count and weigh holdings. Allocated storage is the standard used by central banks, sovereign wealth funds, and institutional investors, and is required by the LBMA for clearing.  
https://gbidirect.com/insights/gold-vault-storage-allocated/

[S-LPMCL] LPMCL, *Loco London Clearing*. Allocated accounts are opened when a customer requires metal to be physically segregated, with a detailed list of bar numbers, weights, and assay qualities provided by the vault holder acting as custodian on behalf of the client.  
https://www.lpmcl.com/loco-london-clearing

[S-BV] BullionVault, *Vault FAQs*. Independent vault operators (Loomis, Malca-Amit, Brink's) send bar lists evidencing what they hold; the platform has no influence over the production of the bar lists against which ownership is proven; and the platform has the right to send an independent assayer into the vault to verify the bars.  
https://www.bullionvault.com/help/FAQs/FAQs_vaulting.html

[S-LBMA-CUST] LBMA, *Custodians offering vaulting services in London*. The listed custodians include the Bank of England (gold only), Brink's, HSBC, ICBC Standard Bank, JP Morgan Chase, and Malca-Amit.  
https://www.lbma.org.uk/market-standards/vaulting/custodians-offering-vaulting-services-in-london

[S-FSB] Financial Stability Board, *Vulnerabilities in Government Bond-backed Repo Markets* (February 2026). In the triparty repo segment, collateral is managed by a third-party custodian, typically for a fee, while counterparty risk remains bilateral between the cash lender and the collateral provider, and outsourcing collateral management reduces some operational risk.  
https://www.fsb.org/uploads/P040226.pdf

[S-ICMA] ICMA, *What is tri-party repo* and triparty course materials. Triparty repo outsources post-trade processing, collateral selection, settlement, custody, and management to a third-party agent; the agent does not participate in the risk of the transaction, so if a party defaults the impact falls on the counterparty. Principal European triparty agents include Clearstream, Euroclear, BNY Mellon, and JP Morgan.  
https://www.icmagroup.org/market-practice-and-regulatory-policy/repo-and-collateral-markets/icma-ercc-publications/frequently-asked-questions-on-repo/24-what-is-tri-party-repo/

[S-WGCL] World Gold Council and Linklaters, *Pooled Gold Interests and Wholesale Digital Gold* white paper (2025). Unallocated gold, being a credit claim, generally cannot be treated as eligible collateral under UK and EU law; allocated gold is legally usable as collateral but is operationally complex, in practice requiring physical transfers, deliveries, and segregation.  
https://www.gold.org/what-we-do/gold247/linklaters-white-paper

[S-ALLOC] Industry analysis of allocated versus unallocated gold. Banks and specialist lenders frequently accept allocated bullion as collateral through triparty or pledge structures; identified bars, clean title, and custody confirmations reduce haircuts and legal friction compared with pooled claims.  
https://goldenarkreserve.com/blog/allocated-vs-unallocated-gold-key-differences/

[S-GBI] LBMA, *Gold Bar Integrity Ecosystem*. The GBI Ecosystem digitally traces gold through the supply chain to confirm provenance and provide transparency. Its GBI Database, built on distributed-ledger technology by aXedras (appointed March 2024), collects London vault-holdings and refiner data. As of the start of 2026, all Good Delivery refiners use the database. The published timeline includes custodians reporting aggregated vault holdings from December 2026, intent to move to bar-level reporting for custodians in 2027, and an ultimate goal of near real-time reporting.  
https://www.lbma.org.uk/gold-bar-integrity-ecosystem

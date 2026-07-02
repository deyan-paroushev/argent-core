# Collateral as Locked Value: the Missing Instrument in Physical-Commodity Finance

**Why the constraint in financing physical commodities is instrumentation, not asset quality, and what the market signals say about the gap Argent fills.**

*This document is evidence-led. It reads the market's own signals, from central banks, standard-setters, the largest collateral-infrastructure providers, and the academic literature, and lets them define the problem before naming where Argent sits. It is not investment advice, and it does not claim a single addressable market figure. Figures are cited inline; where sources disagree, the disagreement is stated. The closing section separates Argent's specific position from the neutral reading that precedes it. Companion reading: `physical-collateral-and-trade-finance.md` (the neutral market primer), `collateral-control.md` (the product thesis), and `protocol.md` (the technical specification).*

---

## 1. The thesis in one line

A physical commodity in custody is stored value. The owner cannot rotate it, and the lender cannot lend fully against it, not because the value is in doubt but because neither party has a reliable, shared, real-time instrument for governing control over it. The value is locked by the absence of instrumentation, not by any shortage of the asset. Give the parties a shared control instrument and the same tonne of metal or barrel of oil becomes operational for everyone in the chain at once: the owner keeps working capital moving, the lender lends closer to true value, the custodian reconciles less, and the auditor reconstructs nothing after the fact. This document assembles the market signals that support that reading.

## 2. The value is locked, and the market says so in its own words

The clearest evidence that this is a locked-value problem is that the market already describes it that way. The language of "trapped liquidity" and "releasing working capital" is not Argent's framing; it is the industry's.

Citi's 2026 supply-chain finance report finds that, on average, 6.3% of corporate working capital is now tied up in funding costs, and that companies are deploying inventory finance and structured receivables specifically to release trapped liquidity.[^citi] J.P. Morgan puts a number on one slice of it: roughly US$633 billion in potential liquidity trapped in the supply chains of S&P 1500 companies alone.[^jpm] The Asian Development Bank's benchmark survey holds the global trade finance gap at about US$2.5 trillion, roughly 10% of global trade, and attributes it substantially to compliance burden, Basel III liquidity requirements, and the difficulty of establishing trust in the underlying transaction, rather than to any lack of capital willing to be deployed.[^adb2025][^adbbasel]

The through-line is consistent. The capital exists. The goods exist and hold value. What is missing is the confidence and legibility that would let the capital meet the goods. That missing element is an instrument problem.

## 3. The binding constraint is instrumentation, confirmed by how banks behave

If the problem were asset quality, banks would decline. Instead they lend, but conservatively, and the specific pattern of that conservatism reveals that the true constraint is visibility and control.

The consulting literature on collateral operations is unusually direct. EY finds that, without an automated, holistic view of collateral, firms pledge more expensive or less efficient collateral than necessary, which raises funding costs and ties up liquidity that could be deployed elsewhere.[^eyauto] The same body of work names the concrete failure modes of today's manual, spreadsheet-bound, fragmented processes: improper release of collateral, and misreporting of collateral valuations, worst during volatility when workload spikes and errors compound.[^eyservices] EY's collateral-optimization analysis is more pointed still: limited transparency and fragmented infrastructure are the primary drivers of inefficiency, with many institutions running upward of ten separate groups managing collateral on inconsistent data, unable to see available inventory, requirements, and eligibility at the enterprise level.[^eyopt]

The academic evidence closes the argument. Movable assets, receivables, equipment, inventory, make up roughly 78% of the capital stock of firms in the developing world, yet they were traditionally the least-accepted collateral, because their transferability made banks fear misappropriation and undersized recovery.[^ongena] The euro-area evidence is the same in a developed market: physical movable assets are among the least-pledged collateral forms despite the economic weight they carry.[^ecb3095] Most telling of all: when European legal reforms finally let firms pledge movables without surrendering possession, and required collateral registries to reduce misappropriation risk, firms in movable-intensive sectors did issue more secured loans, but the average cost of those loans rose and covenants increased, because banks still had to compensate for the residual monitoring and control risk the law alone could not remove.[^ongena] The reform fixed the legal barrier. The operational control barrier remained, and banks priced it.

That residual, priced control risk is the exact wedge. It is not a legal gap and not an asset gap. It is the absence of an instrument that makes multi-party control over a movable asset continuously legible. The Federal Reserve's loan-level work points the same way from the recovery side: banks set advance rates and loss-given-default expectations according to how legible and enforceable recovery is, not according to the raw value of the asset, so anything that makes control and recovery more legible directly expands borrowing capacity.[^fed]

## 4. Everyone in the chain holds a different fragment of the same truth

The reason no single party can supply this instrument alone is that a financed commodity is a shared state, and each party sees only its own fragment. The owner knows the goods and wants flexibility. The lender bears the credit risk and wants control and coverage it can see in real time. The custodian holds the goods and attests to their state, and wants less reconciliation and liability. The insurer carries exposure it cannot always see. The auditor and supervisor must reconstruct, after the fact, what was pledged, who released it, and whether coverage held.

The industry's own diagnosis is that these fragments live in siloed systems that do not reconcile. ISDA's derivatives-market work reframes this bluntly: inefficiencies in collateral eligibility, settlement, and mobilization are no longer tolerable frictions but systemic vulnerabilities, and fragmented infrastructure across trading, treasury, and risk, and across jurisdictions, is what inhibits efficient mobilization.[^isda] The Bank of England's Project Meridian names the difficulty of mobilizing and substituting collateral quickly as a core inefficiency and points to programmability, embedding the control conditions into the workflow, as part of the remedy.[^boe]

The instrument the chain lacks is therefore not another siloed system. It is a single shared control record the parties can rely on together: one place where the pledge, the coverage, the release authority, and the settlement are the same signed fact for everyone, rather than five records that drift apart and are reconciled at cost.

## 5. The market is reaching for this, but with the wrong tool for physical goods

Here the signals become sharpest, because the market has already identified the need and is reaching for the only instrument it has: tokenization.

The 2026 investor consensus has moved decisively from "can we tokenize an asset" to "what can the asset do once it is on-chain," and the answer everyone gives is collateral usability: assets that can be pledged, mobilized, and reused as collateral in real time.[^rwapred] The IMF describes tokenized securities being mobilized as collateral in near real time to improve high-quality liquid asset usage, and, crucially, records that the operative requirement in live pilots was mobilizing collateral while maintaining legally enforceable control.[^imf][^imfsp] Control is the word that matters. The largest infrastructure players say the quiet part directly: Digital Asset markets its platform as a way to tokenize hard-to-move assets such as gold or commodities in order to free trapped assets and lock them at source as mobilizable collateral.[^digitalasset] Deutsche Bank frames the whole institutional driver as freeing up capital by redeploying collateral instantly rather than tying it up.[^dbliquidity]

But tokenization is the wrong instrument for a physical commodity, and the market's own data shows it. Across roughly US$22 to 25 billion of tokenized real-world assets, the value is overwhelmingly financial paper, Treasuries, private credit, and tokenized funds.[^rwaxyz][^medium] Tokenized commodities are a small fraction of that, and more than 80% of the commodity slice is gold, which is a tokenized ownership claim, not programmable control of physical goods in custody.[^investax] Even where tokenization is technically available, most tokenized real-world assets remain thinly traded, with long holding periods and little secondary activity, because a digital wrapper does not change the economics or the custody of the underlying thing.[^investax] For a bank holding physical metal, fuel, or warehouse stock, tokenizing the asset is precisely what it does not want: it introduces custody, legal-title, and liquidity problems it does not have, to solve a control problem it does have.

So the market has correctly named the need, real-time, legally enforceable control that frees trapped collateral, and is applying an instrument (tokenized ownership) that works for securities and fails for physical commodities. That mismatch is the opening.

## 6. Why now: the timing signals converge

Three independent clocks are striking at once, which is what makes the gap live rather than perennial.

Regulation is pushing banks toward efficient, legible, mobile collateral. The final Basel III package presses banks toward more active management of risk-weighted assets and more efficient collateral strategies, and Basel III liquidity requirements are named by banks themselves as a barrier to extending trade finance under the old manual model.[^adbbasel][^basel] Market infrastructure is building the rails: the Eurosystem's Pontes initiative targets settlement in central-bank money for DLT-based transactions, and DTCC, the core of US securities settlement, has an SEC no-action path to tokenize DTC-custodied assets with rollout anticipated in the second half of 2026, selecting Stellar in part because asset-level control is first-class in the protocol.[^ecbappia][^dtcc] And capital is flowing into RWA and collateral infrastructure on the explicit thesis of collateral usability, while leaving physical-commodity control untouched.[^rwapred]

A European bank reading these signals is being told from above to modernize collateral handling, to prepare for on-chain settlement, and to free trapped working capital, and is being offered, for its physical commodity book, essentially nothing that does not require tokenizing the asset. The instrument gap and the mandate to close it are arriving in the same window.

## 7. What the market is truly missing, stated plainly

Pulling the signals together, the missing piece is specific and narrow:

> A shared, real-time, multi-party instrument that makes *control* over a physical commodity in custody legible and governable, signed by the parties, enforceable in its own logic, and auditable by construction, without tokenizing, moving, or re-titling the asset.

Not a tokenized ownership claim. Not another siloed bank system. Not a documentary-digitization layer that speeds up paper but leaves the live control state fragmented. An instrument for the control relationship itself, the one thing that, once legible, lets the locked value become operational for the owner, the lender, the custodian, and the insurer simultaneously, subject always to the lender never being left under-collateralized.

Every signal in this document points at that shape of instrument, and every existing tool either solves it for the wrong asset class (tokenized securities) or fails to solve it at all (manual, siloed collateral operations).

## 8. Claims this document does not make

Discipline matters more here than anywhere, because this is the section a skeptical reader checks first.

This document does not claim that a chain or database verifies physical truth; a control record captures signed attestations, not ground truth. It does not claim gold is a Basel III high-quality liquid asset; it is not, and no change has been announced. It does not claim the addressable market is the whole trade-finance or commodity-finance figure; those are surrounding markets, and the addressable opportunity is the control-instrument gap inside them. It does not claim tokenization is useless; it is the right tool for many financial assets and the wrong one for physical commodities in custody. It does not claim release or enforcement happens automatically in the physical world; a custodian still performs the physical act. And it does not claim any instrument can replace custodians, banks, legal agreements, or courts. The defensible claim is narrower and stronger: the binding constraint on financing physical commodities is the absence of a shared control instrument, and that gap is real, named by the market, and currently unfilled for physical goods.

## 9. Where Argent fits

*This section is separated from the neutral reading above.*

Argent is one specific instrument for the gap this document identifies. Its position follows directly from the signals: the physical asset stays in professional custody and is never tokenized, and what becomes legible is the *control* over it, recorded as a sequence of signed, ordered, verifiable events, who attested to the goods, who pledged them, who may release them, under what conditions, and whether the lender's coverage still holds at each step. The governing structure carries no commodity field, which is what makes the same instrument reusable across metals, energy, minerals, and warehouse-held commodities rather than tied to any one of them. That is the "more globally than gold" property expressed in the contract design itself: gold is simply the first adapter, not the market.

In the terms of this document, Argent is the shared control instrument the parties currently lack. It reduces the drift between the owner's, lender's, custodian's, legal, and settlement records by making the control state one signed record they share, rather than five they reconcile. It is the answer to the market's stated need for collateral that can be mobilized while maintaining legally enforceable control, applied to the asset class, physical commodities in custody, that tokenization cannot serve. And it is deliberately signer-agnostic, so an institutional signing layer governs authority while Argent holds no party's keys.

Whether this instrument proves out at scale is exactly the open, adoption-and-proof question the neutral sections frame, and it will be settled by design partners and live deployment rather than by argument. The technical design, boundaries, and current status, 224 passing tests across three deployed Soroban contracts, are documented in `protocol.md`, `argent-architecture.md`, and `collateral-control.md`. The bank pain points and the four programmable-control primitives that operationalize this thesis, substitution, multi-party controlled release, conditional release on verified sale, and partial release with a dynamic borrowing base, are set out in `collateral-control.md`. An invitation to institutions that live with this gap is in `design-partners.md`.

---

## References

[^citi]: Citi, *Supply Chain Financing: Durable Global Trade in the Age of AI* (Citi GPS), 2026 (6.3% of working capital tied up in funding costs; inventory finance and structured receivables deployed to release trapped liquidity). https://www.citigroup.com/global/news/press-release/2026/citi-supply-chain-financing-report-durable-global-trade-in-the-age-of-ai

[^jpm]: J.P. Morgan, "5 Trends in Trade" (approx. US$633 billion in potential liquidity trapped in S&P 1500 supply chains). https://www.jpmorgan.com/insights/payments/trade-and-working-capital/trends-in-trade-2024

[^adb2025]: Asian Development Bank, *ADB Trade Finance Gap Survey* (gap ~US$2.5 trillion, ~10% of global trade). https://www.adb.org/publications/adb-global-trade-finance-gap-survey

[^adbbasel]: Asian Development Bank, trade finance gap briefs (90% of surveyed banks cite AML/KYC and 77% cite Basel III liquidity requirements as barriers to expanding trade finance). https://www.adb.org/news/global-trade-finance-gap-reaches-16-trillion-smes-hardest-hit-adb

[^eyauto]: Ernst & Young / industry analysis, collateral-management automation (without a holistic real-time view, firms pledge more collateral than necessary, raising funding costs and tying up liquidity). EY Insights, Banking & Capital Markets.

[^eyservices]: Ernst & Young, "How banks can align collateral functions to a services-based model" (improper release of collateral and misreporting of valuations as concrete risks of manual, fragmented processes). EY Insights, Banking & Capital Markets.

[^eyopt]: Ernst & Young, "Collateral optimization: capabilities that drive financial resource efficiency" (limited transparency and fragmented infrastructure as primary inefficiency drivers; siloed teams unable to see enterprise-level inventory, requirements, and eligibility). EY Insights, Banking & Capital Markets.

[^ongena]: S. Ongena, W. Saffar, Y. Sun, and L. Wei, *Movables as Collateral and Corporate Credit: Loan-Level Evidence from Legal Reforms across Europe*, Swiss Finance Institute Research Paper No. 22-75 (movables ~78% of firm capital stock; post-reform secured issuance rose but loan cost and covenants also rose as banks priced residual monitoring and misappropriation risk).

[^ecb3095]: H. Degryse, O. De Jonghe, L. Laeven, and T. Zhao, *Collateral and credit*, ECB Working Paper Series No. 3095, 2025 (physical movable assets among the least-pledged collateral forms in the euro area). https://www.ecb.europa.eu/pub/pdf/scpwps/ecb.wp3095~0e81ee7f34.en.pdf

[^fed]: A. Gupta, H. Sapriza, and V. Yankov, *The Collateral Channel and Bank Credit*, Federal Reserve Board Finance and Economics Discussion Series 2022-024r1, 2025 (advance rates and expected loss given default track legibility and enforceability of recovery, not raw asset value). https://doi.org/10.17016/FEDS.2022.024r1

[^isda]: International Swaps and Derivatives Association, *Collateral and Liquidity Efficiency in the Derivatives Market* (collateral inefficiencies reframed as systemic vulnerabilities; fragmented, siloed infrastructure inhibits mobilization). https://www.isda.org/a/TbfgE/Collateral-and-Liquidity-Efficiency-in-the-Derivatives-Market.pdf

[^boe]: Bank of England and BIS Innovation Hub London Centre, *Project Meridian Securities* (difficulty of mobilizing and substituting collateral as a core inefficiency; programmability as part of the remedy).

[^rwapred]: Industry 2026 outlooks on real-world assets (the shift from "can it be tokenized" to "collateral usability"; RWAs as functional building blocks that can be pledged and reused as collateral). Falcon Finance and Centrifuge commentary, as reported by Cryptonews / Yahoo Finance and Centrifuge Labs, December 2025 to January 2026.

[^imf]: International Monetary Fund, *Tokenized Finance*, IMF Notes No. 26/01, April 2026 (tokenized securities mobilized as collateral in near real time to improve HQLA usage). https://www.imf.org/-/media/files/publications/imf-notes/2026/english/insea2026001.pdf

[^imfsp]: International Monetary Fund, "Tokenized Finance and Money," May 2026 (DTCC pilot: banks, custodians, CSDs, and CCPs mobilized tokenized US Treasuries as collateral in real time while maintaining legally enforceable control). https://www.imf.org/en/news/articles/2026/05/11/sp051126-tokenized-finance-and-money

[^digitalasset]: Digital Asset, "Collateral and Asset Mobilization" (tokenize hard-to-move assets such as gold or commodities to free trapped assets and lock them at source as mobilizable collateral). https://www.digitalasset.com/use-cases/collateral-and-asset-mobilization

[^dbliquidity]: Deutsche Bank, "How tokenised assets transform liquidity management" (institutional driver: free up capital by redeploying collateral instantly rather than tying it up). https://flow.db.com/Topics/trust-and-securities-services/how-tokenised-assets-transform-liquidity-management

[^rwaxyz]: RWA.xyz, tokenized real-world asset analytics, 2026 (category composition dominated by tokenized Treasuries and private credit). https://app.rwa.xyz/

[^medium]: J.-H. Liu, "Tokenization of Real-World Assets: Technology Stacks, Platform Architectures, and Comparative Evaluation," 2026 (on-chain tokenized asset market exceeding ~US$22 billion, concentrated in financial paper).

[^investax]: InvestaX, "What Is Real-World Asset (RWA) Tokenization" and referenced arXiv analysis, 2026 (tokenized commodities a small fraction of the market, 80%+ gold; most tokenized RWAs thinly traded with long holding periods). https://investax.io/blog/what-is-real-world-asset-rwa-tokenization

[^basel]: Basel III finalization commentary on collateral efficiency and risk-weighted-asset management (banks pushed toward more efficient collateral strategies).

[^ecbappia]: European Central Bank, Pontes initiative and the Appia roadmap for a European tokenised financial ecosystem, 2026.

[^dtcc]: The Depository Trust & Clearing Corporation and Stellar Development Foundation, "DTC's tokenization service to connect with Stellar public blockchain," DTCC press release, May 2026; and DTC SEC no-action path with rollout anticipated in H2 2026 (asset-level control first-class in the Stellar protocol). https://flow.db.com/Topics/trust-and-securities-services/how-tokenised-assets-transform-liquidity-management

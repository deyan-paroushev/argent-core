# Gold Market Notes

> **Document role:** Market background only. Current product positioning is defined in `reserve-obligation-infrastructure.md`; the implemented code remains the secured-credit reference branch.

Notes on the gold market Argent works in: how much gold exists, who holds it, whether lending against it is already common, and where the institutional side seems to be heading. This is background research, not a pitch. Sources are linked at the end so anything here can be checked, and where a number is an estimate or a range it is written that way.

Related docs: `collateral-control.md`, `commodity-finance-positioning.md`.

---

## How much gold there is, and who holds it

The World Gold Council estimates roughly 220,000 tonnes of gold have been mined and sit above ground, worth on the order of US$31 trillion at end-2025 [S1]. It splits, very roughly, like this:

```
Jewellery                    ~97,650t   45%  ██████████████████
Bars & coins                 ~46,950t   22%  █████████
Central bank reserves        ~38,670t   18%  ███████
Other / industrial           ~22,600t   10%  ████
Institutional/HNW OTC (est.) ~5-10,000t  3%  █
Gold ETFs                     ~4,025t    2%  █
```

The part that matters here is the investment gold, not the jewellery or the industrial use. The World Gold Council puts the bullion held by individual and institutional investors, bars, coins, gold ETFs, and over-the-counter holdings together, at around US$9 trillion. In their framing that is about 3 percent of the roughly US$320 trillion held across global financial assets [S1]:

```
Global financial assets  ~US$320tn  ████████████████████████████████████████
Investor gold bullion    ~US$9tn    █   (about 3%)
```

Within that investor gold, the slice most relevant to a lending or collateral use is the gold held over-the-counter by institutions and high-net-worth investors, since it tends to be allocated, in professional custody, and held privately rather than through a fund. The World Gold Council estimates that OTC pool in a range of roughly 5,000 to 10,000 tonnes, on the order of US$1 trillion to US$1.5 trillion at recent prices [S1].

A note on the exact numbers: they move with the gold price, and the tonnage split above is the World Gold Council's own primer estimate. So treat these as good-order-of-magnitude figures rather than precise counts, which is how the World Gold Council itself presents them.

## Whether the pool is growing

Recent demand has been unusually strong. In 2025, total gold demand including OTC passed 5,000 tonnes for the first time, at a value of about US$555 billion, in a year with 53 record-high gold prices [S2]. Bar and coin buying hit a multi-year high and gold ETF holdings grew over the year [S2].

Central banks have been a steady part of this. They added 863 tonnes to official reserves in 2025, above their long-run annual average, and have been consistent net buyers of gold on an annual basis since 2010 [S3]. So the holdings that would be relevant to a collateral or lending product are not a shrinking legacy pool; they have been growing.

## Is lending against gold already a thing?

Yes, and it is worth being clear that this is an existing practice rather than a new behavior that would need to be created.

Private banks and wealth managers offer Lombard-style loans against pledged bullion, letting a holder raise cash without selling the metal [S4]. Bullion dealers and specialist lenders provide loans secured against allocated gold as a standard product [S4]. In the wholesale market, precious-metal lending and borrowing runs through the London Bullion Market Association over-the-counter market among institutional participants [S5]. In the UAE, the National Bank of Fujairah describes gold-loan products secured against gold held as collateral, including financing for margin calls on those loans [S-GTM3].

What is generally not standardized across these channels is the control record around the loan: the pledge, the custody confirmation, the borrowing base, the drawdown and release, and the enforcement trail tend to live across custody books, paper, legal files, and reconciliations rather than in one shared, verifiable place. So the gap is less "does anyone lend against gold" and more "how cleanly is the control state tracked."

One qualification worth keeping in mind: not every gold holder can actually pledge the asset. Whether a fund or institution can encumber its gold depends on its own mandate and documents, and some regulated structures cannot. So the practically addressable holders are the professional, pledge-eligible ones, funds and family offices whose mandates permit it, precious-metals businesses, and banks or lenders already close to collateralized lending, rather than the entire gold-holding universe.

## Where the institutional infrastructure seems to be heading

Two data points suggest the broader institutional interest is in collateral mobility and control rather than in creating new gold instruments.

First, the Depository Trust and Clearing Corporation, the core post-trade utility in the US (its depository subsidiary provides custody and servicing for securities valued at over US$100 trillion, and DTCC subsidiaries processed US$4.7 quadrillion in transactions in 2025), has described collateral mobility as the key use case for its distributed-ledger work [S-DTCC1]. A participant in that work framed the point as speeding up collateral mobility "without losing operational control" [S-DTCC2]. That work is aimed at financial collateral, Treasuries, index ETFs, equities, which are already in a registry and already priced [S-DTCC3]. Physical gold sits outside that: no central registry, no continuous per-bar price, verification through custody rather than a book entry.

Second, and more directly about gold, Euroclear (a central-securities-depository group) ran a pilot in 2024 with Digital Asset, the World Gold Council, and the law firm Clifford Chance that used gold, alongside gilts and Eurobonds, as collateral over distributed-ledger technology, with 27 participants including banks, custodians, and central counterparties [S-WGC1]. The World Gold Council's own market-structure lead described the aim as overcoming the restrictions on moving and storing physical metal so gold can be used within financial markets [S-WGC1]. Worth noting: that pilot mobilizes gold by creating a tokenized digital twin of it, which is a different technical route from keeping the metal untokenized in custody and recording control over it. Both approaches are aimed at the same underlying need, gold that can serve as usable collateral.

## Short version

Gold is held at large scale, on the order of US$9 trillion in investor bullion, with a pledge-relevant institutional OTC pool of roughly 5,000 to 10,000 tonnes, and that base has been growing. Lending against gold already exists across private banks, bullion dealers, and the wholesale market, so the demand is established; what is fragmented is the control record around it. And the larger institutional infrastructure, from DTCC to a World Gold Council collateral pilot, has been moving toward collateral mobility and control. What none of this settles is adoption: whether a given bank or lender will take up a particular product is a commercial question that market data cannot answer, only a real counterparty can.

## Sources

[S1] World Gold Council, *Gold Market Primer: Market size and structure*. Roughly 220,000 tonnes of gold above ground, worth on the order of US$31 trillion at end-2025. The total financial physical gold market (bars, coins, ETFs, central-bank reserves, OTC) is on the order of US$14 trillion, about 45 percent of above-ground stock. Investor bullion (bars, coins, gold ETFs, OTC) is estimated at around US$9 trillion, about 3 percent of roughly US$320 trillion in global financial assets. OTC holdings by institutions and high-net-worth investors are estimated at about 5,000 to 10,000 tonnes. Figures are estimates and move with the gold price.  
https://www.gold.org/goldhub/research/market-primer/gold-market-primer-market-size-and-structure

[S2] World Gold Council, *Gold Demand Trends: Q4 and Full Year 2025*. Total gold demand including OTC passed 5,000 tonnes for the first time in 2025, at a value of about US$555 billion, in a year with 53 record-high gold prices; bar and coin demand reached a multi-year high and ETF holdings grew.  
https://www.gold.org/goldhub/research/gold-demand-trends/gold-demand-trends-full-year-2025

[S3] World Gold Council, *Gold Demand Trends: Q4 and Full Year 2025*. Central banks added 863 tonnes to official gold reserves in 2025, above their long-run annual average, and have been consistent net buyers of gold on an annual basis since 2010.  
https://www.gold.org/goldhub/research/gold-demand-trends/gold-demand-trends-full-year-2025

[S4] Lombard and bullion-backed lending, market practice. Private banks, wealth managers, bullion dealers, and specialist lenders offer loans secured against allocated gold and bullion, letting holders raise liquidity without selling the metal. Terms vary by provider and should be confirmed per counterparty.  
Representative example: https://jrotbart.com/gold-backed-loans/

[S5] London Bullion Market Association, *The OTC Guide: Lending and Borrowing Metal*. The LBMA over-the-counter market includes established lending and borrowing of precious metal among institutional participants.  
https://www.lbma.org.uk/publications/the-otc-guide/lending-and-borrowing-metal

[S-GTM3] National Bank of Fujairah, *Precious Metals and Diamond Financing*. NBF describes precious-metals financing including gold-loan products secured against gold held as collateral, and financing for margin calls on gold loans.  
https://nbf.ae/en/business/industries/precious-metals-diamonds

[S-DTCC1] CoinDesk, *DTCC taps Chainlink for its tokenized collateral platform* (May 12, 2026). DTCC's Collateral AppChain uses Chainlink to automate pricing, valuation, margining, and settlement, framed around near real-time collateral mobility across markets and blockchains. DTCC subsidiaries processed US$4.7 quadrillion in transactions in 2025.  
https://www.coindesk.com/business/2026/05/12/dtcc-taps-chainlink-for-its-tokenized-collateral-platform-ahead-of-q4-launch

[S-DTCC2] Traders Magazine, *DTCC Tokenization Initiative Will be 'Transformational'*. A participant describes the aim as speeding up collateral mobility "without losing operational control."  
https://www.tradersmagazine.com/departments/digital-assets/dtcc-tokenization-initiative-will-be-transformational/

[S-DTCC3] The Block, *DTCC subsidiary authorized to offer tokenization service*. DTC's tokenization scope covers Russell 1000 equities, major index ETFs, and US Treasuries, all financial assets already in central custody. DTC provides custody and servicing for securities valued at over US$100 trillion.  
https://www.theblock.co/post/382331/dtcc-subsidiary-tokenization-service-us-securities

[S-WGC1] Euroclear, *Using DLT to enhance collateral mobility*, and the Digital Asset / World Gold Council / Clifford Chance pilot (June to July 2024). A pilot with 27 participants, including banks, custodians, and central counterparties, used gold alongside gilts and Eurobonds as collateral over distributed-ledger technology. The pilot mobilizes gold by creating a tokenized digital twin; the World Gold Council framed the aim as overcoming restrictions on moving and storing physical metal.  
https://www.euroclear.com/newsandinsights/en/Format/Whitepapers-Reports/dlt-to-enhance-collateral-mobility.html

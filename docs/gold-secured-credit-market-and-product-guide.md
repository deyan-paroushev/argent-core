# Gold-secured credit: market and product guide

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**Status:** Public market and product rationale  
**Audience:** lenders, custodians, institutional gold holders, implementation partners, investors, and technical reviewers  
**Research basis:** public sources reviewed through 11 July 2026  
**Companion documents:** `argent-architecture.md`, `protocol.md`, `product-roadmap.md`

---

## Executive proposition

Gold-secured lending is not a new financial category. Private banks already extend Lombard facilities against pledged assets; bullion markets already support lending, borrowing, leases, forwards, and swaps; and banks in major gold centres already offer short-term liquidity against precious metals. The commercial question is therefore not whether gold can support credit. It can.

The unresolved problem is how to make **specific, independently custodied physical bars** usable as repeatable operational collateral without selling them, transferring normal-course title to the lender, or replacing them with a freely circulating token.

Argent addresses that operating problem:

> **Argent converts a physical bar list into a continuously calculated and controllable borrowing base without converting the bars themselves into transferable tokens.**

The gold remains with an approved custodian. The borrower remains the owner, subject to the agreed security interest and enforcement terms. The lender controls eligibility, valuation policy, borrowing capacity, release, margin response, and enforcement. Argent provides the shared state, role authorisation, ordered control workflow, and evidence needed to make those rights operational across institutions.

This document explains:

- why an owner may prefer to borrow against gold rather than sell it;
- why a lender may accept allocated gold as collateral;
- where gold-secured financing already exists;
- why allocated bars remain operationally difficult to mobilise;
- what Argent adds to the established legal, custody, and banking structure;
- which borrowers and facilities are suitable;
- what a credible design-partner pilot must prove.

> **Boundary:** Argent is not a lender, custodian, dealer, investment adviser, or legal-security instrument. It records and enforces the contractually authorised collateral-control state. The facility agreement, pledge or security agreement, custody acknowledgement, governing law, regulatory treatment, and physical enforcement route remain off-chain and must be established by the relevant regulated parties.

---

## 1. What “operational credit” means

“Operational credit” is Argent’s shorthand for a credit facility that turns an existing reserve asset into usable liquidity for a defined business purpose. It is not a separate legal category.

The reference product is:

> **A revolving working-capital or liquidity facility secured by allocated gold held with an independent custodian.**

Depending on the lender and borrower, the same collateral framework could support:

- a revolving cash facility;
- a fixed-term advance;
- a supplier-payment or inventory facility;
- a letter of credit or bank guarantee;
- a settlement-liquidity line;
- a temporary acquisition or bridge facility;
- another secured exposure whose utilisation can be measured against an approved borrowing base.

UBS describes the established Lombard model in similar terms: a borrower obtains liquidity without selling pledged assets, and the available credit line is based on the value and performance of those assets [1]. Argent applies that familiar credit logic to bar-identified physical collateral and adds a shared control and evidence layer across borrower, bank, and custodian.

The product is strongest when it matches a **temporary liquidity requirement** with a **credible repayment source**. The gold is the lender’s controlled secondary recovery source. It should not replace normal underwriting of the borrower’s ability to repay.

---

## 2. Why borrow against the gold instead of selling it

### 2.1 A temporary need should not require a permanent disposal

An institution may hold gold as:

- a long-term treasury reserve;
- a portfolio hedge or diversification asset;
- a family-office or holding-company reserve;
- bullion inventory;
- a strategic liquidity buffer;
- a raw material or working asset within the gold value chain.

Selling the gold resolves a temporary cash requirement through a permanent asset decision. Borrowing can be more proportionate when the liquidity need is short-lived and repayment is expected from operations, receivables, asset sales, refinancing, or another identifiable source.

The central economic distinction is:

```text
sale of gold      = permanent disposal of the reserve position
secured borrowing = temporary liquidity against the reserve position
```

The borrower keeps the economic exposure to the asset while the facility remains properly covered. It also avoids the operational and market friction of selling and later rebuilding the position. The value of those benefits is borrower- and jurisdiction-specific; they must be compared with interest, custody, valuation, legal, and facility costs rather than assumed.

### 2.2 The owner preserves strategic optionality

Gold may be held precisely because management does not want to depend entirely on operating cash, bank deposits, public markets, or one currency. A forced sale can weaken that reserve strategy at the moment liquidity is most valuable.

A secured facility creates an additional option:

1. preserve the strategic asset;
2. obtain liquidity against a conservative portion of its value;
3. repay from the intended cash-flow source;
4. release the collateral after the exposure is cleared.

This is the logic underlying securities-backed and Lombard facilities generally. It does not mean borrowing is always preferable. UBS explicitly warns that the borrowing cost, asset-return uncertainty, collateral volatility, and margin-call risk must be stress-tested [12].

### 2.3 The facility can support repeated liquidity cycles

A sale is a one-time conversion. A revolving secured facility can support repeated draw, repayment, and redraw cycles while the eligible collateral remains in place.

That matters for borrowers with recurring needs such as:

- seasonal inventory purchases;
- supplier settlement;
- short-dated trade cycles;
- working-capital peaks;
- temporary cash mismatches;
- collateralised guarantees or letters of credit.

The commercial value is not merely “cash from gold.” It is an approved liquidity capacity that can be reused without repeatedly disposing of and reacquiring the reserve asset.

### 2.4 The structure can separate ownership from control

Normal-course ownership and lender control are different concepts.

The borrower can remain the owner while agreeing that:

- identified bars are immobilised;
- the custodian will not release or substitute them without the required authority;
- the bank controls specified release and enforcement decisions;
- title transfers or sale occur only if the contractual enforcement conditions are satisfied.

This separation is central to Argent. The lender does not need the gold to circulate as a token or sit on the lender’s own balance sheet during the normal life of the facility. It needs legally effective security and reliable operational control.

---

## 3. Why allocated gold can be effective collateral

Gold is not risk-free and is not equivalent to cash. It nevertheless has several characteristics that can make it attractive collateral when the legal and operational structure is sound.

### 3.1 Observable, globally referenced valuation

Gold benefits from established international pricing conventions and benchmark infrastructure. LBMA prices are recognised global benchmarks for precious metals [2]. A lender can therefore define an approved valuation source, currency conversion method, staleness threshold, and calculation time without depending on an opaque appraisal process.

That does not eliminate basis, timing, FX, or liquidation risk. It makes those risks measurable and suitable for a documented haircut and margin policy.

### 3.2 Deep market liquidity

The World Gold Council describes the gold market as large, global, and highly liquid, estimating average trading volumes above USD 200 billion per day in 2024 [5]. Market depth matters because a collateral value is useful only if the asset can be realised within the lender’s assumed liquidation horizon.

The relevant facility question is not whether “gold is liquid” in the abstract. It is whether the lender has a defined route to sell or transfer the **particular bars**, in the relevant location, through approved counterparties, within the legal and operational timetable used to set the haircut.

### 3.3 No corporate issuer credit risk in the metal itself

A physical bar does not depend on a company’s promise to pay. Its value can still fall, and the custodian, ownership, fraud, title, sanctions, transport, insurance, and market-access risks remain. But the underlying metal does not default like a bond issuer or trade receivable.

This is one reason gold appears within recognised financial-collateral frameworks. Basel CRE22 includes gold among the forms of eligible financial collateral under the standardised credit-risk-mitigation approach and applies volatility adjustments under the comprehensive method [9]. BIS supervisory analysis likewise emphasises that recognition depends on legal certainty, effective protection, existence, availability, prudent valuation, and the applicable supervisory haircut [8].

### 3.4 Specific bars can be identified and verified

Allocated gold is attached to specific bars rather than only a general entitlement to metal. LBMA describes allocated accounts as accounts used when a customer requires ownership of specific bars; its glossary refers to physical segregation, a detailed bar list, and client title while the dealer acts as custodian [2][3].

A collateral record can therefore bind to evidence such as:

- refiner or manufacturer;
- serial number;
- format and weight;
- fineness;
- fine weight;
- custody account and location;
- allocation and segregation status;
- inspection, assay, or integrity evidence where required;
- insurance and custody acknowledgements;
- current encumbrance state.

That identity layer is stronger than a generic description such as “gold worth USD X.” It supports bar-level eligibility, exclusion, substitution, release, and enforcement.

### 3.5 Prudential recognition is possible, but not automatic

The existence of Basel recognition does not mean every gold-secured loan receives the same capital or liquidity treatment. Recognition depends on the applicable jurisdiction, exposure type, method, documentation, enforceability, valuation, segregation, and supervisory rules.

The distinction between **credit-risk mitigation** and **liquidity-buffer eligibility** is especially important. The EBA has noted that physically held gold was not considered eligible as high-quality liquid assets for EU LCR purposes and was not eligible for standard EU central-bank operations in the analysis it reviewed [10]. Argent should therefore avoid suggesting that gold is cash-equivalent, universally capital-efficient, or automatically eligible for central-bank mobilisation.

The correct claim is narrower:

> Gold can be recognised and managed as valuable financial collateral under defined conditions; each lender must determine the actual prudential, accounting, and liquidity treatment of its facility.

---

## 4. Is this already a real market?

Yes. Several established markets demonstrate different parts of the proposition.

### 4.1 Lombard and securities-backed lending

Lombard lending is the established model of obtaining liquidity against pledged marketable assets while retaining the underlying position. The lender determines an advance against eligible collateral and revises lending value as market value and risk change [1].

Gold is not always included in a standard securities portfolio facility, but the product logic is established: pledged assets support a flexible or fixed advance, subject to collateral value, haircut, margin, and liquidation rights.

### 4.2 Bullion lending, borrowing, leases, forwards, and swaps

The wholesale precious-metals market already treats metal as a financing asset. The LBMA OTC guide documents lending and borrowing of metal and the use of spot-forward swaps [6]. These transactions are not identical to Argent’s borrower-custodian-bank facility, but they establish that gold participates in sophisticated funding and liquidity markets rather than functioning only as a passive store of value.

### 4.3 Bank-provided liquidity against precious metals

In May 2025, Emirates NBD announced a physical bullion transaction service for corporate, institutional, retail, and wealth clients. The bank stated that the service would include market-based lease rates and **short-term liquidity solutions with precious metals as collateral**, and that vault-backed gold certificates could be pledged as collateral [7].

This is direct market evidence that a major bank in a leading gold hub sees a commercial connection between professionally held bullion and client liquidity.

### 4.4 Collateral-mobility initiatives

The market is also investing in technology to make gold easier to mobilise. Euroclear, Digital Asset, and the World Gold Council completed a 2024 pilot using digital twins of gold and other assets for real-time collateral transactions, with the stated goals of greater collateral mobility, liquidity, and transactional efficiency [11].

That pilot validates the problem, but it does not require Argent to follow the same asset-representation model. Argent takes a different route:

- the physical bars remain allocated with the custodian;
- the protocol records control rights and lifecycle evidence;
- no freely transferable ownership token is required;
- the lender receives a governed collateral position rather than a speculative instrument.

---

## 5. Why allocated bars are still difficult to use

The strongest evidence for Argent’s opportunity is not that allocated gold is unknown to lenders. It is that allocated ownership creates operational friction.

The 2025 World Gold Council and Linklaters whitepaper describes allocated gold as specific bars with direct ownership and protection from custodian credit exposure, but also identifies increased operational complexity, whole-bar limitations, and difficulty using allocated gold as collateral through customary security and custody mechanisms [4].

Those constraints translate into concrete operating problems.

### 5.1 Whole-bar indivisibility

A borrower cannot release 17% of a specific bar. If the collateral pool contains discrete bars, repayment and release must be translated into a safe selection of whole bars.

The system must answer:

- which bars remain pledged;
- which bars are eligible for release;
- whether releasing one bar would breach coverage;
- whether another bar must be substituted first;
- how residual excess collateral is handled;
- whether mixed formats, refiners, locations, or legal accounts create concentration limits.

This is not a cosmetic detail. It is one of the main reasons a bar-level control engine is needed.

### 5.2 Fragmented books and evidence

The relevant facts usually sit in different systems:

- the custodian knows physical existence and allocation;
- the bank knows facility approval and exposure;
- a price provider supplies valuation inputs;
- legal documents define the security interest and enforcement rights;
- the payment or core-banking system records utilisation and repayment;
- dealers or liquidation agents control the realisation route.

A spreadsheet or periodic certificate may summarise these facts, but it does not by itself create a continuously reconciled shared state.

### 5.3 Release control

The risk is not only that collateral is missing at origination. It is that collateral is released, substituted, moved, or re-used while an exposure still depends on it.

A credible facility needs explicit controls for:

- initial immobilisation;
- exclusive pledge;
- no unauthorised release;
- no release while exposure remains;
- bank authorisation before release;
- custodian confirmation after release;
- gap-free substitution;
- default freeze and enforcement instructions.

### 5.4 Revaluation and margin operations

Gold prices and FX rates move. The lender must therefore distinguish:

- approved credit limit;
- current borrowing base;
- drawn exposure;
- available capacity;
- maintenance threshold;
- margin deficit;
- cure deadline;
- enforcement threshold.

A static pledge record cannot answer those questions. The collateral position must be revalued against current, policy-approved inputs.

### 5.5 Legal effectiveness and operational evidence are separate

A digital record can prove that authorised parties followed an agreed workflow. It cannot, by itself, create a legally enforceable pledge over physical bars.

For recognised credit-risk mitigation, supervisors care about legal certainty, effectiveness, existence, availability, lack of prior encumbrance, prudent valuation, and timely liquidation [8]. The legal-security package and the operational-control system therefore need to reinforce each other:

```text
legal documents define the rights
custody acknowledgement makes them actionable
Argent records and governs the authorised control state
```

---

## 6. The reference credit product

The product should be defined as a secured facility, not as an exchange rate between kilograms of gold and dollars of credit.

### 6.1 Core terms

A design-partner facility should define at least:

- approved borrower and obligor group;
- approved lender and facility type;
- approved custodian and custody account;
- eligible bar standards and locations;
- valuation source and FX source;
- maximum price age;
- advance rate or haircut;
- maintenance and liquidation thresholds;
- approved credit limit;
- interest, fees, and tenor;
- permitted use of proceeds;
- primary repayment source;
- substitution and release rules;
- cure periods;
- default and enforcement events;
- liquidation counterparties and waterfall.

### 6.2 Borrowing-base formula

A simplified facility calculation is:

```text
eligible collateral value
  = Σ (eligible fine weight × approved gold price × approved FX rate)

borrowing base
  = eligible collateral value × approved advance rate
    − policy reserves
    − concentration adjustments
    − pending or disputed amounts

current credit capacity
  = min(approved facility limit, borrowing base)

available capacity
  = current credit capacity
    − drawn exposure
    − authorised but unsettled utilisation
```

The bank supplies and approves the policy. Argent enforces the approved result and records the inputs and policy version used for each decision.

### 6.3 Primary and secondary repayment sources

The facility should have an identified primary repayment source, for example:

- operating cash flow;
- collection of financed receivables;
- sale of financed inventory;
- maturity of another asset;
- committed refinancing;
- a scheduled treasury inflow.

The collateral is the secondary source of repayment. A borrower that can repay only by permanently selling the gold is not using temporary liquidity; it is delaying an asset disposal while adding financing cost.

### 6.4 Interest and utilisation

Argent should distinguish clearly between:

- approved limit;
- current borrowing base;
- amount drawn;
- accrued interest and fees;
- settlement amount;
- available capacity.

The initial reference implementation may leave interest calculation to the bank’s core system, but the control layer must be able to bind the bank-certified exposure used for release, margin, and enforcement decisions.

---

## 7. The full lifecycle

### 7.1 Onboarding and eligibility

1. The bank approves the borrower and facility.
2. The custodian and custody account are approved.
3. The security documents and custodian acknowledgement are executed.
4. The bar list and evidence package are validated.
5. Each bar is checked against eligibility and concentration policy.
6. The initial valuation and borrowing base are recorded.

### 7.2 Pledge and immobilisation

1. The borrower requests the pledge.
2. The custodian confirms existence, allocation, and immobilisation.
3. The bank confirms eligibility and pledge activation.
4. The position becomes available to support the facility.

The ordering matters. A bank approval without custody immobilisation is incomplete; a custody flag without a valid security arrangement is also incomplete.

### 7.3 Draw and ongoing monitoring

1. The bank approves or records utilisation.
2. Argent verifies that the draw does not exceed the lower of the facility limit and borrowing base.
3. Exposure, collateral value, and available capacity are updated.
4. Revaluation runs on the bank’s approved schedule or after material events.
5. Stale or invalid valuation inputs are refused.

### 7.4 Margin response

If coverage weakens:

1. a margin state is opened;
2. the deficit and cure deadline are recorded;
3. the borrower may repay, add collateral, or substitute eligible bars;
4. the bank records cure or escalation;
5. release remains blocked during the unresolved margin state.

The system should not pretend that a margin call can be resolved automatically without legal and operational authority. It should make the state, deadline, required action, and evidence unambiguous.

### 7.5 Repayment and release

Repayment must not automatically release physical collateral.

A safe sequence is:

```text
repayment settles
→ exposure is reduced and reconciled
→ bank authorises a defined release
→ custodian confirms the physical/custody-book release
→ evidence package is closed
```

For partial repayment, the whole-bar selection rules determine whether any bar can be released without breaching coverage.

### 7.6 Substitution

Substitution must be gap-free:

```text
new collateral proposed
→ new collateral validated and valued
→ custodian immobilises new collateral
→ bank approves replacement
→ old collateral becomes releasable
→ custodian confirms completion
```

The old collateral must not be released before the replacement collateral is secured and coverage is confirmed.

### 7.7 Default and liquidation

A credible facility must define the exit before origination:

```text
default or enforcement event
→ custodian freeze confirmed
→ current valuation obtained
→ bank issues enforcement instruction
→ approved dealer or liquidation agent quotes
→ bars are transferred or sold under the legal documents
→ cash settles
→ costs, interest, and principal are applied
→ any surplus is returned as required
→ evidence package records the outcome
```

Banks underwrite this route as seriously as the initial advance. Argent can sequence and evidence it, but it cannot replace the dealer, custodian, court, insolvency process, or governing law where those are required.

---

## 8. What Argent adds

### 8.1 A shared collateral book of record

Argent creates a decision-ready position that connects:

- exact collateral identity;
- custody and immobilisation state;
- legal-document references;
- valuation inputs;
- policy version;
- borrowing base;
- exposure and available capacity;
- margin status;
- pending release or substitution;
- default and enforcement state;
- authorisations and transaction evidence.

No participant is asked to replace its own system of record. Argent provides the shared control state between them.

### 8.2 Multi-party authority

The borrower, bank, custodian, verifier, and other actors do not have interchangeable permissions.

Argent makes the authority boundary explicit:

- the custodian attests existence and performs custody actions;
- the bank approves credit policy, utilisation, release, margin, default, and enforcement;
- the borrower requests actions and repays;
- approved verifiers or service providers supply defined evidence;
- no single party can silently rewrite the full lifecycle.

### 8.3 Continuous borrowing-base control

The bar list is not stored merely as documentary evidence. It becomes the input to a current collateral position.

Argent can answer:

- which bars are eligible now;
- which bars are pledged;
- what current value the approved policy assigns;
- what capacity is available;
- whether a draw is permitted;
- whether a margin deficit exists;
- which bars may be released;
- whether a substitution preserves continuous coverage.

### 8.4 Evidence rather than asset tokenisation

Argent does not need to create a transferable token that represents ownership of the gold.

Instead, it records:

- the identity of the real asset;
- the authority of each actor;
- the state transitions affecting control;
- the evidence supporting each transition;
- the current relationship between collateral and exposure.

This keeps the product close to established secured-credit and custody structures. It also avoids implying that a token alone resolves title, custody, redemption, or enforcement.

### 8.5 Integration rather than institutional replacement

A production deployment should connect to existing systems through narrow adapters:

- custody/bar-list adapter;
- valuation adapter;
- core-banking or loan-servicing adapter;
- payment and settlement adapter;
- institutional signing and approval adapter;
- document and evidence repository;
- liquidation/dealer workflow.

The bank continues to underwrite and book the loan. The custodian continues to hold and operate the metal. Argent coordinates the control lifecycle between them.

---

## 9. Why not simply tokenise the gold?

Tokenisation can improve transferability, fractionalisation, and collateral mobility. The Euroclear/Digital Asset/World Gold Council pilot demonstrates meaningful institutional interest in that path [11], and the World Gold Council’s 2025 wholesale digital gold proposal is designed to bridge the operational gap between allocated and unallocated gold [4].

Argent addresses a different requirement.

| Question | Transferable token model | Argent control-layer model |
|---|---|---|
| What becomes digital? | an ownership, beneficial-interest, or redemption representation | the collateral-control lifecycle and evidence |
| Does the asset circulate? | potentially, through token transfer | no; the bars remain in the approved custody structure |
| Is fractionalisation central? | often yes | no; whole-bar constraints are managed explicitly |
| Primary legal question | rights represented by the token and redemption structure | enforceability of the pledge/security and custody control |
| Main operational objective | mobility and transfer | controlled eligibility, borrowing base, release, and enforcement |
| Natural buyer | market infrastructure, issuer, asset platform | lender, underwriter, custodian, credit insurer |

The approaches can coexist. Argent’s position is not that tokenisation is inherently wrong. It is that a bank should not need to redesign ownership or issue a transferable instrument merely to obtain reliable control over collateral that already exists in professional custody.

---

## 10. When the facility makes sense

The strongest borrower profile has most of the following characteristics:

- already owns professionally custodied allocated gold;
- holds multiple eligible bars rather than one indivisible bar;
- wants to retain the gold for strategic, treasury, or inventory reasons;
- has a recurring or clearly temporary liquidity need;
- has an identifiable primary repayment source;
- accepts conservative advance rates and daily or policy-defined valuation;
- can meet margin calls or partial repayment requirements;
- operates at a facility size that justifies bank, legal, and custody onboarding;
- can provide clear ownership, source-of-funds, responsible-sourcing, sanctions, and AML evidence;
- accepts a defined enforcement and liquidation route.

Potential first design-partner segments include:

- bullion dealers and distributors;
- refiners and jewellery manufacturers with segregated reserve holdings;
- family offices and holding companies with allocated kilobar reserves;
- commodity businesses holding gold as treasury collateral;
- institutional investors with an allocated reserve tranche and short-dated liquidity needs.

The first customer should not be chosen merely because it owns gold. The borrower must have a repeatable credit use case and a repayment cycle that can be underwritten.

---

## 11. When it does not make sense

The facility is a poor fit when:

- the borrower has no credible repayment source other than selling the collateral;
- proceeds fund persistent operating losses;
- ownership or lien priority is unclear;
- the custodian cannot provide an enforceable acknowledgement or release control;
- bars cannot be reliably identified, valued, insured, or liquidated;
- sanctions, AML, responsible-sourcing, or provenance questions remain unresolved;
- the facility is too small to cover legal, custody, and operational costs;
- the borrower cannot tolerate margin calls;
- the desired advance rate leaves insufficient protection for price, FX, time, legal, and liquidation risk;
- the lender expects the blockchain record to replace legal perfection or physical enforcement.

A conservative refusal is better than converting weak collateral into a digitally polished weak facility.

---

## 12. Bank economics and the buyer case

The bank buys Argent only if the control layer improves the economics or risk of the facility.

A design-partner business case should quantify:

### Revenue

- interest margin;
- commitment or availability fee;
- arrangement fee;
- custody and administration fees;
- guarantee or letter-of-credit fees;
- potential cross-sell value.

### Costs

- funding cost;
- capital and liquidity cost;
- underwriting and legal review;
- borrower and collateral onboarding;
- custody integration;
- valuation and monitoring;
- margin and exception handling;
- liquidation readiness;
- audit and reporting.

### Risk

- probability of default;
- collateral volatility;
- haircut adequacy;
- time to enforce;
- liquidation discount and costs;
- operational and fraud risk;
- legal and jurisdictional risk;
- custodian and dealer dependencies;
- concentration by borrower, custodian, location, refiner, or bar type.

Argent’s value proposition is strongest if it can demonstrate one or more of the following:

- lower cost to onboard and operate each facility;
- faster and safer collateral verification;
- reduced release and double-pledge risk;
- earlier identification of margin deficits;
- clearer evidence for credit, risk, audit, and regulators;
- faster substitution and whole-bar release decisions;
- a repeatable operating model across borrowers and custodians;
- better recovery evidence and shorter operational enforcement time.

The product is infrastructure, not balance sheet. Argent should charge for implementation, integration, active facilities, evidence, and operational reliability rather than for the protocol merely existing.

---

## 13. The commercial structure Argent still needs

A software-only demonstrator proves that the state machine works. It does not prove that the credit product can be originated and enforced.

A credible pilot requires a complete operating triangle:

### 13.1 Borrower

Provides eligible bars, ownership and provenance evidence, a real liquidity requirement, and a credible repayment source.

### 13.2 Lender

Defines underwriting, facility terms, advance rate, margin policy, permitted utilisation, release authority, default rights, and regulatory treatment.

### 13.3 Custodian

Confirms the bar list and allocation, acknowledges the control arrangement, immobilises the pledged bars, refuses unauthorised release, executes approved substitution, and follows valid enforcement instructions.

The triangle should be supported by:

- an approved valuation provider;
- legal counsel in the relevant jurisdictions;
- an approved bullion dealer or liquidation agent;
- insurance arrangements;
- institutional signing and approval infrastructure;
- an agreed settlement route.

> **A pilot involving only the software proves the engine. A pilot involving the borrower, lender, and custodian proves the business.**

---

## 14. Minimum pilot definition

A first design-partner pilot should be deliberately narrow.

### 14.1 Suggested scope

- one borrower;
- one lender;
- one custodian;
- one jurisdiction and governing-law structure;
- one allocated custody account;
- one homogeneous bar format where possible;
- one approved valuation source;
- one revolving or fixed-advance facility;
- one settlement asset and bank-account route;
- one approved liquidation counterparty.

### 14.2 Required scenarios

The pilot should demonstrate:

1. onboarding and bar-list validation;
2. custody attestation and immobilisation;
3. exclusive pledge activation;
4. initial valuation and borrowing-base calculation;
5. permitted draw;
6. refused overdraw;
7. scheduled revaluation;
8. margin call and cure;
9. partial repayment;
10. whole-bar release decision;
11. gap-free collateral substitution;
12. full repayment;
13. bank-authorised and custodian-confirmed release;
14. default freeze and enforcement dry run;
15. evidence reconstruction from approval through final state.

### 14.3 Success criteria

The pilot is successful only if the parties can show that:

- every pledged bar is uniquely identifiable and reconciled to custody records;
- no bar can support two active pledges;
- no draw can exceed the approved capacity;
- stale valuation cannot authorise a draw or release;
- a margin deficit is detected and acted on within policy;
- replacement collateral is secured before old collateral is released;
- repayment does not trigger automatic physical release;
- release requires bank authority and custodian confirmation;
- the legal documents and digital state refer to the same facility and collateral;
- the lender can reconstruct the full evidence chain without relying on one participant’s private spreadsheet;
- the default and liquidation route is operationally credible, even if not executed with real loss.

---

## 15. Strategic conclusion

The opportunity is not to prove that gold can secure credit. The wholesale market, Lombard lending, bank bullion services, and prudential frameworks already establish that proposition.

The opportunity is to make a difficult version of that transaction repeatable:

- the collateral consists of specific physical bars;
- the bars remain allocated and independently custodied;
- the owner does not have to sell or tokenize them;
- the lender receives enforceable security under conventional legal documents;
- the custodian receives unambiguous, role-authorised instructions;
- the borrowing base remains current;
- whole-bar release and substitution remain safe;
- margin, default, and enforcement states are explicit;
- every material control action produces evidence.

Argent’s category is therefore best described as:

> **Institutional collateral-control infrastructure for credit secured by physical assets that remain in custody.**

Gold is the first reference asset because its market, identity, custody, and valuation structures make the problem visible and testable. The durable product is the control layer that can later support other custody-stable physical collateral through asset-specific adapters.

---

## References

[1] UBS Switzerland AG, **“Lombard loans – your bridge to financial flexibility.”** https://www.ubs.com/content/dam/assets/microsites/credit-suisse/static/doc/lombardkredit-factsheet-en.pdf

[2] London Bullion Market Association, **“About Loco London.”** https://www.lbma.org.uk/market-standards/about-loco-london

[3] London Bullion Market Association, **“Glossary: Unallocated and Allocated Accounts.”** https://www.lbma.org.uk/resources/glossary

[4] World Gold Council and Linklaters, **“Pooled Gold Interests and Wholesale Digital Gold – A New Vision for the Gold Market,”** 2 September 2025. https://www.gold.org/download/file/19999/White_Paper_Pooled_Gold_Interests_and_Wholesale_Digital_Gold_A_New_Vision_for_the_Gold_Market-FINAL.pdf

[5] World Gold Council, **“Gold in the Global Economy: Market, Mining and Modernisation,”** 2025. https://www.gold.org/download/file/20299/Gold_in_the_Global_Economy_Market_mining_and_modernisation.pdf

[6] London Bullion Market Association, **“A Guide to the Loco London Precious Metals Market: Lending and Borrowing Metal.”** https://www.lbma.org.uk/publications/the-otc-guide/lending-and-borrowing-metal

[7] Emirates NBD, **“Emirates NBD Pioneers Physical Cross-Border Gold and Silver Bullion Transaction Service,”** 19 May 2025. https://www.emiratesnbd.com/en/media-center/emirates-nbd-pioneers-physical-cross-border-gold

[8] Bank for International Settlements, Financial Stability Institute, **“Challenges in supervising banks’ large exposures,”** FSI Insights No. 52, 2023. https://www.bis.org/fsi/publ/insights52.pdf

[9] Basel Committee on Banking Supervision, **“CRE22 – Standardised approach: credit risk mitigation.”** https://www.bis.org/basel_framework/chapter/CRE/22.htm

[10] European Banking Authority, **“Report on the impact of the NSFR on the functioning of the precious metals market,”** 2021. https://www.eba.europa.eu/sites/default/files/document_library/Publications/Reports/2021/1024251/EBA%20Report%20on%20impact%20of%20NSFR%20on%20fuctioning%20of%20precious%20metals%20markets.pdf

[11] Euroclear, **“Using DLT to enhance collateral mobility: successful pilot of tokenising gold, Gilts and Eurobonds,”** 2 October 2024. https://www.euroclear.com/newsandinsights/en/Format/Whitepapers-Reports/dlt-to-enhance-collateral-mobility.html

[12] UBS Global Wealth Management, **“Why borrow if you are already wealthy: borrowing benefits and considerations.”** https://www.ubs.com/global/en/wealthmanagement/insights/chief-investment-office/life-goals/liquidity-longevity-legacy/2021/borrow-benefits-and-considerations.html

---

*This document describes a product and operating model. It is not legal, regulatory, tax, accounting, credit, or investment advice. Facility terms and collateral treatment must be determined by the relevant lender, custodian, advisers, and authorities in each jurisdiction.*

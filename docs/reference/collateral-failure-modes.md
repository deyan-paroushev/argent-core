# Collateral Failure Modes: the Documented Loss Record of Physical-Collateral Finance

> **Positioning status:** These failure modes apply to any bank obligation secured by physical collateral, not only a loan. Duplicate allocation, false evidence, unauthorized release, stale valuation, and enforcement-time discovery can undermine guarantees, documentary credits, treasury exposure, and accepted obligations as readily as funded credit.

**Why physical-collateral finance keeps failing, what the documented losses have in common, and which control invariants in the Argent core address each failure mode.**

**Status:** market evidence and design rationale for the shipped core
**Companion documents:** `collateral-control.md`, `threat-model-and-security-boundaries.md`, `collateral-eligibility-and-risk-policy.md`, `auto-collateralisation-layer.md`
**Last updated:** 2026-07-09

*This is a risk-thesis and product-design document, not legal, banking, audit, custody, fraud-prevention, or investment advice. It summarizes publicly reported and, where noted, court-adjudicated cases; descriptions rely on the cited public record, and where a matter was alleged but not adjudicated the text says so. Argent does not verify physical truth, replace a custodian, perfect a security interest, inspect goods, underwrite credit, or guarantee enforcement. It records signed control state, evidence commitments, role authority, and lifecycle transitions for physical collateral that remains in professional custody. Section 7 states plainly what a control record does not catch.*

---

## 1. Why this document exists

The other documents in this repository argue capability: what the Argent contracts do, what the architecture guarantees, what the product becomes. This document argues necessity, and it argues it in the language a bank actually uses. A credit, collateral, or operations team does not first ask whether a system can tokenize an asset. It asks which known failure modes in secured commodity finance the system makes harder, earlier to detect, or operationally less likely. That is a risk-control conversation, not a technology one, and it is the conversation this document is built for.

The argument is not theoretical. Over the past decade, physical-collateral and commodity-trade finance has produced a loss record measured in billions of dollars per episode, documented in court judgments, insolvency reports, and exchange notices. The same small set of failure modes recurs in every case. Banks responded not by fixing the control problem but by leaving the market, which is the strongest evidence available that the control problem is real, unsolved, and expensive.

Every invariant in the Argent core corresponds to one of these documented failure modes. This document makes that correspondence explicit, and it treats each market loss as a failed control invariant rather than as a fraud story, so that a credit officer, risk reviewer, or auditor reading the contracts can see not just what each rule does, but which loss it answers and, just as importantly, which loss it does not.

---

## 2. The nine failure modes

Across the case record, the failures reduce to nine recurring patterns. Each is stated as a failed invariant, not an anecdote.

**F1. Duplicate financing.** The same physical asset is pledged as collateral to more than one lender, each believing it holds an exclusive claim. *Invariant: one physical lot must not support two active pledges.*

**F2. Phantom or recycled documents.** The collateral never existed, or the documents of title were forged or reused: warehouse receipts, bills of lading, storage certificates. *Invariant: collateral identity must be attested by the responsible custody party, not self-asserted.*

**F3. False physical content.** Physical material exists but is not what the record says: gilded copper as gold, stones as nickel briquettes, tungsten cores in bars. *Invariant: document state is not physical truth, and quality and quantity must be independently attested.*

**F4. Unauthorized disposal.** The pledged asset is sold, shipped, or removed while encumbered, discovered only at default. *Invariant: collateral must not leave control before lender consent and coverage checks.*

**F5. Borrower-controlled verification.** Access to the collateral and its testing are controlled by the party with the incentive to misrepresent it. *Invariant: the party that establishes the record must be independent of the party being financed.*

**F6. Stale and unverifiable state.** The lender's record and the physical state diverge silently, sometimes for years, because nothing forces reconciliation before enforcement. *Invariant: valuation and attestation must be fresh, on a cadence, or capacity is withheld.*

**F7. Fragmented lender visibility.** Each lender sees only its own private slice; no shared state exists in which a duplicate pledge would be visible. *Invariant: parties need one shared state of who signed what, when.*

**F8. Enforcement-time discovery.** The defect surfaces only when the borrower defaults and the lender moves to enforce, by which time the asset may be missing, non-conforming, over-claimed, or legally harder to reach than the file implied. *Invariant: default and enforcement must be a continuously built evidence trail, not improvised later.*

**F9. Collusion and stale authority.** A person, key, or agent keeps acting after its authority should have lapsed, or one party acts under another's operational authority. *Invariant: signing power must be role-bound, revocable, and auditable.*

No case in the record exhibits only one mode. Duplicate financing needs fragmented visibility to survive. False physical content survives on borrower-controlled verification and stale state. Unauthorized disposal is possible because release is not tied to a shared control state. Enforcement-time discovery is the final symptom, not the root cause. The modes compound, which is why the losses are large.

---

## 3. The case record

### 3.1 Qingdao, 2014: duplicate financing at port scale (F1, F2, F7)

According to public statements by local police and an intermediate court in Qingdao, the trading company Decheng Mining duplicated warehouse certificates for copper and aluminium stored at the port and pledged the same stock as collateral for loans from multiple lenders. The estimated result was around US$3 billion in losses across Chinese banks and foreign institutions including Citigroup and Standard Chartered, and trading houses including Mercuria [1]. The same reporting records the enabling practice from the preceding credit boom: traders were told they could move an encumbered batch of steel to a different warehouse and pledge the same batch against another loan from another lender [1]. Reuters' subsequent "ghost collateral" investigation documented the pattern's persistence, including a 291-tonne pile of steel pledged to China CITIC Bank that was no longer in the warehouse when the bank moved to take possession [2].

The lesson for Argent is not that warehouses are unsafe. It is that a lender relying on documents without a shared control state can be exposed to the same goods supporting multiple claims. Lot identity, uniqueness, custodian immobilisation, and role-signed release are first-order controls, not implementation details.

### 3.2 Shaanxi, 2016: adulterated bars (F3)

Bars pledged as gold collateral to 19 lenders in Shaanxi province were found to be adulterated, with cores of tungsten [3]. A regional precedent for the larger event four years later.

### 3.3 Access World Asia, 2017: forged receipts against real warehouses (F2)

French and Australian banks took loan losses totalling over US$300 million after discovering fake documents for nickel purportedly stored in Asian warehouses operated by Access World [4]. The warehouses were real and reputable; the receipts were not. Bloomberg has also reported that Sberbank discovered in 2018 that containers of nickel in Rotterdam it had financed on behalf of Liberty Commodities had already been emptied [4].

### 3.4 Hin Leong, 2020: the collapse that emptied the market (F1, F2, F4, F8)

Hin Leong Trading, one of Asia's largest fuel traders, collapsed owing US$3.85 billion to 23 lenders, with HSBC's US$600 million the largest exposure [5]. Its founder had disclosed that the company had hidden roughly US$800 million in losses and that oil pledged as collateral for lending had also been sold [6], and insolvency reporting documented the same cargo, on the same vessel, supporting financing from multiple lenders through bills of lading that differed only in date [7]. The outcome is adjudicated: founder Lim Oon Kuin was convicted of cheating HSBC and abetting forgery over financing for oil trades that did not occur, sentenced in November 2024 (reduced to 13.5 years on appeal in 2026), and, with his children, consented to a US$3.5 billion civil judgment followed by bankruptcy [8], [9].

Hin Leong combined four failure modes in one balance sheet: phantom trades, duplicate financing of the same cargo, disposal of pledged collateral, and discovery only at enforcement. The design implication is that document commitments alone are insufficient; the lifecycle must separate facility, custody, valuation, payment, default, and enforcement evidence so that no single forged document can carry the whole claim.

### 3.5 The Singapore cluster and the bank exodus, 2020

Hin Leong did not fall alone. Agritrade International, ZenRock Commodities, Hontop Energy, and Phoenix Commodities failed in the same period amid fraud allegations, with financing banks' losses in some cases well over US$100 million each [10]. Agritrade's former CFO was sentenced to 20 years for fraud [8]. The structural consequence is Section 4.

### 3.6 Kingold, 2020: borrower-controlled verification (F3, F5, F8)

According to reporting by Caixin, Wuhan Kingold Jewelry pledged 83 tonnes of purportedly pure gold to more than a dozen lenders, securing around 20 billion yuan (US$2.83 billion) in loans covered by 30 billion yuan of insurance; testing commissioned by creditors after a default found bars of gilded copper alloy [11], [12]. The company denied wrongdoing [12]. Two operational details matter more than the headline. First, per a lender employee cited in the reporting, access to the pledged gold and the testing procedures were controlled by Kingold itself [11]. Second, discovery came only through default: a missed interest payment led one trust to test its collateral, and a second obtained a court order to test before its debts came due [11], [13]. The collateral had sat in creditors' coffers, wrong, for years, because nothing forced independent verification before the moment it was needed. The Shanghai Gold Exchange revoked the affiliate's membership [13].

### 3.7 Trafigura and the nickel that wasn't, 2022-2026: adjudicated false content (F2, F3)

Trafigura took a US$577 million reserve in February 2023 after discovering that containerized cargoes bought as LME-grade nickel contained stainless steel, iron briquettes, or rubble, supported by false descriptions in bills of lading and insurance certificates [14], [15]. Around 100 cargoes that should have been worth just over US$500 million realized less than US$10 million [16]. In January 2026 the London High Court ruled that Trafigura was the victim of "fraud on a grand scale" devised and implemented by Prateek Gupta through the corporate defendants, awarding US$500 million in relief, increased to US$700 million in damages in February 2026 [15], [17]. The fraud ran through one of the world's most sophisticated trading houses, financed by a global bank, and unravelled only when investigators physically opened a container in Rotterdam [16]. The design implication is the sharpest boundary in this document: never claim that on-chain state verifies physical truth. The correct claim is that Argent records who attested what, when, under which role, against which evidence.

### 3.8 The LME bags of stones, 2023: the gold standard fails a weighing (F3, F6)

In March 2023 the London Metal Exchange invalidated nine warrants, representing 54 tonnes of nickel at an Access World warehouse in Rotterdam, after bags supposed to contain nickel briquettes were found to contain stones; Bloomberg reported the warrant owner was JPMorgan [18], [19]. The material had been registered as live LME stock since early 2022 [18]. The episode is instructive precisely because LME warrants were, in the words of one industry veteran, treated as a near-cash equivalent, the most trusted document of title in the metals world [18]. Whether the cause was error, theft, or fraud was not established at disclosure [19]; what is established is that the discrepancy was discoverable by weighing at any time and was discovered by no one for over a year, until metal was shipped out. The LME ordered inspections of warranted nickel across its network in response [19].

### 3.9 Trafigura Mongolia, 2024: what better controls surface (F6)

In October 2024 Trafigura reported that individuals in its Mongolian petroleum supply business had, over roughly five years, concealed overdue debts and manipulated data, resulting in a US$1.1 billion writedown [15]. The detail relevant here is how it surfaced: Trafigura stated it found the problem through the compliance overhaul and tighter risk controls implemented after the nickel fraud [15]. Control regimes do not merely prevent; they detect, and detection pulled forward by years is the difference between a writedown and a collapse.

---

## 4. What the market did about it

The banking system's response to this loss record was not better verification infrastructure. It was exit, and the exit is quantified.

In the fallout from Hin Leong, ABN AMRO announced a complete withdrawal from trade and commodity finance and shed 800 jobs, having sustained heavy losses including the Agritrade fraud [21], [25]. Deutsche Bank's own trade-finance analysis records that Société Générale and BNP Paribas scaling back or consolidating commodity finance removed more than US$20 billion of available liquidity from the market, and that the Asian Development Bank finds SMEs account for 40 percent of rejected trade-finance requests [25]. Bank commodity trade finance revenues globally fell 29 percent to US$1.7 billion in the first half of 2020 [26]. BNP Paribas suspended new commodity trade finance deals and closed its Geneva commodity operations, Société Générale exited commodity finance in Singapore, and Rabobank closed trade and commodity operations in London, Shanghai, and Sydney [21], [22]. Credit concentrated around the largest trading houses at the expense of mid-tier traders, in an explicit flight to quality [23].

Three consequences follow, and together they define Argent's market.

First, the collateral did not stop being valuable; the banks stopped being able to verify control of it at acceptable cost. As one commodity-finance fund CIO put it, when banks moved away from transactional trade finance they lost control over the goods they fund, which is what made them vulnerable to multiple financing of the same trade [26]. The exit priced the verification problem, not the assets.

Second, the mid-tier borrowers who lost bank access did not stop needing secured credit. That demand moved to private credit and to whoever could underwrite it, mostly without any improvement in the underlying control infrastructure [24], [26].

Third, the institutions that remained now carry concentrated exposure to the same failure modes, mitigated by procedure and relationship rather than by shared, verifiable state. The Trafigura nickel case demonstrated that scale and sophistication are not the mitigant they were assumed to be [16].

A control layer that makes exclusive pledge, custody confirmation, independent attestation, and release discipline verifiable at low cost is not a convenience for this market. It is the missing precondition for banks to come back to it, and it should be sold as risk-control infrastructure that helps credit stay available, not as a tokenization layer.

---

## 5. The structural diagnosis

Strip the cases to their mechanics and the shared root cause is visible: physical-collateral finance runs on private, per-lender, paper-derived records of a shared physical reality, verified mainly at origination and at default, by processes the borrower can often see, influence, or control.

Four properties of that arrangement produce the nine failure modes:

1. **Documents of title are bearer-style artifacts.** A warehouse receipt or bill of lading asserts a state of the world but carries no link to a live, shared record of that state. It can be duplicated (Qingdao), forged against real warehouses (Access World 2017), or issued against trades that never occurred (Hin Leong).

2. **Each lender's record is private.** As industry commentary after the Trafigura case put it, banks rarely discuss their deals with each other, creating space to double-finance cargo [23]. Duplicate financing is not a clever fraud; it is the default outcome of per-lender record silos against a single physical asset.

3. **Verification is event-driven, not continuous.** Collateral is checked when pledged and when enforced. Between those events, the record and the vault can diverge for years (Kingold, LME stones) because nothing forces reconciliation on a cadence.

4. **The verifying party is often not independent.** When the borrower controls access and testing (Kingold), or when the only weight check is performed by the same operator whose records are in question (LME stones), verification exists on paper and not in fact.

None of these is fixed by more diligence within the existing arrangement, because each is a property of the arrangement itself. They are fixed by changing what a pledge is: from a private paper claim into a signed transition in a shared, role-separated, continuously reconciled control record.

---

## 6. Failure modes, invariants, and controls

This is the correspondence between the loss record and the shipped Argent core, with the forward-looking additions of the auto-collateralisation layer noted where relevant. The fourth column is as important as the third: it states, per mode, what Argent does not solve. Contract functions and types named are in the open-source contracts; design principles are specified in `collateral-control.md`, `collateral-eligibility-and-risk-policy.md`, and `threat-model-and-security-boundaries.md`.

| Failure mode | Failed invariant | Argent control | What Argent does not solve |
|---|---|---|---|
| F1. Duplicate financing | Uniqueness | The current contract refuses an identical active `uniqueness_hash`. The target production profile derives a deterministic nullifier from custodian-controlled canonical lot identity, so another evidence salt or facility does not create another identity inside that domain. | Differently supplied keys in the current reference; double-financing outside the governed domain; compromised authorities; weak or absent external registries |
| F2. Phantom or recycled documents | Attested identity | `register_position` binds a five-field `LotEvidence` commitment; `confirm_and_immobilize` requires the custodian's own signature; a forged receipt has no custodian willing to sign it in | Truth of the document itself; a determined off-system forgery |
| F3. False physical content | Independent attestation | `LotEvidence` commits `manifest_hash`, `uniqueness_hash`, `quality_cert_hash`, `quantity_cert_hash`, `location_hash` separately, so assay and weight are their own dated commitments by an independent verifier role | Physical assay, inspection, and warehouse honesty; a fake that passes the tests performed |
| F4. Unauthorized disposal | Authority and separation | Custody transitions require the custodian's signature; release is two-step, `bank_authorize_release` then `custodian_confirm_release`; `apply_repayment` reduces exposure without releasing | Physical release outside the control system |
| F5. Borrower-controlled verification | Verifier independence | Role separation with deny-by-default (`approve_party`, `revoke_party`): the owner cannot sign attestations, custody confirmations, or releases; the operator cannot sign for any institutional role | Independence of the parties in the real world, if a lender accepts a compromised verifier |
| F6. Stale and unverifiable state | Freshness | `revalue_and_check` enforces maximum valuation age and confidence before capacity; the margin lifecycle (`Covered`, `Warning`, `Called`) forces reconciliation on a cadence; the auto-collateralisation layer adds shock-triggered revaluation and recorded monetization drills | Price sourcing and the bank's own credit policy |
| F7. Fragmented lender visibility | Shared integrity | The transparent reference uses a replayable `CollateralEventV1` book. The production profile uses a confidential role-authorized read model anchored by uniform Soroban state-root batches, so authorized parties reconcile one state without publishing the facility graph. | Off-chain archive discipline, indexing, integration quality, and activity inference from public cadence |
| F8. Enforcement-time discovery | Continuous evidence | `open_enforcement_readiness`, `populate_enforcement_readiness`, `record_enforcement` accumulate the pack across the exposure's life, so the enforcement state is the state signed throughout | Court enforcement, insolvency stays, and local perfection of the security interest |
| F9. Collusion and stale authority | Role-bound revocable authority | `approve_party` / `revoke_party`, `GovernanceEventV1` governance events, and the DFNS role-wallet and approval-policy direction; state-changing functions require the correct role | Internal identity and access-management failures, or collusion outside the system |

Three properties of the mapping deserve emphasis.

First, the controls are structural, not procedural. Every case in Section 3 occurred inside institutions with procedures; the frauds routed around procedure by exploiting the arrangement's structure. Argent's answer changes the structure: an act that is not signed by the right role in the right sequence does not exist in the record, and an act that conflicts with the record's state fails in the contract, not in a later audit.

Second, the controls are cheap at the moment they matter. The Kingold divergence was discoverable by one independent test; the LME divergence by one weighing. What the incumbent arrangement lacked was not capability but a standing structure that made verification someone's signed, dated, attributable obligation before default forced it.

Third, the evidence and event structure is already granular. The five-field `LotEvidence` shape is the shipped separation of manifest, identity, quality, quantity, and location, and the `CollateralEventV1` and `GovernanceEventV1` streams are the shipped replayable record from which a reviewer can rebuild positions, pledges, lines, repayments, valuations, adjustments, releases, default, and enforcement. Neither is a placeholder.

---

## 7. What Argent can credibly claim, and what to avoid

The discipline that makes this document trustworthy is knowing exactly where the claims stop.

**Argent can credibly claim** that it records the collateral-control lifecycle as role-signed state transitions; binds facility, pledge, custody, eligibility, margin, and enforcement documents by hash; creates shared state between owner, bank, and custodian; separates custody control, pledge activation, credit exposure, repayment, release, default, cure, and enforcement; prevents certain inconsistent states inside the participating perimeter; creates a replayable event record for audit, reconciliation, and dispute reconstruction; supports an asset-agnostic collateral model; and can run as a design-partner pilot around one pledged pool, one custodian, one lender, one eligibility schedule, and one evidence pack. These claims are strong enough without overclaiming.

**Argent must not claim** that it prevents commodity fraud in general; proves the physical asset exists; proves metal grade, oil volume, or crop quality; replaces custody agreements or security-law perfection; creates a legally final warehouse receipt; makes all commodities equally financeable; creates global uniqueness without registry or market adoption; allows release without bank consent; automates court or insolvency enforcement; makes physical collateral equivalent to liquid securities collateral; turns pledged goods into freely transferable tokens; or guarantees cheaper credit.

The boundaries, stated as they appear in the case record:

- **Argent does not verify physical truth.** A gilded bar that passes the tests actually performed defeats any record system. What the record changes is who tested, under whose authority, on what date, with what evidence, and how long a divergence can persist undetected. In the Kingold pattern, testing controlled by the borrower cannot be represented as independent attestation, and the absence of fresh independent attestation is itself visible.
- **A collusive custodian defeats custody confirmation.** If the custodian signs false claims, the record is wrong with a signature on it. The mitigation is accountability, not omniscience: the false claim is attributable, dated, and non-repudiable.
- **Off-system agreements stay off-system.** A lot lock prevents duplicate financing inside the Argent universe but not outside it unless the bank and custodian make the record operationally mandatory. Side letters, unrecorded encumbrances, and claims in jurisdictions that ignore the record are legal-system problems; the mitigation is commercial, a lender that requires the record as a condition of credit.
- **Receivables and trade-flow fraud are adjacent, not covered.** Hin Leong's forged invoices and Trafigura's Mongolian receivables manipulation attacked the cash-flow leg. Argent's scope is the collateral control state; it narrows the fraud surface, it does not eliminate categories outside it.

Stated as a principle: Argent records signed control state over physical collateral that remains in custody. It makes specific collateral-control failures harder to hide inside the participating workflow, and it gives lenders, custodians, borrowers, and reviewers a shared evidence trail for the facility lifecycle. It converts silent, unattributable, slowly compounding divergence into loud, attributable, early divergence. That is the whole claim, and the case record suggests it is worth billions.

---

## 8. Design implications for the next build

The loss record should shape the build in a specific order.

**Treat uniqueness as a primitive, not a convenience.** An arbitrary or randomly salted `uniqueness_hash` is not the direct answer to Qingdao-style duplicate financing. The production control must use versioned canonical identity, custodian-controlled deterministic nullifiers, governed domain scope, and key continuity. Every future pool, substitution, auto-collateralisation, and read-model feature must preserve that invariant and state where its visibility ends.

**Keep evidence categories separate.** The five-field `LotEvidence` separation must survive into the read-model, so a bank sees manifest, identity, quality, quantity, and location freshness independently rather than as one opaque hash.

**Add an inspection heartbeat before adding more automation.** Auto-collateralisation without freshness discipline is dangerous. Before self-triggered credit events become credible, the system needs a policy-defined inspection, assay, valuation, or custodian-reconfirmation heartbeat, and a failed heartbeat must reduce or freeze capacity. This is the direct lesson of Kingold and the LME stones.

**Separate repayment from release, permanently.** Repayment reduces exposure; it must never automatically release collateral. Physical release stays bank-authorized and custodian-confirmed, because unauthorized disposal is a recurring loss mode.

**Design for partial adoption, and be honest about its limit.** A lot lock prevents duplicate financing inside the Argent universe but not outside it unless adoption is operationally mandatory. The design-partner questionnaire should ask precisely how a bank would require custody instructions and security documents to reference the record.

**Make exceptions explicit states, not comments.** Disputes, legal holds, stale audits, conflicting claims, non-conforming material, expired insurance, sanctions flags, and custodian exceptions must become explicit states or evidence flags in the read-model and pool model, as specified in `collateral-eligibility-and-risk-policy.md`. A failure mode with no state is a failure mode with no control.

---

## 9. Test requirements derived from the loss record

Following the repository's test-surface convention, these families trace directly to the cases and should exist or be added as the product moves beyond the current core.

**Duplicate financing.** Current tests prove that a second active position with the identical supplied `uniqueness_hash` is refused. Target tests must prove that alternate evidence salts and facility contexts still derive the same custodian nullifier; canonicalization is consistent; key rotation preserves active locks; a released lot is reusable only after bank and custodian release complete; and domain-scope limitations are explicit.

**Evidence completeness.** Missing manifest, uniqueness, quality, quantity, or location commitments are rejected; positions cannot be registered against non-admitted instruments; custodian confirmation must come from the position's own custodian; stale attestation blocks registration or capacity use.

**Role separation.** Owner cannot confirm custody; custodian cannot authorize bank release; bank cannot sign owner selection; operator cannot sign any institutional role; a revoked or unapproved party cannot perform state-changing functions.

**Release and disposal.** Repayment does not release collateral; `custodian_confirm_release` fails unless `bank_authorize_release` occurred first; release on a defaulted or enforcement-locked line requires explicit policy permission; a release event records the evidence hash of the off-chain release document.

**Freshness and margin.** Stale valuation rejects new capacity; low-confidence valuation rejects new capacity; margin warning and call states are visible in the read-model; a price shock requires revaluation before an auto-credit opening; expired insurance or audit evidence freezes capacity.

**Physical-truth boundary.** The system accepts only evidence commitments, not raw physical-truth claims; changing a quality certificate requires a new signed evidence event; conflicting quality evidence blocks capacity pending resolution; false evidence remains attributable to the signing role.

**Cross-lender visibility and replay.** A read-model query shows whether a lot is free, selected, immobilized, pledged, pending release, released, defaulted, or enforcement-locked; a reviewer can rebuild the full lifecycle from the ordered `CollateralEventV1` and `GovernanceEventV1` history; future multi-bank deployments must distinguish data confidentiality from encumbrance visibility.

---

## 10. The industry is already moving

The direction of travel after these losses supports the design, and it is worth recording because it shows Argent building with the current, not against it.

Post-Trafigura commentary identified the mechanism plainly: paper bills of lading and letters of credit remain susceptible to forgery, and the absence of shared visibility between banks creates the space for double financing; the same commentary pointed to digitalisation and the UK's electronic trade documents legislation as the response [23]. The LME's response to the stones incident was mandatory re-inspection of warranted nickel across its network [19]. In the gold market specifically, the LBMA's Gold Bar Integrity programme is moving the Good Delivery ecosystem toward structured digital reporting on a distributed-ledger database, with voluntary Country of Origin reporting for refiners from April 2026 becoming mandatory in 2027 and custodian vault reporting following, as documented in `auto-collateralisation-layer.md`.

Each of these is a partial instance of the same lesson: records of physical collateral must be shared, digital, attributable, and reconciled on a cadence, or the nine failure modes recur. Argent applies that lesson to the relationship among a reserve owner, bank, custodian, beneficiary, verifier, and settlement systems. The implemented secured-credit branch proves the core controls; the target obligation facility extends them across guarantees, documentary credits, treasury exposure, accepted obligations, claims, reimbursement, release, and enforcement.

---

## References

Independent sources, cited to evidence the case record and market response described above. Case descriptions follow the cited reporting and judgments; no partnership or endorsement by any named organization is implied.

[1] South China Morning Post, "How Kingold Jewelry's fake gold bars slipped through scrutiny in one of China's biggest loan scams" (including the Qingdao 2014 account per local police and court statements), July 2020. https://www.scmp.com/business/companies/article/3092042/explainer-how-kingolds-fake-gold-bars-slipped-through-scrutiny

[2] Hubbis, "83 Tons of Fake Gold Bars discovered in China" (citing Reuters' ghost-collateral reporting, including the China CITIC Bank steel case), 2020. https://www.hubbis.com/news/83-tons-of-fake-gold-bars-discovered-in-china

[3] Small Caps, "China in counterfeit gold scandal as Wuhan company uses fake bars to gain $4.1bn in loans" (including the 2016 Shaanxi tungsten-core precedent), July 2020. https://smallcaps.com.au/article/china-counterfeit-gold-scandal-wuhan-kingold-jewelry-fake-bars-loans

[4] Mining.com / Bloomberg, "LME rocked by new nickel scandal after finding bags of stones" (including the 2017 Access World forged-documents losses and the 2018 Sberbank/Liberty report), March 2023. https://www.mining.com/web/lme-finds-some-nickel-underlying-its-contracts-is-missing/

[5] Wikipedia, "Hin Leong" (liabilities, lender exposures, procedural history with underlying citations to FT, Straits Times, Business Times). https://en.wikipedia.org/wiki/Hin_Leong

[6] Global Trade Review, "ABN Amro to exit trade and commodity finance after losses," August 2020. https://www.gtreview.com/news/global/abn-amro-to-exit-trade-and-commodity-finance-after-losses/

[7] Global Trade Review, "Analysis: Hin Leong's 'vicious cycle' of trade finance fraud," August 2020. https://www.gtreview.com/news/asia/analysis-hin-leongs-vicious-cycle-of-trade-finance-fraud/

[8] Global Trade Review, "Hin Leong founder jailed over fraud scandal that shocked Singapore," November 2024. https://www.gtreview.com/news/asia/hin-leong-founder-jailed-over-fraud-scandal-that-shocked-singapore/

[9] South China Morning Post, "Singapore sentences O.K. Lim to over 17 years in prison," November 2024. https://www.scmp.com/news/asia/southeast-asia/article/3287035/singapore-sentences-ok-lim-over-17-years-prison

[10] Redbridge, "Commodity Trade Finance between fraud, pandemic and digitization" (Hin Leong, Agritrade, Phoenix, Hontop, Zenrock cluster and bank losses), 2020. https://www.redbridgedta.com/market-intelligence/commodity-trade-finance-between-fraud-pandemic-and-digitization/

[11] Sixth Tone / Caixin, "The Mystery of $2 Billion of Loans Backed by Fake Gold," July 2020. https://www.sixthtone.com/news/1005879

[12] Yahoo Finance / SCMP, "How Kingold Jewelry's fake gold bars slipped through scrutiny," July 2020. https://finance.yahoo.com/news/kingold-jewelrys-fake-gold-bars-093000408.html

[13] BusinessToday, "Big gold fraud busted in China! Gold market spooked by massive counterfeiting scandal," June 2020. https://www.businesstoday.in/latest/world/story/biggest-gold-fraud-busted-in-china-83-tons-of-fake-gold-bars-used-as-loan-collateral-262661-2020-06-30

[14] Trafigura, "Statement re Legal Action" (February 2023 statement and January 2026 judgment welcome). https://www.trafigura.com/news-and-insights/press-releases/2023/statement-re-legal-action/

[15] Wikipedia, "Trafigura" (nickel fraud reserve, January and February 2026 High Court outcomes, October 2024 Mongolia writedown, with underlying citations to WSJ, FT, Reuters). https://en.wikipedia.org/wiki/Trafigura

[16] Insurance Journal / Bloomberg, "Businessman Accused in Trafigura Nickel Nightmare Goes on Trial," November 2025. https://www.insurancejournal.com/news/international/2025/11/17/847928.htm

[17] Hill Dickinson, "Commodity trader successful in fraudulent nickel trade claims: Trafigura Pte Ltd v Prateek Gupta [2026] EWHC 159 (Comm)," 2026. https://www.hilldickinson.com/our-view/articles/commodity-trader-successful-in-fraudulent-nickel-trade-claims/

[18] Bloomberg, "LME Rocked by New Nickel Scandal After Finding Bags of Stones," March 2023. https://www.bloomberg.com/news/articles/2023-03-17/lme-finds-irregularities-in-nickel-underlying-nine-contracts

[19] Mining.com / Reuters (Andy Home), "Column: The return of the London Metal Exchange's nickel curse," March 2023. https://www.mining.com/web/column-the-return-of-the-london-metal-exchanges-nickel-curse/

[20] Global Trade Review, "ABN Amro to exit trade and commodity finance after losses," August 2020. https://www.gtreview.com/news/global/abn-amro-to-exit-trade-and-commodity-finance-after-losses/

[21] TXF, "Looking for the commodity finance upside" (ABN AMRO exit, BNP Paribas Geneva, Société Générale Singapore, Rabobank closures), March 2021. https://www.txfnews.com/articles/7149/looking-for-the-commodity-finance-upside

[22] Global Trade Review, "Analysis: Hin Leong's 'vicious cycle' of trade finance fraud" (Société Générale and BNP Paribas consolidation), August 2020. https://www.gtreview.com/news/asia/analysis-hin-leongs-vicious-cycle-of-trade-finance-fraud/

[23] TXF, "Trafigura and the case of the missing nickel" (flight to quality, per-lender opacity and double financing, electronic trade documents direction), March 2023. https://www.txfnews.com/articles/7514/trafigura-and-the-case-of-the-missing-nickel

[24] Compounding Zero, "The Tide Is Going Out: Commodity Houses, Private Credit, and the AI Debt Bomb" (bank withdrawal and the shift of mid-tier commodity borrowers to private credit), 2026. https://compoundingzero.substack.com/p/the-tide-is-going-out-commodity-houses

[25] Deutsche Bank, "Fighting trade-related fraud" (US$20bn liquidity removed, ADB 40 percent SME rejection finding, document-based fraud analysis), April 2023. https://flow.db.com/topics/trade-finance/fighting-trade-related-fraud

[26] S&P Global Market Intelligence, "Funds see 'strong business case' for commodity trade finance amid banks' retreat" (H1 2020 revenue decline, banks losing control over financed goods, multiple financing of the same trade), 2020. https://www.spglobal.com/marketintelligence/en/news-insights/latest-news-headlines/funds-see-strong-business-case-for-commodity-trade-finance-amid-banks-retreat-62560607

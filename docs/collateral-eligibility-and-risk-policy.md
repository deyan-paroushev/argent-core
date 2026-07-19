# Collateral Eligibility and Risk Policy Framework

> **Positioning status:** The policy applies to the full reserve obligation facility. Eligibility determines which reserve value may enter the capacity calculation. Separate reservation and deliverability rules determine what is available, issuable, and operationally usable for a particular obligation. Product sublimits, contingent exposure, claims, reimbursement exposure, and funded settlement determine how capacity is consumed. The current contracts implement the secured-credit reference branch.

**How a bank defines eligible physical collateral, valuation, haircuts, concentration limits, substitution rules, and escalation, and how Argent enforces that policy once it is signed.**

**Status:** design-partner policy framework, not current committed build
**Purpose:** define the bank-owned collateral policy that Argent enforces
**Boundary:** Argent enforces signed policy. It does not set credit policy, price risk, custody assets, decide eligibility, or perform legal enforcement.
**Companion documents:** `collateral-control.md`, `auto-collateralisation-layer.md`, `collateral-failure-modes.md`, `threat-model-and-security-boundaries.md`
**Last updated:** 2026-07-09

*This is a design and policy note, not legal, banking, regulatory, or investment advice. It describes a framework a bank or secured-credit provider would define and Argent would enforce; it is not a claim that Argent is a financial market infrastructure, a central bank collateral system, or a regulated entity. The institutional references below establish the discipline a bank will expect, not a status Argent holds. Any production deployment requires jurisdiction-specific legal review, custody agreements, bank credit policy, security review, and independent audit.*

---

## 1. The one thing this document establishes

Every other document in this repository describes what Argent controls, evidences, or automates. This one describes what it does **not** decide.

> Argent does not decide what collateral is acceptable, what it is worth, or how much may be lent against it. The bank does. Argent's role begins the moment that policy is signed, versioned, and mapped to contract controls, and its role is to enforce it exactly, on every position, on every credit event, without exception and without discretion.

This is the institutional boundary that makes the rest of the system safe to adopt. A bank cannot outsource its collateral risk policy to a protocol, and should not trust one that offers to set it. What a bank can do is encode its own policy, once, as signed static data, and require that no credit event, revaluation, substitution, or release ever occur outside it. That is the discipline this document specifies, and it is the discipline that the auto-collateralisation layer depends on: policy first, earmark second, automation third.

The framework below is deliberately the mindset of the institutions that have solved collateral risk at scale. It is not a claim to their status.

---

## 2. What a bank already expects, and where it comes from

The collateral risk disciplines a bank will look for are not novel. They are codified in the frameworks that govern how the most demanding institutions accept collateral, and a serious counterparty will expect Argent to speak this language even though Argent is not one of these institutions.

**PFMI Principle 5 (collateral).** The CPMI-IOSCO Principles for Financial Market Infrastructures set the reference standard. Principle 5 requires an entity that takes collateral to accept assets with low credit, liquidity, and market risk; to value collateral daily and mark it to market; to set stable, conservative haircuts calibrated to include stressed market conditions and to backtest them; to avoid concentrated holdings; to address cross-border and wrong-way risk; and to operate a collateral-management system flexible enough to do all of this in a timely way [1], [2], [3]. Every one of those is a policy input in the framework below.

**Eurosystem collateral practice.** The Eurosystem operationalises the same principles at scale. Eligible assets are assessed against defined criteria and listed daily; collateral is valued daily using a single reliable price, with a theoretical price used only where no market price exists; a valuation haircut is applied on top; and close-links rules prevent a counterparty from posting assets issued by itself or an affiliated entity [4], [5], [6]. Concentration limits cap exposure by counterparty and by issuer [7]. Crucially, eligibility is not permanent: the Eurosystem reserves the right to exclude an asset at any time, and an asset that loses eligibility ceases to count [4], [6]. Argent is not a Eurosystem system, but a bank collateral officer will recognise this exact logic and expect Argent to be able to encode it.

**Uncleared-margin treatment of gold.** For the first asset, the discipline is concrete. The BCBS-IOSCO framework for margin on non-centrally cleared derivatives lists gold as eligible collateral and assigns it a 15 percent standardised haircut, while requiring diversification, wrong-way-risk avoidance, and dispute-resolution procedures [8]. This gives the policy a defensible default: Argent does not invent gold's haircut, it enforces a bank policy that can adopt the standardised figure or the bank's own more conservative number.

**The market is moving toward programmable collateral, on existing rails.** The direction of travel supports the design, and it is consistent across infrastructure, standards, and law.

On infrastructure, the priority is collateral mobility, not tokenization for its own sake. The ECB has opened Eurosystem collateral eligibility to DLT-based marketable assets from 30 March 2026, admitted through the existing eligibility framework rather than around it [9]. DTCC has announced plans to connect DTC's tokenization service to Stellar, with DTC-tokenized assets expected in the first half of 2027, framed around collateral mobility [10]. The Bank of England's Project Meridian Securities tested synchronisation between tokenised securities and central bank money with explicit objectives of accelerating collateral mobilisation, reducing settlement risk, and enabling interoperability between conventional systems and DLT, including intraday repo with automated eligibility checks and collateral allocation [11]. The pattern is uniform: existing legal and settlement rails stay authoritative, and programmable systems coordinate faster, safer execution around them. Argent applies exactly this pattern to physical collateral.

On standards, collateral eligibility is already becoming a structured digital object rather than prose. The ISDA Common Domain Model's eligible-collateral representation encodes include and exclude logic, haircut treatments, and concentration-limit caps by percentage or value as reusable, machine-readable profiles [12]. Argent's instrument registry and framework-level eligibility treatment point in the same direction; this document specifies the bank-owned policy those fields compress.

On law, stored goods are becoming more financeable. The 2024 UNCITRAL-UNIDROIT Model Law on Warehouse Receipts supports the issuance and transfer of electronic and paper receipts alike and expressly contemplates stored goods, including metals, being used as collateral while warehoused [13]. This matters because Argent's architecture is asset-agnostic: a lot can be a bar set, a warehouse receipt, a bulk parcel, or a tank certificate, provided the legal and evidence layer can identify it and a bank admits it under policy.

And for gold specifically, the LBMA Gold Bar Integrity programme is moving the Good Delivery ecosystem toward structured digital reporting on a distributed-ledger database: voluntary Country of Origin reporting for refiners opened from April 2026 and becomes mandatory in 2027, with custodians to be onboarded for aggregated vault-holdings reporting and an intent to move toward bar-level reporting, as documented in `auto-collateralisation-layer.md`. But structure is not control: a better bar database strengthens the upstream evidence environment, while Argent operates downstream on pledge control, borrowing base, utilisation, release, substitution, default, and enforcement.

The unifying point is that collateral is becoming more data-rich and more mobile, but the data still needs a control, pledge, valuation, substitution, and credit-event layer that enforces a bank's policy against it. The market-relevant claim is not that Argent tokenizes gold; it is that Argent makes bank policy over physical collateral programmable without pretending the asset itself has become a free-floating token. That layer is what this document specifies.

---

## 3. The policy object

A bank's collateral policy is not one rule; it is a stack of them, held as a single signed, versioned object. It does not need to ship as one monolithic contract type. Much of it already exists across the shipped surfaces: the control framework, instrument admission, the eligible-schedule hash, the margin policy, valuation records, credit-line terms, collateral adjustment, and evidence packs. The design-partner output is a single signed policy pack whose components are stored and enforced through existing and future objects.

Illustrative structure, in the roadmap convention of real signatures that are explicitly not committed for the current build:

```rust
#[contracttype]
pub struct CollateralRiskPolicy {
    pub policy_id: BytesN<32>,
    pub policy_version: u32,
    pub bank: Address,
    pub policy_doc_hash: BytesN<32>,
    pub eligible_schedule_hash: BytesN<32>,
    pub valuation_policy_hash: BytesN<32>,
    pub haircut_policy_hash: BytesN<32>,
    pub concentration_policy_hash: BytesN<32>,
    pub mobilisation_policy_hash: BytesN<32>,
    pub substitution_policy_hash: BytesN<32>,
    pub release_policy_hash: BytesN<32>,
    pub exception_policy_hash: BytesN<32>,
    pub evidence_policy_hash: BytesN<32>,
    pub active: bool,
}
```

Seven invariants govern the object, and they are the reason a bank can trust automation built on top of it:

1. **Versioned.** Every collateral decision points to a policy version.
2. **Hash-anchored.** Every off-chain policy document has a canonical hash.
3. **Bank-owned.** Risk parameters are supplied by the lender, never derived by Argent.
4. **Party-signed.** Policies affecting owner or custodian obligations require their countersignature or explicit acceptance.
5. **Fail-closed.** Missing, inactive, stale, or inconsistent policy data blocks automated action.
6. **Evidence-bound.** Every policy decision records the evidence it relied on.
7. **Exceptions are visible.** Overrides are separate signed events, never hidden operator choices.

A policy version is immutable once referenced by a live pledge, line, pool, or credit event. A later policy supersedes it for new activity; it never silently rewrites the rules under which an existing exposure was opened.

---

## 4. Eligibility policy

Eligibility is the first gate. A position that fails it has no advance value, however high its market price.

### 4.1 Asset eligibility

For the first gold adapter, the bank defines which forms it accepts, answering at least:

```text
Asset type accepted (allocated physical gold first)?
Allocated or unallocated?
Specific bar identity and serial number required?
Refiner and Good Delivery status required?
Fineness and weight tolerance required?
Exchange warrant accepted? Warehouse receipt accepted?
Fractional entitlement or pooled holding accepted?
Jurisdiction restrictions? Transfer or pledge restrictions?
```

The safest V1 default:

> Accept only allocated or specifically identified positions with unique identity, custodian confirmation, and no known competing encumbrance.

Pooled or fractional claims may be valuable, but they carry separate legal and operational treatment and are not assumed equivalent to identified allocated bars.

### 4.2 Custodian and vault eligibility

The custodian is the physical root of trust. The policy defines accepted custodians and vaults before collateral is admitted, by at least: legal identity and regulatory status, jurisdiction, custody terms, segregation or allocation model, insolvency and client-asset protection, release-control procedure, lien and set-off limitations, insurance, audit frequency, reporting cadence and format, incident-notification requirement, sanctions and AML expectations, and business-continuity process. Argent does not decide whether a custodian is acceptable; it records whether the bank admitted that custodian under a specific policy and whether the custodian signed the relevant custody-control events.

### 4.3 Evidence eligibility

The asset is eligible only if its evidence pack is. For allocated gold, minimum fields include custodian and owner identity, account reference, bar or position list, serial or warrant number, refiner, gross and fine weight, fineness, allocation date, custody-statement date, insurance reference, audit reference, encumbrance statement, release-control agreement reference, eligible-schedule hash, valuation reference, and sanctions and dispute status. The policy distinguishes evidence by role: hard evidence required for admission, fresh evidence required for mobilisation, periodic evidence required to keep the position eligible, and exception evidence required when a normal feed is missing or disputed. A missing bar list, stale statement, unresolved discrepancy, or dispute flag is not merely a warning where the bank treats it as essential; it sets advance value to zero until cured.

### 4.4 Legal eligibility

Legal eligibility is not solved by a smart contract. The policy requires legal documentation before the system treats a position as eligible: facility agreement, pledge or security agreement, custody agreement, account-control or blocking agreement, release-control procedure, enforcement waterfall, governing law and jurisdiction, insolvency opinion where required, title or beneficial-ownership evidence, consent to disclose, and custodian acknowledgement of the bank's control rights. Argent hashes and references these documents. It cannot make an invalid security interest valid by recording it. This is the control-not-title thesis in its most exact form, and it is how real clearing houses already operate: LME Clear takes a separate English-law charge and pledge over the warrants and the metal they represent [3].

### 4.5 Negative eligibility rules

A position is ineligible if any of these hold: identity not unique; custodian has not confirmed control; owner has no recognised interest under the policy documents; a competing pledge, lien, freeze, legal hold, or dispute exists; evidence stale beyond tolerance; valuation stale or unsupported; asset type not admitted; custodian or vault not admitted; jurisdiction excluded; sanctions or AML flag; refiner or chain-of-origin failure; insurance or audit evidence missing where required; concentration limit would be breached; wrong-way-risk rule would be breached.

The implementation rule, following Eurosystem practice where an asset that loses eligibility ceases to count [4], [6]:

> Ineligible collateral can remain recorded. It cannot support borrowing capacity.

---

## 5. Valuation policy

Eligibility says whether collateral counts. Valuation says what it is worth, on a defined cadence.

**Required inputs.** Price source and fallback source; frequency (Eurosystem practice is daily [4]) plus event-driven revaluation on a price shock beyond a threshold; a maximum price-age staleness rule; a minimum confidence rule; FX source and its own freshness rule where collateral and credit differ in currency.

**Stale-price and zero-value rules.** Beyond the staleness or confidence bound, a position cannot support a new credit event. Where no acceptable price exists, the policy defines a theoretical value or zero. An ineligible or unpriceable position contributes zero to the borrowing base.

**Haircut stack.** For a physical asset the bank may stack several buffers, each an explicit policy number:

```text
base market haircut
liquidity haircut
FX mismatch haircut
custody or documentation haircut
jurisdiction haircut
concentration add-on
operational-risk add-on
stress add-on
manual exception add-on
```

The BCBS-IOSCO 15 percent standardised figure for gold [8] is a benchmark, not an Argent rule; a bank may use a higher haircut, lower advance rate, different stress logic, or zero eligibility depending on its mandate. The contract enforces the final policy numbers; it does not derive them.

**Borrowing-base decomposition.** The contract already computes capacity in the right conceptual form. For the policy layer, the computation is decomposed into audit fields a bank reviewer can reproduce from the evidence pack:

```text
raw_value
minus ineligible_value
minus haircut_amount
equals haircut_adjusted_value
multiplied by max_advance_rate
equals borrowing_base
minus existing_exposure
minus reserved_amount
equals available_capacity
```

For the target obligation facility, this is not the end of the decision. `available_capacity` is a portfolio quantity. A specific request becomes issuable only after applicant, beneficiary, product, tenor, currency, jurisdiction, evidence, approval, and operational-route checks pass. See [`capacity-reservation-and-deliverability.md`](capacity-reservation-and-deliverability.md).

---

## 6. Advance-rate, margin, and escalation policy

Eligibility gates counting; valuation sets worth; advance policy sets how much exposure collateral can support. The bank defines: maximum LTV, opening LTV, maintenance LTV, warning threshold, margin-call threshold, cure window, suspension threshold, default threshold, enforcement-readiness threshold, minimum collateral surplus, minimum transfer amount, and minimum top-up amount.

**Margin state.** A compact state model is enough, and it maps directly to the shipped margin lifecycle:

| State | Meaning | Action |
|---|---|---|
| `Covered` | Coverage satisfies policy | Normal operations allowed |
| `Warning` | Coverage deteriorated, no call yet | New drawdowns blocked or manually approved |
| `Called` | Deficit exists | Cure required within policy window |
| `Suspended` | Bank blocks new exposure | Repayment and top-up allowed; new exposure blocked |
| `DefaultNoticeIssued` | Cure failed or default met | Enforcement-readiness path opens |
| `EnforcementReady` | Evidence pack prepared | Off-chain enforcement proceeds under legal documents |

The governing rule: risk-reducing actions remain possible in every state; actions that increase exposure or release collateral fail closed.

**Cure methods.** The policy specifies which cures are permitted: cash repayment, partial repayment, collateral top-up, substitution into higher-quality collateral, bank-approved waiver, valuation correction, documentation cure. Every cure is recorded with evidence. A phone call is not a cure unless it becomes a signed waiver event.

The full graduated escalation ladder, rebalance, cure, relocate, penalise, suspend, default notice, enforcement readiness, enforcement recording, is specified in `auto-collateralisation-layer.md`, so that default is the rare terminal event and not the first response to a missed timer.

---

## 7. Concentration and wrong-way-risk policy

Collateral that looks safe position by position becomes risky when concentrated. Following PFMI Principle 5's requirement to avoid concentrated holdings and address wrong-way risk [1], [3], the policy defines concentration limits by owner, borrower group, custodian, vault, country, asset type, refiner, warehouse operator, insurer, legal framework, facility, maturity bucket, currency, and valuation source.

Wrong-way risk is stated explicitly, because it is the failure that caused the Kingold pattern in `collateral-failure-modes.md`, where the borrower controlled evidence production without independent custodian confirmation. Examples the policy must flag: owner and custodian affiliated; owner and refiner affiliated; collateral price highly correlated with the borrower's business stress; collateral in a jurisdiction exposed to the same stress event as the borrower; collateral value dependent on a counterparty connected to the borrower. This is the direct analogue of the Eurosystem close-links rule [4].

Per flag, the policy chooses a treatment: hard exclusion, lower advance rate, additional haircut, manual approval, higher evidence requirement, concentration cap, or zero advance value. Argent does not infer wrong-way risk; it enforces the bank's defined relationship flags and limits once supplied.

---

## 8. Mobilisation, substitution, and release policy

**Mobilisation and earmarking** (the policy the auto-collateralisation layer consumes). The bank defines who may request collateral selection, who may trigger automated mobilisation, the permitted uses of proceeds, the standing amount that may be mobilised without fresh manual approval, and the carve-outs where automation stops and a human signature is still required.

**Substitution.** Replacement collateral must be admitted, valued, attested, and locked before the replaced collateral can be released, and a substitution may not reduce post-haircut coverage below policy.

**Release.** Final release remains a two-step, role-signed act: bank authorisation, then custodian confirmation. Repayment reduces exposure; it does not release collateral. Repayment is not release, under automation as today.

---

## 9. Exception and dispute policy

A mature policy specifies what happens when something is wrong, defined in advance rather than improvised at default. Recognised exception types: stale valuation, stale custodian statement, failed custody confirmation, disputed title, competing encumbrance, sanctions or AML flag, unexpected vault movement, insurance lapse, audit qualification, refiner status change, price-feed failure, FX-source failure, legal-document mismatch, signer revocation, policy mismatch, and contract pause or upgrade.

Each exception defines: severity, advance-value effect, allowed actions, blocked actions, required signer, required evidence, cure window, escalation path, and reporting requirement. The recommended default: risk-reducing actions allowed, risk-increasing actions blocked, collateral release blocked unless expressly approved, advance value zero for title, sanctions, competing-claim, or custody-control failures, and the exception recorded as an evidence event. Nothing is resolved silently.

---

## 10. Policy-to-contract mapping

A policy is only as good as the enforcement behind it. Each layer above binds to the shipped Argent core and its documented extensions. Function names are in the open-source `credit_ledger` contract unless marked as roadmap or auto-collateralisation scope.

| Policy requirement | Enforcing control |
|---|---|
| Bank, custodian, owner, verifier roles | `approve_party`, `revoke_party`, role-specific authorization; deny-by-default and role separation |
| Framework documents and eligible schedule | `register_framework`, document hash fields |
| Asset class definition | `register_instrument` |
| Bank admission under a treatment | `admit_instrument` with haircut, max LTV, maintenance threshold |
| Position identity and uniqueness | `register_position`, uniqueness hash, duplicate-pledge refusal |
| Owner selection | `select_lot_for_collateral`, exposed via `get_selection` |
| Custodian immobilisation | `confirm_and_immobilize` |
| Pledge activation | `activate_pledge` |
| Credit line opening | `open_credit_line` |
| Borrowing capacity | `available_capacity`, computed against the internally maintained haircut-adjusted borrowing base |
| Drawdown and reversal | `record_drawdown`, `reverse_drawdown` |
| Repayment | `apply_repayment` |
| Valuation freshness and margin state | `revalue_and_check`, `get_valuation`, `MarginState::{Covered, Warning, Called}` |
| Collateral top-up, substitution, partial release | `request_collateral_adjustment` with `AdjustmentType::{TopUp, Substitution, PartialRelease}`, gated by `bank_approve_adjustment` and `custodian_confirm_adjustment` |
| Release control | `bank_authorize_release`, then `custodian_confirm_release` |
| Suspension and restart | `bank_suspend_line`, `bank_resume_line` |
| Default and cure | `issue_default_notice`, `cure_default` |
| Enforcement evidence | `open_enforcement_readiness`, `populate_enforcement_readiness`, `record_enforcement` |
| Policy versioning | Future `CollateralRiskPolicy` or policy-pack hash |
| Earmarking and mobilisation | Future `EarmarkRecord`, `AutoCreditPolicy`, `LiquidityIntent`, `CreditEvent` (auto-collateralisation scope) |
| Concentration limits, per dimension | Enforced in the eligible schedule and the selector; formal per-dimension limits are roadmap and auto-collateralisation scope |
| Pool-level control | Roadmap `CollateralPool` |
| Design-partner reporting | Evidence read-model, pool risk report |

Two properties of the mapping matter as much as the mapping itself. First, most of the institutional logic is not a new product; it is a policy layer over surfaces Argent already has. Second, policy is versioned and frozen per event: every credit event records the exact policy version it executed under, so a later change never silently rewrites an open exposure, and the DFNS policy decoder resolves method, role, pool, policy version, amount, collateral, and evidence hash before any signature, so a signer never approves an opaque payload.

---

## 11. Minimum viable policy for a design partner

Before a pilot, a bank or secured-credit provider should be able to state, on one page: accepted asset type, custodians, and vaults; the disqualifiers; price source, frequency, staleness, and confidence rules; haircut, maximum LTV, maintenance, call threshold, and cure window; concentration limits and their dimensions; the escalation ladder and its penalties; and the required signer for each act. If a counterparty cannot fill that page, the pilot is not ready, and that is itself a useful finding.

---

## 12. Design-partner questionnaire

The questionnaire converts the abstract framework into one specific signed policy.

**Bank or lender (credit and risk):** Which asset types, custodians, vaults, jurisdictions, and refiners are eligible? What are the disqualifiers? What price source, frequency, staleness, and confidence rules? What haircut, maximum LTV, maintenance, call threshold, and cure window? What concentration limits, by which dimensions? What is the escalation ladder and its penalties? Which roles sign which acts?

**Custodian:** What custody evidence can you attest, and how? What is your confirmation latency for immobilisation, substitution, and release? Under what standing policy, if any, will you act, and within what limits? What is your process for a disputed or failed audit?

**Borrower or collateral owner:** Which positions will you earmark? What purposes and limits do you accept for automated mobilisation? What is your revocation process?

The output is a single policy pack, signed by all parties, that Argent then enforces without further discretion.

---

## 13. Evidence and reporting output

The framework produces three bank-readable artifacts. As with the rest of Argent, the artifact is the product: a bank does not adopt a policy engine in the abstract, it adopts one that can show risk, audit, and operations a defensible answer on demand.

**Collateral policy certificate.** The signed, versioned policy: eligibility schedule, valuation rule, haircut and advance-rate parameters, concentration limits, earmark and mobilisation terms, substitution and release rules, escalation ladder, and signing set, each by hash. This is what a credit committee approves and what every later event references.

**Position eligibility certificate.** For a given lot: identity and custody evidence, current eligibility status against the policy, fresh haircut-adjusted value or the zero-value reason, and any active exception. This turns "we hold gold" into "this bar is eligible, worth this much after haircut, under this policy version, as of this timestamp."

**Pool risk report.** For a facility: total attested, eligible, and haircut-adjusted value; free, earmarked, reserved, pledged, and drawn amounts; available borrowing base; margin status; concentration exposures by dimension against their limits; open exceptions; and escalation state. This is the standing answer to the lender's risk question: what is eligible and available now, under our policy, with proof. A separate deliverability decision answers whether a particular bank product can be issued now.

Reports and certificates should follow the role-specific visibility and evidence-minimization rules in [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md).

---

## 14. Implementation sequence

The framework is defined here so the policy discipline precedes any automation. Within a pilot, the order follows dependency, each phase shippable on its own:

1. **Policy template.** The questionnaire output becomes a versioned, hash-anchored policy pack. No enforcement logic is needed to deliver value: a signed, shared, unambiguous collateral policy is itself something most bilateral secured-credit relationships lack.
2. **Read-model policy fields.** Position eligibility certificates and the pool risk report, built from the existing registration, custody-confirmation, and `revalue_and_check` surfaces. This is what a pilot evaluates, and it carries no new contract risk.
3. **Policy certificate.** The signed policy certificate as a first-class, referenceable object.
4. **Exception reason codes.** The freeze-and-record exception path with the reason taxonomy of Section 9.
5. **Pool-level policy.** Concentration limits and the pool risk report against the roadmap `CollateralPool`.
6. **Earmark and auto-collateralisation readiness.** Only after 1 through 5 are proven does the auto-collateralisation layer consume this policy. Policy first, earmark second, automation third.

---

## 15. Test surface

Following the repository's test-surface convention, enforcement of a signed policy must prove at least:

**Eligibility.** Ineligible asset types, custodians, vaults, and refiners are refused; a position that trips a negative-eligibility rule loses eligibility and is valued at zero; an owner-custodian affiliation triggers the close-links exclusion.

**Valuation.** Stale price refuses a new credit event; low-confidence price refuses a new credit event; haircut is applied before the borrowing base; a price shock beyond threshold requires revaluation; an unpriceable position contributes zero.

**Advance rate.** No drawdown above maximum LTV; maintenance breach raises a warning; call threshold raises a call; missed cure enters escalation.

**Concentration.** A pool breaching a per-dimension limit is capped or penalised; wrong-way collateral is excluded or repriced.

**Release and substitution.** Outgoing collateral cannot be released before incoming is locked; a substitution that would reduce coverage below policy is refused; release requires both bank authorisation and custodian confirmation; repayment does not release.

**Exceptions.** Each exception type sets the correct advance-value effect, blocks the correct actions, and records a reason; a disputed title, sanctions flag, or custody-control failure sets advance value to zero.

**Policy integrity.** Only the bank authority can register or amend a policy; every event records the policy version it ran under; an amendment never alters an open event's terms; the decoder rejects any opaque or mismatched payload.

**Roles.** The owner cannot sign attestations, custody confirmations, or releases; the operator cannot sign for any institutional role; a revoked party cannot sign.

---

## 16. What this framework is not

- **Argent does not set the policy.** It holds and enforces a policy the bank defines and signs. A protocol that offered to choose haircuts or eligibility for a lender would be a liability, not a feature.
- **Argent does not price risk or value assets independently.** It enforces the bank's chosen price source, freshness, and haircut. It does not run a pricing model or assert a value of its own.
- **Argent does not verify physical truth.** Eligibility and valuation act on signed custody and evidence claims, not on physical assay. The boundary is stated in full in `collateral-failure-modes.md` and `threat-model-and-security-boundaries.md`.
- **Argent does not originate credit or custody assets.** The bank lends and carries credit risk; the custodian holds and controls the asset.
- **Argent does not perform legal enforcement.** It produces evidence for enforcement; enforcement happens off-chain, in law and custody.
- **Argent does not claim FMI or Eurosystem status.** The institutional frameworks cited here define the discipline a bank expects, not a status Argent holds.

Stated as a principle: a bank's collateral policy is the bank's to write and Argent's to keep. This document specifies how a bank writes it once, and how Argent then enforces it on every position, every event, every time, without discretion and without drift.

---

## References

Independent sources, cited to evidence the collateral-risk discipline described above. No partnership or endorsement by any named organization is implied, and no FMI, central bank, or regulatory status is claimed for Argent.

[1] CPMI-IOSCO, "Principles for financial market infrastructures," April 2012 (Principle 5, Collateral). https://www.iosco.org/library/pubdocs/pdf/IOSCOPD377.pdf

[2] European Central Bank, "Applicability of CPMI-IOSCO Principles for financial market infrastructures to TARGET2-Securities" (Principle 5 key considerations). https://www.ecb.europa.eu/paym/pol/critical/shared/pdf/CPMI-IOSCO_Principles_TARGET2-Securities.en.pdf

[3] LME Clear, "CPMI-IOSCO Principles for Financial Market Infrastructure Disclosure," 2024 (Principle 5 in practice, and the separate English-law charge and pledge over warrants and the metal they represent). https://www.lme.com/-/media/Files/Clearing/Rules-and-regulations/Disclosure-and-transparency/LME-Clear-CPMI-IOSCO-Disclosure-Document-2024.pdf

[4] Deutsche Bundesbank, "Collateral" (Eurosystem daily valuation, haircuts and variation margins, close-links exclusion, right to exclude assets). https://www.bundesbank.de/en/tasks/monetary-policy/collateral/collateral-625836

[5] Banque de France, "Monetary policy collateral" (daily eligibility assessment and haircut application). https://www.banque-france.fr/en/monetary-strategy/operational-framework/collateral/monetary-policy-collateral

[6] European Central Bank, "Collateral" (eligible asset database updated daily; eligibility assessed against defined criteria). https://www.ecb.europa.eu/mopo/coll/html/index.en.html

[7] European Central Bank, "The valuation haircuts applied to eligible marketable assets for ECB credit operations," Occasional Paper 312, 2023 (haircut guiding rules and confidence-level calibration). https://www.ecb.europa.eu/pub/pdf/scpops/ecb.op312~3f4457b95c.en.pdf

[8] BCBS and IOSCO, "Margin requirements for non-centrally cleared derivatives," July 2019 (gold as eligible collateral, standardised haircut, diversification and wrong-way-risk requirements). https://www.iosco.org/library/pubdocs/pdf/IOSCOPD635.pdf

[9] European Central Bank, "ECB paves way for acceptance of DLT-based assets as eligible Eurosystem collateral," press release, 27 January 2026. https://www.ecb.europa.eu/press/pr/date/2026/html/ecb.pr260127_1~a946167ce1.en.html

[10] DTCC, "DTC's Tokenization Service to Connect with Stellar Public Blockchain as DTC Advances its Multi-Chain Strategy," 27 May 2026. https://www.dtcc.com/news/2026/may/27/tokenization-service-to-connect-with-stellar-public-blockchain-as-dtc-advances-multi-chain-strategy

[11] Bank of England, "Project Meridian Securities" (synchronisation of tokenised securities and central bank money; atomic settlement, intraday repo with automated eligibility and collateral allocation, accelerating collateral mobilisation, interoperability), 2025. https://www.bankofengland.co.uk/payment-and-settlement/rtgs-future-roadmap/project-meridian-securities

[12] FINOS / ISDA, "Eligible Collateral Representation," Common Domain Model (include and exclude logic, haircut treatments, concentration-limit caps by percentage or value, reusable collateral profiles). https://cdm.finos.org/docs/eligible-collateral-representation/

[13] UNCITRAL and UNIDROIT, "Model Law on Warehouse Receipts (2024)" (electronic and paper receipts; stored goods, including metals, usable as collateral while warehoused), adopted 26 June 2024. https://uncitral.un.org/en/mlwr

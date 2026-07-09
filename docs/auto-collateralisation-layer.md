# Argent Auto-Collateralisation Layer: Self-Triggered Credit Events for Physical Collateral in Custody

**How Argent moves from a governed collateral-control record to a policy-driven collateral mobilisation engine, using the Eurosystem's T2S auto-collateralisation as the reference design.**

**Status:** product and architecture direction, not current committed build
**Reference implementation:** Soroban / Stellar
**First collateral adapter:** allocated physical gold
**Last updated:** 2026-07-09

*This is a forward-looking product and design note, not legal, banking, regulatory, or investment advice. Nothing here is a production commitment. The committed build scope is defined in `argent-architecture.md`; the post-mainnet product sequence is defined in `product-roadmap.md`. This document describes the layer that sits after both. Any production deployment would require jurisdiction-specific legal review, custody agreements, bank credit policy, security review, and independent audit.*

---

## Status: where this sits

**Live now.** The Argent core is a governed collateral-control record: three Soroban contracts recording the secured-credit lifecycle for physical collateral in custody, with role-signed transitions for pledge, valuation, margin state, drawdown, repayment, two-step release, default, cure, and enforcement.

**Committed next.** The DFNS-governed mainnet reference implementation, as defined in `argent-architecture.md`.

**Roadmap.** The pool and position model, scheduled revaluation, and safe substitution, as defined in `product-roadmap.md`.

**This document.** The layer after that: automation of the credit event itself. Everything Argent records today is initiated by a person and approved by counterparties. This note describes how the same record, under the same governance, becomes self-triggering: a permitted liquidity need mobilises pre-approved collateral automatically, within limits and policies the parties signed in advance, with every automated act recorded as a signed, auditable event.

The design is not invented. It is a translation of one of the clearest large-scale production precedents for automated collateral mobilisation: the Eurosystem's T2S auto-collateralisation (T2SAC), live since 2015 inside TARGET2-Securities. Where T2SAC mobilises securities against central bank money, the Argent auto-collateralisation layer mobilises the control state of physical collateral against a bank's credit line. The asset never moves and is never tokenized. What becomes automatic is the sequence of signed control events.

---

## 1. Summary

Today the Argent lifecycle is governed but manual: a static pledge, held and released as a whole, with every transition initiated by a person. That is correct and safe, and it is already stronger than the traditional secured-lending pattern, because every act is role-signed and evidenced. But it leaves the collateral operationally passive.

A modern collateral system behaves differently. In T2S, when a settlement instruction lacks liquidity, the system itself uses eligible collateral on flow or on stock, generates the needed intraday credit, settles the underlying transaction, creates the reverse reimbursement instruction, substitutes collateral automatically when it is needed elsewhere, and escalates through a graduated path if reimbursement fails by the deadline [1], [2], [3].

Argent already solves the part T2S never attempts for private physical collateral: the governed evidence layer. It records who signed, under what authority, against what evidence, in what order, and with what resulting collateral-control state. What Argent does not yet have is the automated credit-event layer.

The auto-collateralisation layer is that missing layer:

> When a permitted liquidity need appears, Argent checks a pre-approved credit policy, selects free eligible collateral, reserves it, computes borrowing capacity, routes the required approvals, opens a time-bounded credit event, binds the funding instruction to that event, starts the repayment and cure timers, and escalates through a graduated path if the event is not reimbursed or cured.

The central design rule is stated once and holds everywhere:

> **Automate execution, never consent.** Consent is granted in advance through signed, human-approved policy. Execution then happens at machine speed inside those limits, and every act is recorded as evidence.

The core boundaries do not move. The asset does not move on-chain and is never tokenized. Argent does not become a bank, custodian, card issuer, commodity token, payment system, CSD, or legal-enforcement engine. The bank still owns credit policy and funding. The custodian still controls the physical asset. Law and custody agreements still govern enforcement. What this layer automates is the control and evidence path around a bank-defined credit event.

---

## 2. Why now

Four external signals make this the right next product direction.

**Collateral policy is moving on-chain.** The ECB has announced that certain marketable assets issued in CSDs using DLT-based services will be accepted as eligible Eurosystem collateral from 30 March 2026, provided they meet existing eligibility and collateral-management requirements, with further work exploring how assets issued and settled entirely on DLT networks could become eligible [4]. The Eurosystem's collateral guideline consolidates the direction: harmonised, standardised collateral mobilisation through ECMS, eligible links, triparty agents, and collateral pools [5], [17]. This does not make privately held gold or Argent records eligible Eurosystem collateral, and Argent must never claim it. It shows the direction of travel: collateral eligibility and mobilisation are being modernised without abandoning regulated market-infrastructure discipline.

**Regulated assets are coming to Argent's chain.** DTCC and the Stellar Development Foundation have announced plans to connect DTC's tokenization service to the Stellar network, with DTC-tokenized assets expected in the first half of 2027, carrying the same investor protections, entitlements, and safeguards as traditionally held securities, and framed explicitly around greater asset mobility and collateral mobility [6]. DTCC addresses DTC-custodied financial assets. Argent is the complementary pattern for physical collateral that does not belong inside a securities-tokenization model.

**The liquidity problem is measured and large.** BIS staff measure daily intraday liquidity usage at around USD 630 billion in Fedwire and USD 443 billion in TARGET2, roughly 2.8 percent of GDP across major systems [7]. On the market side, tokenized repo infrastructure built for the same collateral-mobility problem in securities is settling hundreds of billions per day, with intraday repo named as the next expansion [8]. Collateral that can mobilise itself at the moment of need is worth more than collateral that waits for an operations team.

**Gold is regulator-recognised collateral without a mobilisation rail.** Under the Basel-based framework, allocated gold bullion of recognised good delivery is eligible collateral for uncleared derivatives margin with a 15 percent standardised haircut [9], [18]. CCPs may accept gold to cover margin, and CME Clearing accepts COMEX gold warrants and London gold bullion as collateral today [10], [20]. The capital treatment must be stated precisely. Allocated client gold under custody is neither an asset nor a liability of the custodian bank and sits entirely off its balance sheet [12]. Bank-held gold bullion, whether in own vaults or on an allocated basis, receives a 0 percent risk weight only to the extent that it is backed by bullion liabilities [21]. Separately, the NSFR assigns an 85 percent required stable funding factor to physical traded commodities including gold, which penalises on-balance-sheet gold positions [11], [22]. The regulatory geometry therefore favours exactly the configuration Argent records: allocated physical gold, held with a professional custodian, pledged under a clean, verifiable control regime, never carried as an unallocated balance-sheet claim. What that gold lacks is what a security in T2S has: a rail that lets it back credit in the same second the need arises. Argent's committed build closes the record-keeping half of that gap. This layer closes the mobilisation half.

The correct inference is narrow and strong: regulated finance is converging on programmable collateral mobilisation, DTCC is addressing DTC-custodied assets, T2S addresses securities settlement liquidity, and Argent addresses custody-stable physical collateral.

---

## 3. The reference design: what T2SAC actually does

T2SAC deserves close reading because it solves, at production scale and under central bank governance, exactly the problem Argent's manual lifecycle leaves open: how credit against collateral is extended at the moment of need without a human in the loop, and without either party ever holding an uncollateralised exposure. The scale is material: in 2025, T2SAC reached a daily average value of EUR 141.4 billion, of which EUR 116.0 billion on flow and EUR 25.4 billion on stock, and for every euro of DVP settlement in T2S, 13.3 cents were settled thanks to auto-collateralisation [24]. The mechanics, condensed from the ECB's functional documentation [1], [2], [3], [13]:

1. **The need triggers the credit.** When a settlement instruction lacks sufficient cash, T2S itself detects the shortfall during the provision check and triggers auto-collateralisation. No party instructs anything at that moment. Based entirely on pre-set static data from central banks, CSDs, and counterparties, all process steps, collateral selection, settlement of the intraday credit, settlement of the underlying transaction, and the reverse steps at reimbursement, are executed without manual intervention [1].

2. **Collateral on flow and on stock.** The credit is collateralised either by the securities being purchased in the very transaction that lacked cash (on flow) or by securities the counterparty already holds and has earmarked for this purpose (on stock) [1]. Earmarking is the counterparty's standing designation of holdings as available for automatic mobilisation.

3. **Joint settlement means zero uncollateralised exposure.** The originating transaction and the auto-collateralisation transactions settle jointly and atomically, so at no instant does either the central bank or the counterparty carry an exposure without collateral against it [1].

4. **Selection is an optimisation, not a choice.** T2S selects collateral so that no more than the necessary securities are used, after checking eligibility, valuation with haircuts, price freshness, and close links, meaning collateral linked to the counterparty itself is excluded or repriced [1], [14]. The night-time settlement engine treats auto-collateralisation as one resource inside a formal optimisation that maximises settled volume and value [14], and the same problem class has been formalised as mixed binary optimisation in the research literature [15].

5. **Credit is born with its own reimbursement.** Every auto-collateralisation operation automatically generates its reverse instruction, held in a pending state. The counterparty may release it any time during the day, freeing the collateral for other uses. If the counterparty needs a collateralised security for a new settlement, T2S performs automatic substitution: a new operation replaces the old one with no intervention by the counterparty [2].

6. **A hard deadline with a graduated failure path.** At 16:30 CET, T2S automatically reverses all outstanding operations. If funds are insufficient, the failure path is graduated, not binary: first liquidity is rebalanced across the counterparty's own cash accounts; then the collateral is relocated to the counterparty's general collateral pool at its central bank, converting the intraday credit into ordinary central bank credit and, if still unpaid at end of day, into marginal lending. Relocation is rare in practice and carries a EUR 1,000 penalty [3], [13].

7. **Limits are shared, visible objects.** The Credit Memorandum Balance is the standing static-data object that carries the auto-collateralisation limit per cash account; headroom against it is checked before every operation and visible to both sides [3].

8. **The mechanics are re-extensible.** Payment banks can offer the same automation to their own clients, using their own eligibility and limits on the same rails [1], [3]. That client tier is the closest precedent for Argent's private-sector translation: bank-defined automation, not central bank credit.

9. **The pricing signals what the mechanism is for.** Central bank auto-collateralisation credit is interest-free and its automated settlement instructions currently carry a zero fee [1]. In the TARGET Services pricing guide (version 3.0, July 2026), auto-collateralisation with a central bank is priced at EUR 0, while client collateralisation offered by a payment bank is a separately priced service item charged to the collateral provider [16]. Automated mobilisation is treated as shared infrastructure, and its private extension as a billable product.

### What is borrowed, and what is not

Argent borrows T2S's operating logic, not its legal or institutional status. Argent does not provide central bank money, does not settle securities, does not decide Eurosystem eligibility, and does not become a CSD, payment system, or central bank credit facility. What it borrows is the design pattern:

| T2S pattern | Argent translation |
|---|---|
| Static eligibility and account configuration | Bank-defined `AutoCreditPolicy` over the roadmap's `CollateralPool` |
| Earmarking of on-stock holdings | Owner-signed, custodian-confirmed `EarmarkRecord` per lot |
| Trigger from settlement cash shortfall | Signed `LiquidityIntent` for a permitted liquidity need |
| On-flow collateral | Collateral generated by the transaction being financed, where applicable |
| On-stock collateral | Free, earmarked, eligible collateral already in the custody pool |
| Automated collateral selection | Deterministic smallest-surplus selector over earmarked lots |
| All-or-none settlement linkage | No credit event opens unless reservation, coverage, and approvals complete atomically |
| Reverse reimbursement instruction | Repayment and cure obligation created at credit-event opening |
| Automatic substitution | New collateral locked before old collateral released, under standing policy |
| Intraday time boundary | Policy-defined repayment, cure, and escalation timers |
| End-of-day rebalance, relocation, and penalty | Rebalance from settlement balance, relocation to the general pool, recorded penalty, then the existing default path |
| Central bank or payment-bank limits | Bank-defined auto-credit limit and facility headroom |

---

## 4. Product thesis

Argent's current thesis:

> A physical holding becomes usable collateral when its identity, custody, eligibility, valuation, exclusivity, control state, and release path are made legible to a lender as a shared, signed, replayable control record.

The auto-collateralisation thesis extends it:

> A physical holding becomes mobilisable collateral when a pre-approved credit policy can transform free, earmarked, eligible collateral into a time-bounded credit event without reconstructing the evidence path manually each time.

This is the shift from evidence to operation. The current product says: this collateral is pledged, controlled, evidenced, and releasable only through the right parties. The next product says: this collateral pool has available borrowing capacity now, under this bank policy, for this permitted liquidity need, with this event timer and this escalation path. That is the difference between a pledge database and collateral infrastructure.

For the company that owns the reserve, the shift is equally concrete. Today a gold reserve is a just-in-case asset: it backs nothing until someone runs a pledge process. An earmarked reserve under a registered policy is a standing liquidity facility that activates itself. The gold does not move, is not tokenized, and remains the owner's property under custody. What stands ready is the control state.

---

## 5. Target user and first use cases

The first user is not a retail borrower. The first user is a company or institution holding custody-stable physical reserves, together with a bank or secured-credit provider willing to lend against those reserves under a pre-approved facility.

The first asset remains allocated physical gold, because it has unusually strong evidence primitives: bar lists, serial numbers, refiner identity, fineness, weight, vault custody, allocation records, audit reports, and custodian acknowledgements. The institutional infrastructure for these primitives already exists and is becoming more data-rich. The LBMA Good Delivery system defines refiner accreditation, bar specification, and serial-number traceability for the London market [19]. The market's own integrity infrastructure is digitising: as of the start of 2026, 100 percent of Good Delivery refiners submit data through LBMA's Gold Bar Integrity database, built on distributed ledger technology, with monthly country-of-origin reporting becoming mandatory for all refiners from 2027, custodians onboarded to report aggregated vault holdings by December 2026, and a stated intent to move custodians toward bar-level reporting [23]. The uncleared-margin framework accepts gold as eligible collateral [18], and COMEX gold warrants function today as electronic documents of title, pledgeable to CME Clearing with same-day settlement [10], [20]. Argent does not claim that all gold is bank collateral or central bank collateral; it uses gold as the cleanest first test case for custody-stable physical collateral, and the core stays asset-agnostic behind an adapter.

The first use cases are deliberately narrow:

1. **Reserve-backed working capital.** A company holds allocated gold and draws short-term liquidity without selling the gold.
2. **Supplier-payment liquidity.** A bank pays a supplier or processor against the collateralised facility, with proceeds bound to the permitted payment purpose.
3. **Bridge liquidity.** A company uses free collateral capacity for a short, time-bounded credit event.
4. **Margin or cure liquidity.** A pre-approved facility supports a margin requirement, collateral top-up, or temporary deficit cure.
5. **Collateral rotation.** A company substitutes physical collateral without closing the facility.

The first version is not a lending marketplace. The product wedge is a bank-readable collateral pool with self-triggering, policy-bound credit events.

---

## 6. Core objects

The layer adds a small set of objects on top of the existing lifecycle and the roadmap's pool model. It deliberately introduces no parallel vocabulary: the pool object is the roadmap's `CollateralPool`, positions extend the existing registered-position types, and the substitution machinery extends the existing `CollateralAdjustment` flow. All Rust below is illustrative, in the roadmap's convention of real signatures that are explicitly not committed for the current build. Time fields use unix seconds (`u64`), matching the existing contract convention (`priced_at`, `max_age_secs` in `revalue_and_check`).

### 6.1 Pool and position states

The pool gains an explicit lifecycle status, and it remains what the architecture documents already establish: a control and evidence mirror, never a custody account. The custodian's books stay authoritative for physical custody.

```rust
#[contracttype]
pub enum CollateralPoolStatus {
    PendingSetup,
    Active,
    Suspended,
    Frozen,
    Closed,
}
```

Positions inside a pool gain a state machine. Exclusivity is the invariant: a position is in exactly one state, and the same lot can never secure two credit events. The first version forbids any pooled pari passu treatment.

```rust
#[contracttype]
pub struct PoolPosition {
    pub position_id: BytesN<32>,
    pub pool_id: BytesN<32>,
    pub asset_adapter: Symbol,           // "gold_allocated" first; core stays asset-agnostic
    pub asset_identity_hash: BytesN<32>,
    pub uniqueness_hash: BytesN<32>,
    pub custody_evidence_hash: BytesN<32>,
    pub quantity_e7: i128,
    pub valuation_ref: BytesN<32>,
    pub last_valued_at: u64,
    pub reserved_for: BytesN<32>,        // credit event or zero
    pub pledged_to: BytesN<32>,          // credit event or zero
    pub state: CollateralPositionState,
}

#[contracttype]
pub enum CollateralPositionState {
    Free,                 // attested, in custody, not designated for automation
    Earmarked,            // owner-designated and custodian-confirmed for automatic mobilisation
    Reserved,             // selected for a pending credit event, not yet locked
    Pledged,              // locked to an open credit event
    PendingRelease,       // release authorized, custodian confirmation outstanding
    PendingSubstitution,  // replacement in progress, no unsecured gap permitted
    Frozen,               // administratively blocked (dispute, sanction, documentation)
    EnforcementLocked,    // bound to a recorded enforcement path
}
```

### 6.2 `EarmarkRecord`: the unit of owner consent

Earmarking is the on-stock concept from T2SAC applied to vault holdings, and it is the layer's most important governance object. An earmark is a standing, signed, revocable authorization by the owner, confirmed by the custodian: this lot may be pledged automatically under policy P, for facilities of bank B. A lot that is not earmarked can never be auto-pledged, regardless of policy.

```rust
#[contracttype]
pub struct EarmarkRecord {
    pub earmark_id: BytesN<32>,
    pub pool_id: BytesN<32>,
    pub position_id: BytesN<32>,
    pub owner: Address,
    pub custodian: Address,
    pub policy_id: BytesN<32>,
    pub earmark_evidence_hash: BytesN<32>,
    pub confirmed_at: u64,
    pub status: EarmarkStatus,
}

#[contracttype]
pub enum EarmarkStatus {
    PendingCustodianConfirmation,
    Active,
    Suspended,
    Revoked,
}
```

Revocation is subject to no-unsecured-gap ordering: an earmark on a currently pledged lot revokes forward, blocking new mobilisations, without touching the open event.

Earmarking changes the commercial meaning of a gold reserve. A non-earmarked reserve is a just-in-case asset. An earmarked reserve is standing liquidity capacity under policy. The owner still owns it. The custodian still holds it. Argent records the standing authority to mobilise its control state.

### 6.3 `AutoCreditPolicy`: the static-data layer

The policy is the signed act that all later automation executes under. Without it, the system remains manual. The bank owns the policy; Argent enforces it; Argent must never become the bank's risk model.

```rust
#[contracttype]
pub struct AutoCreditPolicy {
    pub policy_id: BytesN<32>,
    pub pool_id: BytesN<32>,             // roadmap CollateralPool
    pub bank: Address,
    pub trigger_party: Address,          // role wallet allowed to submit intents
    pub policy_doc_hash: BytesN<32>,     // the signed legal policy this encodes
    pub eligible_schedule_hash: BytesN<32>,
    pub valuation_policy_hash: BytesN<32>,
    pub haircut_bps: u32,
    pub max_ltv_bps: u32,
    pub maintenance_bps: u32,
    pub max_auto_credit_e7: i128,        // standing automation limit within the line
    pub max_event_tenor_secs: u64,
    pub cure_period_secs: u64,
    pub permitted_purpose_hash: BytesN<32>,
    pub approval_policy_hash: BytesN<32>,
    pub escalation_policy_hash: BytesN<32>,
    pub relocation_pool_id: BytesN<32>,  // escalation target
    pub relocation_penalty_e7: i128,
    pub version: u32,
    pub active: bool,
}
```

The policy encodes at minimum: asset eligibility, accepted custodians, valuation source and freshness rule, haircut, maximum LTV, maintenance threshold, maximum automatic credit amount, permitted use of proceeds, required signer set, repayment window, cure window, the escalation path including relocation target and penalty, and suspension and kill-switch conditions. Each policy version is frozen at credit-event opening: every event records the exact version it executed under.

### 6.4 `LiquidityIntent`: the trigger

Argent does not detect liquidity shortfalls, just as it does not custody gold or originate credit. Detection belongs to the bank's or owner's systems, exactly as shortfall detection in T2SAC belongs to the T2S provision check [1]. What Argent adds is a governed trigger interface: a signed intent referencing a pool and policy, validated before anything is mobilised.

```rust
#[contracttype]
pub struct LiquidityIntent {
    pub intent_id: BytesN<32>,
    pub pool_id: BytesN<32>,
    pub policy_id: BytesN<32>,
    pub requested_by: Address,
    pub amount_e7: i128,
    pub currency: Symbol,
    pub purpose_hash: BytesN<32>,
    pub payment_instruction_hash: BytesN<32>,
    pub required_by: u64,
    pub trigger_ref: BytesN<32>,    // hash of the external event that caused it
    pub status: LiquidityIntentStatus,
}

#[contracttype]
pub enum LiquidityIntentStatus {
    Submitted,
    PolicyChecked,
    CollateralReserved,
    ConvertedToCreditEvent,
    Rejected,
    Expired,
}
```

The trigger can come from a borrower request, a bank operator, a payment workflow, a margin need (Section 11), or another integrated system. The first implementation accepts only explicit, signed intents from the policy's trigger role. External system triggers connect later, through the same interface, never around it.

### 6.5 `CreditEvent`: the time-bounded credit act

```rust
#[contracttype]
pub struct CreditEvent {
    pub credit_event_id: BytesN<32>,
    pub pool_id: BytesN<32>,
    pub policy_id: BytesN<32>,
    pub policy_version: u32,
    pub intent_id: BytesN<32>,
    pub amount_e7: i128,
    pub currency: Symbol,
    pub reserved_collateral_value_e7: i128,
    pub borrowing_base_e7: i128,
    pub opened_at: u64,
    pub repay_by: u64,
    pub cure_by: u64,
    pub settlement_instruction_hash: BytesN<32>,
    pub evidence_pack_hash: BytesN<32>,
    pub state: CreditEventState,
}

#[contracttype]
pub enum CreditEventState {
    OpenRequested,
    PendingApproval,
    CollateralLocked,
    Funded,
    PartiallyRepaid,
    Repaid,
    CureOpen,
    Relocated,             // escalated to the general pool with recorded penalty
    Suspended,
    DefaultNoticeIssued,
    EnforcementReady,
    EnforcementRecorded,
    Cancelled,
}
```

A credit event can be funded through whatever rail the bank approves: off-chain payment with signed confirmation, a bank ledger instruction, or the existing Stellar settlement asset path. Argent binds the credit event to the collateral and evidence path. It does not pretend to control bank cash unless the payment rail is actually integrated (Section 12).

---

## 7. Core flow

### 7.1 Setup

1. Bank, owner, and custodian register the pool (roadmap `CollateralPool`) and its framework documents.
2. The bank registers an `AutoCreditPolicy` for the pool, signed under DFNS role governance.
3. The custodian registers and confirms positions with evidence references, as today.
4. The bank admits positions as eligible under the policy, the pool-scale successor of the existing `admit_instrument`.
5. The owner earmarks positions; the custodian confirms each earmark.
6. Argent computes the standing view: total attested, eligible, haircut-adjusted, free, earmarked, reserved, pledged, drawn, and available borrowing base.

### 7.2 Trigger and validation

A `LiquidityIntent` is validated in a fixed sequence that mirrors T2SAC's pre-execution checks [1]:

1. The submitting wallet holds the trigger role for this policy. Deny by default.
2. The pool, policy, and facility are active and unsuspended.
3. The requested purpose matches the permitted-purpose policy.
4. Amount, tenor, and currency are within policy, and headroom exists under the standing limit: the existing `available_capacity` and `borrowing_base` checks.
5. Valuation is fresh and within confidence tolerance: the same discipline `revalue_and_check` already enforces.
6. Earmarked, eligible collateral exists and is not excluded by close-link rules, meaning the owner is not affiliated with the custodian for the lot, and the lot is not referenced by another framework.

If any check fails, nothing is mobilised and the rejection is recorded with its reason. Failed checks mean no operation, silently and safely, which is T2SAC's behaviour too [1].

### 7.3 Collateral selection

1. Argent enumerates positions in state `Earmarked` under the policy.
2. It excludes frozen, reserved, pledged, stale-valued, ineligible, or disputed positions.
3. It applies haircuts and computes coverage.
4. It selects the smallest sufficient collateral set, or the bank-configured strategy, so that no more than the necessary collateral is used, subject to eligibility, minimum-lot constraints, and any concentration limits per custodian, vault, refiner, or jurisdiction. This generalises the existing `select_lot_for_collateral` from a manual choice into a constrained selection, the same problem class the T2S settlement engine runs at scale [14], [15].
5. It reserves the selected positions and emits a selection event recording the inputs, exclusions, and selected collateral, so the selection is itself auditable evidence, not a black box.

The first selector is deterministic and explainable. A richer optimiser, and formal concentration limits, come later and change nothing about the interface. The commercial value in version one is trustable output, not mathematical sophistication.

### 7.4 Approval and opening

1. Argent routes the event to the required role approvals under the policy's approval set.
2. The DFNS policy decoder checks contract, method, role, pool, policy, event, amount, collateral references, and evidence hash before any signature (Section 13).
3. The event becomes valid only when the required approvals are present.
4. Reserved collateral moves to `Pledged`, atomically with the opening, so no unsecured instant exists.
5. The repayment and cure timers start, and the reimbursement obligation is created together with the event, exactly as every T2SAC operation is born with its pending reverse instruction [2].
6. The funding or payment instruction is bound to the event by hash.

### 7.5 Repayment and substitution

1. Repayment before the deadline, through the existing settlement path (`settle_repayment` reducing exposure via `apply_repayment`), closes or reduces the event. Release of collateral back to `Free` remains the existing two-step act: `bank_authorize_release`, then `custodian_confirm_release`. Repayment is not release, under automation as today.
2. Partial repayment recalculates exposure, LTV, and coverage.
3. If an earmarked, currently pledged lot is needed elsewhere, automatic substitution executes under standing policy: replacement collateral of sufficient haircut-adjusted value is locked before the outgoing lot is released, in one atomic sequence (Section 10).
4. If the event is not repaid by the deadline, the graduated escalation path runs (Section 8).

---

## 8. Graduated escalation

The traditional secured-lending failure mode is binary: repay or lose the asset. T2SAC fails differently: reimbursement is attempted, partial reimbursement is possible, unreimbursed credit is converted into another fully collateralised form, and only then do penalties and further consequences apply [3], [13]. Argent translates that into a private secured-credit ladder:

1. **Rebalance.** Attempt reimbursement or reduction from the facility's available settlement balance or a bank-confirmed repayment record.
2. **Cure.** Open a cure window if repayment is late or margin coverage is deficient.
3. **Relocate.** Convert the mobilisation from event-specific reservation to the bank's general collateral pool (the policy's `relocation_pool_id`), recorded as a relocation event. The credit changes character; collateral coverage never lapses.
4. **Penalise.** Record the pre-agreed late or relocation penalty, where legally permitted and configured in the facility documents, with a defined appeal window, following the graduated discipline structure used by CSD settlement-discipline regimes rather than an ad hoc sanction.
5. **Suspend.** Stop further auto-credit events through `bank_suspend_line` or pool-level suspension.
6. **Default notice.** Issue `issue_default_notice` only after the policy-defined conditions are met.
7. **Enforcement readiness.** Build and record the evidence pack required for off-chain legal and custody enforcement, through the existing enforcement-readiness machinery.
8. **Enforcement recording.** Record the outcome via `record_enforcement`. Argent never performs legal enforcement on-chain.

The design intent is the Eurosystem's: relocation should be rare, priced, and boring [3]. Default remains what it is in Argent today, a governed, evidenced, terminal process, but it stops being the first thing after a missed timer.

---

## 9. Invariants

The layer is useful only if the invariants are simple and hard.

1. **No unsecured opening.** A credit event cannot open unless sufficient eligible collateral is reserved and locked under the bank's policy, atomically with the opening.
2. **No mobilisation without consent.** Only earmarked lots can be auto-pledged. No earmark, no automation, regardless of policy.
3. **No double pledge.** A position cannot secure two credit events. The existing uniqueness lock and duplicate-pledge refusal hold unchanged.
4. **No stale valuation.** A credit event cannot open on a stale, low-confidence, or unsupported valuation.
5. **No release before replacement.** In substitution, incoming collateral must be locked and coverage satisfied before outgoing collateral can be released.
6. **No silent purpose drift.** The credit event stays bound to its liquidity intent and permitted-purpose evidence.
7. **No hidden policy.** Every automated event records the hash and version of the policy it executed under, so the evidence trail answers not only what happened but under whose standing authority.
8. **No unbounded time.** Every credit event carries repayment, cure, and escalation timers.
9. **No operator override.** The operator role cannot sign for bank, custodian, owner, verifier, or sponsor roles. Deny by default survives automation.
10. **No automated consent.** Automation never widens authority. Consent is given once, explicitly, in a signed policy with hard limits; execution then happens at machine speed inside those limits (Section 13).
11. **Repayment is not release.** `apply_repayment` reduces exposure only; release remains the two-step, role-signed act.
12. **No unrecorded enforcement path.** Default and enforcement evidence are recorded as events, never reconstructed later from emails.
13. **No claim of physical truth.** Argent records signed custody and evidence claims. It does not physically verify the asset.

---

## 10. Safe substitution

Substitution is the feature that makes physical collateral operationally usable, and it is where a pledge database and a collateral-mobility product part ways. The rule is strict:

> The replacement collateral must be admitted, valued, attested, and locked before the replaced collateral can be released.

This hardens the existing `CollateralAdjustment` machinery (`AdjustmentType::Substitution`, with its `Requested`, `CustodianConfirmed`, `Approved` flow) into a lock-before-release ordering, executable under a standing policy approval instead of a per-event manual round trip. Illustrative surface:

```rust
pub fn request_substitution(
    env: Env,
    credit_event_id: BytesN<32>,
    outgoing_position_ids: Vec<BytesN<32>>,
    incoming_position_ids: Vec<BytesN<32>>,
    request_hash: BytesN<32>,
) -> Result<BytesN<32>, Error>;

pub fn custodian_attest_incoming(env: Env, substitution_id: BytesN<32>, custody_evidence_hash: BytesN<32>) -> Result<(), Error>;
pub fn bank_approve_substitution(env: Env, substitution_id: BytesN<32>, valuation_ref: BytesN<32>) -> Result<(), Error>;
pub fn lock_incoming_collateral(env: Env, substitution_id: BytesN<32>) -> Result<(), Error>;
pub fn release_outgoing_collateral(env: Env, substitution_id: BytesN<32>) -> Result<(), Error>;
```

`release_outgoing_collateral` MUST revert unless `lock_incoming_collateral` has succeeded and post-substitution coverage satisfies the policy. Automatic substitution under standing policy uses the same path with the standing approval substituted for the per-event one; the ordering invariant is identical.

---

## 11. Scheduled revaluation and margin automation

Pool-scale scheduled revaluation is roadmap scope; this layer does not redefine it. What this layer adds is the consumption side: margin events become a trigger source for the automation, closing the loop between valuation discipline and liquidity response.

The contract already validates price freshness and confidence and evaluates `MarginState` (`Covered`, `Warning`, `Called`) in `revalue_and_check`. The layer extends this in two directions:

- **Shock-triggered revaluation.** A price move beyond a policy threshold requires revaluation before any new credit event can open, in addition to the scheduled cycle.
- **Margin calls as liquidity intents.** A margin call can generate a `LiquidityIntent` with a margin-cure purpose (use case 4 in Section 5), so that a coverage deficit is answered by the same governed path as any other permitted liquidity need: policy check, selection from earmarked stock, top-up or credit event, timers, escalation.

Illustrative surface:

```rust
pub fn revalue_pool(env: Env, pool_id: BytesN<32>, price_set_hash: BytesN<32>, valuation_time: u64) -> Result<PoolValuationResult, Error>;
pub fn issue_margin_call(env: Env, credit_event_id: BytesN<32>, deficit_e7: i128, valuation_ref: BytesN<32>) -> Result<BytesN<32>, Error>;
pub fn record_margin_cure(env: Env, margin_call_id: BytesN<32>, cure_evidence_hash: BytesN<32>) -> Result<(), Error>;
pub fn escalate_margin_call(env: Env, margin_call_id: BytesN<32>) -> Result<(), Error>;
```

Argent does not compute the bank's haircut model. It enforces the bank's approved haircut, LTV, maintenance, freshness, and confidence rules, and it records every margin state transition as evidence.

---

## 12. Credit-event execution adapters

The layer needs a clean boundary between the control event and the cash or payment rail. Argent's job is to bind the credit event to the collateral and evidence path; the funding leg belongs to whatever rail the bank approves.

```rust
#[contracttype]
pub struct ExecutionAdapterRef {
    pub adapter_id: BytesN<32>,
    pub adapter_type: Symbol,
    pub institution: Address,
    pub instruction_hash: BytesN<32>,
    pub confirmation_hash: BytesN<32>,
    pub status: ExecutionAdapterStatus,   // PendingInstruction, InstructionSent, Confirmed, Failed, Cancelled
}
```

Adapters, in order of realism:

1. **Manual bank confirmation.** The bank signs that funding occurred off-chain. First, and always available.
2. **Stellar settlement asset.** The existing `settlement_vault` path is the reference on-chain adapter: a configured SAC / SEP-41 settlement asset moves and exposure adjusts in the same transaction path.
3. **Bank ledger API.** The bank or secured-credit provider confirms disbursement through an integration, built only when a design partner defines the exact rail.
4. **Supplier-payment adapter.** Funds go to a named beneficiary rather than the borrower, binding the payment to the permitted purpose.
5. **Tokenized cash or deposit token.** A future institutional rail, if available and approved.

The production-safe first version supports adapters 1 and 2. Deeper bank integration is deferred until a design partner exists; the interface is what ships.

---

## 13. Governance: automation of execution, never automation of consent

The central design rule of this layer, and the reason it does not weaken Argent's security model, is that no automated act ever exceeds authority granted in a signed, human-approved policy. T2SAC works this way: every automatic operation is the mechanical consequence of static data that central banks, CSDs, and counterparties configured and signed up to in advance [1]. The Argent translation:

- **The policy is the signed act.** The `AutoCreditPolicy` is registered per facility and signed by the bank, owner, and custodian role wallets under DFNS governance, exactly like a framework registration today.
- **Every automated event carries the policy reference.** Mobilisation, substitution, reimbursement, relocation, and penalty events each record the policy hash and version they executed under.
- **Deny by default survives.** No policy, no automation. No earmark, no auto-pledge. No trigger role, fail closed.
- **The kill switch already exists.** `bank_suspend_line` halts automated mobilisation on a facility immediately; pool-level suspension halts it across the pool; earmarks are owner-revocable forward.
- **Roles do not blur.** The bank owns credit risk and margin policy. The custodian confirms custody-state transitions, including automated substitutions, through its own role wallet, which may itself operate under a standing DFNS policy with defined limits. The owner owns the asset. Argent owns none of these things; it records and governs the sequence.

Each event class maps to a required authority, and the DFNS policy decoder must verify the institutional act before any signature:

| Event | Required authority |
|---|---|
| Register policy | Bank policy authority, with owner and custodian counter-signature |
| Register collateral position | Custodian custody authority |
| Admit collateral under policy | Bank collateral authority |
| Earmark position | Owner, with custodian confirmation |
| Submit liquidity intent | Policy trigger role |
| Reserve collateral | Protocol under policy, recorded with selection evidence |
| Approve credit event | Bank credit authority per approval policy |
| Confirm collateral lock | Custodian control authority |
| Confirm funding | Bank funding authority or execution adapter |
| Record repayment | Bank or settlement adapter |
| Approve substitution | Bank and custodian authorities, or standing policy within limits |
| Release collateral | Bank release authority, then custodian confirmation |
| Relocate and penalise | Protocol under policy, evidence recorded |
| Suspend pool or line | Bank authority |
| Issue default notice | Bank enforcement authority |
| Record enforcement | Bank and custodian, with evidence |

The decoder must resolve at least: contract address, method, role, pool ID, policy ID and version, credit event ID, amount, currency, collateral references, evidence pack hash, current state, and target state. A signature must never approve an opaque payload; the signer must know which institutional act is being authorised.

---

## 14. Evidence read-model

The layer produces two bank-readable artifacts, and they are the commercial product. The bank does not buy blockchain evidence in the abstract. It buys a defensible answer to one question: what is lendable now, under our policy, with proof we can show risk, audit, and operations?

### 14.1 Collateral pool report

Minimum fields: pool ID, owner, bank, custodian, policy ID and version, total custody-attested value, eligible value, haircut-adjusted value, free value, earmarked value, reserved value, pledged value, drawn exposure, available borrowing base, margin status, open credit events, pending substitutions, pending releases, stale valuations, relocation and penalty history, default or enforcement states, evidence pack hash, last reconciliation time.

This report is what turns a gold reserve into what a bank recognises: not gold pledged, but advanceable reserve capacity.

### 14.2 Credit-event certificate

Minimum fields: credit event ID, liquidity intent ID and trigger reference, policy ID and version, requested and approved amount, collateral selected with selection evidence, borrowing base and LTV at opening, maintenance threshold, required signers and approval timestamps, settlement instruction hash, funding confirmation hash, repayment and cure deadlines, current state, relocation and penalty records if any, evidence pack hash, Stellar transaction hashes, DFNS approval references.

Both artifacts should be exportable as JSON, PDF, and API response: simple enough for a credit officer, precise enough for operations, replayable enough for audit. They extend the existing evidence-certificate flow; they do not replace it.

---

## 15. What already exists to build on

As with the roadmap, this layer extends shipped types and functions; it does not replace them.

| Existing element | Role in the auto-collateralisation layer |
|---|---|
| `register_framework`, document hash fields | Registration surface for the `AutoCreditPolicy` and its schedule, margin, purpose, and penalty parameters |
| `register_position`, `confirm_and_immobilize` | The attested, custody-confirmed lots that earmarking designates |
| `admit_instrument` | Predecessor of bank admission of positions under a policy |
| `select_lot_for_collateral` | Manual predecessor of automated smallest-surplus selection |
| `available_capacity`, `borrowing_base`, `ltv_bps`, `maintenance_bps` | The headroom and coverage checks run on every trigger, Argent's Credit Memorandum Balance analog |
| `revalue_and_check`, `MarginState` (`Covered`, `Warning`, `Called`), freshness and confidence validation | The valuation discipline every automated mobilisation must satisfy, and the source of margin-driven triggers |
| `record_drawdown`, `apply_repayment`, `settle_repayment` | The exposure and reimbursement legs of each time-bounded event |
| `settlement_vault` | The reference on-chain execution adapter |
| `CollateralAdjustment`, `AdjustmentType::{TopUp, Substitution, PartialRelease}` | The substitution machinery, hardened to lock-before-release and executable under standing policy |
| `bank_authorize_release`, `custodian_confirm_release` | Unchanged: final release back to free stock remains a two-step, role-signed act |
| `bank_suspend_line`, `bank_resume_line` | The automation kill switch and restart |
| `issue_default_notice`, `cure_default`, `record_enforcement`, enforcement readiness | The terminal path that graduated escalation feeds into, unchanged |
| Roadmap `CollateralPool` | The pool object this layer's policy binds to, and the relocation target in escalation |
| Evidence certificate flow | Extended by the credit-event certificate and pool report |

The point is not to restart architecture. The point is to move from manually initiated lifecycle events to policy-triggered lifecycle events while keeping the same role separation and evidence guarantees.

---

## 16. Implementation sequence

This layer starts only after the roadmap's own gate conditions: the DFNS-governed mainnet reference implementation is live and the pool and position model exists. Within the layer, phases are ordered by dependency, each shippable and reviewable on its own.

**Phase 1: Documentation and modelling.** This document in the repo; a sequence diagram for credit-event opening; the event taxonomy (`LiquidityIntentSubmitted`, `CollateralReserved`, `CreditEventOpened`, `CreditEventFunded`, `CreditEventRepaid`, `SubstitutionExecuted`, `CureOpened`, `Relocated`, `PenaltyRecorded`, `DefaultNoticeIssued`, `EnforcementRecorded`); a test matrix before any code.

**Phase 2: Read-model first.** Build the collateral pool report from existing position, pledge, valuation, margin, and repayment data. Show free, earmarked, reserved, pledged, drawn, and available views. No credit automation before the bank-readable view is correct: the read-model is the product a pilot bank evaluates, and it requires no new contract risk.

**Phase 3: Policy and earmark.** `AutoCreditPolicy` as a versioned, hash-anchored object; the earmark lifecycle with custodian confirmation; risk modelling kept off-chain and bank-owned. After this phase Argent already offers something no gold holder has today: a custodian-confirmed, bank-recognised, revocable standing designation of reserve collateral, before any automation runs.

**Phase 4: Liquidity intent.** Signed intents, the validation sequence, the policy-bound trigger role, rejection reasons.

**Phase 5: Reservation and selection.** Deterministic selection with recorded selection evidence, reservation state, double-use prevention proofs.

**Phase 6: Credit event opening.** The `CreditEvent` state machine, atomic lock-with-open, DFNS approval routing, timers, and the credit-event certificate.

**Phase 7: Repayment, cure, and escalation.** Repayment and partial repayment, cure state, and the rebalance, cure, relocate, penalise, suspend ladder with the penalty record and appeal window.

**Phase 8: Substitution hardening.** Lock-before-release, tests proving no unsecured gap is reachable, substitution evidence certificate, then standing-policy execution.

**Phase 9: Execution adapters.** Manual bank confirmation first; Stellar settlement confirmation where a permitted settlement asset is used; deeper bank integration only when a design partner defines the rail.

---

## 17. Test surface

Following the repository's existing test-surface convention, the suite must prove at least:

**Policy.** Non-bank authority cannot register; invalid LTV or maintenance thresholds refused; inactive policy rejects intents; suspended policy rejects new credit events; policy version frozen per event; maximum automatic amount enforced; unsupported purpose rejected.

**Earmark.** Owner can request an earmark only over its own position; custodian confirmation required before an earmark is selectable; frozen positions cannot be earmarked; revoked earmarks are never selectable; revocation cannot create an unsecured event; revocation blocks new mobilisation without touching open events.

**Valuation.** Stale valuation rejects opening; low-confidence valuation rejects opening; haircut applied before borrowing base; maintenance breach opens warning or call; price shock triggers revaluation requirement.

**Reservation and selection.** Pledged, frozen, and disputed collateral never selectable; insufficient free collateral rejects the intent; selector minimises over-collateralisation under the configured strategy; reservation prevents double use; selection evidence recorded.

**Credit event.** No opening without required approvals; no opening without collateral lock; no opening above borrowing base; funding confirmation must match amount and reference; repayment closes or reduces exposure; repayment does not release collateral; missed repayment opens cure; missed cure relocates with recorded penalty; only post-relocation failure escalates to default; enforcement recording requires evidence.

**Substitution.** Outgoing collateral cannot be released before incoming is locked; incoming must be eligible and satisfy post-substitution coverage; both legs evidenced; failed substitution leaves original collateral locked.

**DFNS decoder.** Method, role, pool, policy, event, amount, currency, and evidence hash decoded correctly; opaque or mismatched payload rejected; revoked signer rejected; operator cannot sign institutional roles.

---

## 18. What remains explicitly out of scope

The following are not part of this layer unless separately approved and designed with a regulated partner:

- originating loans, setting credit policy, or pricing credit risk;
- custodying physical assets or validating physical truth directly;
- issuing tokenized gold or any transferable claim on the metal;
- rehypothecating collateral;
- becoming a CSD, payment system, lender, custodian, broker, exchange, or clearing house;
- claiming Eurosystem collateral eligibility for physical assets;
- direct card-network integration;
- general-purpose retail lending;
- automated legal enforcement;
- bank-core integration without a design partner.

---

## 19. Positioning

Stated in one paragraph:

> Argent is a governed auto-collateralisation layer for physical reserves held in professional custody. The current build proves role-signed collateral control. The next layer makes credit events self-triggering under bank-defined policy: eligibility, valuation, earmarking, collateral selection, reservation, approval, drawdown evidence, repayment, cure, substitution, and escalation.

Stated in one sentence:

> Argent turns custody-stable physical reserves into bank-readable, policy-bound collateral capacity.

Claims this document deliberately does not make: that Argent is tokenized gold; that Argent makes gold central bank collateral; that Argent automates legal enforcement; that Argent lends.

---

## References

Independent sources, cited to evidence the reference design and market direction described above. No partnership or endorsement by any named organization is implied.

[1] European Central Bank, "T2S auto-collateralisation: benefits, conditions and functioning," November 2025. https://www.ecb.europa.eu/pub/pdf/other/ecb.t2sautocollateralisation.202511.en.pdf

[2] European Central Bank, "T2S auto-collateralisation using cost-free intraday credit in central bank money" (brochure), December 2025. https://www.ecb.europa.eu/paym/target/target-professional-use-documents-links/t2s/shared/pdf/2025-12_T2S_Auto-collateralisation_Brochure.en.pdf

[3] De Nederlandsche Bank, "T2S auto-collateralisation (T2SAC)," onepager. https://www.dnb.nl/media/l2lp4jj5/78919-2400257_dnb-onepager-t2s-auto-collateralisation_tg_pdfa.pdf

[4] European Central Bank, "ECB paves way for acceptance of DLT-based assets as eligible Eurosystem collateral," press release, 27 January 2026. https://www.ecb.europa.eu/press/pr/date/2026/html/ecb.pr260127_1~a946167ce1.en.html

[5] Guideline (EU) 2024/3129 of the European Central Bank of 13 August 2024 on the management of collateral in Eurosystem credit operations, Official Journal of the European Union. https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=OJ:L_202403129

[6] DTCC, "DTC's Tokenization Service to Connect with Stellar Public Blockchain as DTC Advances its Multi-Chain Strategy," 27 May 2026. https://www.dtcc.com/news/2026/may/27/tokenization-service-to-connect-with-stellar-public-blockchain-as-dtc-advances-multi-chain-strategy

[7] BIS Working Papers No 1089, "Intraday liquidity around the world," Bank for International Settlements, 2023. https://www.bis.org/publ/work1089.pdf

[8] Broadridge Financial Solutions, "Broadridge's Distributed Ledger Repo Platform Achieves 508% Year Over Year Growth in January," press release, February 2026. https://www.broadridge.com/press-release/2026/broadridges-dlr-platform-achieves-508-percent-year-over-year-growth-in-january

[9] Norton Rose Fulbright, "The Basel Framework and regulatory status of gold: clarifying the status quo," 2025. https://www.nortonrosefulbright.com/en-us/knowledge/publications/ca01cac9/the-basel-framework-and-regulatory-status-of-gold

[10] CME Group, "Acceptable Collateral," CME Clearing. https://www.cmegroup.com/solutions/clearing/financial-and-collateral-management/acceptable-collateral.html

[11] LBMA, "Gold and HQLA: Correcting Misleading Online Information." https://www.lbma.org.uk/articles/gold-and-hqla-correcting-misleading-online-information

[12] World Gold Council, "Basel III and the Gold Market," June 2021. https://www.gold.org/goldhub/gold-focus/2021/06/basel-iii-and-gold-market

[13] European Central Bank, "Information Guide for TARGET participants, Part 4: T2S Cash." https://www.ecb.europa.eu/paym/target/consolidation/profuse/shared/pdf/Part4_T2S_Cash.en.pdf

[14] 4CB, "T2S NTS Algorithms Objectives," version 1.1, 2018. https://www.ecb.europa.eu/paym/target/target-professional-use-documents-links/t2s/shared/pdf/T2S_NTS_algorithms_objectives_V1.1.pdf

[15] L. Braine, D. J. Egger, J. Glick, and S. Woerner, "Quantum Algorithms for Mixed Binary Optimization Applied to Transaction Settlement," IEEE Transactions on Quantum Engineering, vol. 2, 2021.

[16] European Central Bank, "TARGET Services pricing guide," version 3.0, July 2026. https://www.ecb.europa.eu/paym/target/

[17] European Central Bank, "Collateral management" (Eurosystem collateral framework overview). https://www.ecb.europa.eu/mopo/coll/coll/html/index.en.html

[18] BCBS and IOSCO, "Margin requirements for non-centrally cleared derivatives," July 2019. https://www.iosco.org/library/pubdocs/pdf/IOSCOPD635.pdf

[19] LBMA, "Good Delivery Current List: Gold." https://www.lbma.org.uk/good-delivery/gold-current-list

[20] CME Group, "COMEX Gold Warrants," information sheet. https://www.cmegroup.com/trading/metals/files/comex-gold-warrants-info-sheet.pdf

[21] European Banking Authority, "2017_3649 Credit Risk on Gold Bullion," Single Rulebook Q&A, 21 December 2018. https://www.eba.europa.eu/single-rule-book-qa/qna/view/publicId/2017_3649

[22] Basel Committee on Banking Supervision, "Basel III: The Net Stable Funding Ratio," October 2014. https://www.bis.org/bcbs/publ/d295.pdf

[23] LBMA, "Gold Bar Integrity Ecosystem." https://www.lbma.org.uk/gold-bar-integrity-ecosystem

[24] European Central Bank, "TARGET Services Annual Report 2025," 2026. https://www.ecb.europa.eu/press/targetservar/pdf/ecb.targetservar2025.en.pdf

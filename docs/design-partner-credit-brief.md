# Design-Partner Credit Brief

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**Illustrative product:** revolving working-capital line secured by allocated gold  
**Purpose:** two-page discussion brief for one borrower, one custodian, and one lender  
**Companion:** `docs/why-gold-secured-operational-credit.md`  

> **Discussion draft only.** This is not a term sheet, valuation, legal opinion, credit offer, or commitment to lend. All terms remain subject to credit, legal, custody, regulatory, accounting, prudential, and operational approval by the participating institutions.

> **Implementation status.** Section 3 distinguishes controls the contract **enforces today** from **candidate controls that are not implemented**. The illustrative thresholds in this brief (advance rate, suspension band, cure period, enforcement trigger) are **placeholders**, not commitments — no lender has stated them and the contract does not enforce them. Every claim about the contract in this document can be checked against the open-source core at `contracts/credit_ledger/`. See `credit-control-extension-points.md` for what each candidate would take to build.

---

## 1. Pilot proposition

A design-partner borrower uses an existing pool of **eight allocated one-kilogram gold bars** to support a revolving business credit line. The bars remain with an approved independent custodian. The borrower retains allocated title during the normal life of the facility, subject to the agreed security interest. The lender sets credit policy and controls draw, release, margin response, and enforcement. Argent maintains the shared collateral-control state and produces an ordered evidence record across the lifecycle.

The pilot tests one proposition:

> **Can a physical bar list be operated as a continuously controlled borrowing base, without selling or tokenising the gold?**

The intended use is temporary liquidity against a durable reserve asset. The borrower must have a credible primary repayment source; enforcement against the gold remains the lender’s secondary recovery route.

---

## 2. Illustrative borrower and facility

| Item | Pilot assumption |
|---|---|
| Borrower | Established precious-metals business, jewellery manufacturer, commodity trader, family office, or holding company with professionally custodied allocated gold |
| Use of proceeds | Recurring 30–90 day working-capital gaps, supplier settlement, inventory acquisition, trade settlement, or another approved short-term operating need |
| Primary repayment | Operating receipts, customer collections, inventory sale proceeds, refinancing, or another lender-approved cash-flow source |
| Collateral pool | 8 × 1 kg allocated kilobars; total fine weight 8 kg / approximately 257.2 troy oz |
| Required bar evidence | Refiner, serial, format, fineness, fine weight, custody account, vault location, allocation status, and encumbrance status |
| Illustrative opening value | **USD 1,059,333** for the pool, used only as the scenario value in this brief |
| Initial advance rate | **60%** of eligible collateral value |
| Initial credit limit | **USD 635,600** |
| Product and tenor | 12-month revolving working-capital line, committed or cancellable as approved by the lender |
| Expected utilisation | 40–70% of the approved limit, with repeated draw and repayment cycles |
| Pricing | Interest on utilised principal; benchmark, margin, commitment fee, custody fee, and Argent fee to be agreed |

The market value is variable; the bar record is not. In production, the lender selects the approved gold-price source, valuation time, currency-conversion method, staleness threshold, haircut, concentration rules, and fallback procedure. The displayed credit limit is therefore a calculated facility state, not a permanent “one kilogram equals X dollars” promise.

```text
eligible collateral value
= eligible fine weight × approved price × approved FX rate
× eligibility and concentration adjustments

available amount
= min(facility limit, eligible collateral value × advance rate)
− utilised principal − lender reserves
```

The facility is not suitable where the borrower lacks a credible repayment source, intends to fund persistent losses, seeks speculative leverage, or cannot establish clean ownership and control over the bars.

---

## 3. Credit controls

**Read this section carefully. It mixes controls the contract enforces today with controls that are candidates for a pilot and are not implemented.** The distinction is stated explicitly, because a design partner is entitled to check every claim against the open-source core (`contracts/credit_ledger/`) and find it there.

### 3.1 Enforced by the contract today

These are invariants in code, not procedures. A transaction violating them is rejected.

| Control | Status |
|---|---|
| **Advance rate** (`ltv_bps`) | Bank-set per line. Enforced. |
| **Maintenance threshold** (`maintenance_bps`) | Bank-set per line. Enforced. |
| **Advance rate < maintenance threshold** | A line **cannot be opened** at or above its own margin-call level. Every facility starts with headroom by construction. |
| **Eligibility ceiling** (`max_ltv_bps`) | A line's advance rate cannot exceed the ceiling registered for that instrument. |
| **Haircut** (`haircut_bps`) | Applied to gross value before the advance rate. |
| **Utilisation ≤ borrowing base** | An over-draw is **rejected** (`InsufficientCapacity`). |
| **Two-band margin state** | `revalue_and_check` computes a warning band **below** the call level, yielding `Covered` / `Warning` / `Called`. The borrower's first notification should not be the call. |
| **Stale-price rejection / confidence tolerance** | A revaluation on a stale or low-confidence price fails. |
| **Line suspension** | **Two mechanisms.** *Automatic:* a margin call (`Called`) suspends the line via `revalue_and_check`, and a suspended line cannot draw. *Discretionary:* `bank_suspend_line` lets the bank suspend at any time. |
| **Cure** | `cure_default` exists **as a state**. There is **no deadline or expiry** on this path. |
| **Three-party adjustment sequence** | Owner requests → custodian confirms → bank approves. Enforced in that order. |
| **Release sequencing** | `custodian_confirm_release` **fails** unless the bank has already authorised. The custodian *cannot* release before authorisation. |

### 3.2 Candidate controls — illustrative, and NOT implemented

The numbers below are **placeholders**. No lender has stated them, no policy has been approved, and **the contract does not enforce them.** They are shown to indicate what a facility typically requires, and to be replaced by the design partner's actual credit policy.

| Candidate control | Illustrative placeholder | Status |
|---|---|---|
| Initial advance rate | 60% LTV | *Configurable today* — the bank sets `ltv_bps`. |
| Draw suspension band | No further utilisation at ~65% LTV | **Not implemented.** Suspension **on** a margin call is already automatic. What is missing is a *pre-call* band (stop lending before the call level) — a design decision for the lender. |
| Margin notice | At the maintenance threshold, or a lender-defined deficiency | *Partly today* — the `Warning` / `Called` states exist; the notice itself is the bank's act. |
| Cure period | ~Two business days, shorter for severe events | **Implemented as a ledger-sequence standstill.** `issue_default_notice` sets a cure deadline; `record_enforcement` fails before it lapses (`CurePeriodNotExpired`); a late cure is still accepted until enforcement is recorded. **Open:** mapping contractual *business days* to *ledger sequence*. |
| Permitted cure | Cash repayment, additional collateral, or substitution | *Mechanically supported* — repayment and adjustment both exist. |
| Enforcement trigger | ~72% LTV, uncured deficiency, payment default, title dispute, loss of custody control | **Not implemented as a threshold.** Default is declared by the bank (`issue_default_notice`), not fired by an LTV level. |
| Release condition | Post-release pool must remain within coverage | **Partly.** The three-party sequence is enforced; a coverage test on the *proposed resulting set* is a candidate. |

> **See `credit-control-extension-points.md`** for what each of these would take to build, which are configuration, which is a genuine capability gap, and which should **not** be built because they are determinations the bank makes rather than computations a contract can perform.

### 3.3 The principle

A margin event should first create a controlled state: draw suspended, release blocked, borrower notified, cure options opened, and the deadline recorded. It should not by itself transfer title. Enforcement begins only after the contractual trigger and the required authorisations are satisfied.

**The contract enforces the sequence and the arithmetic. The bank makes the determinations.** A control that requires judgement — sanctions clearance, title dispute, adequacy of a cure — is a control the bank *signs for*, and the signature is the evidence that it exercised that judgement. Building such a check into the contract would create the illusion of an enforced control while enforcing nothing but that someone set a flag.

### Whole-bar release and substitution

The collateral consists of discrete bars. A borrower cannot release 17% of a specific bar merely because an equivalent portion of the loan has been repaid.

A bar may be released or replaced only when:

1. the lender authorises the action;
2. the custodian confirms the bar remains allocated and immobilised before the change;
3. the remaining or replacement pool passes eligibility, concentration, and coverage tests;
4. no unresolved margin, default, sanctions, title, or evidence exception exists;
5. the change creates no temporary collateral shortfall;
6. the custodian confirms completion; and
7. Argent records the resulting bar list, borrowing base, and availability.

Replacement bars must be of equal or better eligibility under the lender’s policy. Partial repayment reduces utilisation immediately, but physical release occurs only on a whole-bar basis when the post-release coverage test passes.

---

## 4. Default and liquidation route

The parties should agree the exit route before the first draw.

```text
contractual default or uncured deficiency
→ lender declares enforcement state
→ custodian maintains the freeze and confirms controlled bars
→ final valuation and exposure statement
→ authorised enforcement instruction
→ approved bullion dealer or liquidation agent executes sale or transfer
→ cash proceeds settle to the controlled account
→ principal, interest, costs, and enforcement charges are paid
→ contractual surplus is returned to the borrower
→ facility and collateral records close with final evidence
```

The facility agreement, pledge or security agreement, custody acknowledgement, governing law, enforcement authority, physical sale, title transfer, and cash waterfall remain off-chain. Argent does not independently create the security interest. It ensures that the digital control state mirrors the authorised contractual state and that each material transition is attributable and reviewable.

---

## 5. Responsibilities

| Party | Core pilot responsibilities |
|---|---|
| **Borrower** | Prove ownership and source of gold; complete KYC/AML and credit review; execute facility and security documents; authorise the pledge; use proceeds for the approved purpose; make payments; respond to margin notices; request substitutions or releases |
| **Custodian** | Verify bar identity, fine weight, fineness, allocation, location, and custody account; acknowledge lender control; immobilise pledged bars; prevent unauthorised release or re-use; confirm substitutions, releases, and enforcement actions; maintain the authoritative physical record |
| **Lender** | Underwrite the borrower; set eligibility, valuation, advance-rate, concentration, margin, pricing, tenor, and enforcement policy; approve draws; suspend availability; issue release, cure, and enforcement decisions; maintain the regulated credit and cash records; appoint approved liquidation routes |
| **Argent** | Bind facility, participant, and bar identifiers; calculate and display the authorised borrowing base; enforce role permissions and multi-party workflow; prevent invalid draw and release transitions; record valuation references, approvals, utilisation, repayments, margin events, substitutions, releases, and enforcement states; produce tamper-evident evidence |

Valuation providers, bullion dealers, insurers, legal counsel, processors, and collateral agents remain external specialists. They may provide signed inputs or receive controlled instructions through agreed adapters.

---

## 6. Evidence and pilot success

### Evidence produced

**Onboarding:** approved facility and security-document references; participant authorities; eligible-bar schedule; custodian allocation and immobilisation acknowledgement; opening valuation and policy version; initial borrowing-base calculation.

**Operation:** valuation inputs and borrowing-base snapshots; draw approvals and utilisation; repayments; margin notices and cures; substitutions and whole-bar releases; rejected actions and policy reasons; bank–custodian–Argent reconciliation exceptions; complete ordered event history.

**Closure or enforcement:** final repayment and release evidence, or default declaration, freeze confirmation, valuation, enforcement instruction, dealer execution, proceeds statement, waterfall, surplus return, and final closure record.

### Success criteria

The pilot is successful when the partners can demonstrate:

1. legal documents and operational control states are aligned;
2. the opening, substituted, released, and closing bar lists reconcile with the custodian;
3. valuation and utilisation correctly change availability, margin, and release status;
4. unauthorised draw, release, substitution, and enforcement attempts are rejected;
5. at least one partial repayment and whole-bar release or substitution completes without a coverage gap;
6. a margin notice and cure are completed;
7. a tabletop enforcement test confirms the freeze, dealer, cash-settlement, and waterfall route;
8. lender and custodian inputs can be supplied through production-relevant APIs, files, messages, or controlled manual adapters;
9. the parties identify facility revenue, custody and operating cost, implementation cost, risk ownership, and who pays Argent; and
10. the partners can decide whether to proceed to a live facility and replicate the model.

---

## Design-partner decisions

A 90-minute facility-design workshop should replace the illustrative assumptions with institution-approved terms:

- legal borrower, lender, and custodian entities;
- exact use of proceeds and repayment cycle;
- bar schedule, vault account, jurisdiction, and custody terms;
- security interest and custodian acknowledgement;
- price and FX sources, valuation timing, staleness limits, and fallbacks;
- advance rate, margin threshold, cure period, and enforcement threshold;
- interest, fees, repayment, and cash-settlement process;
- approved dealer or liquidation agent and expected execution time;
- evidence required by credit, legal, risk, operations, compliance, and audit;
- bank and custodian systems to which Argent must connect.

The output should be a signed-off pilot matrix covering the commercial purpose, underwriting, collateral schedule, legal control, valuation policy, operating workflow, integrations, evidence, liquidation route, and pilot economics. That matrix is the point at which Argent moves from a demonstrated engine to a design-partner credit product.

---

## Market-practice basis

This structure follows established secured-lending concepts: lenders determine borrowing capacity by applying risk-based margins to eligible collateral, while allocated gold provides bar-specific ownership certainty at the cost of whole-bar and operational complexity. See UBS’s Lombard lending materials, the LBMA Gold Price framework, and the World Gold Council/Linklaters paper on allocated and pooled gold structures.

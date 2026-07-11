# Candidate credit controls

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**Credit controls that a lender may ask for, which the contract does not enforce today. What each would take to build, and why none of them should be built before a design partner states their policy.**

*Audience: internal roadmap reference, and the honest answer when a bank asks "can you do a two-day cure window?" The answer is: not today; it is specified, it is scoped, and it is contingent on your policy. Tell us your workout process and we will build to it.*

*Companion documents: `design-partner-lender-brief.md` (what the lender sets), `bullion-collateral-reference-architecture.md` (the domain requirements), `argent-architecture.md` (authoritative where they touch).*

---

## 0. Why these are candidates and not commitments

Certain credit controls appear repeatedly in draft facility documents: a draw-suspension threshold between the advance rate and the margin call, a fixed cure period, a multi-condition release test. They are plausible. They are also **guesses** until a lender states its policy.

A real lender arrives with an advance rate, a suspension trigger, and a cure window driven by its own workout process, its own risk appetite, and its own supervisor. Hard-coding a plausible number now means ripping it out when the first design partner says what they actually use. Worse, it means shipping a document that asserts a control the contract does not enforce, which is the one failure mode these documents exist to prevent.

> **Rule.** The controls listed in Section 1 **are implemented**. The candidate extensions in Sections 2 to 4 are **not implemented** unless explicitly stated otherwise. Every threshold shown as a candidate is **illustrative** — the numbers belong to the lender, and the design belongs to the pilot.

**These are also not all the same kind of thing**, and conflating them produces bad roadmap decisions. Three categories:

| Category | Meaning | Cost |
|---|---|---|
| **Configuration** | The machinery exists; a parameter or band is missing | Low |
| **Capability gap** | The contract genuinely cannot do this; new mechanism required | Real, and raises a design question |
| **Off-chain policy** | It looks like a feature but is a determination the bank makes before signing | **Should not be built at all** |

---

## 1. What exists today

Stated first, because the gaps only make sense against it.

| Control | Status |
|---|---|
| **Advance rate** (`ltv_bps`) | **Enforced.** Bank-set per line. |
| **Maintenance threshold** (`maintenance_bps`) | **Enforced.** Bank-set per line. |
| **Advance < maintenance invariant** | **Enforced.** A line cannot open at or above its own margin-call level. |
| **Eligibility ceiling** (`max_ltv_bps`) | **Enforced** per framework/instrument. A line's advance rate cannot exceed it. |
| **Haircut** (`haircut_bps`) | **Enforced** per framework/instrument, applied before LTV. |
| **Utilisation ≤ borrowing base** | **Enforced.** An over-draw is rejected (`InsufficientCapacity`). |
| **Two-band margin computation** | **Enforced.** `revalue_and_check` computes an `action_band` (= raw value × maintenance) and a `warning_band` (= action band × `warning_bps`), yielding `MarginState` ∈ {`Covered`, `Warning`, `Called`}. |
| **Stale-price rejection** (`max_age_secs`) | **Enforced.** A revaluation on a stale price fails. |
| **Price-confidence tolerance** (`conf_tol_bps`) | **Enforced.** |
| **Line suspension** | **Two mechanisms, both enforced.** (1) *Automatic:* `revalue_and_check` sets `LineStatus::Suspended` when the margin state becomes `Called`, and `record_drawdown` requires an active line — so **a called facility cannot draw**. (2) *Discretionary:* `bank_suspend_line` lets the bank suspend at any time for any reason. |
| **Cure window / enforcement standstill** | **Enforced.** `issue_default_notice` sets a `cure_deadline_ledger` (rejected if not in the future), stored as `cure_expiry_ledger`. `record_enforcement` **fails** with `CurePeriodNotExpired` before it lapses. `cure_default` carries no expiry guard, so a late cure is accepted until enforcement is recorded — the deadline binds the bank, not the borrower. |
| **Three-party adjustment sequence** | **Enforced.** Owner requests → custodian confirms → bank approves. |

**The system is more capable here than it is usually given credit for.** The two-band margin computation in particular is real: a warning band genuinely sits below the call level, so a borrower's first notification is not the call. The gaps below are narrower than they first appear.

---

## 2. Configuration: cheap, and clearly lender-owned

### 2.1 The warning threshold: stored policy, and expressed as an LTV

**Today:** `warning_bps` is a **parameter passed on every `revalue_and_check` call**. It is not a field on any stored type. And it is a **factor applied to the action band**, not an LTV threshold in its own right:

```
action band  = raw_value  × maintenance_bps / 10_000
warning band = action band × warning_bps    / 10_000
```

**Two problems, and the second is easy to miss.**

**First, it is not stored policy.** The warning band is facility policy, but it is supplied per-call. Nothing prevents two revaluations of the same line from using different warning bands, and nothing records what the policy *was*. A lender reviewing the facility cannot read its own early-warning threshold from the record; it can only read what was passed at each revaluation.

**Second, it is a factor, not a threshold.** Because `warning_bps` multiplies the *action band* rather than the collateral value, it is **not the same kind of quantity as `ltv_bps` and `maintenance_bps`.** An invariant of the form `ltv_bps < warning_bps < maintenance_bps` would be **type-incoherent** — it compares a factor against two LTV thresholds. A lender cannot read "8,500" and know what warning level that implies without also knowing the maintenance threshold and doing the arithmetic.

**The fix, and it should be both:** store the warning level on the credit line, **and express it as an LTV threshold** (`warning_ltv_bps`) rather than as a factor on the action band:

```
warning band = raw_value × warning_ltv_bps / 10_000

invariant:  ltv_bps < warning_ltv_bps < maintenance_bps
```

Now all three are the same quantity — an LTV in basis points — the invariant is coherent, and a lender can read its own three thresholds directly off the facility record and audit them without arithmetic.

**Cost:** low. One stored field, one validation, one fewer call parameter, and a change to how the warning band is derived.

**Status:** this is a **policy-normalisation improvement**, not a feature request — it makes an existing control readable, auditable, and stable across revaluations. It is the one item here worth doing regardless of what any design partner says, because it is about the coherence of the model rather than about anyone's credit policy.

### 2.2 A pre-call draw-suspension band

**Asked for as:** *"No further utilisation at 65% LTV"* — a band between the advance rate and the margin call at which the line stops lending but is not yet called.

**What already exists, and it is stronger than usually described.** There are **two** suspension mechanisms today:

- **Automatic, on margin call.** `revalue_and_check` sets the line to `LineStatus::Suspended` when the margin state becomes `Called`. And because `record_drawdown` requires an active line, **a called facility cannot accept a further draw.** Suspension on breach is not a procedure someone must remember; it is an automatic consequence of revaluation.
- **Discretionary, at any time.** `bank_suspend_line` lets the bank suspend for any reason it judges sufficient — stale evidence, a title question, a sanctions hit — independent of any threshold.

**So the accurate statement is:** a margin call **automatically** suspends further drawdown, and the bank **additionally** holds a manual suspension authority.

**What is genuinely missing:** a *distinct pre-call suspension threshold* sitting between `Warning` and `Called`. Today `Warning` is informational — it changes the reported margin state but does not itself stop a draw. A lender wanting "stop lending at 65%, call at 70%" would need that intermediate band.

**The fix:** a stored `suspend_ltv_bps` between `warning_ltv_bps` and `maintenance_bps`, with `record_drawdown` rejecting a draw when the last valuation placed utilisation above it.

**The design question a partner must settle:** should the pre-call band *block* the draw, or merely *flag* it for the bank to decide? Blocking is the stronger control; flagging preserves workout discretion. Lenders differ, and this is theirs to choose.

**Cost:** low. One stored field, one guard in `record_drawdown`.
**Illustrative number:** 65% LTV. **A placeholder, not a recommendation.**

---

## 3. The cure window: implemented, with one open question

**Asked for as:** *"Cure period: two business days, subject to a shorter period for severe market, title, custody, or sanctions events."*

**This is implemented.** A ledger-sequence-based enforcement standstill exists today:

| Step | Mechanism |
|---|---|
| Bank issues a default notice | `issue_default_notice` takes a **`cure_deadline_ledger`** and a notice hash |
| The deadline must be real | A deadline at or before the current ledger is **rejected** (`CureDeadlineNotFuture`) |
| The deadline is stored | Persisted on the line as **`cure_expiry_ledger`** |
| The bank cannot enforce early | `record_enforcement` **fails** with **`CurePeriodNotExpired`** if the current ledger is below the deadline |
| The borrower may still cure late | `cure_default` carries **no expiry guard** — a cure is accepted right up until enforcement is actually recorded |

Read the last two rows together, because the design is deliberate and borrower-favourable: **the deadline binds the bank, not the borrower.** The bank is barred from enforcing until the window lapses; the borrower is not barred from curing after it. The window is a standstill on enforcement, not a guillotine on cure.

**What remains genuinely open** is not the mechanism but the *unit*:

> A facility agreement will say **"two business days."** The contract knows only **ledger sequence.** Something must map one to the other.

That mapping is a real decision with real consequences:

- **Ledger sequence** is what the contract can enforce, and it is tamper-proof — but it drifts against wall-clock time and knows nothing of weekends, holidays, or a Gulf/European calendar mismatch.
- **Business days** is what the legal document will actually say, and what a workout desk will actually operate on — but it requires an off-chain calendar, and therefore a party who computes and submits the resulting ledger deadline.

**The open question for a design partner:** who computes the ledger deadline from the contractual business-day period, and is that computation itself evidence? A bank that says "two business days" and a contract that enforces "ledger sequence N + 34,560" must agree, and the agreement must be auditable.

**Cost:** low, and mostly off-chain. The enforcement standstill already works.

**What to evaluate with a lender:** not *whether* to build a cure window, but whether the **lenient design** — cure accepted after the deadline, until enforcement is recorded — matches their workout practice. Some lenders will want a hard cut-off; that would be a change, and it should be a considered one.

---

## 4. Off-chain policy that only looks like a feature

**Asked for as:** a seven-condition release/substitution test — lender authorises; custodian confirms allocation; remaining pool passes eligibility, concentration, and coverage; no unresolved margin, default, sanctions, title, or evidence exception; no temporary shortfall; custodian confirms completion; the resulting bar list is recorded.

**This should mostly not be built.** Read the conditions and sort them:

| Condition | Where it belongs |
|---|---|
| Lender authorises the action | **Contract.** `bank_approve_adjustment` — enforced. |
| Custodian confirms bars allocated and immobilised | **Contract.** `custodian_confirm_adjustment` — enforced. |
| Resulting bar list and base recorded | **Contract.** Enforced. |
| No temporary collateral shortfall | **Contract.** The adjustment is atomic; there is no interim state. |
| Remaining pool passes **coverage** | **Contract — already enforced.** `bank_approve_adjustment` recomputes the borrowing base from the *proposed* quantity, price, haircut and line LTV, and **rejects the adjustment** with `AdjustmentUndercovered` if the resulting base falls below the drawn balance. Available capacity, bar-set evidence and the uniqueness lock are updated atomically. |
| Remaining pool passes **eligibility and concentration** | **Off-chain.** Concentration rules (max per refiner, per vault, per jurisdiction) are lender policy. |
| No unresolved **sanctions, title, or evidence** exception | **Off-chain, and must stay there.** These are determinations, not computations. A contract cannot know whether a sanctions screen cleared. |

> **The principle.** The contract enforces the *sequence* and the *arithmetic*. The bank makes the *determinations*. A control that requires judgement is a control the bank signs for — and the signature is the evidence that it made the judgement.

Building a "sanctions clear" flag into the contract would create the illusion of an enforced control while actually enforcing nothing but that someone set a boolean. That is worse than not having it, because it invites reliance.

**Cost:** none for coverage — it is already enforced. The remaining conditions (eligibility, concentration, sanctions, title, evidence) should be documented as lender pre-conditions in the facility agreement, **not built**.

---

## 5. Summary: what to build, and when

| Item | Category | Build when |
|---|---|---|
| Warning level stored on the line, expressed as `warning_ltv_bps` | Configuration / **policy normalisation** | **Now.** It makes an existing control readable and auditable, independent of any partner's policy. |
| Pre-call draw-suspension band (between `Warning` and `Called`) | Configuration | **When a lender wants to stop lending *before* the call level.** Suspension *on* call is already automatic. |
| Business-day → ledger-sequence mapping for the existing cure window | Off-chain convention | **When a lender states its workout calendar.** The enforcement standstill itself is already implemented. |
| Coverage test on the proposed resulting set | **Already implemented** | — `bank_approve_adjustment` rejects an under-covered adjustment (`AdjustmentUndercovered`). |
| Eligibility / concentration on release | Off-chain policy | **Do not build.** Lender policy, documented in the facility agreement. |
| Sanctions / title / evidence exceptions | Off-chain policy | **Do not build.** These are determinations. The bank signs; the signature is the control. |

---

## 6. How to answer a bank that asks for these

Plainly, and without pretending:

> *"Not today. The advance rate, maintenance threshold, eligibility ceiling, haircut, and the two-band margin computation are enforced in the contract now. A suspension band and a cure window are specified and scoped, but not implemented — deliberately, because the thresholds and the cure mechanics belong to your credit policy, not to ours. Tell us your workout process and we will build to it, and you will be able to read the result in the code."*

That is a stronger answer than "yes" (untrue, and discoverable in an afternoon by anyone who reads the repo) and stronger than "no" (which concedes ground you have not lost).

---

## 7. Provenance of the illustrative numbers

The 65% suspension threshold, 72% enforcement trigger, and two-business-day cure period appear in a draft design-partner credit brief. They are **reasonable placeholders and nothing more.** No lender has stated them, no policy has been approved, and the contract does not enforce them.

They are recorded here so that:

1. the credit brief can point at this document rather than asserting the controls itself;
2. a design partner can see we have thought about what they will ask for; and
3. nobody, internally or externally, mistakes a placeholder for a commitment.

---

## Boundary reminder

The lender sets credit policy. Argent enforces the policy the lender sets, and records the acts the parties sign. It does not set advance rates, does not decide when a line is suspended, and does not judge whether a cure is adequate. Where this document and `argent-architecture.md` touch, the architecture document is authoritative.

**Section 1 describes controls that are implemented. Sections 2 to 4 describe candidate extensions that are not.** Verify any claim against `contracts/credit_ledger/` at the implementation baseline stated above before relying on it.

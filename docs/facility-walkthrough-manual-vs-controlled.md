# One facility, two ways: manual vs. controlled

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**A step-by-step walkthrough of a single gold-secured credit facility as it runs today, and as it runs on a shared control record. Written for the operations team that would actually run it.**

*Audience: the bullion desk, collateral operations, and credit operations at a bank that already offers gold-backed financing. This is not a protocol document. It is a comparison of a working day. Every "controlled" step names a contract action that exists in the open-source core (Apache-2.0) and can be read before you believe any of it.*

*Companion documents: `design-partner-lender-brief.md`, `why-gold-secured-operational-credit.md`, `argent-architecture.md`.*

---

## Why this document exists

Banks already lend against gold. That is not in question, and Argent is not proposing that they should start.

What this document addresses is narrower and more useful: **how the facility is administered**. The published evidence on collateral operations is consistent and unflattering. Collateral management in wealth-management lending has historically been a siloed process, with data collected from a variety of sources, tracked in spreadsheets manually, and reported from the core banking system at a basic aggregated level at best. Loan-to-value is frequently calculated by hand by a loan officer or operations team member, which raises the risk of human error and can lead to an inadequately secured loan. In the wider market, collateral and margin management was for a long time carried out in a piecemeal way using spreadsheets, with a lack of sophistication that would surprise many observers of the world's great financial engine rooms.[^bd][^hfj]

That is the process this document compares against. Not a strawman — the actual, published, industry-acknowledged state of collateral operations.

**The proposition is not "you cannot do this today." You obviously can. The proposition is that you are doing it with instruments that cost you hours, create disputes, and produce evidence you have to reconstruct after the fact.**

---

## The facility

One facility, held constant across both columns, so the comparison is like-for-like.

| | |
|---|---|
| **Borrower** | A bullion dealer with a segregated reserve tranche |
| **Collateral** | 8 × 1 kg allocated kilobars, 999.9 fine, in a third-party vault |
| **Facility** | Revolving line, drawn and repaid against operating cash flow |
| **Advance rate** | Set by the bank; maintenance threshold above it |
| **Custodian** | Independent vault operator, not the lender |
| **Lifecycle** | Pledge → activate → draw → revalue → margin event → repay → release |

---

## Stage 1 — Establishing the collateral

### Today

The custodian produces a holding statement. It arrives as a PDF, or a spreadsheet, or an email body. Someone in credit operations re-keys the bar serials, weights, and fineness into the bank's collateral record. Someone else checks the re-keying.

The bank's collateral system now holds a *copy* of the custodian's bar list. Two records exist. They are already capable of diverging, and from this moment on, keeping them aligned is a manual task that recurs for the life of the facility.

**The failure mode:** a transposed serial, a stale statement, a bar that moved. Nobody finds out until it matters.

### Controlled

The custodian signs `register_position`: the bar list, the fine weight, the holder. **One record, signed by the party who actually holds the metal.** The bank does not re-key it, because the bank is reading the same record the custodian wrote.

**What changed:** the bar list stops being a document that is copied between systems and becomes a state that is signed once and read by everyone.

---

## Stage 2 — Pledging the bars

### Today

The borrower instructs. The bank sends a pledge instruction to the custodian. The custodian blocks the bars and confirms — by email, by letter, or by a signed PDF that gets filed.

That confirmation is the entire evidentiary basis of the pledge. It sits in an inbox. If the facility is later disputed, or if the auditors ask, or if enforcement becomes real, this email is what the bank produces.

**The failure mode:** the confirmation is ambiguous about *which* bars, or it confirms a block that was never applied, or it is lost, or it cannot be shown to have preceded the drawdown.

### Controlled

The custodian signs `confirm_and_immobilize` against the identified bars. The owner and the bank then jointly sign `activate_pledge`. The pledge is now a state on a shared record, not an email in a folder.

**What changed:** the pledge confirmation is a non-repudiable act by the custodian, bound to specific serials, timestamped, and readable by the bank without asking. The bank does not have to trust that the email is accurate; it can read that the custodian signed.

---

## Stage 3 — Opening the line and computing the borrowing base

### Today

A loan officer opens a spreadsheet. Fine weight × reference price × FX × (1 − haircut) × advance rate. The number goes into the credit system as the approved limit.

Per the published evidence, this is exactly where hand-calculation risk lives: LTV frequently calculated by hand, human error possible, and the consequence of an error is an inadequately secured loan.[^bd]

**The failure mode:** a stale price, a wrong haircut, a fat-fingered weight, an advance rate that quietly sits above the maintenance threshold. Nobody notices until the market moves.

### Controlled

The bank signs `open_credit_line` with the advance rate (`ltv_bps`), the maintenance threshold (`maintenance_bps`), and the price it has approved. The contract computes the borrowing base from the *signed* quantity and refuses the line if the policy is incoherent.

Three checks are enforced in code, not in procedure:

- **the advance rate must be strictly below the maintenance threshold** — a line cannot be opened at or above its own margin-call level;
- **the advance rate may not exceed the eligibility ceiling** recorded for that instrument class;
- **the approved limit is bounded by the computed borrowing base.**

**What changed:** the spreadsheet is not checked by a second person; it is replaced by a computation that cannot produce an out-of-policy line, because the contract rejects it.

---

## Stage 4 — Drawing

### Today

The borrower requests a draw. Someone checks the current utilisation against the approved limit, usually in the credit system, sometimes against a separately maintained borrowing-base certificate. The draw is booked.

**The failure mode:** a draw that exceeds available capacity because the base was recalculated somewhere else and the systems have not caught up. Or a duplicate draw, booked twice.

### Controlled

The bank signs `record_drawdown`. The contract checks the draw against `available_limit` and **rejects it outright if capacity is insufficient** (`InsufficientCapacity`). Duplicate authorisation references are rejected. Utilisation cannot exceed the borrowing base, by construction.

**What changed:** an over-draw is not a control that someone might miss. It is a transaction that fails.

---

## Stage 5 — Revaluation and the margin event

### Today

Gold falls. Somebody reruns the spreadsheet — or does not, until the next scheduled review. The LTV is recomputed by hand. If it has breached, a margin call is issued by phone and confirmed by email.

The borrower's first notification of trouble may be the call itself. The bank's evidence that the call was made, at what price, on what date, against what reference, is once again an email.

**The failure mode:** the breach is discovered late; the price used is disputed; the borrower says they were never told; nobody can reconstruct what the valuation was on the day.

### Controlled

The bank (or an approved valuer) signs `revalue_and_check`, and the act **must reference the valuation source it acted on** — no valuation reference, no state transition. The contract computes the margin state:

| State | Meaning |
|---|---|
| `Covered` | Drawn balance comfortably below the threshold |
| `Warning` | Approaching the threshold — an explicit early-warning band **before** the call level |
| `Called` | Threshold breached |

Stale prices are rejected (`max_age_secs`); confidence tolerance is enforced (`conf_tol_bps`).

**What changed:** three things. The margin state is computed rather than remembered. There is a warning band *before* the call, so the borrower's first notification is not the call. And the valuation is bound to the source it used, so "what was the price on the day, and where did it come from" is answerable from the record rather than from an inbox.

---

## Stage 6 — Substitution and top-up

### Today

The borrower wants to swap a bar, or post an additional bar to cure a margin call. This is a three-way conversation conducted by email: the borrower asks, the bank approves, the custodian executes, and then everyone updates their own record of what is pledged.

**The failure mode:** the three records disagree about which bars are currently in the pledge. This is the single most common source of collateral disputes, and it compounds silently.

### Controlled

Three signed acts, in an enforced order: the borrower signs `request_collateral_adjustment` (status `Requested`); the **custodian** signs `custodian_confirm_adjustment`, confirming it can actually hold and block the proposed set (status `CustodianConfirmed`); then the **bank** signs `bank_approve_adjustment`, which recomputes the borrowing base against the *proposed* set and **rejects the adjustment outright** (`AdjustmentUndercovered`) if it would leave the line under-covered.

The order is not arbitrary and the contract enforces it: `custodian_confirm_adjustment` requires status `Requested`, and `bank_approve_adjustment` requires `CustodianConfirmed`. The lender does not approve a bar set the custodian has not yet confirmed it can hold.

**What changed:** there is no version of the bar list that any party maintains privately. The pledged set is one state, and it only changes when all three have signed in sequence. Nobody can be mistaken about what is pledged, because there is only one record of it.

---

## Stage 7 — Repayment

### Today

Cash arrives. The credit system is updated. The collateral record is updated. These are two operations, and there is a window between them.

**The failure mode:** cash has moved and the exposure has not been reduced, or the exposure has been reduced and the cash has not landed. Reconciliation catches this, eventually.

### Controlled

Through `settlement_vault`, **the settlement asset moves and the drawn balance falls in one atomic transaction, or neither happens.** Repayment can only be applied by the single settlement vault (`apply_repayment` rejects any other caller).

**What changed:** the window closes. There is no state in which the money has moved and the ledger has not.

---

## Stage 8 — Release

**This is the stage that matters most, and it is where the difference is starkest.**

### Today

The borrower repays and asks for the bars back. The bank issues a payoff letter. Someone at the bank instructs the custodian to release. The custodian releases and confirms.

Now consider the dispute, six months later, in which the borrower alleges the bars were released without proper authority, or the auditor asks the bank to demonstrate that the release was authorised *before* it happened.

**The bank's evidence is an email chain.** It must show that the payoff letter existed, that the release instruction followed it, that the custodian acted on the bank's instruction and not on the borrower's, and that the sequence was correct. Reconstructing this after the fact is precisely the work that makes collateral disputes expensive.

### Controlled

Two signed acts, and **the order is enforced by the contract, not by procedure**:

1. the bank signs `bank_authorize_release`, **carrying a payoff-letter hash** — the authorisation cannot be recorded without committing to the document it rests on;
2. the custodian signs `custodian_confirm_release`.

And here is the mechanism that an operations lead should test us on:

> **`custodian_confirm_release` fails unless the pledge is already in `ReleaseAuthorized` state.**

The custodian *cannot* release the bars before the bank has authorised it. Not "should not" — cannot. The transaction is rejected. There is no sequence of events, no mistake, no misread email, and no pressure from a borrower that results in metal leaving the pledge without the bank's prior signed authorisation on the record.

**What changed:** the release evidence is not reconstructed after the fact. It is the record. The payoff letter is committed to, the authorisation precedes the release by construction, and both parties' signatures are on their own acts.

---

## Stage 9 — Default and enforcement

### Today

Default is declared by letter. The custodian is instructed to freeze. Valuation is confirmed. An enforcement pack is assembled — from emails, PDFs, spreadsheets, and whatever the collateral system happens to hold — and handed to the people who must actually realise the collateral.

**The failure mode:** the pack is assembled under time pressure, from records that were never designed to be evidence, by people reconstructing a sequence of events months after they occurred.

### Controlled

`issue_default_notice` (bank-signed). `revalue_and_check` (valuation bound to its source). `record_enforcement` (bank-signed, evidence recorded). `open_enforcement_readiness` / `populate_enforcement_readiness` — the pack is **assembled from the record**, not reconstructed from inboxes.

And `cure_default` is a first-class state, so a cured breach leaves an auditable trail rather than a side letter.

**Stated honestly, because this is where overclaiming would be fatal:** everything after the enforcement instruction — the dealer quotation, the bar transfer or sale, the cash settlement, the debt waterfall, the return of surplus — happens **off-ledger, under your security agreement, exactly as it does today**. Argent does not sell metal and does not move cash beyond settlement-asset repayment. What it changes is that every step *up to* the enforcement instruction is already evidence, in the form your recovery team needs, on the day they need it.

---

## The summary an operations lead cares about

| Stage | Today | Controlled |
|---|---|---|
| Bar list | Re-keyed from a PDF into a second system | Signed once by the custodian; read by all |
| Pledge | Confirmed by email, filed | Non-repudiable act bound to serials |
| Borrowing base | Hand-calculated in a spreadsheet | Computed; out-of-policy lines rejected |
| Drawdown | Checked against a limit someone maintains | Over-draw is a failed transaction |
| Revaluation | Rerun by hand, or at the next review | Computed; warning band before the call; price bound to its source |
| Substitution | Three-way email; three private records | One pledged set; three signatures in sequence |
| Repayment | Two operations with a window between | Atomic: cash and exposure move together |
| **Release** | **Email chain, reconstructed if disputed** | **Custodian cannot release before the bank has authorised — enforced, not procedural** |
| Enforcement | Pack assembled from inboxes under pressure | Pack assembled from the record |

---

## What this is worth — and the honest limit of the claim

Argent's value here is **efficiency and evidence**, not capability. You can run this facility today, and you do. The question a pilot answers is whether running it on a shared control record is cheaper, safer, and less dispute-prone than running it on spreadsheets and email.

**That is a number, and we cannot produce it from the outside.** How many hours per facility, how many reconciliation cycles, how many disputes per year, how long an enforcement pack takes to assemble — only your operations team knows. This is the single most valuable thing a design partner brings, and it is the first question we would want to work through together.

If the answer turns out to be "our current process is cheap enough," that is a real finding and we would rather hear it than argue past it.

---

## What we are not claiming

- **Not** that the on-chain record creates the security interest. Your pledge agreement does. Argent records and enforces the authorised control state.
- **Not** that this replaces credit judgement, KYC/AML, insurance, or the custodian's own books.
- **Not** that the metal is tokenized. It is not, and that is the point: the bars stay allocated, in the vault, under the existing custody agreement. No transferable instrument is created against them.
- **Not** that Argent executes enforcement. It records the authorised acts up to the enforcement instruction; the sale, the waterfall, and the surplus happen under your documents.

---

## Verify before you believe any of it

- The core is **open source (Apache-2.0)**. Every contract action in the "controlled" column exists in `contracts/credit_ledger/` and is covered by the test suite.
- The three enforced invariants in Stage 3 (advance rate strictly below maintenance; advance rate capped by the eligibility ceiling; utilisation bounded by the borrowing base) are in the code and can be read directly.
- The release sequencing in Stage 8 (`custodian_confirm_release` rejects unless the pledge is in `ReleaseAuthorized`) is a state check in the contract, not a claim in a brochure.
- The full lifecycle runs end to end on Stellar testnet.

We would rather you read the contract than take our word for the walkthrough.

---

## Sources

[^bd]: Bank Director, "Taking Control and Mitigating Risk With a Collateral Management System" — on collateral management as a siloed process, manual spreadsheet tracking, and LTV frequently calculated by hand with attendant human-error risk.
[^hfj]: The Hedge Fund Journal, "Collateral Management" — on collateral and margin management historically being carried out piecemeal using spreadsheets.

*Industry-practice characterisations above are drawn from published sources on collateral operations generally, not from any specific institution's internal process. Any bank's actual process may differ, and a design partner's first contribution is telling us how theirs really works.*

## Boundary reminder

Custody stays with the custodian. Ownership stays with the owner. Credit exposure stays with the lender. Control state moves onto Soroban; signing authority is governed under the framework's approval policy. The asset never leaves custody because of Argent, and ownership changes only through an off-chain legal and custody process after default, which Argent records as evidence but never executes.

This document is a commercial and operational comparison, not legal or financial advice. Where it and `argent-architecture.md` touch, the architecture document is authoritative.

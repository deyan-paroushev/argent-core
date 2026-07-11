# Design-partner brief: custodian

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**For a vault operator or independent custodian asked to acknowledge control over allocated bars pledged under a credit facility, with Argent as the collateral-control layer.**

*Audience: operations, compliance, legal, and client-service at a prospective custodian partner. This brief states exactly what you would be asked to sign, exactly what you would not, how the control record maps to your existing bar list and instruction workflow, and what a pilot would involve. Every contract action named here exists in the open-source core (Apache-2.0) and can be verified against the repository before you commit anything.*

*Companion documents: `design-partner-lender-brief.md` (the lender's view of the same facility), `argent-architecture.md` (technical architecture and trust boundaries).*

---

## 1. The short version

A client of yours pledges allocated bars, already in your vault, as collateral for a credit line from a bank. You are asked to do what you already do — hold the metal, act on authorised instructions, confirm status — and to record four of those acts on a shared control ledger that the client, the bank, and you all read from and sign against.

**The metal does not move. Your role does not expand. Your existing bar list, instruction workflow, and client agreements remain the operative documents.** What changes is that four of your confirmations become independently signed, timestamped records that the bank can verify directly, instead of emails and PDFs that must be reconciled by hand later.

**You take on no credit duty, no valuation duty, and no enforcement duty.** That is not a reassurance; it is a property of the system, and §3 shows you how to verify it in the code yourself.

## 2. What you would sign

The custodian's entire surface in the contract is six actions. Four are operational, two are one-time setup. That is the complete list — there are no others.

| Action | When | What it means |
|---|---|---|
| `register_framework` | Setup, once | You are named as the custodian party in the tri-party framework. |
| `admit_instrument` | Setup, per asset class | You confirm you can hold and service this instrument class (e.g. 1 kg allocated kilobars, named refiners). |
| `register_position` | Per holding | You confirm the bar list: which specific bars, what fine weight, held for whom. This mirrors the holding statement you already produce. |
| `confirm_and_immobilize` | Per pledge | You confirm the identified bars are immobilised — allocated, segregated, and not available for release except on authorised instruction. This is the act that gives the pledge its substance. |
| `custodian_confirm_adjustment` | On substitution | You confirm a change to the pledged bar set (e.g. a bar substituted or added), after the bank has approved it. |
| `custodian_confirm_release` | On release | You confirm that bars have been released, **only after the bank has separately authorised the release.** |

Each is an independent signature by you, on the act you actually perform. You are never asked to co-sign someone else's decision.

## 3. What you would *not* sign — and how to verify it

This is the section that matters most, and it is checkable rather than promised.

The custodian signs **no** credit action, **no** valuation action, and **no** enforcement action. Specifically, you do not sign, and cannot be asked to sign:

| Action | Signed by | Custodian involvement |
|---|---|---|
| `open_credit_line` | Bank | **None** |
| `record_drawdown` | **Approved Processor** (role-checked) | **None** |
| `revalue_and_check` (revaluation, margin check) | **Approved Valuer** (role-checked) | **None** |
| `bank_suspend_line`, `bank_resume_line` | Bank | **None** |
| `bank_authorize_release` | Bank | **None** — you execute a release, you do not authorise it |
| `issue_default_notice` | Bank | **None** |
| `record_enforcement` | Bank | **None** |
| `apply_repayment` | **Settlement vault only** | **None** |

You are therefore **not** being asked to:

- decide whether credit should be extended, or on what terms;
- value the collateral, supply a price, or run a margin calculation;
- monitor the loan, or notify anyone of a margin breach;
- declare a default, or judge whether one has occurred;
- enforce, sell, or transfer title to any bar;
- take on any credit exposure to the borrower or the bank;
- guarantee the borrower's obligations in any respect;
- accept any instrument, token, or synthetic asset representing the metal — **the bars are never tokenized**.

**How to verify.** The contract core is open source. **None of the functions above requires the custodian's signature** — they are signed by the bank, or by an approved Processor, Valuer, or the settlement vault, depending on the action. You do not have to accept our characterisation: read `contracts/credit_ledger/src/lib.rs` and confirm that `custodian.require_auth()` appears in exactly the six actions in §2 and nowhere else.

## 4. How this maps to what you already do

Nothing here is a new operational primitive for a custodian. The mapping is deliberately one-to-one:

| Your existing process | The control record |
|---|---|
| Holding statement / bar list | `register_position` — the same bar list, signed |
| Blocking or earmarking bars against a client instruction | `confirm_and_immobilize` — the same block, recorded |
| Acting on an authorised release instruction | `custodian_confirm_release` — the same release, executed only after the bank's authorisation is on the record |
| Substitution or top-up of pledged bars | `custodian_confirm_adjustment` — the same swap, after bank approval |
| Your fee schedule, KYC/AML, insurance, audit | Unchanged, and outside the protocol entirely |

**The bars stay where they are, allocated to the same client, under the same custody agreement.** Argent does not take custody, does not hold title, and does not become a party to your client relationship. It records the control state that you, the client, and the bank would otherwise reconcile between three private systems.

**Release remains dual-control, and that protects you.** Metal cannot leave the pledge on the borrower's request alone, and it cannot leave on Argent's say-so at all. It requires the bank's authorisation (`bank_authorize_release`) *and* your execution (`custodian_confirm_release`), as two separately signed acts. This materially reduces the risk of releasing bars against an instruction whose authority is later disputed: the bank's authorisation is on the record, signed, before you act.

## 5. What you get from it

Stated without exaggeration, because the ask is small and so is the claim.

**A defensible record of every instruction you acted on.** Today, if a release is later questioned, the evidence is an email chain. Under this model it is a bank-signed authorisation, timestamped, immutable, and independently verifiable, that existed *before* you executed. That is a materially better position for you in a dispute.

**Fewer reconciliation cycles.** The bank reads the pledge state directly rather than asking you to confirm it repeatedly. Fewer status calls, fewer PDF confirmations, fewer "can you re-confirm the bars are still blocked" emails.

**No credit, valuation, or lending role is added,** and the code enforces that. The operational and evidential consequences of signing shared-ledger confirmations must nevertheless be reviewed and agreed by your legal and operations teams — a signed confirmation is a stronger assertion than an email, which is the point, and also the thing to review.

**A position in the collateral-finance chain.** Allocated custody is being pulled into lending, and the current alternative is tokenization — where the client's metal is represented by a transferable token issued against it. Argent is explicitly the non-tokenized path: the bars stay allocated, in your vault, under your custody agreement. If you would rather not see client metal wrapped into circulating instruments, this is the model that does not require it.

## 6. The pilot ask

A pilot proves the business only if all three counterparties are real: a borrower with eligible bars, a lender willing to define a credit policy, and a custodian willing to acknowledge control. This is what we would ask of **you**.

### What we need from you

1. **A control acknowledgement**, in a form your legal team is comfortable with: a written confirmation that identified bars are immobilised against a pledge, and that release occurs only on the bank's authorised instruction. This is the single most important artefact, and it is a document you almost certainly already have a template for.
2. **Confirmation of the instrument class you can service** — bar formats, accepted refiners, allocation and segregation terms, and how substitution is handled today.
3. **One operations contact** who would actually perform the four confirmations, so we can map the control record to your real instruction workflow rather than an assumed one.
4. **Your existing bar-list format**, so `register_position` mirrors the statement you already produce instead of asking you to generate a new artefact.
5. **A signing arrangement** for the four operational confirmations. Under the current core these are role-signed acts; how that maps to your internal authority matrix (who at your firm may confirm an immobilisation, who may confirm a release) is yours to define, and defining it is part of the pilot.

### What we are *not* asking you to do

- Not to move, re-vault, or re-allocate any metal.
- Not to change your custody agreement, fee schedule, insurance, or client relationship.
- Not to accept a token, a synthetic asset, or any transferable instrument representing the bars.
- Not to take on any credit, valuation, margin-monitoring, default, or enforcement duty. (§3 — verifiable in code.)
- Not to guarantee, indemnify, or take exposure to the borrower or the lender.
- Not to grant exclusivity, or to commit to production before the pilot has answered your operational questions.

### What you get from the pilot

- One holding, run end to end — bar list registered, bars immobilised, pledge activated, a release authorised by the bank and executed by you — on a testnet control record, before any mainnet commitment and with no client metal at risk.
- A clear read on how the four confirmations fit your existing instruction workflow, and what they would cost you operationally.
- The dispute-evidence property tested against a real scenario: a release with the bank's prior authorisation on the record.
- No exclusivity, and no obligation to proceed.

### Success criteria

We would consider the pilot successful, and you should hold us to this, if at the end:

- your legal team confirms the control acknowledgement is a document you would actually sign;
- your operations team can state whether the four confirmations fit their workflow, and at what cost;
- your compliance team is satisfied that no credit, valuation, or enforcement duty has attached to you;
- and any gap found in the above is written down, not argued away.

If the pilot shows this does not fit your operating model, that is a valid and useful outcome, and we would rather find it in a pilot than in production.

## 7. The honest caveats

Three things a custodian should press us on, stated here rather than waiting to be asked.

**The on-chain record is not the legal instrument.** The pledge is created by the security agreement between the borrower and the lender, and your acknowledgement is a document under your governing law. Argent records and enforces the *authorised control state*; it does not create the security interest and does not replace your acknowledgement. If those two ever disagree, the legal documents govern.

**The signing arrangement is real work.** Mapping "who at your firm may confirm an immobilisation" onto a signing key is not a formality, and it touches your internal authority matrix and key custody. This is one of the things a pilot exists to work through, and we would rather scope it honestly than call it trivial.

**Argent is early.** The core is open source and tested, the lifecycle runs end to end on testnet, and there is not yet a production facility. You would be a design partner, not a customer of a finished product, and the pilot is explicitly structured so that you can say no at the end.

---

## Boundary reminder

**Custody stays with you.** Ownership stays with the owner. Credit exposure stays with the lender. Control state moves onto Soroban; signing authority is governed under the framework's approval policy. The metal never leaves custody because of Argent, and ownership changes only through an off-chain legal and custody process after default, which Argent records as evidence but never executes.

This document is a commercial brief, not legal advice, and it does not itself create or vary any custody obligation. Where this brief and `argent-architecture.md` touch, the architecture document is authoritative.

# Design-partner brief: borrower

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**For an owner of allocated gold considering a credit line secured by bars that stay in your custodian's vault.**

*Audience: treasurer, CFO, or principal at a bullion dealer, refiner, jewellery manufacturer, family office, or holding company with a segregated tranche of allocated bars and a recurring short-term liquidity need. This brief states what you get, what it costs you, what happens to your gold, and — plainly — what can go wrong. Every contract action named here exists in the open-source core (Apache-2.0) and can be verified before you commit anything.*

*Companion documents: `design-partner-lender-brief.md` and `design-partner-custodian-brief.md` (what we ask of the other two parties — you are entitled to read both), `why-gold-secured-operational-credit.md` (market rationale).*

---

## 1. The proposition

You hold allocated gold. You need liquidity. You do not want to sell.

A bank extends you a revolving credit line secured by identified bars that **stay in your custodian's vault**. You keep title. You keep the price upside. You draw what you need, repay from operating cash flow, and redraw. Argent is the control layer that makes the pledge verifiable to the bank without the metal ever leaving custody or being tokenized.

**What does not happen to your gold:** it is not sold, not moved to the bank, not transferred to Argent, not re-vaulted, not, under the proposed custody arrangement, available for re-use or re-hypothecation, and not turned into a token or any transferable instrument. It sits where it sits, allocated to you, under your existing custody agreement.

**What does happen:** specific bars are immobilised — flagged in your custodian's system as pledged and not available for release except on the bank's authorised instruction. That immobilisation is what the credit line is secured against.

## 2. Why borrow rather than sell

Four reasons, and they are the reasons institutions actually do this.

**Tax.** Selling realises capital gains on everything the gold has appreciated since you bought it. Borrowing is not a disposal, so it does not trigger that event. For a holder sitting on appreciated metal this is usually the largest single number in the comparison. *This is general logic, not tax advice; your jurisdiction and circumstances govern, and you should take your own counsel.*

**You keep the upside.** The bars remain yours. If gold keeps rising, that gain is yours, not the buyer's. In a rising market the appreciation can offset a meaningful part of the interest cost.

**You avoid the round trip.** If you sell to raise cash and later want the position back, you re-buy at a higher price and cross the dealer spread twice. Borrowing avoids that loss entirely.

**Gold borrows well.** Lenders like collateral that is liquid, price-transparent, and portable. Allocated bullion is all three, which is why advance rates on it are high relative to most asset classes.

**When this is the wrong structure.** If the liquidity is needed to fund continuing operating losses, this facility postpones the sale of your gold while adding interest expense and the risk of a forced liquidation on someone else's timetable. The structure is designed for a *temporary* liquidity need repaid from identifiable cash flow. If that is not your situation, selling may genuinely be the better decision, and we would rather say so now.

## 3. What it costs you

You should have a straight answer to this before anything else.

- **Interest**, set by the lender, accruing on what you actually draw — not on the whole line. This is the main cost.
- **Custody and valuation fees**, largely what you pay already, plus whatever your custodian charges for the pledge confirmations.
- **An advance rate well below 100%.** You will not get the full market value of the metal as credit. Expect a substantial haircut; the bank sets the number.
- **Operational obligation.** You accept daily or periodic valuation, and you must respond to a margin call if one comes.

Argent does not set your rate, does not take a cut of your draw, and does not price your gold. Who pays Argent, and how, is a commercial question that a pilot exists partly to answer — and we would rather state that openly than pretend the question does not exist.

## 4. What happens if gold falls — stated plainly

This is the section most likely to be soft-pedalled elsewhere. It is not soft-pedalled here.

**Your line has three margin states**, and they are computed and recorded, not negotiated after the fact:

| State | What it means | What happens |
|---|---|---|
| **Covered** | Your drawn balance sits comfortably below the margin threshold. | Nothing. Normal operation. |
| **Warning** | Gold has fallen enough that you are approaching the threshold. | You are notified. This is an early-warning band, deliberately set *before* the call level, so you have time to act. |
| **Called** | Your drawn balance has breached the maintenance threshold. | The bank can require you to post more bars, repay part of the balance, or, failing that, begin default. |

Two things are worth knowing, because they are in your favour and they are enforced in code rather than promised:

**Your line cannot be opened at its own margin-call level.** The contract refuses to open a facility where the advance rate is at or above the maintenance threshold. Every line starts with headroom, by construction.

**There is a warning band before the call.** The revaluation logic computes an explicit warning level ahead of the breach level. You are not supposed to discover a margin call as your first notification.

**And the hard truth:** if gold falls far enough, and you cannot post additional collateral or repay, the bank can declare default and move to enforce against your bars. That is the risk you are taking, and it is real. Every party in this structure — including us — would rather you understood it before signing than after.

**Cure is a first-class state.** If you remedy the breach, `cure_default` records it, and the cure is on the permanent record rather than in a side letter. A cured default does not vanish, but nor does it sit as an unexplained mark.

## 5. The whole-bar constraint — stated plainly

Allocated gold comes in discrete bars. **You cannot release 17% of a bar.**

If you pledge eight kilobars and repay a fraction of the balance, you do not automatically get a proportional slice of metal back. You get bars back — whole ones — and only when the remaining coverage still satisfies the bank's policy. In practice this means:

- pledge a **multi-bar portfolio**, not a single bar, so partial releases are possible at all;
- expect **release rules** set by the bank: which bars, at what coverage level, on what repayment;
- **substitution** is supported (swap a bar, add a bar), but it is a dual-signed act: you request it (`request_collateral_adjustment`), the **custodian confirms** it can hold the proposed set, then the **bank approves** it. You cannot swap bars unilaterally, and neither can anyone else.

This is a genuine constraint of allocated custody, not a limitation of the protocol. It is also one of the main reasons Argent exists: managing which specific bars are pledged, released, or substituted across a live borrowing base is exactly the operational problem that is currently handled with spreadsheets and email.

## 6. What you sign, and what you control

Your surface in the contract is small and every action is one you would expect to take:

| Action | What it means |
|---|---|
| `register_position` | You and the custodian confirm the bar list: which bars, what fine weight, held for you. |
| `select_lot_for_collateral` | **You choose which bars go into the pledge.** Not the bank, not the custodian, not Argent. |
| `activate_pledge` | You and the bank jointly activate the pledge over the selected bars. |
| `open_credit_line` | You and the bank jointly open the facility. |
| `request_collateral_adjustment` | You request a substitution or top-up. |
| `cure_default` | You and the bank record a cure. |

Three protections follow, and they are structural rather than contractual promises:

**You select the collateral.** The bars that enter the pledge are the ones you nominate.

**Nobody can release your metal unilaterally — including you, and including the bank.** A release requires the bank's authorisation *and* the custodian's execution, as two separately signed acts. Argent cannot release your bars. Neither can we.

**Your bars are never tokenized.** No transferable instrument is created against your gold. It is not wrapped, not issued, not circulated. This is the central design commitment, and it is the reason this model differs from tokenized-gold lending.

## 7. What you get from the record

The bank gets verifiable control. What do *you* get from the shared record, beyond the credit?

**Evidence of what each party asserted, and when.** Every state change — immobilised, pledged, drawn, revalued, released — is a signed, timestamped act by the party that performed it. You can see the custodian's confirmation and the bank's authorisation directly, rather than requesting a statement and waiting.

**No unilateral action against your collateral.** Every act that touches your bars requires a signature from a named party, and you can read who signed what and when.

**A cleaner exit.** On full repayment, release is an authorised, recorded act — not a negotiation about whether the paperwork is in order.

## 8. The pilot ask

A pilot proves the business only if all three counterparties are real: a lender willing to define a credit policy, a custodian willing to acknowledge control, and a borrower with eligible bars and a genuine liquidity need. This is what we would ask of **you**.

### What we need from you

1. **A segregated tranche of allocated bars** you are willing to pledge — ideally a multi-bar holding (see §5), with a bar list from your custodian.
2. **A real liquidity need**, with an honest answer to: what does the money finance, how long is the cycle, and where does repayment come from? A pilot against a hypothetical need proves nothing.
3. **Willingness to accept daily or periodic valuation and margin controls**, including the possibility of a margin call.
4. **Your custodian's name**, and an introduction, since the facility cannot exist without their acknowledgement.
5. **One finance contact** who would actually run the facility day to day.

### What we are *not* asking you to do

- Not to sell, move, re-vault, or re-allocate any gold.
- Not to transfer title to anyone.
- Not to accept a token, a synthetic asset, or any transferable instrument representing your bars.
- Not to change custodians, or your custody agreement.
- Not to grant exclusivity, or commit to a production facility.
- Not to draw a single dollar you do not need.

### What you get from the pilot

- One facility, run end to end — bars selected, pledged, line opened, drawn, revalued, repaid, released — on a **testnet control record first, with no metal pledged and no money at risk**, so you can see the whole lifecycle before committing anything real.
- A clear read on the economics: what the bank would advance, at what rate, on what terms.
- The margin and release mechanics tested against a real scenario, including a simulated price fall, before you are ever exposed to one.
- No exclusivity, and no obligation to proceed.

### Success criteria

We would consider the pilot successful, and you should hold us to this, if at the end:

- you can state plainly what the facility would cost you and what it would give you;
- you understand exactly what happens to your bars in a margin call, and are comfortable with it;
- your custodian confirms the pledge and release workflow fits their operation;
- and any gap found in the above is written down, not argued away.

**If the pilot shows you should sell the gold rather than borrow against it, that is a valid outcome and we will say so.** We would rather lose a design partner than put someone into a facility that is wrong for them.

## 9. The honest caveats

**The on-chain record is not the legal instrument.** The pledge is created by your security agreement with the lender. Argent records and enforces the authorised control state; it does not create the security interest, and if the two ever disagree, the legal documents govern.

**The bank still underwrites you.** Gold improves the credit structure; it does not remove the lender's need to assess your business, your repayment source, and your jurisdiction. This is not "the gold is so good the borrower does not matter."

**Argent is early.** The core is open source and tested, and the lifecycle runs end to end on Stellar testnet, but there is not yet a production facility. You would be a design partner, not a customer of a finished product. The pilot is explicitly structured so you can walk away at the end.

---

## Boundary reminder

Custody stays with your custodian. **Ownership stays with you.** Credit exposure stays with the lender. Control state moves onto Soroban; signing authority is governed under the framework's approval policy. Your gold never leaves custody because of Argent, and ownership changes only through an off-chain legal and custody process after default, which Argent records as evidence but never executes.

This document is a commercial brief, not legal, tax, or financial advice, and it does not itself create or describe an enforceable security interest. Where this brief and `argent-architecture.md` touch, the architecture document is authoritative.

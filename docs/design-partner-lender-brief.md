# Design-partner brief: lender

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**For a bank or secured-credit provider evaluating a pilot facility secured by allocated gold, with Argent as the collateral-control layer.**

*Audience: credit, risk, legal, and operations at a prospective lending partner. This brief states what Argent does, what it explicitly does not do, how the borrowing base and margin mechanics work, what the enforcement path looks like, and precisely what a pilot would ask of you. Every contract function named here exists in the open-source core (Apache-2.0) and can be verified against the repository before you commit anything.*

*Companion documents: `why-gold-secured-operational-credit.md` (market rationale), `argent-architecture.md` (technical architecture and trust boundaries).*

---

## 1. What is being proposed

A revolving credit facility extended by you, secured by allocated gold bars that remain with an independent custodian, where:

- **the metal never moves to Argent, and never moves to you.** It stays in the custodian's vault, allocated and segregated.
- **the borrower retains title** unless and until enforcement occurs under your security agreement.
- **you receive enforceable, continuously verifiable control** over pledge, release, and the borrowing base, rather than possession.
- **you set the credit policy.** Advance rate, maintenance threshold, eligible refiners, tenor, margin, covenants, and borrower limit are all yours. Argent enforces the policy you set; it does not set it.

Argent is the shared control record that you, the borrower, and the custodian all sign against and all read from. It is not a lender, not a custodian, not an issuer, and not a legal-enforcement engine.

## 2. What Argent is not

This section is deliberately placed before the benefits, because the most common objection to a control layer is imagined scope.

Argent does **not**:

- extend credit, hold credit exposure, or make any credit decision;
- take custody of, hold title to, or move the metal;
- tokenize the bars, or create any transferable instrument representing them;
- value the collateral independently of you (in the current core, **the price is supplied and signed by the bank**);
- create the security interest — your pledge or security agreement does that;
- replace custodian acknowledgement, KYC/AML/sanctions checks, insurance, or your credit judgement;
- execute enforcement, sell metal, or move cash beyond settlement-asset repayment.

Argent records and enforces the *contractually authorised control state*. The legal instrument that creates and perfects your security interest sits alongside it and is unchanged.

## 3. The credit product

A revolving working-capital facility secured by allocated gold. The borrowing base is computed, not asserted:

```
eligible collateral value = eligible fine weight
                          x approved reference price      (bank-supplied)
                          x approved FX rate
                          - haircut

available credit          = eligible collateral value
                          x advance rate                  (bank-set)
                          - outstanding utilisation
                          - reserves and adjustments
```

In the contract core this is `borrowing_base(quantity, price, haircut, ltv)`, and it is enforced at line opening and on every drawdown: **utilisation can never exceed the borrowing base.** Two policy invariants are enforced by the contract itself, not by convention:

- the advance rate (`ltv_bps`) must be **strictly below** the maintenance threshold (`maintenance_bps`) — a line cannot be opened at or above its own margin-call level;
- the advance rate on any line may not exceed the **eligibility ceiling** (`max_ltv_bps`) recorded for that instrument class.

You set all three numbers. The contract refuses any line that violates them.

## 4. The control model: who signs what

Each party independently signs only the action it takes. This is the core of the separation-of-duties model, and it is why the record is evidential rather than merely informational.

| Lifecycle stage | Contract action | Signed by |
|---|---|---|
| Position registered | `register_position` | **Owner + custodian** |
| Lot selected for collateral | `select_lot_for_collateral` | **Owner** |
| Bars immobilised in the vault | `confirm_and_immobilize` | **Custodian** |
| Exclusive pledge activated | `activate_pledge` | **Owner + bank** |
| Facility opened at your policy | `open_credit_line` | **Cardholder + bank** |
| Drawdown against the base | `record_drawdown` | **Approved Processor** (role-checked) |
| Repayment applied | `apply_repayment` | **Settlement vault only** |
| Revaluation and margin check | `revalue_and_check` | **Approved Valuer** (role-checked) |
| Line suspended / resumed | `bank_suspend_line`, `bank_resume_line` | **Bank** |
| Collateral adjustment requested | `request_collateral_adjustment` | **Owner** |
| Adjustment approved | `bank_approve_adjustment` | **Bank** |
| Adjustment confirmed | `custodian_confirm_adjustment` | **Custodian** |
| Release authorised | `bank_authorize_release` | **Bank** |
| Release executed | `custodian_confirm_release` | **Custodian** |
| Default notice | `issue_default_notice` | **Bank** |
| Cure | `cure_default` | **Cardholder + bank** |
| Enforcement recorded | `record_enforcement`, `open_enforcement_readiness` | **Bank** |

Note the role model: drawdown and revaluation are not signed by "the bank" generically. They are signed by an **approved Processor** and an **approved Valuer** respectively — roles the framework approves, which may or may not be the bank itself. This matters for your operating model: you decide who holds those roles.

Two properties follow, and they are the ones a credit officer should test:

**Release is dual-control.** Metal cannot leave the pledge on the borrower's say-so, nor on Argent's. It requires your authorisation (`bank_authorize_release`) *and* the custodian's execution (`custodian_confirm_release`), as two separately signed acts. Neither party can complete a release alone.

**Repayment and exposure reduction are atomic.** Through `settlement_vault`, the settlement asset moves and the drawn balance falls in one transaction, or neither happens. There is no window in which cash has moved and the ledger has not, or vice versa.

> **A naming note.** The borrower is called `cardholder` in the contract. This is a legacy artifact: the initial reference design was a card-line product, and the identifier was never renamed. It denotes the borrowing party in a corporate working-capital facility, not a consumer card.

## 5. Margin and default mechanics

Gold is liquid and price-transparent, which is why advance rates on it are high relative to other collateral. It is also volatile, and a pilot must be honest about that.

- **Revaluation** is an explicit, signed act (`revalue_and_check`), governed by the margin policy registered for the framework (`margin_policy_hash`). The valuation reference is committed on-chain; the price itself is yours.
- **The maintenance threshold** (`maintenance_bps`) is the level at which the line breaches. Because the contract forbids opening a line at or above maintenance, every facility starts with headroom by construction.
- **On breach**, your options are the conventional ones and they are all recorded acts: suspend the line, call for top-up (additional bars pledged), require partial repayment, or issue a default notice.
- **Cure** is a first-class state (`cure_default`), not an off-ledger side agreement. A cured default leaves an auditable trail.

**Whole-bar constraint, stated plainly.** Allocated gold comes in discrete bars. A borrower cannot release "17% of a bar." A production facility therefore needs multi-bar portfolios, rules for which bars may be released on partial repayment, and substitution without losing coverage. The contract supports lot selection and collateral adjustment (`request_collateral_adjustment`, `custodian_confirm_adjustment`, `bank_approve_adjustment` — again dual-signed). The *policy* for which bars may be released at what coverage level is yours to set, and defining it is part of the pilot.

## 6. The enforcement path

A facility is only as credible as your ability to realise the collateral. This is the chain, and it is where a lender should push hardest:

```
default declaration          -> issue_default_notice        (bank-signed, on-ledger)
  -> custodian freeze        -> pledge state already exclusive; no release without bank auth
  -> valuation confirmation  -> revalue_and_check           (bank-signed, valuation_ref committed)
  -> enforcement instruction -> record_enforcement          (bank-signed, evidence recorded)
  -> readiness pack          -> open_enforcement_readiness / populate_enforcement_readiness
  -> dealer quotation        -> off-ledger, your approved dealer
  -> bar transfer or sale    -> off-ledger, under your security agreement
  -> cash settlement         -> off-ledger
  -> debt and cost waterfall -> off-ledger
  -> surplus to owner        -> off-ledger
```

Read that boundary carefully, because it is the honest one. **Argent produces the evidence and the authorised control state; it does not execute the sale.** The transfer of title, the dealer sale, the cash waterfall, and the return of surplus happen under your security agreement and the custodian's instruction workflow, exactly as they would today. What changes is that every step up to the enforcement instruction is a signed, timestamped, independently verifiable act rather than an email trail, and the enforcement-readiness pack is assembled from that record rather than reconstructed after the fact.

## 7. What you actually get

Stated without exaggeration:

**A collateral book of record you can rely on without reconciling four systems.** Today, the borrower's position, the custodian's bar list, your credit system, and the legal file are four private records that must be reconciled by hand. Argent is one shared control state each party signs against and each party — including an auditor — can read.

**Non-repudiable evidence of every state change.** Who pledged, who immobilised, who authorised release, who confirmed it, when the line was revalued, against what price reference, and who declared default. Each is an independently signed act bound to the party that performed it.

**A continuously calculated borrowing base**, rather than a periodically re-derived one. Utilisation cannot exceed it, by construction.

**Lower cost to serve.** The candid version of the value proposition: you adopt Argent only if it lets you originate, monitor, or service these facilities more cheaply, safely, or at greater scale than through spreadsheets, emails, and manual custody instructions. That is the test, and a pilot is how you measure it.

**What it does not give you:** relief from credit analysis, capital relief, or a claim that gold's collateral quality makes the borrower irrelevant. Gold is eligible financial collateral in defined contexts with supervisory haircuts; it is not generally an HQLA and it does not remove the need to underwrite the borrower, the repayment source, the jurisdiction, and the liquidation route. Argent improves the *control and evidence* of the structure. It does not improve the borrower.

## 8. The pilot ask

A pilot proves the business only if all three counterparties are real: a borrower with eligible bars and a genuine liquidity need, a custodian willing to sign the control acknowledgement, and a lender willing to define the credit policy. This is what we would ask of **you**, the lender.

### What we need from you

1. **A credit policy for one instrument class.** Specifically: advance rate (`ltv_bps`), maintenance threshold (`maintenance_bps`), eligibility ceiling (`max_ltv_bps`), haircut, eligible refiners and bar formats, and acceptable custodians. These are the numbers the contract enforces. Without them there is no facility.
2. **A valuation source and cadence.** Which reference price, at what frequency, and who signs the revaluation. In the current core the bank supplies and signs the price; if you would rather it came from an oracle adapter, that is a design decision to make together.
3. **Sign-off on the control model** by your legal and operations teams — specifically that dual-control release (`bank_authorize_release` + `custodian_confirm_release`) and the enforcement-evidence path are consistent with the security agreement you would use.
4. **One named credit officer** as the pilot counterparty, and access to the operations team who would actually run the facility. The cost-to-serve question can only be answered by the people who bear the cost today.
5. **A view on capital and eligibility treatment** in your jurisdiction, so the pilot is scoped against a facility you could actually book rather than a hypothetical one.

### What we are *not* asking you to do

- Not to take custody, or change custodians.
- Not to accept a token, a synthetic asset, or any instrument representing the gold.
- Not to delegate any credit decision to Argent or to the protocol.
- Not to rely on the on-chain record as the security interest. Your pledge agreement remains the instrument; Argent records the control state it authorises.
- Not to commit to production before the pilot has answered the cost-to-serve question.

### What you get from the pilot

- A single facility, run end to end — pledge, activation, draw, revaluation, margin event, repayment, release — against a real bar list with a real custodian, on a testnet control record before any mainnet commitment.
- The enforcement-readiness pack, produced from the record rather than reconstructed, so your recovery team can assess it against a real default scenario.
- A measured answer to the cost-to-serve question, against your current manual baseline.
- No exclusivity, and no obligation to proceed.

### Success criteria

We would consider the pilot successful, and you should hold us to this, if at the end:

- your credit and legal teams agree the control model is consistent with your security agreement;
- your operations team can state, with a number, whether servicing this facility through Argent is cheaper than their current process;
- the enforcement-readiness pack is judged sufficient by whoever would actually have to realise the collateral;
- and any gap found in the above is written down, not argued away.

If the pilot shows the model does not work for you, that is a valid and useful outcome, and we would rather find it in a pilot than in production.

## 9. Verification before you commit

Nothing in this brief needs to be taken on trust.

- The contract core is **open source (Apache-2.0)**. Every function named in §4 and §6 exists in the repository and is covered by the test suite.
- The control invariants in §3 (advance rate strictly below maintenance; utilisation never exceeds the borrowing base; advance rate capped by the instrument's eligibility ceiling) are enforced in code and can be read directly.
- The lifecycle in §4 is exercised end to end in a live demonstrator on Stellar testnet.
- The trust boundaries — what is enforced on-chain, what is off-chain, and what is assumed — are stated in `argent-architecture.md` and `threat-model-and-security-boundaries.md`.

We would rather you audit the boundary than accept a claim about it.

---

## Boundary reminder

Custody stays with the custodian. Ownership stays with the owner. **Credit exposure stays with you.** Control state moves onto Soroban; signing authority is governed under the framework's approval policy. The asset never leaves custody because of Argent, and ownership changes only through an off-chain legal and custody process after default, which Argent records as evidence but never executes.

This document is a commercial brief, not legal, tax, or financial advice, and it does not itself create or describe an enforceable security interest. Where this brief and `argent-architecture.md` touch, the architecture document is authoritative.

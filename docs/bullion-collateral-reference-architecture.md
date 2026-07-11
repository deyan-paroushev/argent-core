# Bullion collateral: a reference architecture

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**What physical bullion demands of any system that controls it as collateral, and how those demands are met. The asset is the subject; Argent is one conforming implementation.**

*Audience: an architect at a lending institution, custodian, or collateral-technology vendor evaluating whether bullion is modelled correctly; and any implementer building against this problem. This document describes the domain, not the protocol. Where Argent's implementation is shown, it is as a worked example of a general requirement, not as the requirement itself.*

*For the protocol architecture — Soroban contracts, Stellar integration, DFNS signing, security model — see `argent-architecture.md`. This document deliberately does not restate it.*

---

## 0. Why bullion needs a reference architecture at all

Gold is treated as the easy case in collateral: liquid, price-transparent, fungible, portable. Two of those four are true, and the two that are false are the ones that break naive implementations.

Bullion is **not fungible at the collateral layer**, and it is **not divisible**. A pledged holding is a set of *specific, serialised, individually assayed bars* sitting in a *specific vault* under a *specific allocation*. That is what the lender's security attaches to, what the custodian blocks, and what an auditor asks to see. Model it as a quantity — "401.1 oz of gold" — and every hard problem in the domain is silently discarded: no-double-pledge, whole-bar release, substitution, grade drift, chain-of-custody integrity.

The failure is not theoretical. A system that models bullion as a fungible balance cannot answer the question a credit officer will actually ask, which is not *"how much gold?"* but *"which bars, where, whose, and can anyone else claim them?"*

**This document sets out the demands the asset makes.** Sections 1–7 are asset-side and vendor-neutral. Section 8 shows how Argent satisfies them, and §9 states which parts of the pattern generalise beyond bullion and which are gold-specific.

---

## 1. The two identity layers: instrument and lot

The first modelling decision determines everything downstream, and most implementations get it wrong by collapsing two distinct things into one.

**The instrument** is the *economic identity of one unit*: what the asset is, in what unit it is denominated, and to which grade standard. "One troy ounce of LBMA good-delivery gold" is an instrument. "One kilogram of 999.9 four-nines kilobar" is a *different* instrument. So is LME Grade A copper, or No. 2 wheat. The instrument is defined once, centrally, and referenced.

**The lot** (or holding, or position) is *this specific parcel*: these serials, this weight, in this vault, held for this owner, against this custodian. It references an instrument; it does not restate it.

The separation matters because these change on entirely different timescales and under entirely different authority. A grade standard is redefined rarely, by the body that sets it. A lot changes whenever metal moves. Collapsing them means asset-standard data is replicated into every holding, and a grade redefinition becomes a migration across every position in the book.

### The demand

> **Requirement 1.** Asset identity (commodity, unit, grade standard) must be defined once and referenced, never replicated per holding. Lot identity (serials, weight, location, owner, custodian) must be modelled separately from it.

### The anti-fraud corollary

Whoever can unilaterally define an asset standard can define bad collateral into eligibility. The definition of "what counts as an acceptable bar" must not rest with one party.

> **Requirement 2.** Instrument definition requires two signatures: the party that defines the asset standard, and the custodian that attests it can hold and service that class. Neither alone.

### Grade versioning

A grade standard can be redefined. Bars pledged under the old standard were assessed against the old standard. If a redefinition silently rewrites the standard those holdings reference, the lender's underwriting basis changes retroactively without anyone signing anything.

> **Requirement 3.** Instrument standards are versioned linearly. A redefinition creates a new version. Existing holdings continue to reference the version in force when they were created.

---

## 2. Lot uniqueness: the no-double-pledge problem

**This is the deepest structural requirement in the domain, and the one most often missed.**

The obvious control is: a *position* cannot be pledged twice. That is necessary and insufficient. The real risk is that the *same bars* appear under two different positions, at two different lenders, or at the same lender through two different channels. Position-level uniqueness does not catch this, because the two positions are different objects. The bars are the same metal.

This is not a hypothetical. It is the structural equivalent of duplicate financing of a single warehouse receipt — the classic commodity-trade-finance fraud, and the reason warehouse-receipt registries exist at all.

### The demand

> **Requirement 4.** The uniqueness key must be the **lot identity itself** — the serials, the receipt id, the parcel id — not the position record. The same lot must not be capable of being active under two positions at once. No-double-pledge is enforced at the *lot* level, not the *position* level.

An implementation that cannot answer *"is any bar in this proposed pledge already pledged elsewhere?"* has not solved bullion collateral. It has solved bookkeeping.

---

## 3. The evidence set: what a lender actually underwrites against

Bullion collateral is not underwritten against a number. It is underwritten against a set of documents whose integrity must be provable. The set is cohesive — these are not independent facts, they are one evidentiary basis — and every element is separately load-bearing:

| Evidence | What it establishes | Why it is first-class |
|---|---|---|
| **Manifest** | The bar list: serials, weights, fineness | The definition of what is pledged |
| **Uniqueness key** | The lot's identity | Enforces no-double-pledge (§2) |
| **Quality / assay certificate** | Grade, fineness | The lender **prices against grade**; a substitution to lower grade is a risk event, not an administrative detail |
| **Quantity / weight certificate** | Fine weight, independently certified | Weight is certified separately from the manifest, because the manifest is the owner's statement and the weight is the custodian's |
| **Location** | Which vault, which jurisdiction | Determines enforceability, settlement convention, and what "delivery" means (§6) |

### The demand

> **Requirement 5.** Quality and quantity are **separate, first-class commitments**, not fields buried inside the manifest. A lender underwrites against grade and weight independently, and a change to either is a credit event.

> **Requirement 6.** The full documents are never stored in the shared record. Only cryptographic commitments to them. The shared record proves *which document was relied upon*; it does not become a document repository, and it does not leak bar serials, KYC, or legal terms.

This second point is a boundary, not an optimisation. A shared control record that contains the actual bar list has created a new confidentiality surface and a new place for the bar list to be wrong.

---

## 4. Indivisibility: the whole-bar constraint

**A borrower cannot release 17% of a bar.**

This single sentence invalidates the entire "collateral as a fungible balance" model, and it is the requirement most likely to be discovered late, in production, when a borrower makes a partial repayment and asks for metal back.

A 1 kg kilobar is an atom. A 400 oz good-delivery bar is a larger atom. Partial repayment does not entitle the borrower to a proportional slice of metal; it entitles them, at most, to the release of *whole bars* whose removal still leaves the remaining coverage within policy.

### What follows structurally

- **Holdings must be multi-lot.** A single-bar pledge admits no partial release at all. Any realistic facility pledges a portfolio.
- **Release is a selection problem, not an arithmetic one.** "Which bars can come out, such that coverage still holds?" is a constrained selection over discrete units, and the constraint is the lender's policy.
- **Coverage must be recomputed against the *proposed remaining set*, not against a reduced number.** The question is never "is 60% of the old weight still enough"; it is "does this specific remaining bar list still support the outstanding balance."

### The demand

> **Requirement 7.** Collateral is a set of indivisible lots. Release, substitution, and top-up operate on **whole units**, and coverage is evaluated against the proposed resulting set.

---

## 5. Mutation under a live facility: adjustment as a first-class object

Collateral changes while a facility is live. Three distinct operations, and they are not variations of one another:

| Operation | What changes | Risk |
|---|---|---|
| **Top-up** | Bars added | Low — coverage improves |
| **Substitution** | Bars swapped | **The dangerous one.** Grade, weight, refiner, or location may change. Coverage may look unchanged while the *quality* of the collateral has degraded |
| **Partial release** | Bars removed | Coverage must still hold against the remaining set (§4) |

The naive implementation treats these as edits to the position. That is wrong for a structural reason: **the credit agreement does not change; only the collateral schedule does.** An edit model conflates them, and it destroys the audit trail of *who proposed what, who approved it, and on what basis*.

The correct model is a **state machine on the adjustment itself**, distinct from the position and from the line, advancing through states as each party clears it:

```
Requested          (owner: "I propose this new set")
  -> CustodianConfirmed  (custodian: "I can hold and block that set")
  -> Approved            (bank: "coverage holds; I accept it")
  -> [position's collateral schedule is updated]

  -> Rejected            (at any point)
```

Note the ordering, and that it is not arbitrary. The custodian confirms *before* the bank approves, because the bank cannot sensibly approve a schedule the custodian cannot actually hold. And the owner cannot substitute unilaterally, nor can the bank impose a substitution, nor can the custodian swap bars on the borrower's word alone.

### The demand

> **Requirement 8.** Collateral adjustment is a first-class object with its own state machine and its own signatures, separate from both the credit agreement and the position. All three parties clear it, in order. The agreement stays fixed; only the schedule moves.

---

## 6. Location, loco, and the settlement conventions that actually bind

Location is not metadata. In bullion it is a settlement convention with legal and commercial consequences, and it is the field most often modelled as a free-text string by people who have not traded metal.

Two facts an architect must internalise:

**Format determines venue.** The ~400 oz good-delivery bar is the London wholesale format. The 1 kg four-nines kilobar is the standard settlement unit in Zurich, Dubai, and Singapore — and a kilobar is *not* a London Good Delivery bar. These are different instruments (§1) with different clearing conventions, not two sizes of the same thing.

**Loco determines what delivery means.** Metal "loco Zurich" and metal "loco London" are not interchangeable, and moving between them is a physical and commercial act, not a ledger entry.

The consequences for a control system:

- **location is a first-class commitment**, monitored, not a free-text label;
- **a change in location is a credit event**, because it changes enforceability and the liquidation route;
- **jurisdiction follows location**, which means the enforcement path (§7) is location-dependent;
- **format and loco must be consistent.** A "999.9 fine, ~400 oz, LBMA good-delivery, loco Zurich" bar is a contradiction, and a system that permits it will produce records that a bullion desk will not accept.

### The demand

> **Requirement 9.** Location is a monitored, committed field, not a label. Format, fineness, and loco must be mutually consistent, and a location change is a risk event.

---

## 7. Control, and the line the system must not cross

Here the architecture must be precise about what it is, because the temptation to overreach is strong and the consequence is a system no bank's legal team will accept.

**Control is not possession, and control is not title.**

- The **custodian** holds the metal. The system does not.
- The **owner** holds title. The system does not, and neither does the lender, unless and until enforcement occurs under the security documents.
- The **lender** holds a security interest created by a **security agreement**, and acknowledged by the custodian through a **control agreement**. The shared record does not create either. It commits to them, and it enforces the control state they authorise.

This is the boundary, and it must be stated in the architecture, not left as a caveat:

> **Requirement 10.** The record enforces the *contractually authorised control state*. It does not create the security interest, does not convey ownership, and does not execute enforcement. If the record and the legal documents ever disagree, the documents govern.

### Enforcement: what may be recorded, and what may not

Enforcement is where an over-engineered system does real damage. Recording an enforcement outcome must **not** be mistaken for effecting it. The lawful outcomes differ by governing law, and a system that hard-codes one has assumed a jurisdiction:

| Outcome | Meaning |
|---|---|
| **Sold** | Bars sold, proceeds applied to the debt |
| **Appropriated** | Lender takes the bars in satisfaction of the debt at an agreed valuation |
| **Transferred** | Title transferred under the security documents — *only where the governing law permits it* |

> **Requirement 11.** The system records **which lawful path was taken**, so the trail is undisputed. It does not convey ownership, move metal, or apply the waterfall. Proceeds and surplus flow under the documents.

### Release: the sequencing requirement

The single most important control property in the domain, and the one to test any implementation against:

> **Requirement 12.** Metal must not be capable of leaving the pledge without the secured party's prior, recorded authorisation. Not "should not" — *must not*. The custodian's release act must be **rejected** if the lender's authorisation is not already on the record.

A system in which the correct sequence is a procedure rather than an invariant has not solved the problem it claims to solve. Procedures are violated under pressure; invariants are not.

---

## 8. Argent as a conforming implementation

The requirements above are asset-side. This section shows one implementation that satisfies them, for concreteness. Every type and action named exists in the open-source core (Apache-2.0) and can be read.

| Requirement | Implementation |
|---|---|
| **R1** Instrument/lot separation | `Instrument` (commodity, unit, `grade_hash`) referenced by `InstrumentKey`; `VaultPosition` holds lot-specific evidence and quantity. Asset data is never replicated per holding. |
| **R2** Two-signature instrument definition | `register_instrument` is co-signed by the **issuer** (defines the standard) and the **depository** (attests it can hold the class). Neither can unilaterally define the asset. |
| **R3** Grade versioning | `InstrumentKey.version` tracks linear standard evolution; a grade redefinition bumps the version, so old holdings keep referencing the standard in force when created. |
| **R4** Lot-level no-double-pledge | `uniqueness_hash` on the position is the **collateral-uniqueness key** (bar serials / receipt id / parcel id). The same lot cannot be active under two positions at once. |
| **R5** Quality and quantity first-class | `quality_cert_hash` and `quantity_cert_hash` are separate commitments on the position, not fields inside the manifest. |
| **R6** Commitments, not documents | Only hashes on chain. `manifest_hash`, `location_hash`, `legal_terms_hash`, `control_agreement_hash`. The full documents never leave the off-chain systems. |
| **R7** Whole-unit operations | `select_lot_for_collateral` selects discrete lots; adjustment carries a **proposed new bar list and weight**, and coverage is evaluated against the resulting set. |
| **R8** Adjustment as a state machine | `CollateralAdjustment` with `AdjustmentType` ∈ {`TopUp`, `Substitution`, `PartialRelease`} and status `Requested → CustodianConfirmed → Approved`, cleared by owner, then custodian, then bank, in that order. |
| **R9** Location monitored | `location_hash` is a first-class commitment on the position. |
| **R10** Control, not title | `CustodyControl` commits to the tri-party control agreement; `Pledge` commits to the security agreement (`legal_terms_hash`). The record enforces the authorised state; the documents create the interest. |
| **R11** Enforcement recorded, not executed | `EnforcementOutcome` ∈ {`Sold`, `Appropriated`, `Transferred`} — recorded to anchor which lawful path was taken. Recording "does NOT itself convey ownership or move metal." |
| **R12** Release sequencing as an invariant | `custodian_confirm_release` **fails** unless the pledge is already in `ReleaseAuthorized` state. The custodian *cannot* release before the bank has authorised. The bank's authorisation must carry a payoff-letter hash. |

Two further properties, both enforced rather than procedural:

- **The borrowing base bounds utilisation.** A drawdown exceeding available capacity is rejected (`InsufficientCapacity`). Advance rate must be strictly below the maintenance threshold, and may not exceed the instrument's eligibility ceiling. A line that violates policy cannot be opened.
- **Eligibility is per-framework, per-instrument.** `FrameworkInstrumentEligibility` carries `haircut_bps`, `max_ltv_bps`, and `maintenance_bps` for *this instrument under this framework* — so "what we will lend against gold" is a policy object, not a constant.

---

## 9. What generalises, and what is gold-specific

The requirements above were derived from bullion, but most of them are not about gold. This matters if the pattern is to extend to other custody-stable physical collateral — and it matters as a test of whether the model is right, because a model that is *only* about gold has probably encoded accidents as structure.

**Generalises directly** (the same requirement, different asset):

- R1 instrument/lot separation — copper, wheat, and crude have the same two-layer identity
- R2 two-signature asset definition — the anti-fraud rationale is asset-independent
- R4 **lot-level uniqueness** — this *is* the warehouse-receipt duplicate-financing control, and it is arguably more load-bearing for bulk commodities than for gold
- R5 quality/quantity as separate certificates — warehouse-receipt finance evidences stated quality and quantity exactly this way
- R6 commitments not documents
- R8 adjustment state machine
- R10, R11, R12 control/title boundary, enforcement recording, release sequencing

**Gold-specific, or gold-acute:**

- **R7 indivisibility** is *unusually severe* for bullion. Bulk commodities are divisible: you can release 17% of a grain lot. A kilobar is an atom. Any generalisation must treat divisibility as an instrument property, not assume the gold case.
- **R9 loco conventions** are a precious-metals institution. Other commodities have delivery points and terminals, but the London/Zurich good-delivery-vs-kilobar distinction is specific to this market.
- **Price transparency.** Gold has a twice-daily published reference price and a deep 24-hour market. This is why advance rates on it are high, and why revaluation is tractable. An illiquid asset's valuation problem is qualitatively harder and cannot be solved by the same "signed price + valuation reference" mechanism.

The implementation reflected here treats commodity, unit, and grade as instrument properties precisely so that gold is *one instrument among many* — the `InstrumentKey` examples in the type system are `XAU_LGD`, `CU_LME_A`, `WHEAT_2` — rather than the model's only citizen.

---

## 10. A conformance checklist

For evaluating any system claiming to control bullion collateral. Each is answerable yes or no, and a "no" is a specific, named gap.

- [ ] Can it distinguish "one troy ounce of LBMA good-delivery gold" from "one kilogram of 999.9 kilobar" as **different instruments**?
- [ ] Can a single party unilaterally define what counts as eligible collateral? *(It must not.)*
- [ ] If a grade standard is redefined, do existing holdings silently re-reference the new standard? *(They must not.)*
- [ ] Can it detect that a bar in a proposed pledge is **already pledged elsewhere** — at the *bar* level, not the position level?
- [ ] Are quality and quantity separately certified, or buried in the manifest?
- [ ] Does the shared record contain the actual bar list? *(It must not.)*
- [ ] Can a borrower "release 17% of a bar"? *(They must not be able to.)*
- [ ] Is a substitution modelled as an **edit**, or as a three-party state machine?
- [ ] Does a change of vault location register as a risk event?
- [ ] Does the system claim to create the security interest? *(It must not.)*
- [ ] Does recording an enforcement outcome purport to convey title? *(It must not.)*
- [ ] **Can the custodian release metal before the lender has authorised it?** *(This must be impossible, not merely improper.)*

The last question is the one to ask first.

---

## Boundary reminder

Custody stays with the custodian. Ownership stays with the owner. Credit exposure stays with the lender. The control record enforces the authorised control state; the legal documents create the security interest. The asset never leaves custody because of the control layer, and ownership changes only through an off-chain legal and custody process, which the record anchors as evidence but never executes.

This document describes a domain and a reference model. It is not legal or financial advice. For the protocol architecture, see `argent-architecture.md`, which is authoritative wherever the two touch.

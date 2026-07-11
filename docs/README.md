# Argent documentation

Commercial, architectural, and design-partner documentation for Argent: the collateral-control layer for credit secured by allocated bullion.

> **Implementation baseline.** Claims about contract behaviour in these documents were verified against `contracts/credit_ledger/` at the baseline recorded in each file's header. **Replace `646878f` in those headers with the commit hash at merge time.** Contract code evolves; a document that names a function is only as good as the commit it was checked against.

---

## Start here, by what you need

### If you are evaluating whether this is a real market
- **`why-gold-secured-operational-credit.md`** — the market rationale. Why borrowing against allocated gold rather than selling it is an established institutional market, what already exists, and what the operating-infrastructure gap actually is.
- **`gold-secured-credit-market-and-product-guide.md`** — a deeper companion to the above. Goes further into the reference credit product, the full lifecycle, bank economics, and a minimum pilot definition.

### If you are a bank, custodian, or borrower considering a pilot
- **`design-partner-lender-brief.md`** — what a lender is asked for, what it gets, and what Argent explicitly does *not* do. Includes the full signer matrix.
- **`design-partner-custodian-brief.md`** — the narrowest ask. The custodian signs exactly six actions and takes on no credit, valuation, or enforcement duty. That claim is checkable in the code.
- **`design-partner-borrower-brief.md`** — costs and risks stated plainly, including the margin-call reality and the whole-bar release constraint.
- **`design-partner-credit-brief.md`** — the illustrative facility and its credit controls. **Section 3 distinguishes what the contract enforces today from what is a candidate and not implemented.**

### If you are an operations lead
- **`facility-walkthrough-manual-vs-controlled.md`** — one facility, nine stages, run today versus run on a shared control record. Written for the people who bear the reconciliation cost.

### If you are an architect or implementer
- **`bullion-collateral-reference-architecture.md`** — a **specification**. Twelve numbered requirements that bullion makes of *any* system controlling it as collateral, vendor-neutral, with a conformance checklist. Answers: what does correctness mean here, and how would you know if a system met it?
- **`bullion-collateral-system-design.md`** — a **build plan**. Representation taxonomy, product profiles, lifecycle state machines, integration architecture, deployment patterns, roadmap. Answers: what should be constructed, and in what order?
- **`collateral-eligibility-and-rights-model.md`** — the rights gate. Why *not all gold is collateral*, and why a holding's legal rights must be classified before it can enter a borrowing base.
- **`credit-control-extension-points.md`** — credit controls a lender may ask for that the contract does **not** enforce today, what each would take to build, and which should never be built because they are determinations a bank makes rather than computations a contract can perform.

---

## Implementation status, stated honestly

These documents describe both what exists and what does not. The distinction is always explicit, and it is checkable.

**Implemented and enforced in the contract:**
advance rate, maintenance threshold, the advance-below-maintenance invariant, the eligibility ceiling, haircut, utilisation bounded by the borrowing base, two-band margin computation (`Covered` / `Warning` / `Called`), stale-price and confidence guards, and:

- **Dual-control release.** The custodian's release act **fails** unless the bank has already authorised (`ReleaseAuthorized`).
- **Three-party adjustment, in enforced order.** Owner requests → **custodian confirms** it can hold the proposed set → **bank approves**. The bank's approval recomputes the borrowing base against the *proposed* set and **rejects** it (`AdjustmentUndercovered`) if the line would be left under-covered.
- **Automatic suspension on margin call.** `revalue_and_check` suspends the line when the margin state becomes `Called`, and a suspended line cannot draw. The bank additionally holds a discretionary manual suspension authority.
- **A cure-window enforcement standstill.** `issue_default_notice` sets a cure deadline (rejected if not in the future); `record_enforcement` **fails** with `CurePeriodNotExpired` before it lapses. A late cure is still accepted until enforcement is recorded — the deadline binds the bank, not the borrower.

**Specified but NOT implemented:**
- The **rights gate** in `collateral-eligibility-and-rights-model.md` (§9 of that document says so plainly): `RightsEvidence`, the machine-readable holding classification, and the `register_position` invariant that would reject unallocated, non-transferable, consent-gated, or lien-unknown holdings.
- The **candidate credit controls** in `credit-control-extension-points.md`: a lender-readable stored warning threshold, a *pre-call* draw-suspension band (suspension *on* call is already automatic), and the business-calendar interpretation of the existing cure deadline. **Every threshold shown in that document is a placeholder.** No lender has stated them.

Anyone verifying these documents against the repository will find exactly that. It is the correct expectation to set.

---

## The two documents that overlap

`why-gold-secured-operational-credit.md` and `gold-secured-credit-market-and-product-guide.md` both argue the market rationale, from different depths. **The first is the canonical position.** The second is the deeper product and pilot companion; its treatment of bank economics and the minimum pilot definition goes further than the first.

Similarly, `bullion-collateral-reference-architecture.md` (requirements and conformance) and `bullion-collateral-system-design.md` (build plan) address the same domain from different angles. They are complements: one defines correctness, the other defines construction.

---

## What is not here

The corporate-treasury argument for holding gold at all is **not included in this repository.**

Argent does not advise companies how to allocate treasury reserves. The documentation does, however, explain why institutions that already hold — or have independently decided to hold — allocated gold may gain additional resilience and secured-borrowing optionality from making that reserve operational. That is the boundary: we do not argue anyone into the asset; we address what the asset can do once held.

---

## Boundary

Across every document: custody stays with the custodian, ownership stays with the owner, credit exposure stays with the lender. Argent records and enforces the authorised control state. It does not create the security interest, does not tokenize the metal, does not value the collateral independently of the lender, and does not execute enforcement.

Where any of these documents and `argent-architecture.md` touch, **the architecture document is authoritative.**

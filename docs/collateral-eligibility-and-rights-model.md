# Collateral eligibility and rights model — the allocated-bullion profile

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**Not all gold is collateral *for this product*. Before a holding may enter a borrowing base, the rights attaching to it must be classified — and most representations of gold fail the test for the allocated-bullion profile.**

> **Scope.** This document defines eligibility for **Profile A: externally custodied, allocated-bar credit** — the product Argent implements today. Other rights profiles (issuer-controlled certificates, unallocated account claims, pooled interests, bank-internal deployment) may support valid facilities under *different* control models and different credit treatment. They are out of scope here, not invalid in principle. See `bullion-collateral-system-design.md` for the profiles anticipated later.

*Audience: architects and credit-risk readers evaluating what may be pledged; implementers extending the eligibility gate. This document defines the legal-rights classification that must precede economic eligibility, and states which holdings are ineligible by construction.*

*Companion documents: `bullion-collateral-reference-architecture.md` (the asset-side domain model), `design-partner-lender-brief.md` (the credit product), `argent-architecture.md` (protocol architecture, authoritative where they touch).*

---

## 0. The failure this prevents

A bank offers a gold product. The customer holds a digital confirmation stating weight, fineness, and type. It is denominated in gold. It has a market price. It appears on a statement. Every economic test a naive eligibility engine applies — *is it gold? does it have a weight? is there a reference price?* — returns **yes**.

Now read the product's terms.

> The investment is **backed by unallocated physical gold bars** held with the bank *or with any other entity designated by the bank at its sole discretion*. The investment or the digital confirmation is **non-negotiable and non-transferable**, and **cannot be endorsed, assigned or pledged to third parties without the bank's prior written consent**. The bank's records **shall be conclusive evidence** of the transaction.

This is not a hypothetical. It is the published retail gold T&Cs of a tier-one UAE bank, and the same institution separately markets an institutional bullion service offering gold loans, precious-metals repo, and certificates that *can* be pledged.

**The same brand, the same metal, four different products, and only some of them are collateral.**

The customer holding that certificate does not own bars. They hold an **unsecured contractual claim on the bank**, for metal the bank may keep wherever it likes, which they may not pledge without that bank's permission, and whose existence is proven only by that bank's own book. If it entered a borrowing base, a lender would believe it had a security interest in gold. It would have an unperfected claim against a third party's balance sheet, subordinate to that party's discretion.

**An eligibility engine that gates only on economics will ingest a claim and call it metal.** This document defines the gate that stops it.

---

## 1. The two gates, and why the order is not negotiable

> **Rights first, economics second.** A holding that fails the rights gate never reaches the economic gate. Haircut, advance rate, and maintenance threshold are meaningless questions about an asset that cannot be pledged.

The existing eligibility object (`FrameworkInstrumentEligibility`) governs the **economic** gate: `haircut_bps`, `max_ltv_bps`, `maintenance_bps`, and a commitment to the eligibility schedule. It answers *"how much will we lend against this?"*

It does not answer *"may this be pledged at all?"* — and that is the question that must be answered first.

```
    incoming holding
          │
          ▼
  ┌───────────────────┐
  │  RIGHTS GATE      │   May this be pledged? By whom? To whom?
  │  (this document)  │   Perfectable? Encumbered? Enforceable?
  └───────────────────┘
          │
     fail ─┴─► REJECTED — never valued, never enters a base
          │
       pass
          ▼
  ┌───────────────────┐
  │  ECONOMIC GATE    │   Haircut. Advance rate. Maintenance.
  │  (already exists) │   Concentration. Price source.
  └───────────────────┘
          │
          ▼
    borrowing base
```

An implementation that runs these in the other order, or merges them, has built a system that can be told a claim is a bar.

---

## 2. The classification: eleven facts that must be known before valuation

A position may not be valued until each is established and committed. These are not metadata. Each one, alone, can render the holding worthless as collateral.

| # | Fact | The question it answers | Failure mode if unasked |
|---|---|---|---|
| 1 | **Holding type** | Physical bars? A claim? A certificate? An account balance? | A claim is treated as metal |
| 2 | **Allocation model** | **Allocated** (identified bars) or **unallocated/pooled** (a claim on a pool)? | The lender's security attaches to nothing specific |
| 3 | **Legal owner** | Who holds title? | Pledge granted by a party without the right to grant it |
| 4 | **Beneficial owner** | Who bears the economic interest, if different? | Nominee structures obscure the real party; sanctions and AML exposure |
| 5 | **Custody model** | Third-party custodian, or *the counterparty's own vault*? | Metal held by the lender's own counterparty — no independence |
| 6 | **Bar identifiers** | Which specific serials? | No `uniqueness_hash`; no no-double-pledge control |
| 7 | **Transferability** | May it be assigned at all? | The security cannot be perfected |
| 8 | **Pledgeability** | May it be pledged **to a third party**? | *The Emirates NBD failure. Pledgeable only with the issuing bank's consent.* |
| 9 | **Required consent** | Whose written consent is a precondition? | A pledge that is void until a third party agrees — and may never agree |
| 10 | **Existing liens** | Is it already encumbered? | Second-ranking security on metal the lender believed was clean |
| 11 | **Permitted enforcement route** | On default, what may lawfully be done? Sale? Appropriation? Transfer? | A perfected security interest with **no lawful way to realise it** |

### The three that most often fail

**Allocation model (2).** This is the deepest one, and it is the fault line the Emirates NBD terms expose. *Unallocated* gold is **not the same asset as allocated gold**, though both are quoted in the same unit. It is a contractual claim on the institution holding it, for metal that need not exist as identified bars and that the institution may hold where it chooses. Allocated gold is specific, serialised, segregated metal held under bailment, and — **subject to the governing law and the custody agreement** — the holder's title is intended to survive the custodian's insolvency.

For **Profile A**, an unallocated claim is ineligible: there is nothing specific for the security to attach to, and the metal is not independently custodied. That is a statement about *this product*, not a claim that a bank-issued account claim can never support a facility. It can — under a different control model (issuer consent, transfer blocked, an effective account-control arrangement, counsel confirming enforceability), and with **claim-on-bank credit treatment rather than bar treatment.** That is a different profile, not this one.

**Pledgeability (8).** A holding can be gold, allocated, owned outright — and still be contractually unpledgeable, because the product terms forbid assignment to third parties without consent. Consent-gated pledgeability is not pledgeability: it is an option held by someone who is not your customer and may be your competitor.

**Enforcement route (11).** The most commonly deferred question and the most expensive to get wrong. A perfected interest in metal that cannot lawfully be sold in the jurisdiction where it sits is a decoration. This must be established at onboarding, not discovered at default.

---

## 3. Ineligible for Profile A

> The gate's output is not binary. It is:
>
> | Outcome | Meaning |
> |---|---|
> | **Eligible for Profile A** | Allocated, externally custodied, freely pledgeable, clean, enforceable |
> | **Consent or evidence required** | Pledgeable *once* a named consent or lien release is obtained and committed |
> | **Eligible only under another control profile** | A valid facility may exist — under a different control model and different credit treatment |
> | **Ineligible** | No lawful route to a perfected, realisable security interest |

The holdings below are ineligible **for Profile A**. Several could support a facility under another profile.

Certain holdings must be rejected by the gate rather than haircut by the credit committee. A haircut prices risk. It cannot cure the absence of a security interest.

| Rejected | Because |
|---|---|
| **Unallocated / pooled gold accounts** | The holder has a claim on an institution, not title to identified bars. Nothing specific for a bar-level security to attach to. *May be eligible under an account-control profile with claim-on-bank credit treatment — not this one.* |
| **Non-transferable certificates** | If it cannot be assigned, the security cannot be perfected. |
| **Consent-gated instruments where consent is not held** | A pledge void until a third party agrees is not a pledge. Consent must be **obtained and committed**, not assumed. |
| **Metal in the issuing counterparty's own vault** | If the entity that issued the claim also holds the metal, there is no independent custodian and no separation of duties for *this* profile. *A bank-internal deployment is a distinct profile with a distinct trust model.* |
| **Holdings with undisclosed or unresolved prior liens** | Ranking must be established, not hoped for. |
| **Holdings where the enforcement route is unknown** | Do not perfect an interest you cannot realise. |
| **Holdings whose only evidence is the issuer's own book** | *"The bank's records shall be conclusive evidence"* makes one party's ledger the sole truth. For a tri-party control model that is disqualifying — the shared record exists precisely to end that condition. |

> **Rule.** Ineligibility is a **rejection**, not a discount. A haircut is a price for volatility and liquidity risk. It is not a price for the possibility that the collateral does not legally exist.

---

## 4. What the current model already commits to, and what it does not

The position's evidence (`LotEvidence`) commits to five facts, all **physical**:

| Committed today | Fact |
|---|---|
| `manifest_hash` | The bar list |
| `uniqueness_hash` | Lot identity — the no-double-pledge key |
| `quality_cert_hash` | Assay / grade |
| `quantity_cert_hash` | Certified fine weight |
| `location_hash` | Vault / jurisdiction |

Two further commitments carry the legal instruments:

| Committed today | Fact |
|---|---|
| `control_agreement_hash` (on `CustodyControl`) | The tri-party control agreement |
| `legal_terms_hash` (on `Pledge`) | The security agreement |

**What is not committed anywhere: the rights facts themselves.** Nothing in the current model records *whether the holding is allocated or pooled*, *whether it is pledgeable*, *whose consent was required and obtained*, *what liens exist*, or *what enforcement route is lawful at its location*. Those facts live in the security agreement, which is committed to only as an opaque hash — meaning the contract cannot *reason* about them, and cannot reject a holding that fails them.

> **The gap.** Today the system can prove *which security agreement was relied upon*. It cannot prove *that the holding was pledgeable in the first place*. The physical evidence is first-class; the rights evidence is not.

---

## 5. The extension: a rights commitment as a first-class object

The minimal, honest fix — additive, and consistent with the existing design principle that the chain holds commitments, never documents.

### 5.1 A `RightsEvidence` commitment on the position

Alongside `LotEvidence`, the position carries a commitment to the **rights determination**: the off-chain legal opinion or onboarding memorandum establishing the eleven facts of §2.

| Field | Commits to |
|---|---|
| `rights_opinion_hash` | The legal determination: allocation model, title basis, transferability, pledgeability, enforcement route |
| `lien_search_hash` | The encumbrance search and its result |
| `consent_hash` | Third-party consent **actually obtained**, where the instrument requires it. `None` is not a default — where consent is required and absent, the holding is ineligible |

### 5.2 Machine-readable classification, so the contract can *reject*

A hash alone cannot be reasoned about. The classification facts that determine eligibility must be on-chain as values, not merely committed:

```
AllocationModel  ::= Allocated | Pooled | Unallocated
TitleBasis       ::= OutrightOwnership | Bailment | ContractualClaim
Pledgeability    ::= FreelyPledgeable
                   | ConsentRequired { consent_obtained: bool }
                   | NotPledgeable
EnforcementRoute ::= Sale | Appropriation | Transfer | Undetermined
LienStatus       ::= Clean | Ranked { rank: u32 } | Unknown
```

### 5.3 The gate as an invariant, not a procedure

> **Requirement.** `register_position` must **fail** where:
> - `AllocationModel` is `Pooled` or `Unallocated`; or
> - `TitleBasis` is `ContractualClaim`; or
> - `Pledgeability` is `NotPledgeable`, or is `ConsentRequired` with `consent_obtained = false`; or
> - `EnforcementRoute` is `Undetermined`; or
> - `LienStatus` is `Unknown`.

This is the same design discipline already applied to release sequencing, where `custodian_confirm_release` *fails* unless the pledge is in `ReleaseAuthorized`. The property that makes that control trustworthy is that it is an **invariant, not a procedure** — procedures are violated under pressure; invariants are not.

An ineligible holding must not be *flagged for review*. It must be **incapable of being registered**.

---

## 6. Consequences for the instrument registry

The classification also sharpens the instrument model. Recall that the instrument is the *economic identity of one unit*, and that instrument definition already requires two signatures (issuer + depository) precisely so that no single party can define bad collateral into eligibility.

The rights model adds a second reason for that safeguard: **"gold" is not one instrument.**

| Representation | Same metal? | Same instrument? | Collateral? |
|---|---|---|---|
| Allocated 1 kg kilobar, third-party vault, loco Zurich | — | Yes | **Yes** |
| Allocated 400 oz good-delivery bar, loco London | — | **No** — different format, fineness, venue | **Yes**, as a distinct instrument |
| Unallocated gold account | Nominally | **No** | **No** — a claim, not metal |
| Non-transferable bank gold certificate | Nominally | **No** | **No** — cannot be assigned |
| Consent-gated pledgeable certificate | Nominally | **No** | **Only** with consent obtained and committed |

A registry that admits all five under `XAU` has already lost. The `InstrumentKey` (issuer, depository, id, version) must distinguish them, and the depository co-signature is what makes that distinction credible: the custodian is attesting *it can actually hold and service this class*, which an issuer of unallocated claims cannot do.

---

## 7. Why this matters more than another contract function

It is tempting to treat this as onboarding paperwork — a checklist for the credit team, not an architectural concern. That is the mistake.

**The economic gate is a pricing question. The rights gate is an existence question.** Get the haircut wrong and the lender is under-secured. Get the rights wrong and the lender is *unsecured while believing itself secured*, which is the condition under which collateralised lending fails catastrophically rather than gradually.

Every downstream control in the system assumes the answer to the rights question is yes:

- the **no-double-pledge** control assumes there are identified bars to be uniquely keyed;
- **dual-control release** assumes a custodian who can actually block metal;
- the **enforcement-readiness pack** assumes a lawful realisation route exists;
- the **borrowing base** assumes the collateral is *there*.

If the rights gate is absent, all of those controls operate correctly on a holding that was never collateral. The system would be **precisely, verifiably, cryptographically wrong**.

That is the risk this document exists to close.

---

## 8. Conformance checklist

For any system claiming to admit gold as collateral:

- [ ] Does it distinguish **allocated** from **unallocated** holdings — and reject the latter?
- [ ] Can it be told that a non-transferable certificate is collateral? *(It must not be able to be.)*
- [ ] Where an instrument is pledgeable **only with consent**, does it require the consent to be **obtained and committed**, or does it assume it?
- [ ] Does it record who holds **title**, distinctly from who holds the **economic interest**?
- [ ] Does it reject holdings where the metal sits in the **counterparty's own vault**?
- [ ] Is a **lien search** a precondition, or an afterthought?
- [ ] Is the **enforcement route** established at onboarding, or discovered at default?
- [ ] Are ineligible holdings **haircut**, or **rejected**? *(A haircut cannot cure a missing security interest.)*
- [ ] Is the rights gate an **invariant** or a **procedure**? *(It must be impossible to register an ineligible holding, not merely improper.)*

The last question is the one to ask first.

---

## 9. Implementation status

**This document specifies a gap, not a shipped feature.** Stated plainly, because these documents are meant to be checkable against the repository:

- The **economic** gate exists: `FrameworkInstrumentEligibility` (haircut, max LTV, maintenance, eligibility commitment), enforced at line opening.
- The **physical** evidence exists: `LotEvidence` (manifest, uniqueness, quality, quantity, location).
- The **legal instruments** are committed: `control_agreement_hash`, `legal_terms_hash`.
- The **rights gate described in §5 is not yet implemented.** `RightsEvidence`, the machine-readable classification, and the `register_position` invariant are a specification, not code.

Anyone verifying this against `contracts/credit_ledger/` will find the first three and not the fourth. That is the correct expectation to set, and closing it is the highest-value item on the collateral-model roadmap — higher than any additional lifecycle function, because it is the gate on which every other control depends.

---

## Sources

The unallocated / non-transferable / consent-gated pattern described in §0 and §3 is drawn from the published *Terms and Conditions for Precious Metal – Gold* of Emirates NBD Bank (P.J.S.C.), and the contrasting institutional offering from the same bank's *Physical Gold & Silver Bullion Transaction Service* client FAQ, which markets gold loans, precious-metals repo, and pledgeable certificates. Both are public documents and are cited as a **worked example of the eligibility problem**, not as a criticism of the institution: the products are correctly designed for their purpose, and the purpose of the retail product is not to be collateral. That is exactly the point.

## Boundary reminder

Custody stays with the custodian. Ownership stays with the owner. Credit exposure stays with the lender. Argent records and enforces the authorised control state; it does not create the security interest, does not opine on title, and does not replace the legal determination that a holding is pledgeable. The rights gate consumes that determination — it does not perform it.

This document is an architecture specification, not legal advice. Where it and `argent-architecture.md` touch, the architecture document is authoritative.

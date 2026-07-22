# Argent: Programmable Collateral Control

> **Positioning status:** This document describes the control foundation shared by all Argent profiles. The implemented profile is secured credit; the target commercial profile is a non-cash-drawable reserve obligation facility. Read current function references as evidence of the implemented branch, not as a limit on the protocol direction.

**How physical collateral in custody becomes controllable, mobile, and lender-safe, without ever being tokenized.**

*This is a product and thesis document, not legal, banking, or investment advice. The forward-looking features described are not production commitments. Any production deployment would require jurisdiction-specific legal review, custody agreements, bank credit policy, security review, and independent audit. The committed build scope and its engineering sequence are defined separately in `product-roadmap.md`.*

---

## Status: what is live, what is direction

**Live now.** A working Argent prototype runs on Stellar testnet with 224 passing tests across the contract suite: pledge, valuation, two-step release, and settlement recorded as role-signed events.

**Confidentiality status.** Those contracts are a transparent reference profile and use synthetic data. Their exact values, participant addresses, stable identifiers, states, and replayable events are public. Production uses a confidential operating projection plus a minimized Soroban batch anchor; it does not publish the same event book.

**The control layer described here.** The next set of control primitives: substitution, multi-party controlled release, conditional release on verified sale, and partial release with a dynamic borrowing base. Forward-looking, not yet built.

**Longer-term.** Refinancing without custody disruption, lender assignment and participation, collateral baskets, and a secured-credit offer book. Noted here for direction only; out of scope for the near term. The automation extension of this control layer, self-triggered, policy-bound credit events modelled on the Eurosystem's T2S auto-collateralisation, is described separately in `auto-collateralisation-layer.md`.

The features below are ordered by institutional value. The committed build order, with its gate conditions and code-level extension points, lives in `product-roadmap.md`.

---

## Why this matters to a lender

The features below are not invented conveniences. Each maps to a documented, named pain point in institutional collateral operations.

Bank collateral processes today are largely manual and error-prone. Industry analysis finds that many banks still run on spreadsheet-based, fragmented workflows that produce operational errors, worst during volatility, when volume spikes trigger missed calls and fails without transparency into settled collateral [1]. The same analysis names two concrete risks from these manual processes: improper release of collateral assets, and misreporting of collateral valuations [2]. Argent's core answer to both is a single governed, role-based control projection: a release cannot occur without the required signatures, and every approved state version is anchored to a neutral integrity history. Exact valuation and release details remain private in production.

The harder structural problem the market has named is collateral mobility. The Bank of England's Project Meridian Securities identifies a central inefficiency as the difficulty of mobilising and substituting collateral quickly and efficiently, and points to programmability, embedding settlement conditions directly into transaction workflows, as part of the answer [3]. DTCC, the core of US securities settlement, frames its own tokenization work around speeding up collateral mobility without losing operational control [4].

Argent's chain choice sits directly inside that trend. In May 2026, DTCC and the Stellar Development Foundation announced plans to make DTC-custodied assets available on the Stellar public blockchain, targeting the first half of 2027 [4]. DTCC selected Stellar in part because asset-level controls are first-class in the protocol, supporting the regulated workflows institutional markets require [4]. Argent is built on the same property, first-class, protocol-level control, applied to physical collateral, which unlike securities has no live registry to mobilize against.

The timing is regulatory, not only technical. Under the final Basel III package, banks are being pushed toward more efficient collateral strategies and more active management of risk-weighted assets [5]. In Europe, the Eurosystem's Pontes initiative aims to enable settlement in central-bank money for DLT-based transactions, with an initial launch planned for the second half of 2026, under the broader Appia roadmap for a European tokenized financial ecosystem [6]. A European bank reviewing this direction is being told from above to modernize collateral handling and to prepare for on-chain settlement.

---

## The idea

Today the collateral-control layer is, in effect, a lockbox: collateral is pledged, held, and released as a whole. That is correct and safe, but a lockbox is a blunt instrument. Real secured lending is more flexible than "all locked or all free," and that flexibility, encoded on-chain with the lender's protections intact, is what makes the control layer genuinely useful to both sides.

The insight is not to make physical collateral "productive" in the way tokenized collateral can be. A silo of wheat cannot be staked or lent out on-chain, and Argent never tokenizes the asset. The insight is that the control lifecycle itself can be programmable: collateral can be substituted, released on verified conditions, released in part, and unlocked only by multi-party agreement. These are established mechanics in traditional secured and commodity finance [7], [8]. Argent's production contribution is to govern those facts privately and anchor their authorized state history publicly while the physical asset stays in custody and is never tokenized.

The governing constraint throughout, and it is non-negotiable: **no control action may ever leave the loan under-collateralized.** Every release, substitution, or partial unlock is checked against the lender's coverage requirement first. This mirrors established practice, where lenders use conservative release formulas to keep the remaining collateral sufficient to cover the outstanding debt [9]. The features below add flexibility for the owner without ever reducing the lender's protection below the agreed margin.

## What Argent does not verify

This boundary is central, so it is stated up front. Argent does not verify physical truth directly. It records signed attestations, evidence hashes, role authority, state transitions, and settlement facts. The legal and physical truth still comes from custodians, documents, inspections, agreements, and courts. Argent's value is that those facts become ordered, attributable, and impossible to rewrite inside the control record. The contract can make a release permitted, routable, and signable; the custodian still performs the physical release. This is the on-chain form of the "constructive possession" model of commodity finance, where the lender's interest is exercised through a custodian's acknowledged, attorned interest rather than physical possession [8].

## The analogy we borrow, and the boundary we keep

Digital collateral platforms have shown that escrow is more valuable when the locked asset stays useful, through mechanisms like flexible terms, refinancing, and secondary liquidity. Argent translates that idea to physical collateral, but with a stricter boundary: no on-chain yield, no transfer of the asset, no rehypothecation of the same lot, and no claim that the chain verifies the physical asset itself. The physical equivalent of "collateral with utility" is controlled business use while pledged: substitution, partial release, conditional sale release, and live borrowing-base monitoring.

A note on the collateral model. Argent's V5 direction maps to how institutional collateral is described in standard frameworks: an instrument, an eligibility treatment, and risk parameters (`haircut_bps`, `max_ltv_bps`, `maintenance_bps`), with valuation references and lifecycle events. This aligns with the ISDA Common Domain Model's treatment of collateral through asset descriptors, eligibility criteria (including AND/OR/NOT-style logic), and haircut concepts [10].

---

## Feature 1: Substitution and collateral mobility

**What it is.** Swap one attested batch of collateral for another eligible batch without unwinding and re-establishing the entire pledge.

**Why a bank cares.** This maps most directly to the pain the market has named. The Bank of England identifies the difficulty of mobilising and substituting collateral quickly as a core inefficiency [3]. In commodity finance, inventory is not a static museum object: goods are routinely rotated, replaced, sold, and processed while pledged, and the lender's need is control over those state changes, not permanent physical possession of every item [7], [8]. Substitution encodes that reality. The owner keeps operating, rotating stock in and out of the pledge, without the cost and friction of closing and reopening the loan each time. This is "collateral with utility" for physical goods: not on-chain yield, but the operational mobility a working producer and a modern lender both need.

**The guardrail.** A substitution executes only if, after the swap, the post-substitution borrowing base, computed under the lender's haircut, eligibility treatment, and LTV policy, still covers the outstanding exposure and the required margin. Equal nominal value is not the test: a new lot with a worse haircut, lower grade, or lower liquidity may be insufficient even at the same face value, and an over-collateralized line may admit a smaller replacement. The old lot is not released until the replacement lot is attested, admitted, valued, and immobilized.

**On-chain shape.** A substitution event references the outgoing and incoming attested units, verifies the incoming attestation and recomputes coverage, then, only if coverage holds, pledges the new and releases the old in a single signed control event. The audit trail records both sides of the swap.

## Feature 2: Multi-party controlled release

**What it is.** Release governed by a configurable policy over the required signer set, so no single party can free collateral alone.

**Why a bank cares.** Improper release of collateral assets is named directly as one of the concrete risks of today's manual, fragmented processes [2]. Argent's role-signed release is the structural prevention: a release event is valid only when the loan's release policy (an M-of-N signer set) is satisfied, enforced by the contract before anything is signed.

**Configurable, not a borrower veto.** Release policies are set per loan. A normal *voluntary* release may require owner, bank, and custodian signatures, so the owner's goods cannot be released, and thus exposed, without their consent. An *enforcement* release, after a defined default, follows the pre-agreed enforcement waterfall and typically requires bank and custodian signatures with owner notification, not owner veto. This preserves the borrower's protection in the normal course without handing the borrower a veto over enforcement.

**On-chain shape.** Each loan carries a release policy defining the required signer set and threshold for each release type. This maps directly onto the institutional signing layer's approval-policy model (M-of-N approvals enforced before a transaction is signed), so it composes with the existing DFNS-governed architecture rather than requiring new trust machinery.

## Feature 3: Conditional release on verified sale

**What it is.** Release that becomes available when a defined, verified condition is satisfied, most importantly a verified sale of the collateral, with repayment routed from the sale proceeds.

**Why a bank cares.** This is the programmable-settlement workflow that market infrastructures are actively building toward: settlement conditions bound to the transaction, so proceeds are captured before control changes [3], [4]. It solves the borrower's real need, selling pledged inventory without manually unwinding the loan first, while giving the lender proceeds-first protection. Traditional secured lending already does this in legal language: the borrower sells through an approved broker, the broker irrevocably undertakes to deliver a portion of the proceeds to the lender, and on that delivery the security interest is released [9]. Argent encodes the same logic. It is also the primitive for future sale-backed inventory finance: repayment from verified sale proceeds, followed by controlled release.

**How the release actually works.** The condition being met makes release *available*: the contract verifies the sale attestation and the tied repayment, then routes the required approvals and records the release path. The custodian still performs the physical release. The chain does not move goods; it authorizes and records who may.

**The hard part, stated honestly.** The difficulty is not the release logic; it is the trustworthy attestation that a binding sale has occurred. That is an edge-attestation problem, requiring integration with the owner's sales or contract system or the buyer's confirmed commitment. The on-chain release is straightforward once the attestation is trustworthy.

**The guardrail.** Repayment and release are bound together: release cannot be recorded without the tied repayment settling, and repayment cannot be skipped on release, so the asset can never be both pledged and sold. The same discipline extends to insurance or loss events, where proceeds should follow the secured path rather than silently releasing the collateral state.

## Feature 4: Partial release and dynamic borrowing base

**What it is.** Release a portion of the pledged collateral while the remainder continues to secure the outstanding debt, triggered as the loan is repaid or as collateral value rises. Paired with a live borrowing base that revalues against an approved price source rather than a periodic spreadsheet.

**Why a bank cares.** Two documented pain points converge here. A static, manually maintained borrowing base is exactly the spreadsheet-bound process the industry flags as error-prone [1]; a live on-chain revaluation with freshness and confidence checks replaces it. And Basel III's pressure toward more efficient collateral strategies rewards releasing excess coverage promptly rather than over-immobilizing a borrower's assets [5]. For the owner, partial release means progressively regaining the free use of part of their goods as the debt falls, which for a producer whose inventory is working capital is materially valuable. Warehouse-receipt finance already advances against a discounted fraction of value (commonly 50 to 80 percent depending on the commodity and system) [7], so releasing the excess above the required margin is a natural, well-understood operation.

**Trigger types (established, encodable as contract logic):** loan-to-value milestones (the contract computes current coverage and releases only the excess above the required margin), scheduled review points (a graduated release schedule), and repayment thresholds.

**The guardrail.** A partial release executes only if the value of the remaining collateral, after the release, still covers the outstanding debt at the agreed haircut. The contract enforces this before any release event is signed. This is the on-chain encoding of the conservative release formula prudent lenders already require [9].

---

## How these fit together

The four features are one coherent upgrade: the control layer moves from whole-pledge release to programmable release, substitution, and partial unlock, where collateral can be substituted, released on verified conditions, released in part, and unlocked only by multi-party agreement, always subject to the coverage guardrail. Together they create the primitives from which richer instruments, including sale-backed and time-bound inventory finance, can later be composed.

## What this deliberately does not do

The credibility of the model depends as much on the boundaries as on the features. Argent does not pursue: tokenization of the physical asset; any claim that the chain verifies physical truth; automatic physical enforcement; on-chain yield against physical inventory; rehypothecation of the same physical lot; or owner-side unilateral release. Each would weaken the boundary that makes the model defensible to a bank. That most tokenized real-world assets remain thinly traded despite the technology being available [11] reinforces the choice: the hard, valuable problem is control and mobility, not issuing another token.

## Honest engineering note

None of this is a quick add. The prototype works today with 224 passing tests. Each feature is new contract surface: new state (divisible pledged sets, substitution references, release policies), new invariants (the coverage check must hold across every path), new tests (especially adversarial: attempts to release into under-collateralization, to substitute in over-valued or unattested collateral, to release without the required signer set), and new audit exposure. The coverage guardrail in particular must be proven to hold on every release, substitution, and partial-unlock path, because it is the single rule that protects the lender. This is deliberate, staged work, sequenced after the current contracts and their protections are settled.

## A note on build order

The features above are presented by institutional value: substitution and mobility matter most to a lender, so they lead. The engineering order is different, sequenced by contract complexity and risk, and the committed build plan, with its gate conditions and code-level extension points, lives in `product-roadmap.md`. As an indication, the likely order is:

1. **Multi-party controlled release** first. Smallest surface (a release policy plus signer-set enforcement), composes directly with the existing signing layer, strengthens safety immediately.
2. **Partial release and dynamic borrowing base** next. Clear owner benefit, encodes the well-established loan-to-value-milestone mechanic, with the coverage check as the core invariant.
3. **Conditional release on verified sale** third. Higher value but gated on the harder edge-attestation problem, so sequenced after the release machinery is solid.
4. **Substitution** last as a build. The most operationally complex (two-sided attest-and-value check in one atomic event), best built once partial release and coverage recomputation are proven, even though it is the highest-value feature to a lender.

---

## References

[1] Ernst & Young, "Transforming collateral management functions for regional banks," EY Insights, Banking & Capital Markets.

[2] Ernst & Young, "How banks can align collateral functions to a services-based model," EY Insights, Banking & Capital Markets.

[3] Bank of England and BIS Innovation Hub London Centre, "Project Meridian Securities," Bank of England.

[4] The Depository Trust & Clearing Corporation and Stellar Development Foundation, "DTC's tokenization service to connect with Stellar public blockchain as DTC advances its multi-chain strategy," DTCC Press Release, May 2026.

[5] Accuracy, "2026 strategic challenges for the banking sector," 2026.

[6] European Central Bank, "Pontes and the Appia roadmap for a European tokenised financial ecosystem," ECB, 2026.

[7] World Bank Group, "Using commodities as collateral for finance (commodity-backed finance)," World Bank.

[8] Baker McKenzie, "Commodity finance: the complete security package," 2021.

[9] World Bank Group, "Can warehouse receipts unlock farmer finance?," World Bank; and standard secured-lending partial-release drafting practice (illustrative).

[10] FINOS / ISDA, "Common Domain Model: eligible collateral representation," ISDA/FINOS.

[11] R. Mafrur, "Tokenize everything, but can you sell it? RWA liquidity challenges and the road ahead," arXiv:2508.11651, 2025.

# Argent: Post-Grant Roadmap

**Direction beyond the funded SCF build. Not in scope for the SCF Build grant.**

*This document is explicitly out of scope for the Argent SCF Build (Integration Track, DFNS) submission. Nothing here is a funded deliverable, a milestone, or a budget line for that grant. The funded work is defined solely in `argent-architecture.md` and the application: take the tested Soroban prototype to a DFNS-governed mainnet reference deployment in twelve weeks, with a reusable DFNS and Soroban authorization adapter. This file records where the product goes after that, so contributors and reviewers can see the direction without mistaking it for the committed scope.*

---

## 1. How to read this document

The SCF Build grant funds one thing: a working, DFNS-governed, single-facility collateral-control application on Stellar mainnet, with the reusable authorization adapter open-sourced. That scope is deliberately tight, and it is complete on its own.

This roadmap describes the institutional product that the funded build makes possible. It is written so that a reviewer can confirm two things at a glance: that the team understands where this goes, and that none of it is being smuggled into the funded period. Every item here is post-mainnet, post-traction, and contingent on the core shipping first.

The guiding principle is unchanged from the architecture document. Argent is a collateral book of record for physical assets under custody. The funded build proves that book for a single facility. The roadmap extends it to the way a bank actually runs a collateral operation: pools, positions, scheduled revaluation, and safe substitution.

## 2. Completion gate: what must be true before this roadmap starts

No roadmap work begins until the funded SCF build is complete and proven on mainnet. Completion means all of the following are true:

1. Argent is live on Stellar mainnet.
2. The full reference lifecycle runs under DFNS-governed role wallets.
3. Bank, custodian, verifier, sponsor, and operator roles are separated by wallet and policy.
4. The DFNS and Soroban authorization adapter is published open-source.
5. The policy decoder maps Soroban action, method, role, and reference before signing.
6. Release and enforcement are governed by role approvals and recorded on-chain.
7. Settlement reduces exposure through the configured Stellar settlement asset.
8. The event indexer and evidence certificate prove approval-to-transaction history.
9. A pilot package exists for a bank, custodian, and sponsor to review.
10. The core boundaries remain explicit: Argent is not a bank, custodian, card issuer, commodity token, or legal-enforcement engine.

Only after those conditions hold should anything below become active.

## 3. What already exists to build on

The roadmap is not a fresh design. Several of its foundations are already present as shipped, tested contract functions in the open-source core, in primitive single-facility form:

- **Revaluation and margin.** `revalue_and_check` already revalues a position and evaluates margin state against policy. The roadmap schedules and scales this, it does not invent it.
- **Adjustment and substitution.** `request_collateral_adjustment`, `bank_approve_adjustment`, and `custodian_confirm_adjustment` already model a three-party collateral change. The roadmap hardens this into a no-unsecured-gap substitution flow.
- **Borrowing base and exclusivity.** The contract already enforces one credit line per pledge, a borrowing base, and refusal of double-pledge. The roadmap generalizes these from one facility to a pool.

In other words, the roadmap turns primitives that are proven at the single-facility level into the operating model a bank uses across many positions. That is product maturation, not new research.

## 4. Collateral-pool and position model

The first reference implementation binds one specific collateral set to one specific credit line. The natural institutional extension is a pool model, where a bank manages many positions against shared eligibility and policy. This mirrors how a mature collateral operation is actually run, through asset accounts, collateral pools, and mobilisation against shared eligibility, rather than as isolated single pledges [1].

Candidate state, for a future major version (illustrative, not committed): a pool identifier and collateral account a position belongs to; collateral earmarked against a line but not yet pledged; the live composition of the pool across free, pledged, pending-release, and pending-substitution value; and the shortfall when revaluation puts a position below its margin threshold.

The deliverable that matters here is not the fields. It is the output: a **collateral position report** a bank can read as a current control view, what exists, what is pledged, what is free, what is pending release, what is under margin call, what is in default, and the evidence behind each state. That report is the book-of-record idea taken from a single facility to a portfolio, the same decision-ready, current-state record that institutions build because accounting and custody books alone are too slow or fragmented [2].

## 5. Scheduled revaluation and margin operations

`revalue_and_check` exists today as an on-demand function. The operational model a bank expects is a scheduled process: a daily, or policy-defined, revaluation pass across a pool, a margin check against threshold, and an auditable margin-call record when a position falls short. Daily valuation, risk control measures, and margin calls are standard components of how central-bank-grade collateral operations are run [1], and a sound margin and haircut policy depends on liquidation horizon, market risk, and confidence level rather than a single static ratio [3].

Roadmap scope: a scheduled revaluation pass over all live positions in a pool, each writing a timestamped valuation and margin state; a margin-call lifecycle (issue, cure window, cure or escalate) expressed as role-signed events in the same style as the existing default and enforcement flow; and stale-price and data-quality refusal at the pool level, extending the per-position stale-price rule already enforced today.

This stays inside the product boundary. Argent records and enforces the margin state the bank's policy defines. It does not compute the bank's risk model, and it never moves the asset.

## 6. Safe collateral substitution

The architecture document and the DTCC, Clearstream, Euroclear, and BCG interoperability framework both point at the same hard requirement: a borrower may need to swap pledged collateral without unwinding the facility, and the swap must never leave a moment of unsecured exposure [4].

The invariant: **the new collateral is secured before the old collateral is released.** The roadmap flow, building on the existing adjustment functions:

1. Owner proposes substitute collateral.
2. Custodian attests the substitute exists and is held.
3. Bank approves the substitute against eligibility and borrowing base.
4. The new collateral locks.
5. Only then does the old collateral release.
6. The event sequence proves no unsecured gap existed at any point.

This is a direct extension of `request_collateral_adjustment`, `bank_approve_adjustment`, and `custodian_confirm_adjustment`, ordered so that release can never precede the new lock.

## 7. Haircut and valuation policy metadata

Argent enforces the bank's approved haircut output today: valuation, borrowing base, maximum LTV, margin threshold, stale-price rule, and cure trigger. The boundary is firm and stays firm: Argent does not determine the haircut model; the bank supplies the approved policy and Argent enforces its result.

A future version may store additional policy metadata as hashes and typed fields so that the enforced result is fully attributable to a named, versioned policy, for example a liquidation-horizon assumption, a confidence level, a marketability class, a data-quality class, and a stress-scenario reference [3]. These would be recorded as policy inputs the contract enforces against, never as a risk engine the contract runs. The distinction is the whole point: enforcement is in scope; risk modelling is the bank's, permanently. This matters because collateral is not safe merely by existing: its own value is volatile, and heavily collateralized loans can carry more risk when that value drifts, which is exactly the drift a live borrowing base and margin state make visible [5].

## 8. Asset categories beyond gold

The contract core is already asset-agnostic; gold is the first proof, not the limit. Post-grant, the same control structure binds other custody-stable physical assets through asset-specific identity, custody, valuation, and document hashes: base metals and critical minerals, agricultural warehouse receipts, energy inventory, and serialised industrial collateral. Each new category binds through a narrow identity-and-valuation adapter on an unchanged control core, so a new asset adds an adapter, not a new contract product and not a new set of legal assumptions. The bank still controls eligibility, borrowing base, release, and enforcement; the custodian still signs existence and custody state. Banks already classify collateral and credit-risk mitigation by type, including physical and other funded protection, so each category maps onto a framework the bank already uses rather than a new one Argent invents [6].

## 9. Alignment with the Stellar growth path

This roadmap is also the basis for Stellar ecosystem growth support that becomes available only after a successful mainnet launch with verifiable usage. The Stellar Community Fund's post-launch pathways are explicitly gated on live mainnet deployment and demonstrated traction, and are not an automatic continuation of the Build Award. The sequence is therefore deliberate:

1. Ship the funded build: DFNS-governed single-facility control on mainnet, adapter open-sourced.
2. Demonstrate real usage with one or more institutional pilots.
3. On that evidence, pursue ecosystem growth support to build the pool model, scheduled revaluation, and substitution described above.

Argent treats SCF as a launchpad rather than a long-term plan, which is how the program is designed to be used. The funded build earns the right to this roadmap; it does not presume it.

This document does not assume entitlement to any further funding. It identifies, without quantifying, where post-program support would be useful if Argent completes the mainnet launch and shows traction: ecosystem and partner introductions; additional technical review of the Soroban signing path, indexer, and evidence model; further security and audit review of the contracts, the DFNS signing path, and the policy decoder; and any additional grant or investment pathway, only after the first build is complete, only against specific post-grant milestones, and only for expansion work outside the funded scope.

## 10. Why this is useful to the ecosystem, not only to Argent

Post-grant work is mutually useful: done well, it advances the published direction of the Stellar Development Foundation, of DFNS, and of the Stellar x CV Labs accelerator, not only Argent. This section states that case plainly. It claims no partnership or endorsement, and it remains contingent on the funded build shipping first.

**For SDF and Stellar.** SDF has made real-world-asset infrastructure and DeFi composability a stated 2026 priority [7], including a direct strategic investment in compliance-first on-chain credit infrastructure with collateral monitoring and institutional liquidation [8]. Argent is complementary to that direction. Where most RWA work tokenizes ownership so an asset can trade, Argent governs control while ownership and the asset stay in place, the case where a bank wants control rather than a transferable token. That gives Stellar a reference pattern for physical collateral that does not fit the tokenization model, and a reusable open-source authorization adapter other institutional builders can fork. It also exercises Stellar settlement assets where payment is real, repayment and exposure reduction, rather than as a demo. The detachable, independently-signed authorization entry that this depends on is the primitive SDF identifies as Stellar's structural advantage [9].

**For DFNS.** DFNS is publicly repositioning from wallet infrastructure toward a governed operating layer for institutional digital-asset workflows, with a policy engine, governance, and policy-aware service accounts at the centre [10], [11], [12], [13]. Argent is a hard, non-trivial proof of that thesis on Soroban: a multi-party workflow where release, enforcement, custody confirmation, spend evidence, and reward approval each belong to a different authority and no operator can sign for all of them. The funded adapter is the Soroban-shaped piece that connects DFNS governance to Stellar's authorization model, which is reusable by any other Stellar builder facing the same multi-party signing problem.

**For Stellar x CV Labs.** The accelerator targets EMEA builders working on practical financial infrastructure across payments, RWA, and tokenization, with Demo Day at Meridian [14]. Argent fits the institutional RWA side: a European founder, a conservative collateral-finance thesis, and a product aimed at banks, custodians, refiners, commodity finance, and secured lending. Post-program support that turns a technical mainnet launch into institutional conversations, bank pilots, custodian design partners, and asset-backed-finance investors, advances the accelerator's own goal of converting builders into deployed ecosystem traction.

## 11. Explicitly excluded from the funded scope

To remove any ambiguity for a reviewer, none of the following are funded deliverables in the current application. They are post-grant only:

- collateral-pool accounting and the position report;
- scheduled daily revaluation jobs;
- automated margin-call operations;
- safe-substitution production tooling;
- haircut-policy metadata beyond the enforced output that exists today;
- multi-asset production rollout beyond the gold proof;
- full bank or custodian core-system integration;
- legal-enforceability tooling;
- production KYC, AML, and sanctions infrastructure;
- general-purpose collateral-management software as a service;
- direct card-network integration;
- asset tokenization;
- regulatory-approval work;
- full production audit beyond available program support;
- any Series-A product buildout.

These may matter later. None of them is required to complete the DFNS-governed mainnet reference application, and nothing in this document changes the funded twelve-week scope.

## 12. Reviewer summary

The funded grant is a focused twelve-week integration build: move a tested Soroban prototype from local signers to DFNS-governed institutional role wallets, and launch the reference lifecycle on mainnet with a reusable authorization adapter open-sourced.

The post-grant roadmap begins only after that is complete and proven. It extends Argent into a collateral book of record for physical assets under custody: collateral pools, a live position report, scheduled revaluation and margin operations, safe substitution, policy-attributable haircut metadata, and asset categories beyond gold. Much of it builds directly on primitives already shipped and tested in the open-source core. The roadmap is included to show that Argent has a serious institutional path. It does not enlarge the funded scope.

---

## References

Independent sources, cited to evidence the ecosystem direction described in Section 10. No partnership or endorsement by any named organization is implied.

[1] European Central Bank, "Collateral management in Eurosystem credit operations: information for Eurosystem counterparties," European Central Bank, 2026. https://www.ecb.europa.eu

[2] MSCI Inc., "The Investment Book of Record (IBOR)," MSCI, 2026. https://www.msci.com

[3] International Monetary Fund, "A Quantitative Approach to Central Bank Haircuts and Counterparty Risk Management," IMF Working Papers, 2024. https://www.imf.org

[4] DTCC, Clearstream, Euroclear, and Boston Consulting Group, "Building the Path Towards Digital Asset Securities Interoperability," February 2026.

[5] K. Koufopoulos, D. McGowan, S. Perdichizzi, A. Reghezza, and M. Spaggiari, "Risky collateral and default probability," ECB Working Paper Series, No. 3167, 2026. https://www.ecb.europa.eu; and H. Degryse, O. De Jonghe, L. Laeven, and T. Zhao, "Collateral and credit," ECB Working Paper Series, No. 3095 (also NBB Working Paper No. 482), 2025. https://ssrn.com/abstract=5391378

[6] Société Générale, "Pillar 3 Risk Report, 30.06.2025," Société Générale, 2025. https://www.societegenerale.com

[7] Stellar Development Foundation, "Building the Future of Global Finance: SDF's 2026 Strategy," Stellar Development Foundation, 2026. https://stellar.org/foundation/strategy

[8] Stellar Development Foundation, "Stellar Development Foundation Makes Strategic Investment in Ascend to Accelerate Compliant RWA Infrastructure Development," May 2026. https://stellar.org/press/stellar-development-foundation-makes-strategic-investment-in-ascend-to-accelerate-compliant-rwa-infrastructure-development

[9] L. McCulloch, "Stellar's composable auth model," Stellar Development Foundation, May 5, 2026. https://stellar.org/blog/foundation-news/stellars-composable-auth-model

[10] DFNS, "Core Banking Platform for Digital Assets," DFNS. https://dfns.co

[11] C. Hagège and C. Grilhault des Fontaines, "We're Not a Wallet Company Anymore," DFNS, June 3, 2026. https://dfns.co/article/were-not-a-wallet-company-anymore

[12] T. de Lachèze-Murel, "Introducing Governance Engine," DFNS, June 10, 2026. https://dfns.co/article/introducing-governance-engine

[13] H. Tross, "Introducing Policy-Aware Service Accounts," DFNS, April 27, 2026. https://dfns.co/article/introducing-policy-aware-service-accounts

[14] CV Labs and Stellar Development Foundation, "Stellar x CV Labs Accelerator," 2026. https://cvlabs.com

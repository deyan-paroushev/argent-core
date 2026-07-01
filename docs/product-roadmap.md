# Argent: Product Roadmap

**Direction beyond the current committed build.**

*This document records product direction beyond the current committed build. Nothing here is shipped yet; it is where the product goes after the present milestone. The current committed scope is defined in `argent-architecture.md`: take the tested Soroban prototype to a DFNS-governed mainnet reference deployment, with a reusable DFNS and Soroban authorization adapter. This file records where the product goes after that, so contributors can see the direction without mistaking it for what is built today. For the conceptual model behind these features, the collateral-control thesis, the bank pain points it addresses, and the institutional grounding, see `collateral-control.md`.*

---

## 1. How to read this document

The current committed build is one thing: a working, DFNS-governed, single-facility collateral-control application on Stellar mainnet, with the reusable authorization adapter open-sourced. That scope is deliberately tight, and it is complete on its own.

This roadmap describes the institutional product that the current build makes possible. It is written so that any reader can confirm two things at a glance: that the direction is understood, and that none of it is being confused with what ships today. Every item here is post-mainnet, post-traction, and contingent on the core shipping first.

The detail is deliberate. Where the current build is kept narrow on purpose, this document is specific on purpose: it shows the later layers as concrete extensions of the types and functions that already exist in the open-source core, with real signatures and fields, so a reader can see that the long-term direction is engineered rather than aspirational.

The guiding principle is unchanged from the architecture document. Argent is a collateral book of record for physical assets under custody. The current build proves that book for a single facility. The roadmap extends it to the way a bank actually runs a collateral operation: pools, positions, scheduled revaluation, and safe substitution.

## 2. Prerequisites: what must be true before this roadmap starts

No roadmap work begins until the current build is complete and proven on mainnet. Completion means all of the following are true:

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

The roadmap is not a fresh design. Its foundations are already present as shipped, tested contract types and functions in the open-source core, in primitive single-facility form. The later work extends these exact types; it does not replace them.

- **Revaluation and margin.** `revalue_and_check` already writes a `LineValuation` and evaluates `MarginState` (`Covered`, `Warning`, `Called`) against the bank's policy, validating price freshness and confidence before a margin decision. The roadmap schedules and scales this across a pool; it does not invent it.
- **Adjustment and substitution.** The `CollateralAdjustment` state machine already exists with `AdjustmentType::Substitution` and an `AdjustmentStatus` flow (`Requested`, `CustodianConfirmed`, `Approved`, `Rejected`) cleared by all three parties through `request_collateral_adjustment`, `bank_approve_adjustment`, and `custodian_confirm_adjustment`. The roadmap hardens this into a strict no-unsecured-gap ordering.
- **Borrowing base and exclusivity.** `CreditLine` already carries `ltv_bps` and `maintenance_bps`, and the contract enforces `ltv_bps < maintenance_bps <= 10000` so a line can never be configured to lend past the value of its collateral. It already refuses a second line against the same pledge. The roadmap generalizes these from one facility to a pool.

In other words, the roadmap turns types and invariants that are proven at the single-facility level into the operating model a bank uses across many positions. That is product maturation, not new research, and it is why a reviewer should weigh the long-term direction: the current build is not a dead-end demo but the first working layer of a system whose later layers are already designed against real instruments.

## 4. Collateral-pool and position model

The first reference implementation binds one specific collateral set to one specific credit line. The natural institutional extension is a pool model, where a bank manages many positions against shared eligibility and policy. This mirrors how a mature collateral operation is actually run, through asset accounts, collateral pools, and mobilisation against shared eligibility, rather than as isolated single pledges [1].

The extension is a new `CollateralPool` record plus pool references on the existing position types. Illustrative, not committed for the current build:

```rust
#[contracttype]
pub struct CollateralPool {
    pub pool_id: BytesN<32>,
    pub bank: Address,
    pub custodian: Address,
    pub collateral_account_id: BytesN<32>,
    pub eligible_schedule_hash: BytesN<32>,   // shared eligibility for the pool
    pub free_value: i128,                      // unencumbered, available to pledge
    pub reserved_value: i128,                  // earmarked to a line, not yet pledged
    pub pledged_value: i128,                   // actively securing drawn credit
    pub pending_release_value: i128,           // release authorized, not yet confirmed
    pub pending_substitution_value: i128,      // mid-substitution, in transition
    pub margin_deficit: i128,                  // shortfall across called positions
    pub valued_at_ledger: u32,
}
```

```rust
// added to the existing VaultPosition
pub pool_id: BytesN<32>,        // the pool this position belongs to
pub reserved_for: BytesN<32>,   // credit_line_id it is earmarked to, or zero
```

The deliverable that matters here is not the fields. It is the output: a **collateral position report**, a bank-readable current control view assembled from pool and position state and the event log, what exists, what is pledged, what is free, what is pending release, what is under margin call, what is in default, and the evidence behind each state. That report is the book-of-record idea taken from a single facility to a portfolio, the same decision-ready, current-state record institutions build because accounting and custody books alone are too slow or fragmented [2].

## 5. Scheduled revaluation and margin operations

`revalue_and_check` exists today as an on-demand function. The operational model a bank expects is a scheduled process: a daily, or policy-defined, revaluation pass across a pool, a margin check against threshold, and an auditable margin-call record when a position falls short. Daily valuation, risk control measures, and margin calls are standard components of how central-bank-grade collateral operations are run [1], and a sound margin and haircut policy depends on liquidation horizon, market risk, and confidence level rather than a single static ratio [3].

The roadmap adds a scheduled pass and a first-class margin-call object, reusing the existing `LineValuation` and `MarginState`:

```rust
// Scheduled revaluation: iterate live positions in a pool, write a
// LineValuation for each, and open a MarginCall where MarginState::Called.
pub fn revalue_pool(env: Env, pool_id: BytesN<32>, prices_hash: BytesN<32>) -> u32;

// Margin-call lifecycle, role-signed, in the same style as default/cure/enforce.
pub fn issue_margin_call(env: Env, credit_line_id: BytesN<32>) -> BytesN<32>;
pub fn record_margin_cure(env: Env, margin_call_id: BytesN<32>, cure_hash: BytesN<32>);
pub fn escalate_margin_call(env: Env, margin_call_id: BytesN<32>);
```

```rust
#[contracttype]
pub enum MarginCallStatus { Open, Cured, Escalated, Expired }

#[contracttype]
pub struct MarginCall {
    pub margin_call_id: BytesN<32>,
    pub credit_line_id: BytesN<32>,
    pub deficit: i128,                 // shortfall at the acted-on price
    pub opened_at_ledger: u32,
    pub cure_by_ledger: u32,           // cure window, policy-defined
    pub status: MarginCallStatus,
}
```

This stays inside the product boundary. Argent records and enforces the margin state the bank's policy defines. It does not compute the bank's risk model, and it never moves the asset. A stale or low-confidence price is refused at the pool level exactly as it is per-position today.

## 6. Safe collateral substitution

The architecture document and the DTCC, Clearstream, Euroclear, and BCG interoperability framework both point at the same hard requirement: a borrower may need to swap pledged collateral without unwinding the facility, and the swap must never leave a moment of unsecured exposure [4].

The invariant: **the new collateral is secured before the old collateral is released.** This hardens the existing `CollateralAdjustment` machine (`AdjustmentType::Substitution`) into a strictly ordered, gap-free flow:

```rust
// Each step advances AdjustmentStatus; the contract refuses release of the
// old set until the new set is locked, so no unsecured window can exist.
pub fn request_substitution(env: Env, credit_line_id: BytesN<32>, new_barlist_hash: BytesN<32>, new_weight_oz_e7: i128, request_hash: BytesN<32>) -> BytesN<32>;
pub fn attest_substitute_collateral(env: Env, adjustment_id: BytesN<32>);  // custodian: exists and held
pub fn bank_approve_substitution(env: Env, adjustment_id: BytesN<32>);     // coverage holds at current price
pub fn lock_substitute_collateral(env: Env, adjustment_id: BytesN<32>);    // new set immobilized FIRST
pub fn release_replaced_collateral(env: Env, adjustment_id: BytesN<32>);   // only callable after lock
pub fn confirm_substitution_complete(env: Env, adjustment_id: BytesN<32>); // evidence: no gap
```

Ordering is the safety property: `release_replaced_collateral` reverts unless `lock_substitute_collateral` has succeeded for the same adjustment, so release can never precede the new lock. The event sequence is the proof of continuous secured coverage.

## 7. Haircut and valuation policy metadata

Argent enforces the bank's approved haircut output today: valuation, borrowing base, maximum LTV (`ltv_bps`), maintenance threshold (`maintenance_bps`), stale-price rule, and cure trigger. The boundary is firm and stays firm: Argent does not determine the haircut model; the bank supplies the approved policy and Argent enforces its result.

A future version stores the policy as a versioned, hash-anchored record so the enforced result is fully attributable to a named policy, and stores typed risk inputs the contract checks against [3]:

```rust
#[contracttype]
pub struct HaircutPolicy {
    pub policy_id: BytesN<32>,
    pub policy_hash: BytesN<32>,        // anchors the off-chain approved policy doc
    pub haircut_bps: u32,               // applied discount to market value
    pub max_ltv_bps: u32,               // advance rate ceiling
    pub maintenance_bps: u32,           // margin-call threshold
    pub liquidation_horizon_days: u32,  // assumed time to realize
    pub confidence_level_bps: u32,      // VaR-style confidence used by the bank
    pub marketability_class: u32,       // liquidity bucket of the asset
    pub data_quality_class: u32,        // valuation-source reliability bucket
    pub valuation_source_hash: BytesN<32>,
    pub stress_scenario_hash: BytesN<32>,
    pub version: u32,
}
```

These are policy inputs the contract enforces against, never a risk engine the contract runs. The distinction is the whole point: enforcement is in scope; risk modelling is the bank's, permanently. This matters because collateral is not safe merely by existing: its own value is volatile, and heavily collateralized loans can carry more risk when that value drifts, which is exactly the drift a live borrowing base and margin state make visible [5].

## 8. Asset categories beyond gold

The contract core is already asset-agnostic; gold is the first proof, not the limit. Beyond the current build, the same control structure binds other custody-stable physical assets through asset-specific identity, custody, valuation, and document hashes: base metals and critical minerals, agricultural warehouse receipts, energy inventory, and serialised industrial collateral. Each new category binds through a narrow identity-and-valuation adapter on an unchanged control core, so a new asset adds an adapter, not a new contract product and not a new set of legal assumptions. The bank still controls eligibility, borrowing base, release, and enforcement; the custodian still signs existence and custody state. Banks already classify collateral and credit-risk mitigation by type, including physical and other funded protection, so each category maps onto a framework the bank already uses rather than a new one Argent invents [6].

The adapter is a thin per-asset binding, not a fork of the core:

```rust
#[contracttype]
pub struct AssetAdapter {
    pub asset_type: u32,                  // metal, warehouse receipt, energy, industrial
    pub asset_identity_hash: BytesN<32>,  // serial / lot / receipt / batch reference
    pub custody_location_hash: BytesN<32>,
    pub quantity_e7: i128,
    pub quality_or_grade_hash: BytesN<32>,
    pub valuation_source_hash: BytesN<32>,
    pub inspection_certificate_hash: BytesN<32>,
    pub insurance_hash: BytesN<32>,
    pub eligibility_schedule_hash: BytesN<32>,
}
```

The lifecycle, roles, pledge exclusivity, borrowing base, release, and enforcement are unchanged; only identity and valuation are asset-specific. That is what lets one tested core serve many asset classes, and it is the long-term reason this is infrastructure rather than a single-asset app, the point a reviewer weighing durability should register.

## 9. Sequencing

The direction in this document is deliberately sequenced. Each stage earns the next; nothing here is assumed:

1. Ship the current build: DFNS-governed single-facility control on mainnet, adapter open-sourced.
2. Demonstrate real usage with one or more institutional pilots.
3. On that evidence, build the pool model, scheduled revaluation, and substitution described above.

The build earns the right to this roadmap; it does not presume it. This document assumes no entitlement to any particular resource or outcome. Where later work would benefit from outside support, whether ecosystem introductions, additional technical and security review of the Soroban signing path, indexer, and evidence model, or further audit of the contracts, the DFNS signing path, and the policy decoder, that support is useful only after the first build is complete, only against specific milestones, and only for expansion work outside the current scope.

## 10. How Argent fits the Stellar and DFNS ecosystem

Argent is built natively on two platforms' primitives, and the fit is worth stating plainly. This section claims no partnership or endorsement; it explains how the design aligns with the published technical direction of Stellar and DFNS, and cites their own writing as the reference.

**Stellar.** Real-world-asset infrastructure and DeFi composability are a stated Stellar ecosystem priority [7], [8]. Argent is complementary to that direction. Where most RWA work tokenizes ownership so an asset can trade, Argent governs control while ownership and the asset stay in place, the case where a bank wants control rather than a transferable token. That gives Stellar a reference pattern for physical collateral that does not fit the tokenization model, and a reusable open-source authorization adapter other institutional builders can fork. It also exercises Stellar settlement assets where payment is real, repayment and exposure reduction, rather than as a demo. The detachable, independently-signed authorization entry this depends on is the primitive SDF identifies as Stellar's structural advantage [9].

**DFNS.** DFNS is publicly repositioning from wallet infrastructure toward a governed operating layer for institutional digital-asset workflows, with a policy engine, governance, and policy-aware service accounts at the centre [10], [11], [12], [13]. Argent is a hard, non-trivial proof of that thesis on Soroban: a multi-party workflow where release, enforcement, custody confirmation, spend evidence, and reward approval each belong to a different authority and no operator can sign for all of them. The authorization adapter is the Soroban-shaped piece that connects DFNS governance to Stellar's authorization model, and is reusable by any other Stellar builder facing the same multi-party signing problem.


## 11. Explicitly outside the current build

To remove any ambiguity, none of the following are part of the current build. They are later-stage only:

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

These may matter later. None of them is required to complete the DFNS-governed mainnet reference application, and nothing in this document changes the current build scope.

## 12. Summary

The current build is a focused integration milestone: move a tested Soroban prototype from local signers to DFNS-governed institutional role wallets, and launch the reference lifecycle on mainnet with a reusable authorization adapter open-sourced.

This roadmap begins only after that is complete and proven. It extends Argent into a collateral book of record for physical assets under custody: collateral pools, a live position report, scheduled revaluation and margin operations, safe substitution, policy-attributable haircut metadata, and asset categories beyond gold. Much of it builds directly on primitives already shipped and tested in the open-source core. It shows that Argent has a serious institutional path beyond the first deployment.

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


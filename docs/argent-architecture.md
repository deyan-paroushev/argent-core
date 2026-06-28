# Argent: Technical Architecture

**DFNS-governed physical-collateral control on Stellar.**

*Technical Architecture Document for the Stellar x CV Labs Accelerator (SCF Build, Integration Track, DFNS). Submission version 1.1.*

*This document is self-contained. A reviewer should be able to evaluate the Stellar use case, the integration plan, the contract design, the security model, and the build readiness from this document alone, then verify every claim against the open-source contracts in the linked repository. Every contract function named here exists in that repository.*

---

## 1. Executive summary

Argent is a Soroban-based collateral-control layer for physical assets that remain in professional custody. The first reference implementation uses allocated gold because it is the cleanest proof case: high value, allocated, serialised, custodied, and already financed under pledge and custody structures. The contract core is deliberately asset-agnostic. The same control structure binds any custody-stable physical asset (base metals, critical minerals, warehouse-held commodities, energy inventory) through asset-specific identity, custody, valuation, and document hashes.

Argent does not tokenize the physical asset, custody it, issue credit, process card payments, or perform legal enforcement. It records the lifecycle state that a bank, custodian, owner, verifier, and sponsor rely on, collateral identity, eligibility, exclusive pledge, borrowing base, utilization, repayment, release, default, cure, enforcement evidence, and reward claims, and it governs who may authorize each transition.

The current prototype runs with local development signers on Stellar testnet: three deployed Soroban contracts, the full lifecycle, and a clickable demonstrator. The funded work replaces those local signers with DFNS-governed institutional role wallets and a policy-approval workflow, and launches on mainnet. The contracts remain signer-agnostic. **Soroban records what changes. DFNS governs who may change it. Stellar settlement assets move only where value movement is real: repayment through `settlement_vault`.**

The open-source core (Apache-2.0) contains three contracts: `credit_ledger` (the tri-party control framework and full lifecycle), `settlement_vault` (atomic settlement-asset repayment bound to exposure reduction), and `rewards_ledger` (a sponsor-funded, non-transferable rewards overlay, fully separated from pledged collateral).

## 2. Product boundary

Argent is a control layer, not an asset issuer. This boundary determines every architectural decision in this document.

**What Argent is.** A shared, role-signed state layer for physical collateral that stays under professional custody. The on-chain record is not a digital commodity token; it is a control record showing which asset is pledged, what credit line it supports, and whether it has been drawn, repaid, released, defaulted, cured, or enforced.

**What Argent is not.** Not a commodity or gold token; not a custodian; not a bank or credit originator; not a card issuer or processor; not a legal-enforcement engine; not a replacement for security documents, custody agreements, KYC/AML/sanctions checks, insurance, or bank credit judgement.

**The model, stated once and held throughout:** custody stays with the custodian; ownership stays with the owner; credit exposure stays with the bank; control state moves onto Soroban; signing authority is governed by DFNS. The asset never leaves custody because of Argent, and ownership changes only through an off-chain legal and custody process after default, which Argent records as evidence but never executes.

**Stated as a category: a collateral book of record.** The institutional way to read Argent is as a collateral book of record for physical assets under custody. It does not replace the custodian's book, the bank's credit system, or the legal file. It records the shared control state those systems depend on but do not hold in one place: identity, eligibility, exclusive pledge, borrowing base, utilization, margin state, release, default, cure, enforcement, and evidence. Not an accounting book, not a custody book, a shared control book each party can rely on and sign against.

## 3. Why Stellar, and why this is not a superficial integration

Stellar/Soroban is core to the system, not a storage layer. Three protocol features do work no off-chain database can, which is the test SCF applies:

1. **Multi-party authorization (`require_auth`, detached authorization entries).** Each party independently signs the precise action it takes. A bank's authorization and a custodian's confirmation are independently signed authorization acts, each bound to the party that performs it, rather than a single combined signature. This native primitive is what Argent's separation-of-duties model is built on; replicating it in a private database would mean rebuilding non-repudiable multi-party signing from scratch.
2. **Atomic settlement (Stellar Asset Contract / SEP-41).** Repayment and exposure reduction occur in one atomic transaction: the settlement asset moves and the drawn balance falls together, or neither does. A card rail cannot bind a value transfer to a collateral-state change atomically; Soroban can.
3. **A shared, append-only, independently verifiable record.** The owner, bank, custodian, and an auditor read the same collateral state from the ledger rather than reconciling four private systems. This is exactly what a distributed ledger provides and a private bank database does not.

Sensitive detail (bar serials, KYC, card PANs, legal documents) stays off-chain and is referenced by hash. The chain holds the minimum shared control record needed to make the lifecycle verifiable. Without Stellar, Argent is another private collateral database; with Stellar, the parties and an auditor share one role-signed, verifiable control state.

This is not an incidental use of Stellar. The detachable, independently-signed, multi-party authorization entry is the primitive the Stellar Development Foundation identifies as Stellar's structural advantage over other smart-contract platforms, where the same patterns require bolted-on token standards and parallel transaction pipelines.[1] Argent is a direct institutional application of that model: each party signs only the authorization entry for the action it takes, and the bank's release authorization and the custodian's confirmation are independently signed acts in the release lifecycle, each a distinct transition bound to the party that performs it. Where SDF's composable-auth model describes the mechanism, Argent exercises it in the setting that most demands it, multi-party secured credit between parties who do not fully trust each other. The same governance-first direction runs through SDF's recent protocol work, from onchain incident response[2] to the quantum-preparedness plan's separation of account identity from signing keys.[3]

## 4. Actors and responsibilities

| Actor | Real-world responsibility | Argent responsibility | Signing authority |
|---|---|---|---|
| Owner | Owns the asset; requests liquidity | Selects collateral, requests pledge, repays, may claim rewards | Owner-controlled wallet (delegated; self-signs only its own acts) |
| Custodian | Holds the asset; controls custody books | Confirms existence, immobilizes collateral, confirms release/enforcement outcome | DFNS custodian wallet |
| Bank | Extends secured credit; owns exposure | Accepts pledge, opens line, controls release/default/cure/enforcement, sets margin policy | DFNS bank wallet(s), separated into credit, release, and enforcement authorities |
| Verifier | Confirms eligible utilization / posted spend | Records utilization evidence against secured capacity | DFNS verifier wallet |
| Sponsor | Funds reward campaigns (separate from collateral) | Creates campaign, approves claims, confirms vouchers/redemptions | DFNS sponsor wallet |
| Operator (Argent) | Builds transactions, coordinates approvals, indexes, renders evidence | Routes intent to the correct role; never authorizes business actions for others | Fee-sponsor / service account only where explicitly configured |

## 5. High-level architecture

```
   Owner        Bank        Custodian        Verifier        Sponsor
     |            |             |                |              |
     |   business action · approval request · status view      |
     +------------+-------------+----------------+--------------+
                                |
                                v
                 Argent application and service layer
        action builder · Soroban simulation · policy decoder
        approval queue · webhook/polling · Stellar RPC client
        event indexer · evidence-certificate generator
                                |
                                v
                       DFNS authorization layer
        role wallets · deny-by-default policies · quorum approval
        MPC signing (no raw key) · approval-to-transaction trace
                                |
                                v
                          Stellar / Soroban
        credit_ledger · settlement_vault · rewards_ledger
        Stellar settlement asset via SAC (SEP-41) · require_auth · events
                                |
                                v
                            Evidence layer
        transaction hash · contract events · live state · certificate
```

The contracts are intentionally signer-agnostic: they enforce role authorizations and approved-party checks regardless of who holds the keys, so the same contracts run under development signers today and under DFNS-governed institutional wallets after integration. The owner self-signs through a delegated wallet and does not pass through the policy engine; the five institutional roles are DFNS-governed (see §11).

## 6. Contract architecture

### 6.1 `credit_ledger`

The main collateral-control state machine: the secured-credit lifecycle from framework registration through position registration, pledge activation, line opening, utilization, repayment state, release, default, cure, enforcement, and enforcement-readiness evidence. Responsibilities: register the tri-party framework and bind owner, bank, and custodian; record document hashes for facility, pledge, custody, eligible schedule, margin policy, and enforcement waterfall; register positions through asset-specific identity and valuation fields; prevent the same recorded collateral from being pledged twice; enforce one credit line per pledge and borrowing-base and drawdown limits; record repayment state through the approved settlement vault; keep repayment separate from release; require bank authorization and custodian confirmation for release; record default, cure, and enforcement evidence; support revaluation and margin checks and enforcement-readiness records.

### 6.2 `settlement_vault`

The real settlement leg, narrow by design: it transfers a configured Stellar settlement asset (SAC / SEP-41) from the borrower to the bank and calls `credit_ledger.apply_repayment` in the same transaction path. Responsibilities: bind to one configured credit ledger and one settlement token; prevent duplicate payment references; reject repayment on closed or non-repayable lines; reject overpayment above drawn balance; transfer the settlement asset; reduce exposure via `apply_repayment`. Repayment reduces exposure; it does not release collateral.

### 6.3 `rewards_ledger`

A sponsor-funded, non-transferable rewards overlay tied to eligible posted spend and verified claims. Rewards are not yield, not transferable tokens, and never mixed with pledged collateral unless later re-confirmed as new collateral through the credit-ledger workflow. Responsibilities: create sponsor-funded campaigns with budget and per-user caps; record eligible spend and confirm finality; submit, approve, or reject claims; issue non-transferable voucher records; confirm redemption; keep reward accounting fully separate from pledged collateral.

## 7. Contract function map

Every function below exists in the repository. This is the authoritative mapping.

| Workflow | Contract function |
|---|---|
| Initialize ledger | `initialize` |
| Grant / revoke / check role | `approve_party` · `revoke_party` · `is_approved` |
| Register control framework | `register_framework` (read: `get_framework`) |
| Register physical position | `register_position` (read: `get_position`) |
| Select collateral set | `select_bars_for_collateral` (read: `get_selection`) |
| Custodian immobilizes collateral | `confirm_and_immobilize` (read: `get_custody_control`) |
| Activate pledge | `activate_pledge` (read: `get_pledge`) |
| Open credit line | `open_credit_line` (read: `get_line`) |
| Record / reverse utilization | `record_drawdown` · `reverse_drawdown` (read: `get_drawdown`) |
| Available capacity | `available_capacity` |
| Apply repayment to line | `apply_repayment` (read: `get_repayment`) |
| Atomic settlement-asset repayment | `settlement_vault.settle_repayment` |
| Collateral adjustment | `request_collateral_adjustment` · `bank_approve_adjustment` · `custodian_confirm_adjustment` (read: `get_adjustment`) |
| Bank authorizes release | `bank_authorize_release` |
| Custodian confirms release | `custodian_confirm_release` |
| Suspend / resume line | `bank_suspend_line` · `bank_resume_line` |
| Revalue and check margin | `revalue_and_check` (read: `get_valuation`) |
| Issue default notice | `issue_default_notice` |
| Cure default | `cure_default` |
| Record enforcement | `record_enforcement` |
| Enforcement readiness | `open_enforcement_readiness` · `populate_enforcement_readiness` · `expire_enforcement_readiness` (read: `get_enforcement_readiness`) |
| Reward campaign | `create_campaign` · `add_campaign_budget` · `pause_campaign` · `resume_campaign` · `close_campaign` |
| Reward spend / finality | `record_eligible_spend` · `confirm_spend_final` · `cancel_reward` · `expire_reward` |
| Reward claim / voucher | `submit_claim` · `sponsor_approve_claim` · `sponsor_reject_claim` |
| Reward redemption | `confirm_redemption` |

Two design decisions a reviewer should note, because they are deliberate and they differ from a naive design:

**Repayment and release are separate.** `apply_repayment` (and the atomic `settle_repayment`) reduce exposure only. Release is a distinct two-step act: `bank_authorize_release` then `custodian_confirm_release`. The bank retains control after repayment until release is signed by both bank and custodian. This is how a real facility closes, and it is stronger for the lender than an automatic release.

**The chain records enforcement; it does not execute it.** On uncured default, `record_enforcement` records the notice, cure expiry, outcome, and a hash of the off-chain legal instrument. Argent does not flip legal title on-chain or move physical metal. Enforcement happens under the security documents, applicable law, and custody instructions; the contract provides the evidenced, append-only record of it.

## 8. Primary lifecycle

**Framework setup.** Owner, bank, and custodian enter a control framework; the contract stores the parties and document hashes (facility, pledge, custody, eligible schedule, margin policy, enforcement waterfall). The documents stay off-chain.

**Position registration.** A position is registered through asset-specific identity and valuation fields. For gold: bar-set hash, serials hash, refiner/assay references, fine weight, custody-account evidence. For other custody-stable assets: lot ID, batch ID, warehouse-receipt ID, tank ID, grade, quantity, or inspection-certificate hash, the same pattern.

**Selection and immobilization.** The owner selects a specific collateral set; the custodian confirms and immobilizes it. The same recorded collateral cannot enter another active pledge inside Argent.

**Pledge and credit line.** The bank accepts the pledge and opens a line against the borrowing base. The contract enforces that the line cannot exceed the approved base and that drawdowns cannot exceed the active limit.

**Utilization.** A verifier/processor role records utilization evidence (eligible posted spend or a drawdown reference). Argent does not process card rails; it records the secured utilization state the bank or processor relies on.

**Repayment.** `settlement_vault.settle_repayment` moves the settlement asset from borrower to bank and reduces exposure via `credit_ledger.apply_repayment`. Repayment makes release available if the bank's conditions are met; it does not release automatically.

**Release.** Two stages: `bank_authorize_release`, then `custodian_confirm_release`. This preserves the bank's veto and the custodian's operational control.

**Default, cure, enforcement.** The bank issues a default notice; the borrower may `cure_default` before the deadline; if uncured, the bank records enforcement via `record_enforcement` with the legal/custody evidence hash. The chain records; law and custody enforce.

**Reward overlay.** Separate from pledged collateral: a sponsor funds a campaign tied to eligible posted spend; claims, approvals, vouchers, and redemptions are recorded in `rewards_ledger`.

## 9. On-chain / off-chain boundary

| Domain | On-chain (Soroban) | Off-chain |
|---|---|---|
| Asset identity | hashes, IDs, position status, selected-collateral reference | serials, bar list, receipt, certificate, inspection report |
| Custody | attestation, immobilization, release confirmation | vault books, warehouse records, physical possession |
| Legal documents | document hashes | facility, pledge, custody, enforcement documents |
| Credit policy | LTV, limit, borrowing base, margin status, line state | credit-committee decision, underwriting model, approval file |
| Valuation | value fields, price timestamp, margin state | external price source, appraiser, approved valuation file |
| Utilization | drawdown reference, amount, capacity change | card rail, issuer-processor event, internal bank ledger |
| Repayment | settlement transaction, exposure reduction | bank accounting, statement, reconciliation |
| Release | bank authorization, custodian confirmation | physical release instruction, custody-book update |
| Default / cure | notice, cure deadline, cure state | legal notices, borrower communication, bank decision |
| Enforcement | outcome hash and status | legal enforcement, transfer, realization, recovery |
| DFNS | signer addresses, signed transaction, tx hash | policies, approvals, activity records, org wallets |
| Evidence | event log, state, certificate references | rendered certificate, audit file, private document bundle |

## 10. Stellar integration detail

**Soroban contracts.** Soroban records shared lifecycle state no single participant can alter unilaterally, using explicit role authorizations and approved-party checks; each actor signs only actions belonging to that actor.

**Stellar RPC.** The service uses Soroban RPC for transaction simulation, required-authorization discovery, fee/resource estimation, submission, confirmation, contract-event queries, and live state reads. `simulateTransaction` is central to the DFNS integration: it exposes the authorization requirements of an action before signing, which is what the policy decoder routes on.

**Stellar Asset Contract (SEP-41).** The settlement leg moves a Stellar settlement asset (USDC or another bank-approved asset) through the Soroban token interface. `settlement_vault` binds repayment to exposure reduction so payment and debt reduction are provably one workflow.

**Events and indexing.** Every lifecycle act emits a single typed `CollateralEventV1` (the `#[contractevent]` macro on soroban-sdk 23.5.3), the canonical, replayable record of the act. Its topics are four-part: a pinned `collateral_event_v1` marker, the `framework_id` the act is sequenced under, the `entity` kind it affects, and the `action` committed, so an indexer filters the stream by deal, object kind, and act without decoding bodies. The non-topic fields are a self-describing `Map<Symbol, Val>` (the SEP-48 map data format) registered in the contract spec, so any indexer or forker decodes the event by field name with no Argent-specific code, and the wire event and the published spec cannot diverge. Each event carries a monotonic, framework-scoped sequence (a gap means a missing or uningested event; the stream is complete iff contiguous), the previous and new state labels of the affected entity, the actor and the role under which it acted, the relevant evidence, condition, and valuation commitments, and a typed payload. The payload carries exactly the fields needed to reconstruct projection state, so an off-chain reader can rebuild every position, pledge, and line from the event stream alone, a property the suite checks directly through replay-fold tests. Because RPC event retention is not a permanent archive, the service maintains its own indexed event history, links each event to its DFNS approval activity, and renders evidence certificates from the combination.

**Current-state verification.** The service reads contract state through RPC to confirm line status, available capacity, pledge state, repayment state, reward state, and enforcement readiness, for both live UI and certificate generation.

## 11. DFNS integration architecture

DFNS is the institutional authorization layer. The contracts already require distinct role authorizations; the funded integration replaces local development signers with DFNS-governed role wallets and policy approvals. DFNS is not a remote key box Argent calls to sign; it governs the institutional intent lifecycle, permission check, policy evaluation, quorum approval, MPC signing, broadcast, reconciliation, before any Soroban state transition is allowed.

**Where DFNS and Soroban meet (the technical touch points).**
- **The 32-byte authorization-entry hash.** DFNS signs the exact Soroban authorization-entry payload (a `HashIdPreimageSorobanAuthorization` hash) via its Keys API raw-payload signing, not only pre-built transaction envelopes, and the reassembled transaction verifies on-chain.[8]
- **Ed25519, the shared scheme.** Stellar uses Ed25519; DFNS signs Ed25519. A DFNS signature is natively valid for a Stellar/Soroban authorization entry.
- **Quorum maps to `require_auth`.** The two-step release is two independently signed authorization acts, `bank_authorize_release` then `custodian_confirm_release`; DFNS quorum policy expresses exactly that institutional requirement.
- **Decode Soroban XDR, then route.** The service decodes the Soroban action on a pending DFNS activity, identifies contract, method, and business reference, and routes it to the correct role under policy.

**Role-wallet topology.** Authority is expressed as a sub-organization wallet topology on org-controlled wallets, mirroring the institutional org chart and isolating each party's authority: an operator org (Argent) at the top; a bank sub-organization split into credit, release, and enforcement authorities (so the wallet that authorizes a release is not the wallet that records enforcement); a custodian sub-organization (custody and release confirmation); a sponsor sub-organization (campaign and redemption); a verifier sub-organization (spend evidence). No backend service key signs as all parties.

**Deny-by-default policy model, stated precisely.** DFNS policies are not whitelist-by-default: if no policy is configured, activities are allowed, and delegated wallets bypass the policy engine entirely. Argent therefore *constructs* deny-by-default through scoped per-wallet policies: block all signing by default; decode the Soroban action; identify contract, method, reference, and role; require approval where policy demands it; sign only if policy resolves. "Deny-by-default" is a property Argent builds, not one DFNS provides for free, this precision is the difference between an accurate application and an overclaim.

**Soroban-aware policy decoder (the reusable output).** A DFNS service-account helper that decodes Soroban XDR on a pending activity, identifies contract/method/reference, maps it to the correct institutional role, and approves, requests approval, or blocks per the deny-by-default model. DFNS already ships this programmable-policy pattern for EVM-style call data; it does not exist for Soroban. This adapter is the genuinely new, reusable contribution any institutional Stellar RWA builder can fork.

This contribution sits directly inside DFNS's own stated direction. DFNS now positions itself not as a wallet provider but as a core banking platform for institutional digital-asset operations,[4] and its recent product line is precisely a governance layer: a Governance Engine described as a zero-trust architecture for institutions,[5] and Policy-Aware Service Accounts built on the premise that institutions do not lack controls but lack a way to apply them to digital-asset flows without rebuilding their stack.[6] That premise is Argent's thesis stated in DFNS's own words. The gap is that this policy-aware model decodes EVM-style call data and does not yet decode Soroban. Argent's policy decoder is the Soroban-shaped extension of DFNS's governance model, and DFNS support for Stellar primitives such as fee sponsorship is already live,[7] so the integration builds on existing surface rather than speculative capability.

**Signing sequence.** Service builds the invocation → simulates via RPC → extracts required authorizations → policy decoder maps action to role → DFNS evaluates permissions and policy → approvers approve or reject → DFNS MPC-signs the approved payload through the role wallet → service assembles and submits → service reconciles DFNS activity status against Stellar transaction status → indexer records events and renders evidence. DFNS activity states (pending, executing, broadcasted, confirmed, failed, rejected) are tracked separately from Stellar transaction state, keyed on an idempotency id, so the lifecycle reflects the true combined state (e.g. "awaiting bank approval" is a DFNS state with no Stellar transaction yet).

**The one validated unknown (committed path, with fallback).** The integration validates, on testnet, that DFNS signs the exact Soroban authorization-entry hash via the Keys API and that the reassembled transaction verifies on-chain. The capability exists in the API surface; validation proves it end to end for Soroban auth entries specifically. If a gap is found, the documented fallback is signing at the transaction-envelope boundary instead, preserving role separation and reviewer-visible authorization evidence.

## 12. Security model

**Contract-level controls (enforced and tested today).** Explicit role authorization via Soroban auth; approved-party checks for bank, custodian, verifier, sponsor, and service roles; no double pledge of the same recorded collateral; one credit line per pledge; no line above borrowing base; no draw above available limit; stale and future-dated valuation refusal; no repayment above outstanding drawn balance; no automatic release after repayment; bank authorization required before release; custodian confirmation required before collateral returns to free state; default-notice and cure-deadline checks; enforcement only after the relevant lifecycle conditions; evidence-hash validation for legally important references; duplicate payment-reference prevention in `settlement_vault`; strict separation between reward records and pledged collateral.

**Operational controls (added by DFNS).** Role-specific wallets; policy approval before signing; pending and rejected states; approval-to-transaction traceability; no shared backend key that can act for every role; MPC signing with no raw private key in existence; an institution-readable audit trail from approval to transaction hash.

**Residual risks (stated honestly).** Argent reduces collateral-control risk; it does not remove all risk. What still requires trust: custodian honesty at the moment of attestation; physical loss, theft, degradation, or operational failure; pledges created entirely outside Argent; legal-enforceability or perfection defects; sanctions/provenance/KYC/AML/insurance failures; price collapse or a stale external valuation source; jurisdictional enforcement delays; bank underwriting error. Argent removes the control and verification risks that were never priceable; it leaves the bank the credit, market, and legal risks it is in business to price.

## 13. Evidence and audit model

Four layers: **intent** (who requested or approved the action), **signature** (which DFNS role wallet signed it), **ledger** (transaction hash, contract event, current state), and **document** (hashes of off-chain documents, custody records, valuations, notices, enforcement files). The evidence certificate combines them, framework/position/pledge/line IDs, current line status, drawn balance and available capacity, collateral status, key lifecycle events, the signer role for each action, the DFNS activity reference, the Stellar transaction hash, relevant document hashes, a ledger timestamp, and an explicit statement of what the certificate proves and does not prove.

## 14. Testing and verification

The open-source core ships 187 tests across the three crates: `credit_ledger` 125, `rewards_ledger` 45, `settlement_vault` 17. Coverage includes lifecycle happy paths; role authorization and party revocation; double-pledge refusal and second-line-against-same-pledge refusal; borrowing-base and drawdown constraints; stale and future-dated price refusal; repayment and overpayment controls; no release with an outstanding balance; repayment does not auto-release; default/cure/enforcement constraints; duplicate payment-reference safety; reward budget, claim, and redemption lifecycle; zero-hash evidence refusal; terminal-state snapshot checks; and TTL / ledger-advance persistence checks. A reviewer can reproduce the result from the contracts workspace with `cargo test --workspace` (the build script compiles the `credit_ledger` WASM before the settlement-vault tests, which import it), and can exercise the live testnet demonstrator and its on-chain transactions.

## 15. Build readiness and milestone path

The technical architecture is complete and the contracts exist now, so the funded work is integration and hardening, not design or research. This section states what is built in each funded tranche as clear, measurable, verifiable deliverables. The dollar split across tranches is in the application form; this section establishes the technical content and the success signal a reviewer can test for each.

**Already implemented (this repository, live testnet).** Three deployed Soroban contracts on soroban-sdk 23.5.3; the full lifecycle of §8 including the two-step release and the default/cure/enforcement branch; atomic SEP-41 settlement via `settle_repayment`; the typed `CollateralEventV1` canonical event layer with its SEP-48 map schema and replay-fold reconstruction; evidence-certificate output in the live app; 187 passing tests; and a signer interface with a clean `signAuthEntry(payloadHash)` seam currently served by development signers. This is the baseline the funded build starts from, not part of the funded scope.

**Tranche 1, MVP: the DFNS signing foundation.** Replace development signers with DFNS-governed role wallets behind the existing `signAuthEntry(payloadHash)` seam, and prove one governed action end to end.
- Deliverables: DFNS sub-organization role wallets provisioned for the bank, custodian, sponsor, and verifier roles; the signer adapter connected behind the existing interface; DFNS raw-payload signing of the exact 32-byte Soroban authorization-entry hash validated on testnet (the one validated unknown of §11, with the transaction-envelope fallback documented if a gap is found); the first complete lifecycle action (a drawdown or a repayment) executed through the DFNS path; the first approval-to-transaction trace captured.
- Success signal (verifiable): a recorded testnet transaction whose Soroban authorization entry was signed by a DFNS role wallet, with the matching DFNS activity id and Stellar transaction hash shown side by side.

**Tranche 2, Testnet: the full governed lifecycle.** Bring the entire §8 lifecycle under DFNS quorum governance, with deny-by-default policy and reconciliation.
- Deliverables: the Soroban-aware policy decoder that decodes the action on a pending DFNS activity and identifies contract, method, and business reference; deny-by-default policy templates constructed per role wallet (per §11, a property Argent builds, since unconfigured DFNS policy allows by default); the approval queue with pending, approved, rejected, expired, submitted, and confirmed states; webhook handling; the two-step release (`bank_authorize_release` then `custodian_confirm_release`) and the default and enforcement flow demonstrated through real DFNS quorum approvals by separate role wallets; the event indexer and the reconciliation that keys DFNS activity state against Stellar transaction state on an idempotency id.
- Success signal (verifiable): a recorded testnet run of the full lifecycle in which a release requires two distinct role-wallet approvals, a wrong-role approval is refused by policy, and the evidence pack reconstructs every state from its approval to its transaction hash.

**Tranche 3, Mainnet: the launch and the reusable output.** Deploy to mainnet and publish the ecosystem contribution.
- Deliverables: mainnet-deployed contracts with published contract IDs, worked transaction examples, and an operations runbook; the reusable Soroban-aware DFNS authorization adapter and policy decoder published open-source under Apache-2.0, with the standard message-purpose vocabulary of §16; the evidence-certificate pack (current-state, release, enforcement, and rewards certificates); and a pilot package for a bank, custodian, and sponsor to review.
- Success signal (verifiable): live mainnet contract IDs a reviewer can inspect on a block explorer, a public repository containing the forkable adapter and decoder, and a governed lifecycle action executed on mainnet with its approval-to-transaction trace.

Each tranche ships something testable on its own, and the destination is fixed: a functional, DFNS-governed reference facility live on Stellar mainnet with the authorization adapter open-sourced. None of the pool, scheduled-revaluation, margin-call, or substitution work described in the separate post-grant roadmap is in this funded scope.

## 16. Open source and non-goals

The contract core is open-sourced under Apache-2.0. The funded DFNS-and-Soroban authorization adapter and policy decoder will be published open-source as the ecosystem deliverable, so any institutional Stellar RWA builder facing the same multi-party-signing problem can fork them rather than start from a blank page.

**A standard message-purpose vocabulary.** The adapter is not only code; it carries a standard set of Soroban message purposes for institutional collateral actions, so that a DFNS policy evaluating a pending action resolves it against a named, unambiguous purpose rather than a raw method string. The lifecycle vocabulary is: attest, lock, pledge, draw, repay, authorize release, confirm release, substitute, default notice, cure, record enforcement, and issue evidence. Publishing this as a shared vocabulary lets other Stellar RWA builders express the same institutional roles and actions consistently, which is where most of the reuse value sits.

**Non-goals (explicit).** The funded build does not become a bank, issue credit, custody assets, tokenize collateral, issue consumer cards, integrate directly with card networks, automate legal enforcement, replace bank underwriting or custodian onboarding, or attempt a general SDK for every Soroban application. It is focused on one reference workflow, DFNS-governed physical-collateral control on Stellar, proven first on gold and made reusable through the open contract core and the authorization adapter.

## 17. Reviewer summary

Argent is a tested Soroban collateral-control application for physical assets that stay in custody. The contracts prove the core lifecycle today: identity, exclusive pledge, borrowing base, utilization, repayment, the two-step release, default, cure, enforcement evidence, and a separated rewards overlay, with 187 passing tests and a live testnet demonstrator. The funded work brings this from a local-signer prototype to a DFNS-governed institutional workflow and a mainnet reference deployment, and publishes the Soroban-aware DFNS authorization adapter as a reusable ecosystem contribution. The bank receives no token and takes no custody; it receives control evidence, exact collateral identity, exclusive pledge state, contract-enforced borrowing base, a signed release path, default and enforcement evidence, and a transaction-backed audit trail. DFNS governs who signs. Soroban records what changes. Stellar settlement assets move where repayment is real.

---

## References

[1] L. McCulloch, "Stellar's composable auth model," Stellar Development Foundation, May 5, 2026. https://stellar.org/blog/foundation-news/stellars-composable-auth-model

[2] Stellar Development Foundation, "Quorum Freeze (CAP-77): A Governed, Onchain Incident Response on Stellar," May 5, 2026. https://stellar.org/blog/foundation-news/quorum-freeze-cap-77-governed-onchain-incident-response

[3] N. Barry, "Introducing the Quantum Preparedness Plan," Stellar Development Foundation, June 9, 2026. https://stellar.org/blog/foundation-news/introducing-the-quantum-preparedness-plan

[4] C. Hagège and C. Grilhault des Fontaines, "We're Not a Wallet Company Anymore," DFNS, June 3, 2026. https://dfns.co/article/were-not-a-wallet-company-anymore

[5] T. de Lachèze-Murel, "Introducing Governance Engine," DFNS, June 10, 2026. https://dfns.co/article/introducing-governance-engine

[6] H. Tross, "Introducing Policy-Aware Service Accounts," DFNS, April 27, 2026. https://dfns.co/article/introducing-policy-aware-service-accounts

[7] A. Moreau, "Fee Sponsors on Stellar," DFNS, June 12, 2025. https://dfns.co/article/fee-sponsor-on-stellar

[8] "Signing Soroban contract invocations," Stellar Developer Documentation. https://developers.stellar.org/docs/build/guides/transactions/signing-soroban-invocations

---

*Companion document: `argent-dfns-signing-sequence.md` in this repository details the DFNS intent lifecycle, approval flow, webhook transitions, role-wallet topology, and policy model at implementation depth.*

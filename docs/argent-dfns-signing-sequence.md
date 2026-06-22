# Argent — DFNS Governed Intent Layer

**How Argent's lifecycle acts are governed, approved, and signed once DFNS is the institutional intent layer above Soroban.**

*Companion to argent-architecture.md. This document describes the revised sequence for the DFNS integration: the intent lifecycle, the approval flow, the webhook transitions, the role-wallet topology, and the deny-by-default policy model. It exists so the integration is built to match how DFNS actually sequences institutional signing, not a guessed approximation. Written for the build and for the Stellar x CV Labs reviewer.*

The framing that matters, stated once: DFNS is not a remote private-key box that Argent calls to sign Soroban entries. DFNS governs the institutional intent lifecycle, authentication, permission, policy evaluation, approval, MPC signing, broadcast, reconciliation, before any Soroban state transition is allowed to happen. Argent integrates DFNS as the governed intent layer for multi-party Soroban workflows, not merely as a signer. That distinction shapes the sequence, the policy model, and the reusable deliverable below.

---

## 0. What changes, and what does not

The on-chain logic does not change. The Soroban contracts (credit_ledger, settlement_vault, rewards_ledger), the lifecycle (framework, position, selection, immobilization, pledge, line, drawdown, the repay/release performing path, and the default/cure/enforcement path), and every contract invariant stay exactly as built and tested. DFNS does not replace any of that.

What changes is the service layer. Today, each governed act is signed synchronously: the service asks a signer for a signature on the Soroban authorization-entry hash, gets it immediately, assembles the transaction, and submits. Under DFNS, signing becomes asynchronous and policy-governed. A governed act may not sign immediately; it may enter a pending state and wait for a second party's approval before DFNS signs it via MPC. The new work is the pending state and the webhook handling that surround the existing call.

This is the genuinely new layer the integration funds. It is not "build multi-party signing from scratch": the Signer interface and SignerRegistry already exist (service/src/chain/signer.ts), with a clean signAuthEntry(payloadHash) seam and throwing placeholders for the production signers. The work is implementing one DfnsSigner behind that interface plus the approval/pending/webhook layer that institutional signing requires.

---

## 1. The DFNS sequence, as DFNS runs it

DFNS processes every signed action through a fixed sequence. A request meets the Permissions Gatekeeper first (does this user have authority to initiate this action), then the Policy Engine (does this action trigger a rule that requires approval), then, if approval is required, the action enters a pending state and waits for the approval group's quorum, then on quorum it descends to the MPC nodes for signing, then it is broadcast, then delivery is tracked. One API call composes, simulates, evaluates policy, collects approvals, signs, broadcasts, and tracks, with webhooks emitted at each state transition.

The Policy Engine is deny-by-default in the sense that matters here: a rule, once defined, triggers on every matching activity and applies its action (request approval, or block) unless the activity is explicitly outside the rule's scope. Supported rule kinds include AlwaysTrigger, TransactionAmountLimit, TransactionAmountVelocity, TransactionCountVelocity, and TransactionRecipientWhitelist. An approval can carry an auto-reject timeout: if quorum is not reached within the window, the activity is automatically rejected.

One distinction is load-bearing for Argent's role mapping: policies govern org-controlled (custodial) wallets. Delegated, user-controlled wallets are non-custodial and bypass the policy engine. So any Argent party whose actions must be policy-governed has to sit on an org-controlled wallet.

---

## 2. Argent's roles as DFNS wallets

The integration expresses Argent's role authority as DFNS wallets and policies, not only as contract-level require_auth. The contract still enforces its invariants; DFNS adds the institutional approval layer above them.

Bank: org-controlled wallet. Its acts (open line, authorize release, issue default notice, record enforcement) are the institutionally sensitive ones and must be policy-governed.

Custodian: org-controlled wallet. Its acts (immobilize, confirm release, confirm realization) are the physical-control counter-signatures and must be governed and quorum-eligible.

Enforcement / liquidation parties: org-controlled wallets, governed by the strictest policies (quorum, possibly a time delay).

Owner / cardholder: a deliberate choice. If their drawdowns should be policy-checked (velocity, amount limits), they are org-controlled. If they are meant to self-sign with full autonomy, they are delegated, and then their actions bypass the policy engine by design. For a bank-issued secured facility, the default is org-controlled so draws can be governed.

The decision rule: govern it with a policy means org-controlled; let the party self-sign autonomously means delegated. Pick per role, deliberately, knowing delegated bypasses governance.

### Wallet topology: sub-organizations, not a flat list

DFNS does not think in a flat list of role wallets. It thinks in wallet fleets: sub-organizations, tags, metadata, ownership isolation, wallets that mirror an org chart and are structured by customer, entity, product, or strategy. Argent's topology should follow that model, which also isolates each party's authority cleanly and makes a multi-party institutional arrangement legible to a bank's own security team:

- Operator org (Argent / Advisa): the top-level organization.
  - operator wallet: fee sponsor and transaction submitter.
- Sub-organization, pilot bank:
  - bank-credit-authority (open line, drawdown acceptance),
  - bank-release-authority (authorize release),
  - bank-enforcement-authority (default notice, record enforcement).
- Sub-organization, custodian:
  - custody-confirmation-authority (immobilize),
  - release-confirmation-authority (confirm release, confirm realization).
- Sub-organization, sponsor / refiner:
  - campaign-authority, voucher-redemption-authority (the rewards overlay).
- Sub-organization, verifier / processor adapter:
  - spend-evidence-authority (attest card spend).

Splitting the bank into separate credit, release, and enforcement authorities is deliberate: it lets the deny-by-default policies (next section) bind narrowly to each authority, so the wallet that can authorize a release is not the same wallet that can record an enforcement. This is the institutional separation-of-duties a bank expects, expressed in DFNS's own structural vocabulary.

---

## 3. Deny-by-default policy model

A correction that matters, and that the application language must get right: DFNS policies are not a whitelist by default. If no policy is configured, activities are allowed; policies add restrictions on top. Delegated wallets bypass the policy engine entirely; policies apply to org-controlled wallets.

So Argent must not claim "DFNS policies automatically protect the workflow." The accurate claim is: Argent implements a deny-by-default DFNS policy model on its institutional role wallets. Each role wallet carries policies that block actions outside its remit and require approval for its sensitive ones:

- Bank authorities: block unknown actions; require approval for release, enforcement, and suspension; allow only the bank-controlled Soroban methods bound to each authority wallet.
- Custodian authorities: block credit actions; allow custody confirmation, immobilization, and release confirmation.
- Sponsor authority: block credit and custody actions; allow reward-campaign, voucher, and redemption actions only.
- Verifier authority: allow spend-evidence attestation only; block pledge, release, enforcement, and reward-funding actions.

"Deny-by-default" here is a property Argent constructs through these per-wallet policies, not a property DFNS provides for free. That precision is the difference between an accurate application and an overclaim a sharp reviewer would catch.

---

## 4. The revised act sequence (the core change)

Every governed Argent act follows this sequence. The two-stage release and the enforcement flow are the clearest cases, but the pattern is uniform.

State 1 — Initiated. A party initiates an act (e.g. the bank authorizes release). The service composes the Soroban invocation and simulates it to discover the authorization entries, exactly as today.

State 2 — Policy-evaluated. The act is submitted to DFNS. The Permissions Gatekeeper checks the initiator's authority; the Policy Engine evaluates the act against its rules.

State 3a — Signed immediately. If no policy requires approval (e.g. a low-value, whitelisted, routine act), DFNS signs the authorization-entry hash via MPC and the act proceeds. This is the synchronous-feeling path.

State 3b — Pending approval. If a policy requires approval, the act enters a pending state. Nothing is signed yet. The transaction waits. A pending object is created with an initiator, an approval group, and (optionally) an auto-reject timeout. The service records this pending state; the lifecycle UI shows the act as awaiting approval, not done.

State 4 — Approved (or rejected). A member of the approval group reviews and approves with their own passkey. The initiator may not approve their own act, but may cancel it. A single rejection from any approver rejects the act. On quorum, the Policy Engine marks the activity authorized. On timeout without quorum, the activity is auto-rejected.

State 5 — Signed. Only now does the approved act descend to the MPC nodes and the authorization-entry hash is signed.

State 6 — Broadcast and recorded. The signed transaction is assembled and broadcast to Stellar. The service records the on-chain transaction, exactly as today, the contract event and the ledger entry are unchanged.

The only structurally new states are 3b (pending) and 4 (approval/rejection). States 1, 2, 5, and 6 already exist in the current synchronous flow, compressed together. The integration pulls them apart and inserts the pending/approval gap.

---

## 5. Webhook transitions

DFNS is asynchronous, so the service learns of state changes through webhooks rather than a blocking call. The service handles, at minimum:

activity initiated / policy triggered: an act entered pending. The service persists the pending record and reflects "awaiting approval" in the lifecycle state.

approval granted / quorum reached: the act is authorized and about to sign. The service updates the pending record.

activity signed / broadcast: the act is on-chain. The service records the transaction hash and marks the lifecycle step done, the same write the synchronous flow does today.

activity rejected / timed out: the act was rejected (by an approver or by auto-reject timeout). The service marks the act failed and surfaces an honest reason, no on-chain change occurred.

Webhooks are idempotent and replay-proof on the DFNS side; the service must handle them idempotently too (a webhook may arrive more than once), keyed on the activity id.

---

## 6. How this maps onto the two-stage release and enforcement

The two-stage release already built (bank authorizes, then custodian confirms) is a DFNS quorum policy, not two unrelated synchronous calls. The bank's authorization and the custodian's confirmation are the two keys of a quorum; the Policy Engine sees the second approval and authorizes the release. Argent's existing two-party design is therefore aligned with DFNS by construction, the change is expressing it as a policy with an approval group, and handling the pending state between the two signatures.

The default and enforcement flow is the strictest-governed case. Issue default notice, the cure window, and record enforcement are all bank-initiated, governed acts; enforcement in particular should require quorum and may carry a time delay (DFNS supports a time-based delay in the approval path, as in the IBM OSO cold-signing integration). The Enforcement Readiness Certificate's "ready" status, gated on-chain today, can additionally reflect that the enforcement approval policy is configured.

---

## 7. The one open validation item (Tranche 1)

DFNS exposes a low-level Keys API ("Generate Signature") that signs a raw payload for a held key, and lists Stellar among supported networks. This is the path to signing Soroban authorization-entry hashes (a 32-byte payload), as opposed to only signing pre-built transaction envelopes. The DFNS policy docs explicitly acknowledge signature requests of a hash as an activity kind (noting such requests are "unscreenable" by transaction-screening rules, which confirms hash-signing is a first-class operation).

The named Tranche-1 validation item is therefore: confirm, on testnet, that DfnsSigner.signAuthEntry can obtain a DFNS signature over the exact Soroban authorization-entry hash and that the reassembled transaction verifies on-chain. The capability exists in the API surface; Tranche 1 proves it end to end for Soroban auth entries specifically. If a gap is found, the fallback is signing at the transaction-envelope boundary rather than the auth-entry boundary, documented as a contingency.

---

## 8. The reusable deliverable: a Soroban-aware DFNS policy decoder

The ecosystem output of this integration is not "a DfnsSigner." A signer is plumbing. The genuinely reusable, sponsor-aligned artifact is a Soroban-aware DFNS policy decoder.

DFNS already ships a "programmable policy" pattern: a service account watches pending wallet activities, decodes the call data, and approves, requests further approval, or denies based on the function, recipient, and amount. That pattern exists for EVM-style call data. It does not exist for Soroban. The reusable contribution is the Soroban equivalent:

A DFNS service-account policy helper that decodes Soroban XDR on a pending activity, identifies the contract id, the invoked method, and the business reference, maps that to the correct institutional role, and then approves, requests approval, or blocks, enforcing Argent's deny-by-default model at the policy layer rather than only at the contract's require_auth.

This is stronger than a signer for three reasons. It is genuinely new (no Soroban-aware DFNS policy decoder exists in the ecosystem today). It is reusable by any institution building multi-party Soroban workflows on DFNS, not just Argent, which is exactly what makes it an Integration-Track ecosystem contribution rather than private app code. And it is the artifact a sponsor most wants to see, because it extends their own programmable-policy pattern to a chain (Soroban) their pattern does not yet cover.

Argent is the reference implementation: the decoder is extracted from Argent's integration and published with Argent as the worked example. Scope discipline applies (as in argent-architecture.md): it is a documented reference decoder extracted from the integration, not a polished standalone SDK, and if it ever threatens the mainnet milestone it is dropped to protect that milestone.

---

## 9. Roadmap-only alignment points (named, not built pre-grant)

Two further DFNS alignment points belong in the plan, named now, built during the funded phase:

Status reconciliation: DFNS tracks its own activity states (pending, executing, broadcasted, confirmed, failed, rejected) separately from Stellar's transaction state. The service should store DFNS status separately from Stellar status and reconcile the two, so the lifecycle UI reflects the true combined state (e.g. "awaiting bank approval" is a DFNS state with no Stellar transaction yet). Every DFNS request carries an externalId / idempotency key for correlation.

Fee sponsorship: DFNS supports Stellar fee sponsorship via fee-bump transactions, the party signs the inner transaction, an operator fee-sponsor wallet signs the outer. This means institutional actors need not hold XLM to approve operational acts. It is an alignment point, not a core deliverable: name it in the roadmap, do not build it pre-grant.

Neither requires pre-submission code. Both are service-layer build items for the funded tranches.

---



Argent's two-stage release and enforcement flows map directly onto DFNS quorum-approval policies. The integration replaces synchronous development signing with DFNS's sequence (permission check, policy evaluation, approval collection, MPC signing, broadcast, tracking), and adds the pending-approval state and webhook handling that institutional signing requires. The contracts and lifecycle are unchanged; the new work is one DfnsSigner behind the existing Signer interface plus the approval/pending/webhook layer. Role authority is expressed as DFNS policies on org-controlled wallets, with the custodial-versus-delegated choice made deliberately per role. The reusable output, a DFNS-and-Soroban authorization adapter with this pending-approval pattern, is extracted from the integration for the ecosystem, with Argent as the reference implementation.
## 10. What the application can state, precisely

Argent integrates DFNS as the governed intent layer for multi-party Soroban workflows: each business intent is authenticated, permissioned, policy-evaluated, approved, signed, submitted, and reconciled through a DFNS-aligned lifecycle. The two-stage release and the enforcement flow map directly onto DFNS quorum-approval policies. The integration replaces synchronous development signing with DFNS's sequence and adds the pending-approval state, the webhook handling, and the deny-by-default per-wallet policy model that institutional governance requires. The contracts and lifecycle are unchanged. Role authority is expressed through a sub-organization wallet topology on org-controlled wallets, with the cardholder self-signing (delegated) and the institutional roles (bank, custodian, sponsor, verifier, operator) DFNS-governed. The headline reusable output is a Soroban-aware DFNS policy decoder: a service-account helper that decodes Soroban XDR on pending activities and enforces role-mapped, deny-by-default policy, extending DFNS's own programmable-policy pattern to Soroban, with Argent as the reference implementation.

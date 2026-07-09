# Integration and Interoperability: How Argent Sits Beside the Systems Banks Already Run

**The credit and collateral infrastructure a bank already operates, the adapters that let Argent speak to it, and the onboarding path that makes adoption safe and low-friction.**

**Status:** integration design and onboarding framework, not current committed build
**Purpose:** show how Argent connects to existing bank systems rather than replacing them
**Boundary:** Argent is an evidence-and-control layer beside the systems of record. It does not replace the triparty agent, the custodian's books, the CTRM, the messaging network, or the bank's credit core.
**Companion documents:** `bank-integration-and-adapter-strategy.md` (the engineering specification of the adapter gateway that implements this strategy), `argent-architecture.md`, `collateral-eligibility-and-risk-policy.md`, `auto-collateralisation-layer.md`, `argent-dfns-signing-sequence.md`
**Last updated:** 2026-07-09

*This is a design and integration note, not legal, banking, regulatory, or investment advice. It describes how Argent would connect to third-party systems and standards; it does not claim certification by, partnership with, or endorsement from any named vendor, network, or standards body. Message-format names are referenced to show the target of an adapter, not to assert conformance. Any production integration requires the counterparty's own review, security assessment, and contractual arrangements.*

---

## 1. The principle: sit beside the system of record, never replace it

Every institution that has succeeded in this market followed one rule: do not ask the bank to change its system of record. Euroclear's Central Bank Access service lets a bank keep its existing operational setup and continue sending the instructions it sends today while the settlement complexity is handled behind the scenes. Clearstream integrated with the Eurosystem Collateral Management System so that participants use a single collateral pool without ripping out what they had. Komgo reached more than 200 banks and trading companies by digitising the workflow around existing trade-finance operations, not by becoming a new core.

Argent takes the same position. It is not a triparty agent, a custodian, a CTRM, a messaging network, or a bank credit core. It is the layer none of those provides for physical collateral: a shared, role-signed, replayable control-and-evidence record of the pledge lifecycle, sitting beside the systems of record and speaking to them through adapters. The entire adoption thesis is that a bank's collateral operations, risk, and audit functions should be able to consume Argent in the formats and workflows they already use, with the least possible change to the back office.

This is also the pattern the standards bodies are engineering toward. When SWIFT and the ISO 20022 community extended collateral messages to service digital assets, they did so explicitly to leverage existing messages so that back offices need the least possible adaptation. Argent builds with that current, not against it.

---

## 2. What banks already run

Physical-collateral credit sits on four established layers. Argent slots into the seams between them; it does not compete with any.

### 2.1 Triparty collateral agents (the securities benchmark)

Euroclear and Clearstream act as neutral triparty collateral agents, handling allocation, optimisation, and substitution in straight-through processing and real time [1], [2]. In June 2025 the Eurosystem Collateral Management System (ECMS) went live as a single platform replacing the collateral systems previously used by the 20 euro-area national central banks, and Clearstream launched integrated triparty collateral management with it from day one [3], [4]. This is the reference standard for how collateral *should* mobilise, and it is the model the Argent auto-collateralisation layer borrows. It does not cover physical metal in a vault under a bilateral bank-owner-custodian relationship, which is Argent's lane.

### 2.2 The messaging standard (ISO 15022 and ISO 20022)

The triparty lifecycle runs on standardised messages, and this is the most important integration surface for Argent. In the legacy ISO 15022 (MT) form, the core set is MT527 (Triparty Collateral Instruction), sent by a trading party to its triparty agent to instruct an action on a collateral transaction; MT558 (Triparty Collateral Status and Processing Advice); and MT569 (Triparty Collateral and Exposure Statement) [5], [6], [7]. The market is migrating to the ISO 20022 (MX) form under the AMI-SeCo Single Triparty Model for Europe, where the equivalent messages are colr.019 (Triparty Collateral Transaction Instruction), colr.020 (Processing Status Advice), colr.021 (Allegement Notification), colr.022 (Triparty Collateral and Exposure Report), and colr.023 (Status Advice) [8], [9]. Bilateral margining uses the colr series MarginCallRequest, MarginCallResponse, CollateralProposal, and CollateralProposalResponse flow [10].

Two facts make this the highest-leverage adapter target. First, these messages already model exactly the lifecycle Argent records: instruction, status, exposure and collateral valuation, margin call, substitution, and cancellation. Second, the standards bodies have begun extending these same category-5 messages to service digital assets, treating a digital asset as a security type of growing importance and adding structured blocks for digital-asset attributes and oracle-sourced pricing, precisely so that back offices need minimal change [11]. The rails are being widened, conservatively, for exactly Argent's kind of asset.

### 2.3 Commodity trade-finance networks

Physical collateral lives in commodity trade finance, which is less mature than securities. The dominant network is Komgo, used by more than 200 banks and trading companies, founded by ABN AMRO, BNP Paribas, Citi, Crédit Agricole, ING, Mercuria, MUFG, Natixis, Rabobank, Shell, SGS, and Société Générale, among others [12], [13]. Its fraud-response tool, Trakk, lets users register, trace, and authenticate digital documents, linking each registered action to both the company and the authorised individual [13]. That is a document-authenticity layer. It is not a collateral-control-state layer, which is the specific gap Argent fills. Note that Komgo's founding banks are largely the same institutions whose losses and retreat are documented in `collateral-failure-modes.md`: the network exists because these banks already know the problem is real.

### 2.4 CTRM systems and the bank credit core

Traders and lenders run commodity trading and risk management platforms (ION, Murex MX.3, and others) alongside the bank's own credit and collateral core [14], [15]. Murex MX.3 alone reports over 50,000 daily users across trading, treasury, risk, and post-trade [15]. These are systems of record for exposure and valuation. They are not shared control records across bank, custodian, and owner, and they are the source Argent reads a price and exposure feed from, not a thing Argent replaces.

The through-line: securities collateral is highly mobile and standardised; physical-commodity collateral is neither, and the missing piece is a shared, signed control-and-evidence record. That is Argent, located precisely in the stack.

---

## 3. The adapter model

Argent connects through adapters, each a translation between an Argent lifecycle event and a system the bank already runs. Adapters are deliberately thin: they carry evidence and instructions across the boundary, and they never move the system of record inside Argent or move Argent's control state outside it.

### 3.1 Adapter 1: the ISO 20022 collateral-message bridge (highest leverage)

The primary adapter emits and ingests the collateral messages a bank's operations team already processes. When an Argent lifecycle event occurs, the bridge produces the corresponding standardised message, and it accepts standardised instructions as triggers:

| Argent lifecycle event | Outbound message analogue | Inbound trigger analogue |
|---|---|---|
| Pledge activated / transaction opened | colr.019 / MT527 (transaction instruction), colr.020 / MT558 (status advice) | colr.019 / MT527 to instruct an opening |
| Valuation and margin state updated | colr.022 / MT569 (collateral and exposure report) | price feed drives `revalue_and_check` |
| Margin call raised | MarginCallRequest (colr series) | MarginCallResponse / CollateralProposal |
| Substitution executed | colr.019 / MT527 (substitution), colr.020 / MT558 (status) | substitution instruction |
| Exposure and collateral statement | colr.022 / MT569 (exposure report) | statement request |
| Transaction closed / released | colr.020 / MT558 (status), colr.023 (status advice) | cancellation / close instruction |

The point is not conformance certification; it is that a bank's collateral operations desk can see Argent activity in the format its systems already read, and can drive Argent from instructions it already emits, with minimal back-office change. The distinction matters and the document holds it firmly: Argent targets the message families and their semantics, the lifecycle they model, not conformance to the certified SWIFT or ISO rail. Triparty collateral messaging and the SCoRE-aligned Single Triparty Model are real institutional rails; Argent is not a certified triparty messaging product, and the adapter is a translation surface, not a claim of registration. With that boundary held, this is the single most credible line item in an integration plan, because it is standards-based, concrete, and it is the exact adaptation path the standards bodies are building for digital assets, reusing existing category-5 and ISO 20022 messages so back offices need less adaptation [11]. It is future scope, not a certified capability today, and the document says so plainly.

### 3.2 Adapter 2: custodian and CTRM connectors

Two connectors feed the control record its two most important external inputs.

The **custodian connector** is the most important adapter in the whole model, because the custodian's signature is the physical root of trust: `confirm_and_immobilize`, custody-state transitions, and `custodian_confirm_release` are only as good as the link to the custodian's own books. The connector lets the custodian confirm custody state from its own system, and lets Argent reflect that confirmation as a signed event. Where a custodian already reports through a vault or warehouse system, the connector reads that feed; where it signs manually, the connector captures the signed attestation.

The **CTRM and valuation connector** is a read connector supplying the price and exposure feed that `revalue_and_check` consumes, sourced from the bank's or trader's existing risk system rather than from a price oracle Argent asserts on its own. This keeps valuation authority with the bank, consistent with `collateral-eligibility-and-risk-policy.md`.

### 3.3 Adapter 3: the trade-finance evidence pointer

Argent does not compete with a document-authentication network; it complements one. An evidence-pointer adapter lets an Argent evidence-pack hash reference an externally authenticated document, and lets an external record reference the Argent control state that governs that document's collateral. A network that authenticates a warehouse receipt answers "is this document genuine"; Argent answers "what does this document currently control, to whom is it pledged, and can it be released". The two are complementary, and the pointer turns a potential competitor into a distribution surface.

### 3.4 Adapter 4: the settlement and cash leg

The funding leg is specified in `auto-collateralisation-layer.md`: manual bank confirmation first, the Stellar settlement asset (`settlement_vault`) second, a bank ledger API when a design partner defines the rail, and a supplier-payment or tokenized-cash rail later. Argent binds the credit event to the collateral and evidence path; it does not pretend to control bank cash unless the rail is actually integrated.

### 3.5 Adapter 5: the Soroban event indexer

The evidence value of the control record depends on a durable, queryable history of its events, and this is an operational requirement, not an optional nicety. Stellar RPC's `getEvents` method retains at most seven days of recent events, and Stellar's own guidance is that backend components should ingest events into their own database for any longer-lived record [21]. The indexer adapter consumes the `CollateralEventV1` and `GovernanceEventV1` streams as they are emitted, deduplicating on each event's unique id, and persists them so that the pool risk report, the position eligibility certificate, and the replayable audit bundle can be served for the full life of a facility rather than a rolling week. Without this adapter there is no long-term evidence pack; with it, the on-chain event stream becomes a permanent, reconstructable record that a bank's audit and risk functions can query at any point in the exposure's life.

---

## 4. Governance and key custody: the DFNS model

The adapters carry instructions and evidence across the boundary; the governance layer decides who may authorise what, and it is where a bank's security review will focus hardest. Argent's reference governance uses DFNS, and the design principle is worth stating precisely because it maps onto the same principle as the auto-collateralisation layer: automation of execution, never automation of consent.

The distinction DFNS makes explicit is the one that matters. MPC (multi-party computation) protects the *signing process*, distributing key material across nodes in secure enclaves so no single compromised device can sign [16], [17]. But MPC alone is not governance: a transaction can still move fast through an MPC wallet if no approval controls sit above the signing layer [18]. The governance is the *policy engine* that sits above signing, enforcing quorum approvals, amount limits, address allowlists, and method-aware checks, with policies written once and inherited by every wallet, approval groups with named quorums, auto-reject timeouts, and immutable approval logs [17], [19].

This maps directly onto Argent's role model:

- **Role wallets are the signing parties.** Bank, custodian, owner, and verifier each hold a role wallet. `approve_party` and `revoke_party` define the signing set; deny-by-default and role separation mean the owner cannot sign attestations, custody confirmations, or releases, and the operator cannot sign for any institutional role.
- **The policy engine enforces the institutional act before any signature.** A quorum policy on a role wallet requires the right approvers before a release, drawdown, or substitution can be signed, and the policy decoder resolves method, role, pool, policy version, amount, collateral, and evidence hash so a signer never approves an opaque payload. The signing sequence is detailed in `argent-dfns-signing-sequence.md`.
- **Every authorisation is logged and attributable.** The approval log is the off-chain complement to the on-chain `GovernanceEventV1` stream: together they answer not only what happened but who approved it, under what quorum, at what time.
- **Keys can live where the bank's security perimeter already is.** DFNS supports the same governance model whether keys are managed through MPC or inside an HSM, so an institution can extend its existing certified cryptographic boundary to Argent operations rather than adopting a new one [20].

The result a security reviewer needs to hear: Argent does not introduce a new, weaker way to move value. It inherits an institutional-grade policy-and-quorum layer, and every control-state transition is subject to the same deny-by-default, quorum-approved, fully-logged discipline the bank would demand of any signing system.

---

## 5. The onboarding path: read-model first, control later

The safest adoption path is graduated, and it is designed so a bank can derive value at each step while signing and controlling nothing until it chooses to.

**Step 1: reporting overlay (bank signs nothing, controls nothing).** The bank consumes the pool risk report and position eligibility certificate from `collateral-eligibility-and-risk-policy.md` as a read-only view of shared state. It sees what is pledged, to whom, since when, with what evidence and freshness, without Argent controlling anything. This carries zero new contract risk and is exactly what a credit officer evaluates first.

**Step 2: shared control record on one facility (pilot-in-a-box).** One custodian, one lender, one owner, one eligibility schedule, one pledged pool, on testnet or a ring-fenced mainnet facility. The lifecycle (registration, custody confirmation, pledge, valuation, drawdown, repayment, two-step release) runs as role-signed events, with the DFNS policy layer enforcing quorums. Value: the shared control record that the failure-modes losses show is missing.

**Step 3: message-bridge integration.** The ISO 20022 adapter connects Argent events to the bank's collateral operations desk, so the pilot's activity flows into the systems the back office already reads.

**Step 4: policy enforcement and, only then, automation.** The signed collateral policy is enforced on every event; earmarking and auto-collateralisation follow only after the policy layer is proven. Policy first, earmark second, automation third.

At no step does the bank surrender its systems of record. Each step is independently valuable, independently reversible, and independently evaluable by risk and audit.

---

## 6. What risk and compliance need to approve it

Three artifacts, most extending documents already in the repository, turn "interesting protocol" into "the second line can sign off."

**A control-mapping pack.** A short mapping of each Argent control to the bank's own control framework and to the regulatory language it answers to (PFMI Principle 5 for collateral discipline, and the operational-resilience and outsourcing expectations that apply in the pilot jurisdiction). Risk approves what it can map to its own framework, and `collateral-eligibility-and-risk-policy.md` and `threat-model-and-security-boundaries.md` supply most of the content.

**A key-governance and continuity spec.** Who holds the keys, how they are revoked, what the recovery path is, and what happens if Argent the company disappears. Because the core is open-source under Apache-2.0, the answer includes the bank's ability to run it itself, and because governance uses DFNS with HSM support, the answer includes keys living inside the bank's existing certified boundary [20]. This removes the vendor-lock and key-risk objection that ends most infrastructure pilots.

**An exit and fallback story.** The facility must survive Argent's absence. Because Argent records control not title and the asset never leaves custody, the fallback is clean: if the system stops, the custody agreement and security documents still stand, and the last signed state plus the replayable `CollateralEventV1` and `GovernanceEventV1` history is the evidence pack. This is a genuine advantage of the control-not-title design that a tokenization model cannot offer, and it should be stated plainly.

---

## 7. What this integration model is not

- **Argent is not a triparty agent.** It does not hold accounts, settle securities, or act as a neutral collateral manager in the Euroclear or Clearstream sense. It records the control state of a bilateral bank-owner-custodian pledge.
- **Argent does not certify conformance to any message standard.** The ISO 20022 adapter targets the formats a bank uses; it is future scope, and conformance would require the relevant testing and registration.
- **Argent does not replace the custodian's books, the CTRM, or the bank credit core.** It reads from them and writes evidence beside them.
- **Argent does not compete with document-authentication networks.** It records what authenticated documents control, and can point to and from them.
- **Argent does not move keys or value outside the bank's governance.** The DFNS policy-and-quorum layer, optionally HSM-rooted, governs every signature.
- **Argent does not perform legal enforcement or settlement finality.** It produces evidence and binds instructions; settlement and enforcement happen on their own rails.

Stated as a principle: Argent is the thin, shared, signed control-and-evidence layer that the existing stack does not provide for physical collateral, connected to that stack through standards-based adapters, governed by an institutional-grade policy-and-quorum layer, and adoptable one reversible step at a time. It earns its place by making the systems a bank already trusts work together over collateral they currently cannot control cleanly, not by asking the bank to replace any of them.

---

## References

Independent sources, cited to evidence the incumbent stack, the messaging standards, and the governance model described above. No partnership, certification, or endorsement by any named organization is implied.

[1] Euroclear, "Collateral management - Euroclear Bank" (triparty collateral matching, selection, substitution, margin calls, mark-to-market valuation). https://www.euroclear.com/services/en/collateral-management/Collateral-Management-Euroclear-Bank.html

[2] Clearstream, "Triparty collateral services" (neutral collateral agent handling allocation, optimisation, and substitution in straight-through processing and real time). https://www.clearstream.com/clearstream-en/securities-services/collateral-lending-and-liquidity-solutions/collateral-management

[3] Deutsche Börse, "Clearstream Successfully Launches the Only Triparty Collateral Management Solution with ECMS Go-live," June 2025. https://www.deutsche-boerse.com/dbg-en/media/news-stories/press-releases/Clearstream-Successfully-Launches-the-Only-Tri-party-Collateral-Management-Solution-with-ECMS-Go-live-4543260

[4] Securities Finance Times, "Clearstream launches triparty collateral management solution with ECMS," June 2025. https://www.securitiesfinancetimes.com/securitieslendingnews/industryarticle.php?article_id=227996

[5] ISO 20022, "MT 527 Triparty Collateral Instruction." https://www.iso20022.org/15022/uhb/finmt527.htm

[6] ISO 20022, "MT 558 Triparty Collateral Status and Processing Advice." https://www.iso20022.org/15022/uhb/finmt558.htm

[7] ISO 20022, "MT 569 Triparty Collateral and Exposure Statement." https://www.iso20022.org/15022/uhb/finmt569.htm

[8] ISO 20022, "Message Definitions" (colr.019 TripartyCollateralTransactionInstruction, colr.020 ProcessingStatusAdvice, colr.021 AllegementNotification, colr.022 TripartyCollateralAndExposureReport, colr.023 StatusAdvice). https://www.iso20022.org/iso-20022-message-definitions

[9] SWIFT / AMI-SeCo, "Standards Triparty Collateral Management (Standards MX)," 2022 (Single Triparty Model for Europe based on ISO 20022 messaging). https://storage.e.jimdo.com/file/b29a052e-34cb-462b-8f38-4bb3cc5848a3/SR2022_MX_TripartyMDR1.pdf

[10] SWIFT, "Standards Collateral Management (Standards MX)" (bilateral MarginCallRequest, MarginCallResponse, CollateralProposal, CollateralProposalResponse flow). https://www2.swift.com/knowledgecentre/publications/stdsmx_col_mgt_mdrs

[11] ISO 20022, "Standards MT Release Discussion paper and Minutes" (extension of category-5 collateral messages to service digital assets, digital-asset attribute blocks, oracle-sourced pricing, minimal back-office adaptation). https://www.iso20022.org/milestone/23503/download

[12] Consensys, "komgo: Blockchain Case Study for Commodity Trade Finance" (founding banks and single-source-of-truth model). https://consensys.io/blockchain-use-cases/finance/komgo

[13] MUFG EMEA, "Komgo & Commodity Finance" (more than 200 banks and trading companies; Trakk document registration, tracing, and authentication). https://www.mufgemea.com/products-and-services/komgo/

[14] Trade Finance Global, "CTRM (Commodity Trading and Risk Management) Software." https://www.tradefinanceglobal.com/commodity-trading/ctrm-software/

[15] Trade Finance Global / Murex, "Murex MX.3" (integrated trading, treasury, risk, and post-trade; over 50,000 daily users in 60 countries). https://www.tradefinanceglobal.com/commodity-trading/ctrm-software/

[16] Dfns, "Architecture" (policy engine, approval quorum, MPC nodes in secure enclaves). https://docs.dfns.co/core-concepts/architecture

[17] Dfns, "Wallet-as-a-Service" (velocity limits, address allowlists, M-of-N quorums, method-aware checks; policy written once and inherited; approval groups, quorums, auto-reject timeout). https://dfns.co/wallet-as-a-service

[18] BitGo, "Crypto Policy Engines: Approval Workflows and Governance for Institutional Wallets" (MPC protects the signing process; the policy layer governs the decision around it). https://www.bitgo.com/resources/blog/crypto-policy-engines/

[19] Dfns, "Wallet Entitlement Management" (programmatic governance controls, enforced quorum approvals, verifiable approval logs, address whitelisting). https://www.dfns.co/product/wallet-entitlement

[20] Dfns, "Securosys HSM Integration" (same governance model, access policies, and approval workflows whether keys are managed through MPC or inside an HSM). https://dfns.co/article/securosys-hsm-integration

[21] Stellar Developers, "getEvents" and "Ingest events published from a contract" (getEvents retains at most a 7-day window; backend components should ingest events into their own database for a longer-lived record). https://developers.stellar.org/docs/build/guides/events/ingest

# Threat Model and Security Boundaries

> **Positioning status:** The current threat model covers the implemented secured-credit reference branch. The target obligation facility adds beneficiary, instrument authenticity, amendment, presentation, claim, discharge, product-sublimit, and no-unrestricted-cash-draw threats without weakening the existing collateral controls.

**Argent Core V5. Physical-collateral control on Soroban.**

Status: public security-boundary document. This file describes what Argent Core is designed to protect, what it deliberately does not protect, and which trust assumptions remain off-chain.

Argent is a collateral-control system, not a commodity token, custodian, bank, legal-enforcement engine, or price oracle. The contracts record role-signed control facts around physical collateral that remains in custody. The core security question is therefore not “can the chain prove the asset exists?” It cannot. The question is: **given signed attestations, legal documents and role authority, can the system prevent unauthorized, inconsistent, or under-collateralizing control-state changes?**

Throughout this document, a control is marked **(enforced today)** where the open-source contracts enforce and test it now, and **(current build)** where it belongs to the DFNS-governed signing layer that the present build introduces and is not yet in the contracts. This keeps the security claims matched to what the code actually delivers.

---

## 1. System boundary

Argent Core currently consists of three Soroban contracts:

| Contract | Responsibility |
|---|---|
| `credit_ledger` | Control framework, instrument registry, eligibility treatment, lot evidence, pledge, credit line, valuation, drawdown, release, default, cure, enforcement, canonical events. |
| `settlement_vault` | Settlement-asset repayment path. It transfers the settlement asset and calls the bound credit ledger to reduce exposure. |
| `rewards_ledger` | Optional sponsor-funded rewards overlay. It is separated from pledged collateral and is not part of the collateral-control invariant. |

The off-chain service builds transactions, coordinates signatures, indexes events, renders certificates and, in the current build, routes governed actions through DFNS role wallets and approval policies.

---

## 2. Assets and invariants protected by Argent

Argent protects **control state**, not physical title itself.

The protected assets are:

1. The exclusivity of a pledged lot inside the Argent record.
2. The credit line’s drawn balance and available capacity.
3. The lender’s release and enforcement control path.
4. The settlement-to-exposure linkage between repayment and debt reduction.
5. The role registry and instrument eligibility treatment.
6. The chronological event record used by the owner, bank, custodian and auditor.
7. The evidence commitments attached to custody, valuation, pledge, settlement, release and enforcement acts.

The core invariants are:

| Invariant | Required property |
|---|---|
| No duplicate active pledge | The same lot / uniqueness hash cannot back two active pledges in the Argent record. |
| No unauthorized business act | Each state-changing act must be signed by the party whose role authorizes that act. |
| No revoked role action | Governed parties must be checked against the current approval table at the time they act. |
| No draw through capacity | A drawdown cannot exceed the available secured capacity. |
| No missing eligibility treatment | A position or credit line cannot rely on an instrument that is not registered, active and admitted to the framework. |
| No valuation-bearing act without source | Price-based actions require a non-zero valuation reference where the function requires one. |
| No repayment/exposure drift | A repayment through the settlement vault must move settlement value and reduce drawn exposure atomically, or neither happens. |
| No release without the release path | Repayment alone does not release collateral; the bank authorizes and the custodian confirms. |
| No enforcement before default path | Enforcement requires the default/cure path and the required enforcement evidence. |
| No placeholder evidence for critical records | Zero hashes are rejected for critical legal, collateral, valuation and readiness records. |

---

### 2.1 Target obligation-profile assets

The mature profile additionally protects:

- available, reserved, issuable, deliverable, and releasable capacity;
- reservation identity, purpose, amount, policy version, and expiry;
- authoritative product-system and callback correlation;
- beneficiary and obligation lifecycle state;
- role-specific data views and evidence permissions;
- confidentiality of bar, reserve, beneficiary, claim, and group-exposure information.

These are target-profile properties unless implemented in source and tests. Their detailed specifications are [`capacity-reservation-and-deliverability.md`](capacity-reservation-and-deliverability.md) and [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md).

## 3. Trust assumptions

Argent is designed around explicit trust boundaries.

### 3.1 Physical and legal layer

Argent assumes that custody, legal ownership, pledge agreements, control agreements, warehouse records, valuation sources, insurance and enforcement rights are established off-chain.

The contracts do **not** verify:

- the physical existence of a bar, lot, receipt, warehouse stock or commodity;
- the legal validity of a pledge agreement;
- whether a custodian’s statement is true;
- whether a valuation source is economically correct;
- whether a court, liquidator, warehouse operator or custodian will perform as expected;
- whether a pledge or lien exists outside Argent.

The contracts record who signed, in what role, against which evidence hash, and what control state changed because of that signed act.

### 3.2 Signing layer

In the open-source contracts today, Soroban `require_auth` and the role-approval table enforce who may act **(enforced today)**. The DFNS-governed mapping of institution roles to controlled wallets and approval policies is introduced by the current build **(current build)**; it is an additional institutional governance layer, not yet part of the contracts in this repository.

The contract still enforces `require_auth` and role approval. DFNS is an additional institutional governance layer, not a substitute for contract authorization.

### 3.3 Settlement asset

`settlement_vault` assumes the configured settlement token is a trusted Stellar/Soroban asset selected by the deployment. The vault does not independently prove commercial finality outside the token transfer it executes.

### 3.4 Service layer

The service is not trusted to authorize business actions. It builds, routes and submits transactions. The contracts must reject an invalid action even if the service constructs it.

The service is trusted operationally for:

- using the correct contract IDs;
- indexing the correct event stream;
- presenting evidence without altering meaning;
- avoiding blind resubmission after ambiguous transaction status;
- protecting environment variables, API tokens and signing-service credentials.

---

## 4. Threats, controls and residual risks

| Threat | Contract control | Residual risk |
|---|---|---|
| Borrower or owner tries to pledge the same lot twice | Lot uniqueness lock and duplicate-pledge tests. | A pledge outside Argent cannot be detected unless the custodian/bank integrates external lien checks. |
| Bank opens a line against an ineligible instrument | Instrument registry and framework-level eligibility treatment are checked before position/line use. | The bank may admit a poor instrument under its own policy; Argent enforces policy, it does not judge credit quality. |
| LTV exceeds lender ceiling | `open_credit_line` checks requested LTV against the admitted instrument treatment. | Incorrect treatment values remain a bank policy/input risk. |
| Stale or missing valuation used for credit capacity | Valuation-bearing paths require references and freshness checks where implemented. | The chain cannot know whether the external valuation source is economically fair. |
| Unauthorized bank/custodian action | Business functions require `require_auth` plus current role approval. | Compromised role wallet or compromised DFNS organization remains an operational risk. |
| Revoked role continues to act | State-changing role functions must re-check approval after auth. | Every newly added entrypoint must preserve this pattern. New functions require explicit authorization review. |
| Settlement vault applies repayment without token movement | Vault transfers the token and calls credit ledger in one transaction; unapproved vault rollback tests cover failure paths. | Commercial settlement outside the token rail is outside scope. |
| Duplicate repayment reference | Repayment records and vault tests cover duplicate-payment rollback. | Off-chain payment references must be unique and well generated by the service. |
| Repayment releases collateral automatically | Repayment reduces exposure only; release remains a separate bank/custodian path. | Service or UI must not imply that repayment alone frees custody. |
| Unauthorized release | Bank authorization and custodian confirmation are separate required acts. | Real-world custodian must respect the on-chain/control instruction and legal agreement. |
| Owner blocks enforcement through release consent | Enforcement flow should follow pre-agreed enforcement rules; owner consent must not be required for enforcement release unless legally intended. | Release-policy design must distinguish voluntary release from enforcement. |
| Event trail diverges from state | Tests assert canonical events and replay properties. | If future events summarize multi-entity state transitions, projection rules must stay explicit and tested. |
| Governance events omitted or misleading | GovernanceEventV1 records authority acts. | Governance event scope must be kept aligned with every new governance function. |
| Contract ID misconfiguration | Runbook requires getter verification and full reference lifecycle before service switch-over. | Human/operator error remains possible. Use an evidence-pack checklist before public demo. |
| Secret leakage in public repo | `.env`, keys, PEM files and private credentials must never be committed. | Repository review must be repeated before each public push. |

---

### 4.1 Target reservation, interoperability, and privacy threats

| Threat | Target control | Residual risk |
|---|---|---|
| Concurrent requests over-allocate the same capacity | atomic reservation, version checks, and aggregate plus sublimit validation | pledges or holds created outside Argent remain an external authority risk |
| Abandoned reservations block usable capacity | reservation expiry, cancellation, renewal policy, and reconciliation | bad expiry policy can release too early or retain capacity too long |
| Duplicate request creates duplicate obligation | stable idempotency key, canonical request digest, and authoritative product reference | weak external identifiers may require manual repair |
| Issue succeeds but callback is lost | committed reservation, status query, and no blind resubmission | product-system outage delays certainty and capacity reuse |
| Callback is spoofed, duplicated, or reordered | authenticated callback, correlation, signature or request validation, and lifecycle version | compromised external system remains a trust risk |
| Policy or beneficiary status changes after reservation | policy version pinning and revalidation before issue or amendment | emergency override still requires explicit governance |
| Ledger, bank, and custodian states diverge | exception state, fail-closed progression, and reconciliation report | operational resolution may require human and legal judgment |
| Public event reveals reserve or commercial activity | data minimization, role projections, value bucketing or commitments where appropriate, and privacy review | timing, graph, and repeated-identifier analysis may still correlate activity |
| Evidence hash permits dictionary attack | salt or keyed commitment where appropriate; do not publish low-entropy sensitive values | poor canonicalization or key handling can still expose or invalidate evidence |
| Unauthorized evidence disclosure | encrypted storage, purpose-bound access, tenant isolation, access logging, and disclosure receipt | privileged insider or endpoint compromise remains possible |
| Selective proof is mistaken for legal or physical truth | explicit `does_prove` and `does_not_prove` semantics; authoritative-source reference | verifier may still over-rely without legal and operational review |

| Upstream reserve source is compromised or supplies false backing data | bank-approved source list, signed assertions, assurance-scope validation, freshness checks, independent custody/legal evidence, discrepancy state | a compromised authoritative provider remains a concentrated trust risk |
| Upstream assurance is interpreted beyond its scope | explicit assurance scope plus `does_prove` and `does_not_prove`; separate pledgeability, control, and issuability gates | recipients may still over-rely without legal and operational review |
| Asset-layer fungibility is mistaken for transferable facility capacity | equivalence-class metadata, purpose-bound reservations, non-transferability rule, bank-approved amendment path | external systems may still mislabel or aggregate positions incorrectly |
| Source reconciliation tolerance is breached but new issue continues | tolerance status is a hard preflight input; stop new reservation and issuance; open exception | manual override can reintroduce risk and requires governed authority |
| Reserve-profile confusion treats issuer claim as proprietary gold | explicit `ReserveProfile`, rights gate, operator and insolvency analysis, profile-specific policy | legal character can vary by jurisdiction and product terms |
| Same digital entitlement is represented again on Stellar | reference-only adapter and prohibition on duplicate asset representation | external operators may still issue conflicting units outside Argent |
| Upstream record becomes stale or disappears during active exposure | stop new issue, retain reservation and exposure, open reconciliation, require bank-directed resolution | prolonged source outage may require manual margin, substitution, or enforcement |
| Shared bar list or customer balance leaks through evidence | signed minimum assertion, private source record, role-specific access, disclosure logging | source-system insiders and endpoint compromise remain possible |
| Upstream platform and custodian disagree | no silent overwrite, exception state, authority-specific evidence, manual resolution | physical and legal resolution may be slow and jurisdiction-dependent |

See [`shared-gold-infrastructure-and-argent.md`](shared-gold-infrastructure-and-argent.md) for the authority and adapter boundary.

No privacy mechanism changes the hierarchy of authority: the custodian controls physical truth, the bank controls product issuance, and governing law controls legal effect.

## 5. Out of scope

Argent intentionally does not provide:

- physical custody;
- lending decisions;
- credit underwriting;
- legal advice;
- valuation advice;
- insurance;
- KYC/AML/sanctions screening;
- automatic physical enforcement;
- a public commodity token;
- rehypothecation of the same physical lot;
- yield generation from pledged physical assets;
- a guarantee that the same asset has not been pledged in another system.

This boundary is central to the design. Argent records and enforces the shared control state for a facility that has already been legally and operationally structured by the parties.

---

## 6. Security posture by layer

### 6.1 Contract layer

Security expectations:

- all state-changing functions require the correct signer;
- all governed roles are checked against the current approval table;
- all monetary and collateral-capacity math uses checked integer arithmetic;
- critical evidence hashes reject zero values;
- price-bearing acts require valuation references where applicable;
- settlement and exposure update atomically;
- release and enforcement follow state-machine order;
- canonical events are emitted for committed business/governance acts.

### 6.2 DFNS layer (current build)

Security expectations for the signing layer the current build introduces:

- deny-by-default role policies;
- role-specific wallets;
- quorum approval for release and enforcement;
- approval IDs reconciled to Soroban transaction hashes;
- approval payload must match the submitted Soroban authorization-entry hash;
- no role wallet may be reused across incompatible authorities.

### 6.3 Service/indexer layer

Security expectations:

- no blind tx resubmission after ambiguous confirmation;
- no fallback from canonical chain state to stale local state for security decisions;
- all certificates identify contract IDs, network, ledger, tx hashes and evidence hashes;
- indexer treats decode failures for canonical events as errors, not silent warnings;
- service can be restarted without losing the reference lifecycle state.

### 6.4 Documentation layer

Security expectations:

- no claim that Stellar verifies physical truth;
- no claim that the asset is tokenized;
- no claim that repayment alone releases collateral;
- no claim that owner consent is always required for enforcement;
- public docs distinguish live functionality from roadmap features.

---

## 7. Known residual risks before production

The following are not necessarily defects in the prototype, but they must be addressed before real-value production use:

1. Independent legal review of pledge, custody, control, release and enforcement documentation.
2. Independent security audit of contracts and service layer.
3. DFNS policy testing with real approval groups and failure cases.
4. Production-grade indexer and evidence pack generation.
5. Operational runbook for ambiguous transaction status, stuck approvals, role revocation and contract-ID rotation.
6. Settlement-asset selection and legal treatment review.
7. Custodian integration model for off-chain custody-book updates.
8. Monitoring and incident response for service/API key compromise.
9. Reservation concurrency, expiry, idempotency, callback authentication, and ambiguous-outcome testing.
10. Daily reconciliation between bank product, custodian, DFNS, Soroban, settlement, and evidence state.
11. Data-protection impact assessment, role-view review, retention schedule, and metadata-leakage analysis.
12. Independent review of any selective-credential or zero-knowledge implementation before relying on it in production.

---

## 8. Reviewer checklist

A reviewer should verify:

- [ ] `cargo test --workspace` passes with the published test counts.
- [ ] Role revocation tests cover bank, custodian, valuation, processor and vault surfaces.
- [ ] Duplicate pledge tests use the lot uniqueness key.
- [ ] Settlement vault tests prove rollback on failed repayment.
- [ ] Release tests prove repayment does not release collateral.
- [ ] Governance event tests prove authority acts are recorded separately from deal acts.
- [ ] The live evidence pack points to the same commit, contract IDs and testnet lifecycle.
- [ ] Public docs do not overclaim physical truth, legal enforceability or tokenization.
- [ ] Target reservation tests cover concurrent requests, expiry, cancellation, duplicate commands, lost callbacks and ambiguous issue status.
- [ ] Every external callback is authenticated and correlated to an originating request and reservation.
- [ ] Reconciliation fails closed when bank, custodian, DFNS, Soroban, settlement or evidence states disagree.
- [ ] Public and shared payloads exclude raw bar lists, KYC, complete legal documents and unnecessary beneficiary data.
- [ ] Privacy review covers hashes of low-entropy data, event timing, exact amounts, stable identifiers and graph correlation.
- [ ] Selective-disclosure evidence states what it proves, what it does not prove and which authority supplied the underlying fact.

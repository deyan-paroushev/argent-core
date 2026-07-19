# Selective Disclosure and Institutional Privacy

**A target Argent Protocol specification for sharing enough information to authorize, verify, audit, and supervise reserve-backed obligations without exposing the full reserve, facility, or transaction book.**

**Status:** canonical design specification; not yet the implemented contract surface  
**Applies to:** shared ledger state, evidence packages, bank and custodian adapters, beneficiary verification, audit exports, and future privacy-preserving credentials or proofs  
**Primary rule:** shared truth does not require shared access to every underlying fact

---

## 1. Purpose

Argent coordinates information that is commercially sensitive and operationally consequential:

- bar and lot identities;
- reserve quantity and value;
- bank haircuts and limits;
- free and reserved capacity;
- beneficiary and transaction details;
- pending presentations, claims, defaults, and enforcement;
- legal and custody evidence;
- institutional approval paths.

A public or broadly shared ledger is useful for ordered authorization, deterministic refusal, and replayable evidence. It is not an appropriate place for the full bar list, customer position, bank risk model, guarantee wording, KYC file, or claim package.

The privacy objective is therefore:

> **Disclose the minimum fact required for the receiving party to perform its role, while preserving a verifiable link to the authoritative facility and evidence state.**

This document defines the data classes, visibility model, disclosure products, cryptographic options, metadata controls, and operational requirements for that objective.

---

## 2. Privacy is part of correctness

Confidentiality is not a cosmetic feature for an institutional collateral system.

Excess disclosure can:

- reveal a company's liquidity position;
- expose bar serial numbers and vault information;
- disclose the bank's risk appetite or pricing assumptions;
- reveal suppliers, customers, contracts, trade corridors, or tender activity;
- advertise pending claims, distress, or enforcement;
- create a new target for fraud, coercion, or cyberattack;
- conflict with bank secrecy, contractual confidentiality, data-protection, or supervisory duties;
- allow outsiders to infer unused capacity and probe the facility.

Insufficient disclosure can also be harmful. A beneficiary, auditor, custodian, or supervisor may be unable to verify that:

- the issuing bank approved the instrument;
- sufficient capacity was reserved;
- the relevant custodian confirmed control;
- the instrument remains active;
- a release or enforcement event was properly authorized.

The design must therefore balance confidentiality, verifiability, and operational usability.

---

## 3. Governing principles

### Principle 1 - data minimization

Record or disclose only what is necessary for the defined purpose.

### Principle 2 - purpose binding

Every disclosure request should state who is asking, for what purpose, which claims are required, and for how long the result may be used.

### Principle 3 - role-specific visibility

A bank, custodian, owner, beneficiary, auditor, and public observer do not receive the same view.

### Principle 4 - authoritative-source separation

The ledger proves the shared control event. The custodian, bank product system, legal file, and settlement rail remain authoritative for their own facts.

### Principle 5 - privacy by architecture, not redaction after publication

Sensitive information should not be placed on a public ledger with the expectation that it can later be hidden or deleted.

### Principle 6 - verifiable claims, not unverifiable summaries

A minimal disclosure should still be signed, time-bound, policy-versioned, and linked to the relevant facility state.

### Principle 7 - prevent replay and cross-context reuse

A proof issued for one verifier, purpose, amount, or time window should not be reusable silently in another context.

### Principle 8 - minimize linkability

Repeated presentations should avoid unnecessary stable identifiers or exact values that allow unrelated verifiers to correlate the customer or facility.

### Principle 9 - no privacy claim beyond the technology actually deployed

Hashes, encryption, credentials, selective disclosure, and zero-knowledge proofs provide different protections. Documentation must state which level is implemented.

---

## 4. Data classification

### 4.1 Public protocol data

Information safe to publish broadly, such as:

- protocol and contract identifiers;
- network and contract version;
- public schema and event definitions;
- open-source code and verification instructions;
- generic policy identifiers that reveal no client data;
- aggregate system-health information where approved.

### 4.2 Shared facility state

Information shared among authorized facility participants and potentially anchored on-chain in minimized form:

- facility or position pseudonymous identifier;
- lifecycle state;
- role that authorized the event;
- policy version or commitment;
- capacity delta or commitment where required for enforcement;
- evidence hash or opaque reference;
- sequence and timestamp;
- status such as active, suspended, released, or enforcement-locked.

Even shared state should avoid plaintext bar serials, customer names, beneficiary names, account numbers, legal terms, or complete values unless the deployment explicitly accepts that disclosure.

### 4.3 Role-restricted operational data

Examples:

- full bar and lot list;
- reserve quantity, location, and value;
- bank haircut, sublimits, and risk adjustments;
- product request, beneficiary, and commercial purpose;
- guarantee or documentary-credit text;
- presentations, claims, and reimbursement data;
- group-company allocations;
- external bank, custodian, and settlement references.

This data belongs in encrypted systems or evidence packages with role-based access and audit logging.

### 4.4 Highly restricted evidence

Examples:

- KYC, AML, sanctions, and beneficial-owner files;
- legal opinions and executed security documents;
- vault access or security information;
- unredacted account, payment, or custody credentials;
- claim documents containing personal or commercially sensitive information;
- supervisory correspondence;
- incident and investigation records.

This material should not be copied into a general Argent evidence store unless the deployment, legal basis, and controls expressly require it.

### 4.5 Secrets

Never publish or include in evidence packages:

- private keys or seed phrases;
- DFNS credentials, service-account secrets, or API tokens;
- HSM credentials;
- database passwords;
- production encryption keys;
- raw session tokens;
- signing recovery material.

---

## 5. Role-specific visibility matrix

| Party | Minimum view | Normally restricted from |
|---|---|---|
| Reserve owner | own reserve, facility, obligations, approvals, and evidence status | other customers, bank-wide limits, unrelated beneficiary data |
| Issuing bank | eligible reserve, policy, limits, obligations, claims, reimbursement, and enforcement | custodian internal security operations not needed for the facility |
| Custodian | exact reserve identity, control instructions, release and realization authority | bank pricing, unrelated products, full commercial transaction details |
| Beneficiary | instrument authenticity, issuing bank, amount or required predicate, expiry, and current status | bar list, total reserve, other obligations, bank collateral policy |
| Applicant subsidiary | own sublimit, requests, and obligations | parent reserve total and sibling-company allocations unless authorized |
| Verifier or document source | evidence request and result relevant to its function | full facility and reserve book |
| Auditor | evidence necessary to test the defined control objective | data outside the audit scope |
| Supervisor | information legally required for supervision | data outside legal authority and purpose |
| Operator | technical correlation and operational status required to run the service | discretionary access to unredacted bank, custody, or client files |
| Public observer | protocol-level and approved aggregate information only | facility, customer, reserve, beneficiary, claim, and enforcement details |

Access should be derived from role, facility, purpose, current authorization, and data class, not network location alone.

---

## 6. On-chain data minimization

### 6.1 What the shared ledger should prove

A minimized event may need to prove:

- an authorized role performed a defined action;
- the action concerned a specific facility or pseudonymous position;
- required prior state existed;
- capacity or control state changed according to policy;
- an evidence package existed at a specific version;
- the event occurred in a specific sequence and time window.

### 6.2 What should remain off-chain

Normally off-chain:

- legal names and account numbers;
- full bar serial list;
- exact vault location;
- complete reserve value;
- private bank risk model;
- beneficiary identity;
- product wording;
- trade documents;
- claim and reimbursement documents;
- KYC and sanctions evidence;
- legal opinions and executed agreements.

### 6.3 Commitments and references

An on-chain record may contain:

- a cryptographic hash of a canonical evidence manifest;
- a salted or keyed commitment to an identifier set;
- an opaque evidence-package ID;
- a policy-version commitment;
- a pseudonymous facility or obligation ID;
- a coarse state or predicate result.

A raw hash is not automatically private. Low-entropy data, known templates, short identifiers, predictable amounts, and public documents can be guessed and hashed by an attacker. Sensitive commitments should use appropriate salting, domain separation, access control, and canonicalization.

### 6.4 Exact values versus predicates

Where exact values are not needed, prefer predicates such as:

- reserve capacity is at least the required amount;
- the obligation remains within the approved sublimit;
- the custodian attestation is current;
- the instrument is active and not cancelled;
- no unresolved release block exists.

Do not publish an exact reserve value merely to prove sufficiency.

---

### 6.5 Privacy across shared gold infrastructure

An upstream gold platform may maintain shared bar lists, custody data, beneficial ownership, token supply, customer balances, or redemption records. "Shared" in that context does not mean public.

Argent should consume the minimum signed assertion needed for the facility decision, such as:

- this owner or entitlement holder controls an eligible position;
- the position represents at least the stated quantity or value;
- backing, allocation, and custody are current;
- no conflicting hold is known within the source system's scope;
- the assertion is valid until a stated time or version change.

Detailed source records should remain with the authoritative custodian, register, or product operator unless a bank, auditor, supervisor, or enforcement process has a documented need to access them.

Privacy controls must prevent publication of a full shared bar list, correlation across unrelated products, inference of total holdings from a reservation, cross-context reuse of an assertion, and a public proof being mistaken for a complete title or backing opinion.

An Argent evidence record should contain the source-system identifier, assertion type, policy version, timestamp, expiry, commitment, and disclosure log. It should not become a shadow gold registry.

See [`shared-gold-infrastructure-and-argent.md`](shared-gold-infrastructure-and-argent.md).

## 7. Evidence-package architecture

### 7.1 Evidence manifest

Each evidence package should have a manifest containing:

- package ID and version;
- facility, reservation, obligation, or event reference;
- document types, not necessarily plaintext names;
- canonical hashes;
- issuer or source identity;
- creation and expiry times;
- classification;
- permitted roles and purposes;
- retention category;
- encryption and key reference;
- revocation or supersession status.

### 7.2 Storage

Evidence should be stored in an institution-approved encrypted repository. The public chain should contain only the minimum commitment or reference needed by the protocol.

### 7.3 Access control

Access decisions should consider:

- authenticated party;
- facility role;
- requested evidence type;
- declared purpose;
- legal or contractual authority;
- current approval;
- time and session context;
- export restrictions;
- incident or litigation hold.

### 7.4 Audit trail

The evidence service should record:

- who requested access;
- what was requested;
- purpose and authority;
- what was disclosed;
- whether the disclosure was viewed or exported;
- decision and reason;
- timestamp;
- correlation to the facility event.

### 7.5 Retention and deletion

Retention should be policy-driven. The system should support:

- expiry of temporary presentations;
- supersession of stale evidence;
- retention holds where legally required;
- deletion or cryptographic erasure where permitted;
- continued preservation of the minimal immutable event commitment.

The inability to delete a public-chain event is a reason to minimize it before publication.

---

## 8. Disclosure products

### 8.1 Reserve Sufficiency Statement

Purpose: prove that sufficient eligible and unallocated capacity supports a specified request or obligation.

Possible disclosed claims:

- issuing bank or approved facility authority;
- obligation or request pseudonymous reference;
- required capacity threshold;
- result: sufficient or insufficient;
- policy version;
- validity period;
- current status;
- signature or proof.

Normally withheld:

- total reserve value;
- exact haircut;
- full bar list;
- other obligations;
- remaining free capacity.

### 8.2 Custody and Control Statement

Purpose: prove that the relevant reserve is under approved custody and control.

Possible claims:

- approved custodian;
- facility or position reference;
- control state;
- attestation time and expiry;
- no-release or release-pending status;
- evidence commitment.

Normally withheld:

- exact vault location;
- complete serial list;
- unrelated customer positions.

### 8.3 Instrument Authenticity and Status Statement

Purpose: allow a beneficiary or verifier to confirm that a bank instrument exists and remains in the stated status.

Possible claims:

- issuing bank;
- instrument reference;
- beneficiary or verifier binding;
- amount or amount predicate;
- currency;
- expiry;
- status;
- applicable rules reference;
- last verified time.

This statement must not imply that Argent itself issued the instrument.

### 8.4 Facility Participation Credential

Purpose: prove that an applicant, subsidiary, custodian, verifier, or operator is authorized for a defined role and scope.

Possible claims:

- role;
- facility scope;
- product scope;
- validity period;
- revocation or suspension status;
- issuer.

The credential is not transferable facility capacity and does not create credit.

### 8.5 Audit or Supervisory Evidence Pack

Purpose: disclose the evidence needed to test a defined control or meet a legal request.

The pack should be purpose-specific, time-bound, logged, and separable from routine beneficiary or customer views.

---

## 9. Disclosure maturity levels

Argent should not treat advanced cryptography as a prerequisite for the first production pilot.

### Level 0 - role-based private API

- authenticated institution-to-institution API;
- field-level authorization;
- signed response;
- encrypted transport;
- audit logging.

This is the minimum viable institutional pattern.

### Level 1 - signed minimal statements

- signed reserve sufficiency, custody, or instrument-status statement;
- short validity period;
- verifier and purpose binding where possible;
- no disclosure of unrelated fields.

### Level 2 - selectively disclosable credentials

- credentials structured so the holder presents only requested claims;
- verifier requests the strictly necessary claims;
- replay and audience controls;
- credential status and expiry;
- potentially W3C Verifiable Credentials with BBS-derived proofs, SD-JWT-based credentials, or another institution-approved format.

### Level 3 - predicate or zero-knowledge proofs

Examples:

- eligible free capacity is at least X;
- concentration is below a permitted threshold;
- the instrument remains within a product sublimit;
- all required roles approved without revealing their personal identities.

This level requires careful protocol review, implementation maturity, performance testing, and independent cryptographic assessment. It should not be promised before it is built and audited.

---

## 10. Credential and presentation model

A future credential pattern may use the issuer-holder-verifier model:

- **Issuer:** bank, custodian, facility authority, or approved evidence provider;
- **Holder:** reserve owner, applicant, beneficiary, or delegated institutional wallet;
- **Verifier:** bank function, beneficiary, auditor, supervisor, or approved counterparty.

A presentation request should identify:

- verifier;
- purpose;
- requested claims or predicates;
- facility or transaction context;
- nonce;
- audience;
- expiry;
- response encryption requirements.

A presentation should be bound to the verifier and transaction context to reduce replay and cross-context use.

W3C Verifiable Credentials Data Model 2.0 defines an extensible issuer-holder-verifier model. OpenID for Verifiable Presentations 1.0 defines a protocol for requesting and returning credential presentations and emphasizes selective disclosure, purpose legitimacy, strictly necessary claims, nonce binding, and replay prevention. W3C BBS cryptosuites define one approach to selective disclosure and unlinkable derived proofs. These are optional design precedents, not current Argent dependencies.

---

## 11. Metadata and inference risks

Even when payloads are encrypted or hashed, public metadata can leak information.

### 11.1 Timing correlation

A public reservation or claim event may correlate with:

- a tender deadline;
- a shipment;
- a known project award;
- a public dispute;
- market stress.

Mitigations may include coarse timestamps in public views, batching where compatible with control requirements, private read models, or avoiding public events for sensitive detail.

### 11.2 Stable identifier correlation

A stable facility or company identifier can link unrelated obligations over time.

Use scoped pseudonymous identifiers where cross-context correlation is not required. Maintain the authoritative mapping in a restricted system.

### 11.3 Exact amount inference

Exact capacity deltas can reveal reserve size, product pricing, and liquidity pressure.

Use commitments, bands, predicates, or restricted events when exact amounts are not required for shared enforcement.

### 11.4 Hash dictionary attacks

A hash of a predictable document, serial number, or short identifier may be reversible by guessing.

Use random salt or keyed commitments where appropriate, and never treat an unsalted low-entropy hash as anonymization.

### 11.5 Status probing

Repeated preflight or verification requests can reveal free capacity or bank policy.

Require authenticated requesters, rate limits, purpose checks, coarse external reason codes, and anomaly monitoring.

---

## 12. Privacy architecture

### 12.1 Shared execution layer

Soroban should receive only the state required to:

- authorize the correct role;
- prevent invalid sequencing or double allocation;
- enforce capacity and release rules;
- emit a minimized evidence event.

Stellar contract events are durable and broadly observable through network infrastructure. Sensitive data should therefore not be emitted with the expectation of later removal.

### 12.2 Institutional approval layer

DFNS or equivalent signer governance controls whether an institution may sign. It does not by itself make Soroban payloads confidential. The payload presented for approval should be human-readable to authorized approvers but minimized before public submission.

### 12.3 Private read model

The private read model joins:

- DFNS approvals;
- Soroban state and events;
- bank product state;
- custody evidence;
- settlement or reimbursement state;
- access-control and disclosure logs.

It should present different projections by role rather than one universal dashboard.

### 12.4 Evidence service

The evidence service stores and serves encrypted packages, evaluates disclosure policy, and creates signed or selectively disclosable outputs.

### 12.5 Key management

Production design should separate:

- transaction-signing keys;
- evidence-signing keys;
- data-encryption keys;
- per-package or per-tenant encryption keys;
- recovery and rotation controls.

Compromise of one key class should not automatically expose all evidence or authorize facility actions.

---

## 13. Privacy policy object

A target `DisclosurePolicy` may define:

- policy ID and version;
- data class;
- eligible requesters and roles;
- allowed purposes;
- permitted claims or predicates;
- prohibited fields;
- verifier-binding requirements;
- maximum presentation validity;
- retention and export rule;
- approval requirements;
- revocation and suspension behavior;
- audit requirements;
- governing jurisdiction or contractual reference.

A policy change should not silently alter prior presentations. New disclosures should identify the policy version used.

---

## 14. Security threats and controls

| Threat | Required control | Residual risk |
|---|---|---|
| Public event reveals reserve or beneficiary | minimised schema and restricted detail | timing and frequency may still reveal activity |
| Hash exposes predictable private value | salted or keyed commitment; avoid low-entropy hashes | compromised salt or small search space can weaken protection |
| Beneficiary receives full reserve book | purpose-specific status statement | beneficiary may still infer bank confidence from the instrument |
| Verifier replays an old sufficiency proof | nonce, audience, expiry, and current-status check | offline verification may have stale status |
| Two verifiers correlate presentations | pairwise or scoped identifiers; unlinkable proof where mature | operational data can still correlate parties |
| Operator browses client documents | least privilege, separate evidence service, access logging | privileged administrators remain insider risk |
| Bank policy leaks through reason codes | role-scoped and coarse external responses | repeated probing can still infer thresholds |
| Evidence URL leaks | opaque identifiers, authentication, no secrets in URLs | browser or proxy logs remain a risk if misconfigured |
| Encryption key compromise | key separation, rotation, HSM/KMS, incident response | historical ciphertext may remain exposed depending on design |
| Unauthorized export | export controls, watermarking where appropriate, audit and approval | recipients can mishandle legitimately disclosed data |
| Deleted credential remains usable | short expiry, status or revocation check, verifier policy | offline or non-conforming verifiers may ignore status |
| Public chain and custody record disagree | stop affected workflow, preserve evidence, reconcile authority | operational resolution can be delayed |

---

## 15. Operational controls

A production privacy runbook should cover:

- user and service-account provisioning;
- role and facility scope changes;
- access reviews;
- evidence encryption and key rotation;
- credential issuance, renewal, suspension, and revocation;
- disclosure approvals;
- export and download controls;
- anomalous access monitoring;
- incident containment;
- breach assessment and notification duties;
- legal hold and retention;
- data-subject or customer requests where applicable;
- disaster recovery without broadening access.

Privacy failures should be treated as facility incidents, not merely application defects.

---

## 16. Mapping to the current repository

### Current strengths

- contract state stores evidence hashes or references rather than full private documents;
- lot identity uses a uniqueness commitment rather than requiring the full bar list in shared state;
- role-specific authorization is enforced by Soroban;
- DFNS integration is designed around institution-specific wallets, policy, and approvals;
- public documentation distinguishes ledger evidence from physical and legal truth;
- evidence-pack guidance excludes secrets and unredacted client material.

### Current limitations

- public and private event projections are not yet separate production surfaces;
- typed obligation and beneficiary views are not implemented;
- no credential or selective-presentation service is implemented;
- no cryptographic predicate proof is implemented;
- evidence access, retention, and disclosure policy are not implemented as protocol objects;
- privacy leakage from event timing, exact values, and stable identifiers needs deployment-specific review.

### Target additions

- role-specific read-model projections;
- `DisclosurePolicy` and evidence classification;
- reserve sufficiency and instrument-status statements;
- verifier, purpose, nonce, and expiry binding;
- credential status or revocation;
- encrypted evidence store and access audit;
- privacy test matrix and incident runbook;
- optional selective-disclosure or predicate-proof adapters after design-partner validation.

---

## 17. Implementation sequence

### Phase 1 - minimised schemas and private projections

- classify every field and event;
- remove unnecessary plaintext from shared state;
- implement owner, bank, custodian, beneficiary, operator, and auditor projections;
- add access and export logs.

### Phase 2 - signed minimal statements

- reserve sufficiency statement;
- custody and control statement;
- instrument authenticity and status statement;
- short validity and verifier context;
- status endpoint and revocation behavior.

### Phase 3 - evidence policy service

- `DisclosurePolicy`;
- encrypted evidence packages;
- purpose and role checks;
- approval workflow for sensitive disclosures;
- retention and incident controls.

### Phase 4 - interoperable credentials

- evaluate W3C VC, OpenID4VP, BBS, SD-JWT, or institution-selected alternatives;
- implement conformance and replay tests;
- support minimal-claim presentations.

### Phase 5 - predicate proofs

- prove capacity or policy predicates without exact values;
- independent cryptographic review;
- performance, revocation, and operational-fallback testing.

---

## 18. Conformance checklist

A conforming deployment should demonstrate:

- [ ] every shared field has a documented purpose and classification;
- [ ] no private key, API secret, KYC file, or unredacted legal file is published;
- [ ] the public chain does not contain the full bar list or full facility book;
- [ ] beneficiaries cannot access unrelated reserve or obligation data;
- [ ] exact reserve values are withheld where a predicate is sufficient;
- [ ] low-entropy evidence is not protected by an unsalted hash alone;
- [ ] role-specific views are tested for over-disclosure;
- [ ] disclosure requests identify verifier, purpose, claims, and validity;
- [ ] presentations are replay-resistant and audience-bound where supported;
- [ ] credential or statement expiry and revocation are enforced;
- [ ] access and export activity is logged;
- [ ] evidence encryption keys are separated from transaction-signing keys;
- [ ] privacy incidents have an operational response path;
- [ ] documentation distinguishes implemented privacy controls from roadmap controls;
- [ ] the custodian, bank product system, and legal record remain authoritative for their domains.

---

## 19. Non-goals

This specification does not:

- make a public blockchain private;
- claim that a hash anonymizes sensitive data;
- require zero-knowledge proofs for the first pilot;
- replace bank secrecy, data-protection, records-management, or supervisory analysis;
- let a beneficiary inspect the underlying gold;
- make a credential a bank guarantee or legal instrument;
- hide information from a party legally entitled to receive it;
- promise unlinkability where stable identifiers or operational context still permit correlation;
- override custody or legal records through a privacy proof.

---

## 20. Design precedents and references

- Digital Asset, Canton ledger privacy model: https://docs.digitalasset.com/overview/3.5/explanations/ledger-model/ledger-privacy.html
- W3C Verifiable Credentials Data Model 2.0: https://www.w3.org/TR/vc-data-model-2.0/
- W3C Data Integrity BBS Cryptosuites: https://www.w3.org/TR/vc-di-bbs/
- W3C Securing Verifiable Credentials using JOSE and COSE: https://www.w3.org/TR/vc-jose-cose/
- OpenID for Verifiable Presentations 1.0: https://openid.net/specs/openid-4-verifiable-presentations-1_0.html
- NIST SP 800-207, Zero Trust Architecture: https://csrc.nist.gov/pubs/sp/800/207/final
- Stellar smart-contract authorization: https://developers.stellar.org/docs/build/guides/auth/contract-authorization
- Stellar contract events: https://developers.stellar.org/docs/build/guides/events
- DFNS policies: https://docs.dfns.co/core-concepts/policies
- DFNS policy approvals: https://docs.dfns.co/api-reference/policy-approvals

These references are design precedents. They do not imply partnership, endorsement, legal equivalence, or production compatibility.

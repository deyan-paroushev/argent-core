# Confidential control and public integrity

**How Argent uses Soroban without publishing bar identities, facility amounts, counterparties, or commercial activity.**

**Status:** canonical target production architecture; not implemented by the current transparent reference contracts  
**Applies to:** custody and bar identity, bank facilities, obligation capacity, institutional signing, evidence services, relays, Soroban state and events, and production deployment review  
**Primary rule:** the institutional operating book remains private; Soroban proves the integrity and ordering of governed state without becoming that book

---

## 1. Purpose

Argent coordinates facts that a bank, custodian, company, beneficiary, auditor, and supervisor may each need to verify, but that must not be broadcast to the public:

- exact bar and lot identity;
- customer, owner, custodian, bank, and beneficiary relationships;
- reserve quantity, value, haircut, and unused capacity;
- guarantee, documentary-credit, and other obligation amounts;
- claims, defaults, reimbursements, and enforcement preparation;
- legal, custody, KYC, sanctions, and commercial evidence.

The production architecture therefore separates two planes:

1. **Confidential operating plane.** Bank, custodian, title, evidence, and Argent services hold the complete role-authorized state needed to operate the facility.
2. **Public integrity plane.** Soroban anchors authorized state versions and refuses invalid authorization, sequence, replay, and rollback. It proves that governed parties committed to an ordered state transition, not the underlying commercial contents.

> Argent does not place the bank's facility book, custody register, bar list, customer exposure, or obligation graph on a public ledger.

The chain is not a document archive, customer register, bar registry, facility database, or public transaction monitor.

---

## 2. Why the separation is necessary

There are three distinct confidentiality problems. Solving one does not solve the others.

### 2.1 Identity leakage

A raw or predictably salted hash of a serial number can be guessed. A random evidence commitment hides the preimage but does not create deterministic uniqueness: the same record combined with another salt produces another commitment.

### 2.2 Amount leakage

Exact quantity, value, haircut, limit, utilization, guarantee amount, price, and capacity delta reveal commercial position and bank risk treatment. Hashing a predictable number is not confidentiality.

### 2.3 Graph and timing leakage

Even where payloads are encrypted or committed, public accounts, stable identifiers, action-specific events, transaction timing, and repeated counterparties reveal who deals with whom and when. A competitor may infer a tender win, shipment, facility stress, claim, or customer relationship without learning a single plaintext amount.

The production profile must address contents, identifiers, graph, timing, and lifecycle correlation together.

---

## 3. Three implementation profiles

### 3.1 Transparent reference profile - implemented today

The current Soroban contracts are a transparent, event-sourced secured-credit reference implementation. They demonstrate:

- role-specific authorization;
- lot-key collision refusal;
- eligibility and borrowing-base controls;
- utilization and repayment behavior;
- dual-control release;
- default, cure, and enforcement recording;
- typed events that can reconstruct the reference projection.

They also expose participant addresses, stable object identifiers, quantities, limits, prices, balances, state transitions, and action-specific event metadata. They are suitable for synthetic test data, technical verification, and public demonstration. They are not the production confidentiality profile.

### 3.2 Confidential production profile - target funded extension

The production baseline uses:

- a private, role-authorized institutional state engine;
- custodian-controlled canonical identity and nullifier derivation;
- encrypted evidence storage and purpose-bound disclosure;
- signed private transition envelopes;
- state-root or batch commitments;
- a common relay, fixed submission cadence, padding policy, and uniform public event;
- Soroban enforcement of signer authority, sequence, replay protection, and root continuity;
- leakage testing before any real institution or customer data is used.

This profile does not require zero-knowledge proofs.

### 3.3 Cryptographic privacy profile - research extension

Possible later work includes:

- confidential amount arithmetic;
- range proofs;
- private set-transition proofs;
- zero-knowledge proof of non-overallocation;
- unlinkable presentations.

No such feature is a production claim until it is implemented, benchmarked against current Soroban limits, independently reviewed, and supported by an operational key and verifier model.

---

## 4. Data placement

| Data | Confidential operating plane | Public integrity plane |
|---|---:|---:|
| Bar serial, refinery, year, weight, assay | Yes | Never plaintext |
| Evidence random salt and source document | Yes | Never |
| Custodian nullifier key | HSM or equivalent only | Never |
| Deterministic bar nullifier | Yes by default | Optional only in the explicit public-nullifier profile |
| Owner, customer, bank, custodian, beneficiary | Yes | No direct identifiers in the production anchor |
| Facility, position, reservation, obligation IDs | Yes | Opaque batch scope only; no stable customer-scoped ID |
| Quantity, price, haircut, limit, utilization, capacity | Yes | No exact values |
| Action type and lifecycle state | Yes | One uniform epoch or batch event |
| Policy document | Restricted evidence store | Policy-version commitment where safe |
| Evidence package | Encrypted, role-bound store | Aggregate evidence or state-root commitment |
| Previous and next state | Complete private projection | Previous and next root only |
| Institutional approvals | Complete approval record | Aggregate authorization evidence or transaction authorization |

A public commitment is included only where it supports a defined integrity property and has passed a metadata and dictionary-attack review.

---

## 5. Evidence commitment and uniqueness nullifier

Evidence binding and duplicate-allocation control are different requirements and use different values.

### 5.1 Evidence commitment

```text
C = SHA-256(
    "ARGENT:BAR_EVIDENCE:v1"
    || canonical_evidence_record
    || random_salt
)
```

The evidence commitment binds a private canonical evidence record without exposing it. The random salt must be generated with an approved cryptographic random source, retained with the private evidence, and never reused as a general secret.

Changing the evidence or salt changes C. Therefore C must not be used as the uniqueness key.

### 5.2 Uniqueness nullifier

```text
N = HMAC-SHA-256(
    K_nullifier_domain,
    "ARGENT:BAR_NULLIFIER:v1"
    || custodian_namespace
    || canonical_bar_identity
)
```

The nullifier is deterministic within one governed uniqueness domain. An already-active N is refused by the domain's nullifier registry.

The nullifier input must not contain the bank, facility, owner, obligation, or transaction. Including any of those values would allow the same bar to receive a different nullifier in another facility and defeat domain-wide collision detection.

### 5.3 Canonical identity versus canonical evidence

The canonical bar identity contains only stable physical identity fields approved by the custodian profile. A profile may require, for example:

```text
profile_version
refinery_or_brand_identifier
bar_serial
year_or_production_batch_where_required
bar_form_or_standard
additional_disambiguator_required_by_the_custodian
```

Owner, vault account, facility, location, valuation, assay date, evidence version, and obligation are mutable facts and do not belong in the stable identity unless the asset profile expressly establishes that they are part of the physical identifier.

The canonical evidence record may contain the stable identity plus current custody, ownership, certificate, and control evidence. It is versioned independently.

Canonicalization must define field order, encoding, normalization, case, whitespace, character set, missing-field treatment, units, and profile version. Two approved implementations given the same source record must produce identical canonical bytes and N.

### 5.4 Authorized production

Only the approved custodian nullifier service may produce or attest N for its namespace. The service must:

- authenticate the custodian role;
- derive N inside an HSM or equivalent controlled boundary;
- refuse an already-active N;
- bind the result to the evidence commitment, scope, policy version, and validity period;
- create an auditable signed response;
- expose no general HMAC oracle;
- support rate limits, monitoring, recovery, and institutional separation of duties.

The current reference contract accepts a caller-supplied 32-byte uniqueness value and rejects an identical active value. It does not derive N, verify canonical bar identity, or prevent a participant from supplying a different value for the same bar. Those are target production controls.

### 5.5 Domain ownership in the pilot

The uniqueness domain is **owned and keyed by the custodian**, one namespace per
custodian. Concretely:

- the custodian owns the namespace and the nullifier key;
- derivation happens inside the custodian's HSM or equivalent boundary;
- the domain spans every participating Argent facility handled within that
  custodian namespace;
- the domain must not vary by bank, customer, facility, or obligation, for the
  reason given in section 5.2;
- Argent may supply or operate the derivation software as a governed processor,
  under the custodian's controls;
- **Argent does not hold the key and does not become an asset registry.**

This follows from the product boundary. The custodian is already authoritative
for physical identity and control; the accreditation and attestation chain
already exists and Argent consumes it rather than issuing it. An
Argent-operated shared registry would make Argent a new source of physical
truth, concentrate liability in the party least able to verify a bar, and
contradict the position that Argent accredits nobody.

Cross-custodian uniqueness is explicitly **unsolved** in the pilot. Two
custodians running separate namespaces cannot detect that the same physical bar
is active in both. A federated or consortium registry is possible later work
and is not claimed here.

### 5.6 Scope of the claim

The permitted claim is:

> Prevents duplicate active allocation of the same canonical asset identity within the governed Argent/custodian uniqueness domain.

It does not prove:

- global uniqueness across custodians or systems;
- absence of an unregistered paper pledge;
- legal title or perfection;
- physical existence without custodian evidence;
- absence of fraud by a compromised authoritative custodian;
- uniqueness after a key or canonicalization change that was not migrated correctly.

---

## 6. Nullifier placement profiles

### 6.1 Public-nullifier profile

N may be submitted to Soroban as an opaque active-set key. Soroban can then reject a repeated active N directly.

This hides the serial from an observer without the custodian key, but it still reveals:

- the existence and number of registered units;
- a stable identifier;
- activation, release, and reuse timing;
- correlation where N appears in more than one event or call.

This profile is not the default for bank production. It may be used for transparent demonstrations, non-sensitive registries, or deployments whose participants explicitly accept the metadata leakage.

### 6.2 Confidential-nullifier production profile

N remains in the governed private control plane. The private engine refuses duplicate active allocation and includes the resulting nullifier-set root in the signed state transition. Soroban anchors the authorized previous and next roots.

In this baseline, Soroban independently proves authorization, ordering, replay refusal, and non-equivocation. The custodian-controlled private engine proves the duplicate-allocation decision to authorized reviewers through its signed transition record and inclusion evidence.

Soroban does not independently inspect the hidden set. Claiming independent on-chain proof of a correct private set transition would require an additional proof system and belongs to the research profile.

---

## 7. Private transition envelope

Authorized approvers require a complete and human-readable private instruction. A target envelope should include:

```text
transition_version
tenant_and_facility_scope
private_action_type
private_object_references
previous_private_state_root
next_private_state_root
previous_nullifier_set_root
next_nullifier_set_root
evidence_root
policy_version
authorization_policy
nonce
created_at
expires_at
required_roles
institutional_signatures
```

Amounts, parties, products, beneficiaries, bar identity, evidence, and reason codes remain inside the encrypted institutional approval and evidence systems.

The public submission must be derived from the approved envelope through a deterministic minimization step. Approvers must see both the complete private instruction and the exact minimized public payload before signature.

---

## 8. Public batch anchor

A target public anchor may contain:

```text
anchor_version
epoch
previous_state_root
new_state_root
policy_version_commitment
batch_commitment
authorization_commitment
replay_token
```

The public event should use one stable schema and action name, such as `EpochCommitted`, regardless of whether the private batch contained a registration, reservation, issuance, amendment, claim, reimbursement, release, or enforcement act.

The public anchor must not contain:

- customer- or facility-scoped stable identifiers;
- action-specific topics;
- exact item counts where padding policy is intended to hide activity;
- exact amounts or capacity deltas;
- participant addresses other than the common relay or approved aggregate authority;
- evidence hashes whose preimages are predictable or commercially identifying.

---

## 9. Relay, cadence, and batching

Encryption does not hide the sender, receiver, timing, or frequency of transactions. The production relay policy should therefore define:

- a common submission address or governed relay set;
- fixed or bounded submission windows;
- minimum and maximum batch sizes;
- padding or cover-record behavior;
- behavior during quiet periods;
- failure, retry, and replay rules;
- transaction-fee funding;
- relay key rotation and compromise response;
- monitoring that does not recreate a public customer graph.

Batching reduces but does not eliminate metadata leakage. The deployment assessment must state what an observer can still infer.

---

## 10. Key lifecycle and domain continuity

Random evidence salts, nullifier keys, transaction-signing keys, evidence-signing keys, encryption keys, and relay keys are different key classes. They must not be reused.

For nullifier continuity:

- prefer rotating access controls and key-wrapping keys while preserving the stable domain derivation secret inside the HSM;
- version the canonicalization and derivation profile;
- retain the ability to recognize nullifiers produced under every active or historical domain version;
- prohibit a key rotation from creating a second active identity for an existing bar;
- use a signed, audited migration process if the derivation key must change;
- preserve active locks throughout migration;
- define disaster recovery, quorum, and revocation procedures.

A key version must not be used as an uncontrolled facility-specific salt. Domain fragmentation weakens duplicate detection.

---

## 11. What Soroban contributes

In the confidential production profile, Soroban provides:

- a neutral integrity plane not operated solely by the bank, custodian, or customer;
- contract-level authorization for permitted anchor writers;
- deterministic sequence and root-continuity checks;
- replay and rollback refusal;
- an immutable timestamped commitment to the approved state version;
- a common verification point for later evidence disclosure;
- a portable public reference implementation for institutional workflow anchoring.

Soroban does not provide:

- confidentiality merely because data is hashed;
- legal title, custody, valuation, underwriting, or bank issuance;
- truth about an unobserved external system;
- private computation over hidden values without an additional proof or trusted execution model;
- global duplicate-pledge detection outside the governed domain.

The bank, custodian, title system, and evidence providers remain authoritative for their own facts. Soroban makes their governed commitments ordered and non-equivocating.

---

## 12. Production deployment gates

No real institution, customer, facility, bar, beneficiary, or obligation data may enter the public profile until all of the following pass:

- field-by-field public/private data map;
- contract argument, storage, event, authorization-entry, return-value, diagnostic-log, and error review;
- alternative-salt duplicate-registration test;
- canonicalization interoperability test using published test vectors;
- low-entropy dictionary and preimage tests;
- stable-identifier and lifecycle-correlation analysis;
- account-graph and timing analysis;
- batch-size, cadence, and quiet-period leakage analysis;
- nullifier-key generation, HSM, rotation, recovery, and compromise drill;
- relay compromise and replay drill;
- root continuity, skipped epoch, duplicate epoch, and rollback tests;
- private transition/public payload equivalence test;
- role, tenant, and purpose-bound evidence-access tests;
- independent security and privacy review;
- documented `does_prove` and `does_not_prove` statements.

The transparent reference contracts must use synthetic data only unless a deployment explicitly accepts their public fields and event model.

### 12.1 Synthetic-data-only is a governance invariant, not a contract control

The current contracts cannot enforce this restriction and must not be described
as if they could. `credit_ledger` receives a 32-byte uniqueness value and an
evidence commitment as opaque bytes. It can reject a zero value and refuse an
identical active value. It cannot inspect a preimage, distinguish a real bar
record from a test fixture, or determine whether a submitted commitment
describes a live customer position.

A `synthetic = true` flag, an administrator declaration, or a naming convention
would create the appearance of protection while enforcing nothing. No such
control is specified.

The restriction is therefore:

> **Synthetic-data-only is a deployment and governance invariant.** It is
> enforced by operational control over who may write to a deployed transparent
> reference instance, not by contract logic. Any party with write authority on
> such an instance can publish real data, and the contract will accept it.

Operators are responsible for:

- restricting write authority on transparent reference deployments;
- treating any real-data write as a confidentiality incident;
- keeping demonstration instances separate from any instance connected to an
  institutional signer.

### 12.2 Production does not retrofit the transparent contracts

The confidential production profile is delivered by the separate minimized
batch-anchor contract described in section 7, not by adding confidentiality
options to `credit_ledger`, `settlement_vault`, or `rewards_ledger`.

Those contracts are a tested transparent reference implementation. Their storage
model, stable identifiers, exact values, and reconstructable typed event stream
are deliberate properties of that profile. Retrofitting them would either break
the reference behaviour that reviewers verify against, or produce a contract
that claims confidentiality while retaining publicly inspectable fields.

A production deployment runs the batch-anchor contract. The transparent
reference contracts remain available, unchanged, for technical verification and
public demonstration with synthetic data.

---

## 13. Funded open-source deliverables

The confidential production extension can be bounded as reusable components:

1. canonical bar-identity and evidence-commitment library with test vectors;
2. custodian nullifier service reference implementation and key-governance profile;
3. private transition-envelope schema and institutional signing adapter;
4. deterministic state and nullifier-set root implementation;
5. minimized batch-anchor Soroban contract;
6. common relay with cadence and padding policy;
7. inclusion, disclosure, and audit receipt tools;
8. automated public-surface leakage scanner;
9. threat model, deployment runbook, and independent review evidence.

Gold-backed bank obligations are the first reference implementation. The anchoring and leakage-control components should be reusable for other physical or off-chain collateral systems involving an owner, custodian, bank, and governed obligation.

---

## 14. Governing relationship to other documents

- [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md) governs data classification, role visibility, and disclosure products.
- This document governs production public/private state placement, commitment and nullifier semantics, transition minimization, batching, and public metadata controls.
- [`protocol.md`](protocol.md) describes current transparent contract behavior and the wider protocol model.
- [`threat-model-and-security-boundaries.md`](threat-model-and-security-boundaries.md) governs threat ownership and residual risk.
- [`deployment-and-runbook.md`](deployment-and-runbook.md) governs deployment evidence and operational gates.
- Contract source and tests remain authoritative for what is implemented today.

No document may convert this target production profile into a claim that the current contracts already implement confidential nullifiers, private capacity arithmetic, batch anchoring, or zero-knowledge proofs.

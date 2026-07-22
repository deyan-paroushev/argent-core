# Security and privacy

## Security objective

Argent must let authorized institutions prove that collateral capacity changed in an agreed sequence without publishing the commercial book.

The production security claim is deliberately narrow:

> Authorized parties can reconcile a private obligation state against a shared integrity history, while exact asset identity, participants, amounts, beneficiaries, documents, and relationship metadata remain outside the public ledger.

## Current versus target profile

| Profile | Purpose | Data rule |
|---|---|---|
| Transparent reference — built | Prove contract authorization and lifecycle primitives with synthetic testnet data. | Public state and events expose details; never use live commercial data. |
| Confidential obligation profile — funded next | Operate a production-shaped bank obligation facility. | Private operating state with minimized, padded public anchors. |
| Advanced cryptographic privacy — research | Explore confidential arithmetic or stronger unlinkability. | No delivery claim until implemented, benchmarked, and independently reviewed. |

Production does not retrofit real data into the transparent contracts.

## Data placement

### Private operating plane

The bank, custodian, evidence services, and role-authorized Argent components retain:

- bar serials, refiner, year, weight, assay, certificates, and location;
- company, bank, custodian, beneficiary, and user identities;
- ownership, custody, pledge, lien-search, KYC, sanctions, and legal evidence;
- quantity, price, haircut, limit, sublimit, utilization, and margin state;
- obligation type, amount, purpose, beneficiary, expiry, claim, and reimbursement data;
- institutional approvals and signing records.

### Public Soroban integrity plane

The target public surface contains only what is necessary to enforce and reconcile:

- a protocol and policy version;
- an opaque epoch or batch scope;
- previous and next state roots;
- replay protection and sequence;
- aggregate authorization evidence;
- one uniform event shape.

Stable customer IDs, facility IDs, action-specific event names, exact values, and direct institutional addresses must not create a publicly readable relationship graph.

## Evidence binding and uniqueness

A randomly salted commitment and a deterministic uniqueness value solve different problems.

### Evidence commitment

```text
C = SHA-256(
    "ARGENT:BAR_EVIDENCE:v1"
    || canonical_evidence_record
    || random_salt
)
```

`C` binds the private evidence without exposing it. Changing the evidence or salt changes the commitment, so it cannot provide deterministic uniqueness.

### Custodian uniqueness nullifier

```text
N = HMAC-SHA-256(
    K_custodian_domain,
    "ARGENT:BAR_NULLIFIER:v1"
    || custodian_namespace
    || canonical_bar_identity
)
```

`N` is deterministic within the governed custodian namespace. The active nullifier registry refuses a second active allocation of the same `N`.

The identity input must not include owner, bank, facility, obligation, location, or valuation. Those facts can change and would permit the same bar to receive a new nullifier.

The key remains in the custodian's HSM or equivalent controlled boundary. Argent may supply governed software but does not own the domain key or become the source of physical truth.

Permitted claim:

> Prevents duplicate active allocation of the same canonical asset identity within the governed Argent/custodian domain.

No global uniqueness is claimed across custodians or systems Argent cannot observe.

## Canonicalization requirements

The asset profile must define:

- stable identity fields;
- field order and binary encoding;
- normalization, case, whitespace, and character-set rules;
- missing-field and disambiguation treatment;
- versioning and migration;
- deterministic test vectors shared by approved implementations.

Two approved implementations given the same source record must produce identical canonical bytes and `N`.

## Metadata controls

Encryption and hashing do not hide transaction timing or relationships. The target anchor design therefore requires:

- a common relay rather than participant-specific public submitters;
- fixed or bounded submission cadence;
- uniform transaction and event shape;
- batch padding and minimum cohort rules;
- no public action type or customer-scoped identifier;
- delayed or withheld anchors where the anonymity set is insufficient;
- outside-observer leakage testing using realistic traffic.

Batching is a confidentiality control, not merely a cost optimization.

## Authorization and key controls

- Bank, custodian, company, operator, and verifier roles remain distinct.
- Institutional signing policy governs who may approve; the protocol governs whether the transition is valid.
- Nullifier, encryption, relay, and signing keys have separate purposes and owners.
- Key generation, storage, rotation, recovery, revocation, and audit are documented and rehearsed.
- No operator or software administrator can impersonate the bank or custodian.
- Emergency pause cannot silently rewrite, delete, or bypass committed history.

## Principal threats

| Threat | Required control |
|---|---|
| Same bar submitted with another salt | Deterministic custodian nullifier, not commitment-only uniqueness. |
| Different representation of the same bar | Versioned canonicalization and shared test vectors. |
| Excess or concurrent allocation | Atomic private capacity transition plus reservation idempotency. |
| Forged custody status | Custodian-signed evidence and role policy. |
| Early or unilateral release | Bank authorization followed by custodian confirmation. |
| Public amount or relationship inference | Private values, uniform batching, relay, padding, and leakage tests. |
| Replay or rollback | Monotonic sequence, previous-root binding, idempotency, and public anchor continuity. |
| Key compromise | HSM isolation, separation of duties, rotation, incident response, and domain recovery. |
| Private database divergence | Signed transitions, deterministic projection, reconciliation, and exception queues. |
| Availability failure | Recovery objectives, durable queues, manual safe state, and no fail-open allocation. |

## Production security gates

Before live data or collateral:

1. Threat model and data classification approved by every participating institution.
2. Independent review of contracts, private state engine, canonicalization, nullifier service, relay, and signing integration.
3. Synthetic-data guard enforced by deployment policy for transparent contracts.
4. No critical authorization, allocation, privacy, replay, or key-management findings open.
5. Red-team leakage exercise cannot reliably infer protected relationships or lifecycle events.
6. Reconciliation, backup, recovery, key rotation, and institutional offboarding rehearsed.
7. Legal and operational evidence accepted under [LEGAL_PILOT_CHECKLIST.md](LEGAL_PILOT_CHECKLIST.md).

Detailed prior designs remain in [docs/reference/confidential-control-and-public-integrity.md](docs/reference/confidential-control-and-public-integrity.md) and [docs/reference/threat-model-and-security-boundaries.md](docs/reference/threat-model-and-security-boundaries.md).

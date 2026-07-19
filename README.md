# Argent Core

**Gold-backed obligation infrastructure on Stellar.**

Argent is a Soroban-first control protocol for turning identifiable, customer-owned physical reserves into reusable, purpose-bound capacity for bank-issued obligations. The first reference asset is allocated gold. The bars remain in professional custody, the company retains title subject to the pledge, the bank remains the product issuer, and unused facility capacity is not intended to be drawable as unrestricted cash.

> **One reserve. Many obligations. One authoritative capacity state.**
>
> Use gold to secure the promises. Keep cash for operations and final settlement.

The mature product direction is a **corporate reserve obligation facility**. One controlled bullion pool can support approved guarantees, documentary credits, supplier undertakings, regulatory security, treasury exposures, and other beneficiary-specific bank instruments. Argent coordinates reserve identity, eligibility, capacity allocation, authorization, reimbursement, release, default, and enforcement evidence without tokenizing the metal or replacing the bank, custodian, legal documents, or trade-finance system.

Argent is designed to sit above authoritative gold-market infrastructure rather than duplicate it. Provenance, ownership, custody, physical backing, and digital-gold product records may come from a custodian, LBMA Gold Bar Integrity, a future shared gold platform, or another bank-approved authority. Argent remains authoritative for facility encumbrance, capacity reservation, bank-obligation allocation, and release state. See [`docs/shared-gold-infrastructure-and-argent.md`](docs/shared-gold-infrastructure-and-argent.md).

The code in this repository is the tested **secured-credit reference branch** of that broader protocol. It proves the difficult shared primitives through real Soroban state transitions: instrument eligibility, lot identity, exclusive pledge, borrowing-base computation, utilization, atomic repayment, dual-control release, default, cure, and enforcement recording. The obligation profile generalizes those primitives; it does not discard or misrepresent the implementation that exists today.

Start with:

- [`docs/reserve-obligation-infrastructure.md`](docs/reserve-obligation-infrastructure.md) - the current product thesis and institutional value proposition.
- [`docs/obligation-facility-profile.md`](docs/obligation-facility-profile.md) - the target non-cash-drawable facility model and its relationship to the current contracts.
- [`docs/capacity-reservation-and-deliverability.md`](docs/capacity-reservation-and-deliverability.md) - how eligible reserve value becomes reserved, issuable, and operationally deliverable bank capacity without double allocation.
- [`docs/selective-disclosure-and-institutional-privacy.md`](docs/selective-disclosure-and-institutional-privacy.md) - the minimum-disclosure model for banks, custodians, beneficiaries, auditors, and supervisors.
- [`docs/shared-gold-infrastructure-and-argent.md`](docs/shared-gold-infrastructure-and-argent.md) - how Argent complements Gold Bar Integrity, Wholesale Digital Gold, Pooled Gold Interests, and the proposed Gold as a Service platform without duplicating gold ownership or issuance.
- [`docs/DOCUMENT_STATUS_MATRIX.md`](docs/DOCUMENT_STATUS_MATRIX.md) - which documents describe shipped code, product direction, or later extensions.
- [`docs/REVIEWER_QUICKSTART.md`](docs/REVIEWER_QUICKSTART.md) - the five-minute verification path for the implementation.

---

## The product boundary

### What Argent is

A shared, role-signed control layer for physical reserves that remain under professional custody. It gives the participating institutions one ordered state for:

- asset and lot identity;
- custody and immobilization;
- eligibility and valuation treatment;
- exclusive pledge, available capacity, and purpose-bound reservation;
- deliverability checks and allocation to a bank-approved obligation;
- reimbursement, release, default, and enforcement evidence.

### What Argent is not

- not tokenized gold;
- not a bank or credit originator;
- not a custodian;
- not a trade-finance platform;
- not a private currency or freely transferable capacity token;
- not an on-chain legal-enforcement engine;
- not a replacement for bank underwriting, accounting, KYC, sanctions controls, legal opinions, or custody records.

The bank issues the obligation. The custodian controls the reserve. The company remains the owner subject to the security interest. Argent governs and proves the shared capacity state.

---

## Two profiles, one protocol core

### Target product profile - reserve-backed obligations

```text
Allocated reserve
    -> bank-controlled pledge
    -> approved master capacity
    -> purpose-bound obligation reservation
    -> bank guarantee / documentary credit / undertaking / treasury exposure
    -> expiry, reimbursement, claim, or enforcement
    -> capacity restored or collateral realized
```

The target facility rejects an unrestricted customer cash draw. Every utilization is tied to an approved product, purpose, amount, beneficiary, expiry or maturity, and evidence package. Available capacity is not treated as automatically issuable: applicant, beneficiary, product, evidence, institutional approval, and operational-deliverability conditions must also pass before the bank product system may issue.

### Implemented reference profile - secured credit

```text
Allocated reserve
    -> pledge
    -> credit line
    -> utilization
    -> repayment
    -> bank authorization
    -> custodian-confirmed release
```

This branch is valuable because it already proves the shared collateral engine and the adverse paths that any obligation facility also needs. The contract names remain credit-oriented until the next domain-model extension; the repository states that boundary explicitly rather than relabeling unbuilt behavior as complete.

---

## Verify the implementation in five minutes

```bash
git clone https://github.com/deyan-paroushev/argent-core
cd argent-core
cargo test --manifest-path contracts/Cargo.toml     # 224 tests
python3 scripts/check_docs.py                       # docs match the contract
```

### Four properties worth checking first

| Property | Current proof | Failure behavior |
|---|---|---|
| **The same bars cannot support two active pledges.** | Lot-level `uniqueness_hash` | A second active pledge is rejected. |
| **A facility cannot open without risk headroom.** | `open_credit_line` enforces `ltv_bps < maintenance_bps` | An unsafe line is rejected at creation. |
| **Only the bound settlement vault can reduce exposure.** | Credit ledger authorization and vault binding | An unapproved caller cannot apply repayment. |
| **Release requires two distinct institutional acts.** | Bank authorization followed by custodian confirmation | The custodian cannot release before the bank authorizes it. |

These are protocol-level controls, not operating procedures that can be skipped under pressure.

---

## Why Stellar

Stellar/Soroban is not used as a document archive. Three features are structurally important:

1. **Role-specific authorization.** `require_auth` binds each state transition to the party that performs it. The bank, custodian, owner, verifier, and operator do not sign for one another.
2. **Atomic value-state transitions.** Where regulated settlement value moves, the settlement transfer and exposure update can occur together or not at all.
3. **Shared, replayable evidence.** The parties and an auditor can verify one ordered event history rather than reconciling several private records after the fact.

The intended institutional signing layer is DFNS. DFNS policies and approvals govern whether an institutional role may sign; Soroban enforces whether the resulting state transition is valid. The reusable integration contribution is a Soroban-aware DFNS authorization adapter, approval-to-transaction reconciliation, and an institutional role blueprint that can support other Stellar applications requiring governed multi-party acts.

---

## Contracts

| Contract | Implemented role |
|---|---|
| `credit_ledger` | Current collateral and secured-exposure reference branch: instruments, positions, pledge, borrowing base, utilization, margin, release, default, cure, and enforcement. |
| `settlement_vault` | Atomic settlement-asset repayment bound to exposure reduction. |
| `rewards_ledger` | Optional sponsor-funded rewards overlay; not part of the reserve-obligation core. |

The next protocol extension generalizes the facility and exposure objects rather than replacing the collateral controls. See [`docs/obligation-facility-profile.md`](docs/obligation-facility-profile.md).

---

## Documentation

### Canonical direction

| Document | Purpose |
|---|---|
| [`reserve-obligation-infrastructure.md`](docs/reserve-obligation-infrastructure.md) | Product thesis, stakeholder value, product boundary, and market category. |
| [`obligation-facility-profile.md`](docs/obligation-facility-profile.md) | Target facility objects, states, invariants, and mapping to current contracts. |
| [`capacity-reservation-and-deliverability.md`](docs/capacity-reservation-and-deliverability.md) | Reservation, concurrency, preflight, issuability, external finality, and reconciliation. |
| [`selective-disclosure-and-institutional-privacy.md`](docs/selective-disclosure-and-institutional-privacy.md) | Data classification, role-specific visibility, evidence privacy, and selective-disclosure path. |
| [`shared-gold-infrastructure-and-argent.md`](docs/shared-gold-infrastructure-and-argent.md) | Boundary and adapter model between authoritative gold infrastructure and Argent's bank-obligation state. |
| [`argent-architecture.md`](docs/argent-architecture.md) | System architecture, roles, trust boundaries, and implementation relationship. |
| [`protocol.md`](docs/protocol.md) | Open protocol specification and implemented reference profile. |

### Verification and operations

| Document | Purpose |
|---|---|
| [`REVIEWER_QUICKSTART.md`](docs/REVIEWER_QUICKSTART.md) | Verify the current contracts and testnet evidence. |
| [`TEST_SURFACE_MATRIX.md`](docs/TEST_SURFACE_MATRIX.md) | What the 224 tests cover. |
| [`argent-dfns-signing-sequence.md`](docs/argent-dfns-signing-sequence.md) | Institutional signing, policies, approvals, and role separation. |
| [`deployment-and-runbook.md`](docs/deployment-and-runbook.md) | Deployment and operating procedures for the current reference branch. |
| [`evidence-pack-index.md`](docs/evidence-pack-index.md) | Evidence and review artifacts. |

### Domain, risk, trade finance, and integration

See [`docs/README.md`](docs/README.md) and [`docs/DOCUMENT_STATUS_MATRIX.md`](docs/DOCUMENT_STATUS_MATRIX.md) for the complete map.

---

## What is live, what is next

### Live and testable

- three Soroban contracts;
- 224 passing contract tests;
- instrument eligibility and lot uniqueness;
- tri-party pledge and secured-credit lifecycle;
- canonical collateral and governance events;
- atomic repayment;
- controlled release, default, cure, and enforcement evidence;
- Stellar testnet deployment and live demonstrator.

### Next institutional layer

- DFNS-governed organization wallets and approval policies;
- reconciliation between DFNS approvals, Soroban authorization entries, transactions, and evidence;
- mainnet reference deployment;
- general master facility and typed bank obligations;
- contingent versus crystallized exposure;
- product sublimits and available-versus-issuable capacity;
- provisional and committed reservations, expiry, idempotency, and definitive callbacks;
- no-unrestricted-cash-draw invariant;
- role-specific projections, encrypted evidence access, and selective-disclosure controls;
- beneficiary, trade-document, bank-product, and settlement adapters.

### Not yet claimed

- no production bank or custodian pilot is signed;
- no guarantee, documentary credit, or treasury instrument is issued by Argent;
- no legal security interest is created on-chain;
- no asset is tokenized or transferred by the protocol;
- no public capacity token or private currency is proposed.

---

## Status

Stellar testnet. **224 tests.** Full implemented reference lifecycle exercised end to end in the [live demonstrator](https://argent-production-4a3f.up.railway.app).

Apache-2.0 - see [`LICENSE`](LICENSE).

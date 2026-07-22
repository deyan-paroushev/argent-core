# Implementation status

**Status date:** 22 July 2026  
**Rule:** implementation evidence and product direction are reported separately.

## Status vocabulary

| Label | Meaning |
|---|---|
| **Built** | Present in this repository and testable now. |
| **Reference** | Implemented to prove a reusable primitive, but not the target production product. |
| **Funded next** | Defined target work; not represented as deployed. |
| **Research** | Possible later work with no delivery claim. |

## What is built

The repository contains a transparent secured-credit reference with three Soroban contracts and 224 tests.

| Surface | Status | What it proves |
|---|---|---|
| `credit_ledger` | Built reference · 162 tests | Role authorization, instrument and lot evidence, identical supplied-key collision refusal, pledge, borrowing base, utilization, margin, release, default, cure, and enforcement state. |
| `settlement_vault` | Built reference · 17 tests | Settlement-asset repayment and exposure reduction can be bound atomically. |
| `rewards_ledger` | Historical reference · 45 tests | A separated sponsor-funded rewards experiment. It is not part of the obligation product. |
| Testnet demonstrator | Built reference | Real public testnet transitions using synthetic data. |

The 179 tests in `credit_ledger` and `settlement_vault` are the relevant collateral-control and settlement evidence for the current product direction.

## What the current contracts do not prove

- They do not implement a typed customs or bank-guarantee obligation.
- They do not enforce a no-unrestricted-cash-draw facility profile.
- They expose addresses, identifiers, quantities, values, limits, balances, and lifecycle events publicly.
- They accept a caller-supplied uniqueness value; they do not derive a custodian-controlled canonical bar nullifier.
- They do not hide relationship, graph, or timing metadata.
- They do not integrate production institutional signing, bank product systems, custodian systems, or a legal security register.
- They do not create, perfect, prioritize, or enforce a legal security interest.

The transparent contracts are therefore suitable for public technical evidence and synthetic data—not live commercial records.

## Product-to-code map

| Product requirement | Current evidence | Required next implementation |
|---|---|---|
| Role-separated acts | `require_auth` across lifecycle transitions | Institutional wallet policy, approvals, and signing reconciliation. |
| Exclusive use of a supplied lot key | Identical active `uniqueness_hash` is refused | Canonicalization plus custodian-keyed nullifier production inside a governed domain. |
| Capacity control | Borrowing-base and available-limit controls | Master facility, product sublimits, reservations, and typed obligations. |
| Controlled release | Bank authorization then custodian confirmation | Obligation discharge, substitution, partial release, and external-system reconciliation. |
| Settlement integrity | Atomic repayment reference | Product-specific reimbursement and claim settlement adapters where required. |
| Shared history | Transparent state and typed events | Confidential operating state plus minimized, padded batch anchors. |

## Public roadmap: three gates

### Gate 1 — one real design

**Scope:** one bank, one custodian, one company, one obligation.

Required exit evidence:

- chosen obligation and beneficiary acceptance criteria;
- accepted allocated-gold profile and custodian control model;
- bank credit, collateral, product, operations, and compliance requirements;
- legal structure memorandum and conditions precedent;
- agreed private data model, authority matrix, and failure workflow;
- pilot success measures and named institutional owners.

No live collateral or production data is used at this gate.

### Gate 2 — production-shaped implementation

**Scope:** typed obligation contract, confidential anchoring, and institutional governance.

Required exit evidence:

- no-unrestricted-cash-draw invariant;
- typed obligation, reservation, discharge, claim, reimbursement, and release states;
- custodian canonicalization and nullifier service;
- private role-authorized state engine and encrypted evidence access;
- metadata-minimized Soroban batch anchor and relay policy;
- institutional signing policies and approval-to-transaction reconciliation;
- bank and custodian sandbox adapters;
- security review, leakage tests, recovery exercises, and legal sign-off for the controlled pilot.

### Gate 3 — controlled production pilot

**Scope:** limited-value production pilot and mainnet reference.

Required exit evidence:

- all legal conditions precedent satisfied;
- bank, custodian, and company operating approvals;
- mainnet contracts and published reference identifiers;
- monitored end-to-end issue, discharge, capacity-return, and exception paths;
- reconciliation and incident evidence accepted by all three institutions;
- an explicit decision to stop, revise, or expand.

Expansion to multiple banks, multiple custodians, multiple companies, cross-currency capacity, or additional collateral types begins only after Gate 3.

## Verification

```bash
cargo test --manifest-path contracts/Cargo.toml
python3 scripts/check_docs.py --verbose
```

The live demonstrator is available at <https://argent-production-4a3f.up.railway.app>.

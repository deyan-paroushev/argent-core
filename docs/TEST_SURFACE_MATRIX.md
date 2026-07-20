# Argent Core test surface

> **Scope:** This matrix covers the implemented secured-credit reference branch. It proves the collateral-control substrate on which the target obligation facility is designed to build; it does not claim that guarantees, documentary credits, claims, or product sublimits are implemented.

What the contract test suite covers, and the risk each surface addresses. Every
row maps to tests in this repository, runnable with `cargo test --workspace`.

| Surface | Primary risk | Coverage |
|---|---|---|
| Credit lifecycle | unsafe state transition | framework, position, pledge, line, drawdown, repayment, release, default, enforcement |
| Instrument eligibility | a position backed by an asset class the bank never admitted | register-once instrument registry, per-framework admission under a treatment (haircut, max advance, maintenance), refusal of unregistered or unadmitted instruments, LTV ceiling enforced at the instrument level, retired-instrument and revoked-signer guards |
| Credit authorization | wrong institution acts | approved-party checks, revocation, host-level wrong-signer tests |
| Credit accounting | double line, overpayment, reversal abuse | one line per pledge, duplicate-line refusal, overpayment refusal, exact reversal, property-style capacity bounds |
| Collateral key integrity | the same supplied key backs two positions | identical `uniqueness_hash` lock, duplicate-key refusal, reuse only after release; this does not prove that differently generated keys do not describe the same physical lot |
| Oracle input | stale or future valuation | stale price, future price, malformed threshold refusal |
| Evidence anchors | placeholder hashes | zero-hash refusal for critical legal, collateral, valuation and readiness records |
| Enforcement readiness | false certificate readiness | future validity, duplicate readiness, expiry and repopulation, valuation-source and waterfall evidence required for Ready |
| Settlement vault | token movement on failed repayment | insufficient-balance rollback, unapproved-vault rollback, duplicate-payment rollback, wrong-ledger binding refusal |
| Canonical events | missing or malformed event trail | CollateralEventV1 topic marker pinned, spec matches wire contract, lifecycle events fold back to state |
| Governance events | authority acts not provable or not separable from deal acts | GovernanceEventV1 in the contract spec, one-topic marker pinned, map data format, instrument-registration and admission advance a governance sequence independent of the framework sequence |
| Rewards evidence | placeholder reward proof | zero-hash refusal for campaign, spend, finality, claim, voucher, rejection, redemption |
| Rewards accounting | sponsor budget drift | rejected-terminal guard, budget-bucket invariant, property-style user cap and budget bounds |
| Snapshot regressions | silent terminal-state drift | release, enforcement, settlement and redemption snapshot-style assertions |
| TTL assumptions | records disappearing too early | medium-advance read tests for credit and reward records |

## Counts

| Crate | Tests |
|---|---|
| credit_ledger | 162 |
| rewards_ledger | 45 |
| settlement_vault | 17 |
| Total | 224 |

All passing on soroban-sdk 23.5.3. See `docs/REVIEWER_QUICKSTART.md` to run them.

## Production confidentiality test surface - target, not in the 224-test count

The current event-sourced contracts are a transparent reference profile. Before real institution or customer data is used, the confidential production profile requires a separate, reproducible test suite:

| Surface | Primary risk | Required target coverage |
|---|---|---|
| Canonical bar identity | two implementations derive different identities | published cross-language test vectors, normalization, field order, units, version and invalid-input cases |
| Evidence commitment | predictable preimage or salt reuse | cryptographic random salt, domain separation, canonical evidence, low-entropy attack tests |
| Custodian nullifier | same bar accepted with another salt or facility | alternate-salt collision, exclusion of mutable/facility fields, authorized derivation, no general HMAC oracle |
| Nullifier scope | domain-limited control presented as global uniqueness | namespace tests and explicit `does_prove` / `does_not_prove` assertions |
| Key lifecycle | rotation creates a second active identity | stable domain continuity, migration, recovery, compromise and partial-failure tests |
| Private state transition | unauthorized or inconsistent operating-state change | role policy, prior-root match, atomic next-root construction, nullifier-set update and evidence binding |
| Public minimization | private field leaks into Soroban | inspection of arguments, storage, events, auth entries, returns, errors, diagnostic logs and contract spec |
| Batch anchor | replay, rollback, skipped or forked history | epoch uniqueness, previous-root continuity, duplicate batch refusal and recovery behavior |
| Relationship graph | accounts and event types reveal parties | common relay, uniform event, scoped IDs, account-correlation analysis |
| Timing and volume | cadence or batch size reveals activity | fixed windows, padding, quiet-period behavior, retry leakage and observer simulation |
| Approval equivalence | signer approves one act while another is anchored | deterministic private-envelope-to-public-payload derivation and approval reconciliation |
| Evidence disclosure | unauthorized or reusable disclosure | role, tenant, purpose, audience, nonce, expiry, revocation and access-log tests |

These requirements are canonical in [`confidential-control-and-public-integrity.md`](confidential-control-and-public-integrity.md). They must not be added to the passing test count until the corresponding components and tests exist.

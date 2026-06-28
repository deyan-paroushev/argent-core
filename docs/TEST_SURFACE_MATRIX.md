# Argent Core test surface

What the contract test suite covers, and the risk each surface addresses. Every
row maps to tests in this repository, runnable with `cargo test --workspace`.

| Surface | Primary risk | Coverage |
|---|---|---|
| Credit lifecycle | unsafe state transition | framework, position, pledge, line, drawdown, repayment, release, default, enforcement |
| Credit authorization | wrong institution acts | approved-party checks, revocation, host-level wrong-signer tests |
| Credit accounting | double line, overpayment, reversal abuse | one line per pledge, duplicate-line refusal, overpayment refusal, exact reversal, property-style capacity bounds |
| Collateral integrity | the same bars back two pledges | serials lock, duplicate-pledge refusal, reuse only after release |
| Oracle input | stale or future valuation | stale price, future price, malformed threshold refusal |
| Evidence anchors | placeholder hashes | zero-hash refusal for critical legal, collateral, valuation and readiness records |
| Enforcement readiness | false certificate readiness | future validity, duplicate readiness, expiry and repopulation, valuation-source and waterfall evidence required for Ready |
| Settlement vault | token movement on failed repayment | insufficient-balance rollback, unapproved-vault rollback, duplicate-payment rollback, wrong-ledger binding refusal |
| Canonical events | missing or malformed event trail | CollateralEventV1 topic marker pinned, spec matches wire contract, lifecycle events fold back to state |
| Rewards evidence | placeholder reward proof | zero-hash refusal for campaign, spend, finality, claim, voucher, rejection, redemption |
| Rewards accounting | sponsor budget drift | rejected-terminal guard, budget-bucket invariant, property-style user cap and budget bounds |
| Snapshot regressions | silent terminal-state drift | release, enforcement, settlement and redemption snapshot-style assertions |
| TTL assumptions | records disappearing too early | medium-advance read tests for credit and reward records |

## Counts

| Crate | Tests |
|---|---|
| credit_ledger | 125 |
| rewards_ledger | 45 |
| settlement_vault | 17 |
| Total | 187 |

All passing on soroban-sdk 23.5.3. See `docs/REVIEWER_QUICKSTART.md` to run them.

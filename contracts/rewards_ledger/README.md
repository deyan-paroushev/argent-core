# rewards_ledger (hybrid)

A Soroban smart contract for Argent's sponsored gold-rewards program. It models
a capped, sponsor-funded, **non-transferable** bullion-voucher lifecycle, and at
redemption it converts the voucher into **allocated gold weight on-chain**, so
the "accumulate gold" promise is literal and auditable rather than living only
in a partner's off-chain system.

It is independent of the credit ledger: reward gold is never pledged collateral
and never enters the credit ledger's pledge register.

## Design lineage

This is a hybrid of two drafts. It keeps the richer per-transaction claim
lifecycle (finality gate, claim/voucher/sponsor-approval, expiry, and the
reserved/redeemed/cancelled/expired budget buckets) and adds verified gold
settlement at redemption (the fine-weight conversion, a rate ceiling, checked
arithmetic, and a guard that a redeemed reward cannot be clawed back).

## Roles
- **Sponsor**: funds campaigns, approves/rejects one-time claims (the refiner).
- **Verifier**: records finally-posted eligible spend and confirms finality
  (issuer processor / bank).
- **GoldPartner**: confirms redemption into allocated bullion and supplies the
  gold price (often the same entity as the sponsor).
- **Bank**: may claw back a reward before redemption (chargeback / default).
- **Admin**: manages the role registry.

## Lifecycle

```
record_eligible_spend          -> Pending     (reward reserved against budget)
confirm_spend_final            -> Claimable   (finality gate; claim clock starts)
submit_claim                   -> ClaimSubmitted (one-time, with receipt hash)
sponsor_approve_claim          -> VoucherIssued
confirm_redemption (w/ price)  -> Redeemed     (gold weight recorded + accrued)

exits: cancel_reward -> Cancelled   (bank/verifier/sponsor, pre-redemption only)
       sponsor_reject_claim -> Rejected
       expire_reward -> Expired      (after claim window, permissionless)
```

Each transition reconciles the campaign budget buckets. A **Redeemed** reward is
terminal: its gold is allocated and it cannot be cancelled or expired.

## Gold conversion (scaling)

At redemption the gold partner supplies `price_per_oz_e7` (price per troy ounce
in the campaign's program-currency minor units, scaled by 1e7). The contract
records:

```
fine_weight_oz_e7 = redeemed_value * 1e7 * 1e7 / price_per_oz_e7
```

The double 1e7 is required so a small (cents) reward over a large (1e7-scaled)
price does not floor to zero. The result is troy ounces scaled by 1e7. The
per-user running gold balance (`UserCampaignUsage.fine_weight_oz_e7_total`) and
the campaign total (`redeemed_fine_weight_oz_e7`) both accumulate it, which is
what the frontend reads to show "grams of gold accumulated."

Worked figure: a CHF 100.00 reward at CHF 3,000.00/oz redeems to 333_333
(1e7-scaled oz) = 0.0333 oz.

## Reads
`get_campaign`, `get_spend`, `get_accrual`, `get_user_usage`, `get_claim`,
`get_voucher`, `get_redemption`.

## Build and test

Targets `soroban-sdk = 22.0.0`. With the Stellar toolchain:

```bash
cd contracts
cargo test -p rewards_ledger
stellar contract build --manifest-path Cargo.toml
# or: ./deploy.sh   (build + optimize + deploy)
```

The test suite (`src/test.rs`) covers the full flow through gold redemption, the
finality gate, per-user cap truncation and exhaustion, idempotency, bank
clawback, the post-redemption cancellation guard, sponsor reject, expiry timing,
the rate ceiling, policy mismatch, gold-partner authorization, value/price
validation, pause, and budget exhaustion.

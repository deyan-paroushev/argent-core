# Product

## The problem

Many viable companies need recurring bank instruments before they can trade, clear goods, bid, perform, or enter a project. Guarantees, letters of credit, customs security, and related obligations consume bank limits or require collateral. When guarantee headroom is exhausted, a capable company can lose work even when demand and execution capacity remain.

Cash margin solves the bank's collateral problem by creating another problem for the company: operating liquidity is immobilized while payroll, suppliers, duties, and settlement still require cash.

Some companies already own allocated bullion as inventory or treasury reserve. Others may choose to build a gold reserve because one controlled pool can support recurring obligations while remaining a reserve asset. Today, turning that metal into repeatable bank-usable capacity is fragmented across custody records, security documents, bank limits, emails, spreadsheets, and product systems.

Argent exists to coordinate that fragmented control state.

## The product

Argent is a bank-in-the-loop collateral-control layer for purpose-bound obligations.

```text
Customer-owned allocated bullion
        ↓ custodian control
Bank-approved facility capacity
        ↓ bank-authorized allocation
Guarantee or other approved obligation
        ↓ expiry, cancellation, reimbursement, or enforcement
Capacity restored or collateral realized
```

One controlled reserve may support several separately approved allocations under the same facility. An allocation cannot exceed available capacity. When an obligation is validly discharged, its allocation returns to the facility and may support a later bank-approved use.

Unused capacity is not customer-drawable cash.

## Who it is for

The first customer profile has all three characteristics:

1. **Eligible reserve.** It owns individually identifiable, allocated bullion held by a bank-acceptable custodian.
2. **Recurring instruments.** It repeatedly needs bank guarantees, documentary credits, customs security, or another beneficiary-mandated bank instrument.
3. **Capacity constraint.** Cash margin or existing non-funded limits constrain operations or the ability to accept new work.

The most credible first segment is a precious-metals business using ring-fenced company-owned kilobars—not customer metal or fast-moving inventory—to support a customs standing guarantee. Contractors and diversified groups are an expansion path after the first facility works.

## How it helps

### For the company

- Preserves operating cash where existing gold can perform the collateral role.
- Creates a visible, bank-approved pool of obligation capacity.
- Partitions one facility into purpose-bound allocations.
- Returns capacity after confirmed discharge.
- Supports governed substitution or partial release where the legal documents permit it.

Gold is not free liquidity. If a company must buy gold solely for the facility, the purchase uses cash and adds price and margin risk. The strongest first case is existing or strategically desired reserve metal whose opportunity cost is lower than immobilizing operating cash.

### For the bank

- Adds a controlled collateral option without surrendering underwriting or issuance authority.
- Applies bank-defined eligibility, haircut, valuation, product, beneficiary, and sublimit rules.
- Prevents inconsistent or excess allocation within the governed facility state.
- Receives reconciled evidence for custody, approval, issue, discharge, release, and enforcement.
- Can offer a modern product to reserve-owning companies while existing bank systems remain authoritative.

### For the custodian

- Keeps physical control and remains authoritative for bar identity and custody status.
- Produces controlled attestations rather than publishing the bar register.
- Executes immobilization, substitution, release, or realization only through governed instructions.

## What makes it different

### Cash margin

Cash-backed facilities can already pool several guarantees. Argent does not claim that allocation pooling is unique to gold. The potential advantage is capital composition: an accepted bullion reserve performs the backing role while operating cash remains available.

### Surety

Surety underwrites the enterprise and charges a premium for its risk capacity. Argent controls specific collateral behind a bank instrument. Where the beneficiary accepts surety, it may be an attractive alternative. Argent focuses first on obligations for which a bank instrument is required or preferred, or where secured bank capacity complements exhausted unsecured limits.

### Gold lending and tokenization

Argent is not a gold loan and does not issue a gold token. Title does not move merely because capacity is allocated. Selling, lending, leasing, tokenizing, or granting a competing security interest in pledged bars is outside the product boundary unless the bank, custodian, legal documents, and protocol profile expressly permit it.

## Why Soroban

Blockchain is justified only where several institutions need evidence of the same ordered control state but should not let one participant rewrite the history.

Soroban contributes:

- role-bound authorization for bank, custodian, company, and operator actions;
- deterministic refusal of invalid sequence, replay, and over-allocation;
- a neutral, timestamped integrity history;
- atomic value-and-state movement where a regulated settlement asset is actually used.

Soroban is not the commercial database. In the production profile, identities, bar records, values, limits, beneficiaries, documents, and relationship metadata remain private. Only minimized integrity anchors are public.

## First product boundary

- Gold only.
- Collateral control only.
- Bank-issued obligations only.
- No surety or insurance underwriting.
- No unrestricted cash draw.
- No public tokenization.
- One bank, one custodian, one company, and one obligation in the first pilot.

See [PILOT_PROFILE.md](PILOT_PROFILE.md) for the proposed validation scope.

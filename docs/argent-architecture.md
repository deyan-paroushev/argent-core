# Argent: Technical Architecture

**Gold-backed credit on Stellar.**

A Soroban collateral engine for bank-issued secured credit lines backed by vaulted physical gold. The card transaction stays on card rails. The collateral truth lives on Soroban.

*Version 1.1 · Reference architecture for build and for the Stellar × CV Labs application. v1.1 names the repayment-funding rail (SEP-10/SEP-24) and the SDF building blocks the build reuses.*

---

## 0. What this document is

This is the engineering reference for Argent: what it is, what moves where, which parts are genuinely on-chain, and the exact build order. It is written backend-first, because the backend is the spine of the system and the contract is the anchored truth layer beneath it, not the other way around.

Two audiences: the build (you, in Codespaces) and the reviewer (Stellar × CV Labs, and later a partner bank's technical team). Everything here is written to survive scrutiny from both. Where a claim about Stellar or Mastercard is load-bearing, it is stated precisely enough that a sharp reviewer cannot poke it.

The single most important framing, stated once and carried throughout:

> **Argent does not process card payments. It is not a crypto card. It is the collateral-and-credit state layer that an issuing bank relies on when it authorizes, posts, freezes, and, on default, enforces a secured credit line whose security is verified allocated gold.**

---

## 1. The product in one paragraph

A person or institution owns allocated physical gold bars in a regulated vault. A partner bank opens a credit line secured by those bars and issues a Mastercard credit card against the line. The gold never moves while the card is used; the card spends the bank's credit. Argent locks the collateral, records the pledge, derives the borrowing base from a live gold price, tracks utilization as the card is used, restores capacity on repayment, and, if the borrower defaults and does not cure, records the enforcement workflow (bank instruction, custodian confirmation, realization outcome) by which the pledged bars are realized for the bank under the security documents. The verifiable state of that secured facility lives on Stellar via Soroban; the sensitive detail (bar serials, KYC, card PAN, legal documents) stays off-chain, referenced by hash.

---

## 2. The honest boundary: what moves, when, and on which rail

The reason most "blockchain card" designs fall apart under questioning is that they conflate two things a real card system keeps strictly separate: **the credit decision** ("is this spend allowed?") and **the money movement** ("who pays the merchant, and when?"). These happen at different times, on different rails, run by different parties. Argent keeps them separate, exactly as the real system does.

### 2.1 The real card lifecycle

**Authorization (sub-second; no money moves).** The card is tapped. The acquirer routes the request through Mastercard to the issuer processor, which asks the bank's authorization system "approve this amount?" At authorization, **no funds move**, a *hold* is placed against the credit line. This path is off-chain by physical necessity; you cannot put a blockchain round-trip inside a point-of-sale authorization. Argent's role here is to be the *source of truth the authorization system reads* for available collateral-backed capacity, and to *receive* the resulting hold as an event.

**Clearing and settlement (later, batched).** This is where money actually moves, and critically, **none of it touches the gold or the Argent contract**. The issuing bank pays Mastercard; Mastercard pays the acquirer; the acquirer pays the merchant. The cardholder now owes the bank. Settlement between issuers and acquirers has historically run in fiat batches limited by banking hours; as of June 2026 Mastercard also supports on-chain stablecoin settlement (intraday, weekend, holiday) as an *option for issuers and acquirers*, a back-office settlement change, not point-of-sale spending. The thing Argent cares about is only this: the borrower's **drawn balance against the bank rose**.

**Repayment (cardholder → bank).** The cardholder pays their statement. The drawn balance falls. Capacity restores. This is the one money-movement leg that Argent can legitimately put on Stellar, see §2.3.

### 2.2 So what does Argent actually put on-chain?

Not the merchant payment. The on-chain artifact is the thing Argent genuinely governs:

- **The collateral lock**, specific bars pledged to a specific bank, un-pledgeable elsewhere while locked.
- **The secured-debt position**, approved limit, drawn balance, available capacity, LTV, status.
- **The state transitions**, pledge → activate → draw → repay → release, and the default → cure → enforce branch.
- **The enforcement record**, on uncured default, the title-state of the pledged bars flips to the bank, anchored with a hash of the off-chain legal transfer.

This is a **secured-debt position ledger**. Real lenders keep exactly this; the novelty is making it verifiable and shareable between the lender, the collateral provider, the custodian, and an auditor, which is precisely what a distributed ledger is good for and a private bank database is not.

### 2.3 The one real value transfer (and why it's the right one)

For the accelerator's requirement, *moving elements from one party to another over Stellar, processed via a Soroban contract*, the right transaction is **repayment as delivery-versus-payment**, not a fictional escrow drain.

> The cardholder repays in a Stellar-issued cash asset (a regulated stablecoin or tokenized deposit, represented via the Stellar Asset Contract / SEP-41). In a single atomic Soroban transaction: the cash moves cardholder → bank, the drawn balance reduces, and, if the facility is being closed, the pledge releases. Cash in, capacity restored, collateral released, atomically.

This is real, it is useful outside the accelerator, and it does something card rails *cannot* do on their own: bind a repayment and a collateral release into one atomic settlement. That is a genuine Stellar value-add rather than blockchain theatre. It also maps cleanly onto Soroban's atomic-swap / multi-party-authorization pattern, which is a documented, supported primitive.

> **Why not the escrow demo?** An earlier sketch had the bank pre-fund the full credit line into a contract escrow, and the card purchase drained it. That is clean to *watch* but economically fictional, no bank parks its entire credit line inside a smart contract, and a reviewer asks why. We drop it. We keep one honest on-chain value transfer (repayment DvP), and model card draws as debt-position updates (which is what they are). Nothing in the demo describes a system a bank couldn't actually run.

### 2.4 How the cardholder gets the cash to repay with (the funding rail)

The repayment DvP needs the cardholder to hold the settlement asset on Stellar. That "how do they get it" step is not hand-waved, it runs on Stellar's standard anchor flow, the same pattern MoneyGram runs in production for USDC cash-in/out across 170+ countries:

- **SEP-10** authenticates the cardholder's Stellar account (challenge/response; no password).
- **SEP-24** runs the hosted deposit: the cardholder funds from a bank account / card and receives the settlement stablecoin (USDC to start) into their Stellar account, with KYC handled in the anchor's hosted UI via **SEP-9** fields.
- The cardholder then signs the **`repay_and_release`** leg with that asset.

Two consequences worth stating in the pitch. First, identity is not bespoke: KYC/auth route through SEP-9/SEP-10, the same standards a regulated anchor uses, which folds cleanly into the `ComplianceGate`. Second, Argent does not have to build anchor plumbing, SDF's **Anchor Platform** is a production service that implements these SEPs, and the **`@stellar/typescript-wallet-sdk`** is the client library for the auth + deposit flow (it is literally the SDK MoneyGram's integration guide prescribes). For the demo, the funding step can be stubbed (fund the cardholder's testnet account with USDC-test from the Circle faucet); for a pilot, it becomes a real SEP-24 deposit.

---

## 3. The four layers

```
┌─────────────────────────────────────────────────────────────┐
│  1. PHYSICAL COLLATERAL LAYER                                 │
│     Allocated gold bars · Swiss/regulated vault · custodian   │
│     Custody attestation (off-chain, signed)                   │
└──────────────────────────────┬──────────────────────────────┘
                               │  attestation hash + fine weight
                               ▼
┌─────────────────────────────────────────────────────────────┐
│  2. ARGENT COLLATERAL SERVICE  (TypeScript · Railway)         │
│     Bar registry · valuation · LTV · borrowing base           │
│     Authorization API (read) · indexer (write)                │
│     PostgreSQL materialized state                             │
└───────────────┬─────────────────────────────┬───────────────┘
                │  reads/writes               │  authorization read
                ▼                             ▼
┌──────────────────────────────┐  ┌──────────────────────────────┐
│  3. STELLAR / SOROBAN         │  │  4. BANK + MASTERCARD LAYER   │
│     CreditLedger contract     │  │     Issuing bank (lender)     │
│     SettlementVault (DvP)     │  │     BIN sponsor / principal   │
│     Oracle adapter            │  │     Program manager           │
│     Anchored truth + events   │  │     Issuer processor          │
└──────────────────────────────┘  │     Mastercard network        │
                                   └──────────────────────────────┘
```

The **Argent Collateral Service (layer 2) is the spine.** It owns the live state in Postgres, exposes the authorization read the bank/processor calls, runs the indexer that mirrors Soroban events, and submits the state-transition transactions to Soroban. Layer 3 is the durable, verifiable anchor beneath it. Layer 4 is the regulated card infrastructure Argent integrates *beside*, never replaces.

---

## 4. Where Stellar and Mastercard actually sit (the precise, checkable claims)

This section exists so that nothing in the pitch overstates a relationship. State it exactly like this.

### 4.1 Stellar / Soroban: what is true

- Soroban is Stellar's smart-contract platform: Rust compiled to WASM, live on mainnet since the Protocol 20 upgrade (Feb 2024).
- The **Stellar Asset Contract (SAC)** implements the **SEP-41 token interface** (CAP-46), so contracts move issued Stellar assets, including regulated stablecoins like USDC/EURC, through a standard `transfer` interface. This is what the repayment DvP leg uses.
- Soroban's **authorization framework** (`require_auth` / `require_auth_for_args`) provides built-in replay protection and structured multi-party signing; the **atomic-swap example** is the documented pattern for "two parties each sign their leg of one atomic transaction," which is the shape of pledge-acceptance and repayment-DvP.
- **Three-tier storage** (instance / persistent / temporary) with TTL and state archival. Security invariants and balances go in **persistent** storage (archivable, restorable, never silently lost); short-lived holds can go in **temporary**; global config in **instance**. *Never* put a security-critical value in temporary storage, expired temporary entries are deleted permanently.
- **Price data**: Reflector is the Stellar-native, SEP-40-compliant decentralized oracle (≈5-minute updates). For the demo, the oracle is stubbed with an admin-pushed price plus an `expiry_ledger`; for production, Reflector (or a comparable SEP-40 feed) supplies the gold price, with staleness gating in the contract.

### 4.2 Mastercard: what is true, and the one thing to *not* claim

- Mastercard runs a **Crypto Partner Program** (announced/active 2026, 85+ participants) explicitly about pairing on-chain programmability with established card rails. **Stellar is a named participant.** So are the card-issuing primitives Argent's stack would actually use: issuer processors (Marqeta, Galileo, Lithic, Highnote, Thredd), BIN sponsors / banks (CBW Bank, Cross River, Lead Bank, WebBank, Peoples Group), and program managers.
- The issuance stack is layered: **BIN sponsor** (holds the principal Mastercard license; carries scheme-compliance responsibility) → **program manager** (coordinates the parties) → **issuer processor** (connects issuer to the scheme; handles issuance, authorization, clearing, settlement) → the **issuing bank** (extends the credit). Argent is **none** of these. Argent supplies the collateral/credit-decision substrate the issuer relies on.
- The closest existing card category is the **secured credit card**, except the security is verified allocated gold instead of a cash deposit. That reframing is the whole product, and it requires a bank to accept the collateral model.
- As of **June 3, 2026**, Mastercard supports **on-chain stablecoin settlement** for issuers/acquirers across a set of chains and six regulated USD stablecoins (USDC, RLUSD, PYUSD, USDG, USDP, SoFiUSD).

> **The one thing not to claim.** Mastercard's June 2026 settlement-chain list (Ethereum, Solana, Polygon, Base, Arbitrum, XRPL, Canton, Tempo) **does not currently include Stellar.** So do **not** say "Mastercard settles on Stellar." The accurate, still-strong claim is: *Stellar is a Mastercard Crypto Partner Program participant; Argent's Stellar settlement leg is for the **borrower↔bank repayment**, which is independent of Mastercard's issuer↔acquirer settlement rail.* If a bank partner later wants card-settlement on-chain, that runs on a Mastercard-supported chain, orthogonal to Argent's collateral logic, which is chain-internal to Stellar.

> **Do not nominate Mastercard as a technology sponsor.** There is no Mastercard relationship, sandbox, or program acceptance. The correct positioning is **"Mastercard-compatible, Mastercard-first reference design"** on a **network-agnostic core**. Stellar's existing Crypto Partner Program / Crypto Credential standing is all the borrowed credibility needed.

---

## 5. Actors

| Actor | Role | Touches Argent how |
|---|---|---|
| **Cardholder** | Owns the bars; borrows against them; spends; repays or defaults | Signs pledge proposal & repayment (Stellar address / contract account) |
| **Custodian / vault** | Holds the physical bars; signs custody attestations; confirms lock/release | Approved attestor; signs attestation off-chain, hash anchored on-chain |
| **Issuing bank** | Extends credit; issues the Mastercard; owns the exposure; receives title on default | Approved party; signs acceptance, default notice, enforcement |
| **BIN sponsor / program mgr / issuer processor** | Connect the bank to Mastercard; handle issuance, authorization, clearing, settlement | Call Argent's authorization read; post utilization events |
| **Argent operator** | Runs the collateral service, indexer, APIs, UI | Submits state transitions to Soroban; does **not** lend, custody, or issue |
| **Valuation provider** | Supplies gold price + haircut | Reflector / SEP-40 feed (prod); admin-pushed (demo) |
| **Compliance provider** | KYC / sanctions / eligibility | Approved-party gate; credential hash on-chain |
| **Auditor** | Verifies the secured facility independently | Read-only against Soroban state + events |

---

## 6. Domain model

Core entities (shared vocabulary across Postgres schema, the API, and the contract storage keys). On-chain stores the **enforceable state and hashes**; Postgres stores the **full materialized record**; sensitive detail stays off-chain entirely.

```
VaultPosition       owner, custodian, barlist_hash, fine_weight_oz,
                    attestation_expiry, status{Free|Pledged|Enforced|Released}

CollateralPledge    position_id, pledgor, secured_bank, locked_fine_weight,
                    custody_attestation_id, valuation_id, legal_terms_hash,
                    status{Proposed|Active|Released|Defaulted|Enforced}

CreditLine          pledge_id, bank_id, cardholder_id, currency, approved_limit,
                    drawn_balance, available_limit, ltv_ratio, review_date,
                    status{Offered|Active|Suspended|PastDue|Defaulted|Closed}

CardAccountRef      bank_id, processor_card_ref, network=Mastercard,
                    credit_line_id, status{Active|Frozen|Closed}   (ref only; no PAN)

AuthorizationHold   credit_line_id, auth_ref, amount, currency, expires_at,
                    status{Approved|Declined|Reversed|Captured}

PostedTransaction   credit_line_id, auth_ref, amount, currency, posted_at, settlement_ref

Repayment           credit_line_id, amount, currency, rail, paid_at,
                    status{Pending|Confirmed|Failed}

DefaultEvent        credit_line_id, reason, missed_at, cure_deadline,
                    status{NoticeIssued|CureOpen|Cured|CureExpired|Enforced}

TitleTransfer       default_id, from_owner, to_bank, bar_ids, legal_basis_hash,
                    executed_at, status{Pending|Executed|Disputed}
```

### 6.1 What lives where

| Data | Soroban | PostgreSQL | Off-chain only |
|---|---|---|---|
| Position hash, fine weight, status | ✅ | ✅ | |
| Pledge lock, secured party, status | ✅ | ✅ | |
| Credit line limit / drawn / available / status | ✅ | ✅ | |
| Utilization (auth holds, postings) | summary | ✅ full | |
| Repayment event | hash + amount | ✅ | |
| Default state, cure, enforcement | ✅ | ✅ | |
| Title-transfer event | hash | ✅ | legal docs |
| Bar serial numbers | ❌ (only `barlist_hash`) | encrypted | source docs |
| Card PAN / cardholder PII / KYC files | ❌ | ❌ (ref only) | ✅ |
| Raw Mastercard auth payload | ❌ | ❌ | ✅ |

Rationale: Soroban holds the enforceable state and the hashes that make the off-chain record tamper-evident; it never holds private card, identity, or bar-ownership data.

---

## 7. The contract suite (Soroban / Rust)

Two contracts do the real work; a thin oracle adapter and an access registry support them. Keep them as a suite, not a monolith, it isolates the value-moving code (SettlementVault) from the state machine (CreditLedger) and keeps audit surface small.

### 7.1 `CreditLedger`: the secured-debt position ledger

The heart of the system. Holds no money. Holds the pledge, the credit line, and every state transition. Exposes a cheap read (`available_capacity`) for the off-chain authorization path, and write methods that record what the card world reports.

```rust
// State (persistent storage; keyed by position_id / credit_line_id)
VaultPosition  { owner, custodian, barlist_hash, fine_weight_oz_e7,
                 attestation_expiry_ledger, status }
Pledge         { position_id, pledgor, bank, locked_weight_e7,
                 valuation_id, legal_terms_hash, status }
CreditLine     { pledge_id, bank, cardholder, currency,
                 approved_limit, drawn_balance, available_limit,
                 ltv_bps, cure_expiry_ledger, status }

// Lifecycle, collateral & facility
fn register_position(owner, custodian, barlist_hash, fine_weight_e7, expiry_ledger) -> position_id
        owner.require_auth(); custodian.require_auth();
        assert is_approved(custodian, Custodian);
        assert expiry_ledger > ledger.sequence();
        assert position is new;  status = Free

fn activate_pledge(position_id, owner, bank, legal_terms_hash) -> pledge_id
        owner.require_auth(); bank.require_auth();
        assert position.status == Free;
        assert position.attestation fresh;
        assert is_approved(bank, Bank);
        position.status = Pledged;  pledge.status = Active

fn open_credit_line(pledge_id, bank, cardholder, limit, currency, ltv_bps) -> credit_line_id
        bank.require_auth();
        assert pledge.status == Active;
        assert limit <= borrowing_base(pledge, oracle);   // see §7.3
        status = Active;  drawn = 0;  available = limit

// Lifecycle, utilization (mirrors the card world; moves no money)
fn record_drawdown(credit_line_id, processor, auth_ref, amount)
        processor.require_auth();
        assert line.status == Active && pledge.status == Active;
        assert amount > 0 && line.available_limit >= amount;
        assert not already_recorded(auth_ref);
        line.drawn_balance += amount;  line.available_limit -= amount;
        events.publish(("card","draw"), (credit_line_id, auth_ref, amount))

fn reverse_drawdown(credit_line_id, processor, auth_ref)   // auth expiry / reversal
        processor.require_auth();  restore available_limit

// Lifecycle, default & enforcement (records; does not bypass law)
fn issue_default_notice(credit_line_id, bank, reason, cure_deadline_ledger)
        bank.require_auth();  line.status = Defaulted;  pledge.status = Defaulted
fn cure_default(credit_line_id, cardholder, repayment_ref)
        cardholder.require_auth();  line.status = Active;  pledge.status = Active
fn enforce_title_transfer(credit_line_id, bank, legal_transfer_hash)
        bank.require_auth();
        assert line.status == Defaulted;
        assert ledger.sequence() >= line.cure_expiry_ledger;
        pledge.status = Enforced;  position.status = Enforced;  line.status = Closed;
        events.publish(("title","transfer"), (credit_line_id, legal_transfer_hash))

// Read (for the off-chain authorization path)
fn available_capacity(credit_line_id) -> i128
fn line_status(credit_line_id) -> Status
```

### 7.2 `SettlementVault`: the one value-moving contract (repayment DvP)

The only contract that moves a token. Isolated deliberately. Implements the atomic repayment-and-release.

```rust
// Holds a reference to the settlement asset (SAC / SEP-41) and the CreditLedger.
fn repay_and_release(
        credit_line_id, cardholder, amount, release_if_cleared: bool)
    cardholder.require_auth();                       // payer signs their leg
    let line = credit_ledger.get_line(credit_line_id);
    assert amount > 0 && amount <= line.drawn_balance;

    // 1. Move the settlement asset: cardholder -> bank  (SEP-41 transfer)
    let token = token::TokenClient::new(&env, &settlement_token);
    token.transfer(&cardholder, &line.bank, &amount);

    // 2. Reduce the debt atomically (cross-contract call into CreditLedger)
    credit_ledger.apply_repayment(&credit_line_id, &amount);

    // 3. Optionally release the pledge in the same tx if the line is cleared
    if release_if_cleared && line.drawn_balance - amount == 0 {
        credit_ledger.release_pledge(&line.pledge_id);
    }
    env.events().publish(("repay","settled"), (credit_line_id, amount));
```

This is the transaction the demo leads with: a real SEP-41 asset moves cardholder → bank, the debt drops, and the gold is released, one atomic Soroban call. If the transfer fails, nothing commits.

### 7.3 `OracleAdapter` + `borrowing_base`

```rust
fn price_xau(env) -> (price_e7, ts) {
    // PROD: Reflector / SEP-40 client; assert (now - ts) < max_staleness
    // DEMO: admin-pushed price in instance storage + expiry_ledger gate
}
fn borrowing_base(pledge, oracle) -> i128 {
    let (px, ts) = oracle.price_xau();
    assert fresh(ts);
    pledge.locked_weight_e7 * px / 1e7 * ltv_bps / 10_000
}
```

Gold-price staleness is the single biggest risk in any gold-credit product, name the dependency explicitly and gate on it. Never open or draw a line against a stale valuation.

### 7.4 `AccessControl`

Approved custodians, banks, valuation/compliance providers, keyed by role; admin-gated via `require_auth`. (OpenZeppelin's Soroban RBAC is a reasonable base.) The `ComplianceGate` check, only approved owners/banks/custodians/issuers may participate, folds in here.

### 7.5 Refusal paths (the contract's spine: test these explicitly)

- Refuse pledge if position not `Free`, attestation expired, or bank not approved.
- Refuse double-pledge of the same bars.
- Refuse `open_credit_line` if `limit > borrowing_base` or valuation stale.
- Refuse `record_drawdown` above `available_limit`, or if pledge/line not active, or duplicate `auth_ref`.
- Refuse `release_pledge` while `drawn_balance > 0`.
- Refuse `enforce_title_transfer` before `cure_expiry_ledger`, if cured, or if already enforced.
- Refuse any state-changing call by an unapproved party.

---

## 8. The backend (TypeScript · Railway · PostgreSQL): built first

This is where the build starts. The contract is the anchor, but the service is the product surface, and it can be developed and demonstrated against Soroban **testnet** from day one.

### 8.1 Shape

```
argent-service/  (TypeScript, Node)
├── api/            REST: positions, pledges, credit-lines, authorizations,
│                   repayments, defaults, enforcement   (the bank/processor-facing surface)
├── chain/          @stellar/stellar-sdk: build/sign/submit Soroban tx; read contract state
├── anchor/         @stellar/typescript-wallet-sdk: SEP-10 auth + SEP-24 deposit
│                   (cardholder funds the settlement asset to repay with)
├── indexer/        subscribe to Soroban events → upsert into Postgres (materialized view)
├── domain/         the state machine, borrowing-base math, validation
├── db/             Postgres schema + migrations (the live operational truth)
└── auth/           party auth, role checks, request signing
```

### 8.2 The two-speed design (this is the whole trick)

- **Authorization is fast and off-chain.** When a card is tapped, the issuer processor calls `POST /authorizations/check`. The service answers from the **Postgres materialized view** in milliseconds, never a blockchain round-trip. This is correct, not a shortcut: real card auth has always read a fast issuer-side ledger.
- **State transitions are durable and on-chain.** Draws, repayments, defaults, enforcement are written to Soroban (via `chain/`), and the **indexer** reflects the resulting events back into Postgres so the fast view stays consistent with the anchored truth. Soroban is the system of record; Postgres is the system of speed.

### 8.3 The authorization read (the Mastercard touchpoint)

```
POST /authorizations/check
{ "cardRef": "card_001", "amount": "12500.00", "currency": "CHF",
  "merchantCategory": "business_services", "country": "CH" }

→ service checks the materialized view:
    card active? · line active? · pledge active? · valuation fresh?
    available_limit >= amount? · not in default?

200 { "decision": "APPROVED",
      "availableBefore": "720000.00", "availableAfter": "707500.00",
      "reason": "Within active gold-backed credit line" }
   or { "decision": "DECLINED", "reason": "Exceeds available collateral-backed capacity" }
```

After the issuer processor confirms the hold, the service records the draw on Soroban (`record_drawdown`) and the indexer updates the view.

### 8.4 Why Postgres and not just chain state

Card programs need: sub-ms reads, rich query (by merchant, MCC, country, time), reversals/expiries, reconciliation, and reporting. None of that belongs on-chain. Postgres is the operational ledger; Soroban holds the *enforceable* subset and the hashes that make Postgres auditable. This is the same split a regulated issuer would expect to see.

---

## 9. The frontend (TypeScript)

A role-aware cockpit, wrapped around the live service (which is wrapped around testnet Soroban). Views:

- **Cardholder**, bars, pledge status, line, available capacity, repay button (triggers `repay_and_release`).
- **Bank**, collateral seen, limit set, utilization, default/enforce controls.
- **Custodian**, attestation status, lock/release confirmation.
- **Auditor**, read-only reconciliation of Postgres view against Soroban state + events.

The frontend never talks to Soroban directly for writes; it calls the service, which signs and submits. (Repayment can optionally be signed client-side via a Stellar wallet to demonstrate the cardholder's own auth leg.)

---

## 10. The demonstrable transaction (what the demo shows)

Starting state: cardholder owns 3 allocated bars; gold value ≈ CHF 1.2M (live-priced); LTV 60%; line CHF 720k; gold `Free`.

1. **Register**, custodian attests; `register_position` on Soroban → `Free`.
2. **Pledge**, cardholder + bank sign; `activate_pledge` → gold `Pledged`, line eligible. *(first multi-party Soroban tx)*
3. **Open line**, bank; `open_credit_line`, limit derived from live `borrowing_base`.
4. **Card draw**, simulated authorization off-chain → `record_drawdown(25,000)` → available 695k; **gold unmoved**.
5. **Repay (the headline)**, `repay_and_release(25,000)`: **SEP-41 cash moves cardholder → bank, debt → 0, pledge releases, one atomic tx.** *(the real value movement over Stellar via Soroban)*
6. **Default branch**, re-run to a missed payment: `issue_default_notice` → card frozen, line `Defaulted`; cure window lapses; `enforce_title_transfer` → bars' title-state → bank. *(the second strong moment)*

Two visible movements, both honest: **money moving on repayment**, and **collateral title flipping on enforcement**. Neither pretends Soroban is Mastercard.

---

## 11. Build order

The vertical slice to a demonstrable transaction is five contract functions plus the two-speed backend, not the full eight-contract design.

**Phase 0, Skeleton (Codespaces).** Rust workspace (`CreditLedger`, `SettlementVault`, `OracleAdapter` stub, `AccessControl`); TS service skeleton; Postgres schema + migrations; Railway project; Soroban testnet keys. Reuse, don't reinvent, see the building blocks in the Appendix: scaffold the contract + frontend with **Scaffold Stellar**, build `AccessControl` on **OpenZeppelin Contracts for Soroban**, and stand up `anchor/` and `chain/` with the **`@stellar/typescript-wallet-sdk`** / **`@stellar/stellar-sdk`**.

**Phase 1, Backend spine + happy path.** Service API + Postgres view + `chain/` submit + `indexer`. Contract happy path: `register_position` → `activate_pledge` → `open_credit_line` → `record_drawdown` → `repay_and_release`. Demonstrable end-to-end on testnet.

**Phase 2, Refusal paths.** Every assertion in §7.5, with tests. This is where the product's credibility lives.

**Phase 3, Default & enforcement branch.** `issue_default_notice` → `cure_default` / `enforce_title_transfer`.

**Phase 4, Cockpit UI.** The four role views over the live service.

**Phase 5, Oracle + polish.** Swap the oracle stub for Reflector/SEP-40 with staleness gating; Soroban event hardening; TTL/archival handling on persistent entries; reconciliation view.

**Phase 6, Pilot-readiness docs.** Bank API spec, processor integration spec, custodian attestation spec, and a one-page legal/enforcement boundary memo.

---

## 12. The hard parts (state these plainly: they are features here)

- **The partner bank is the real gating dependency.** Argent makes the collateral clean enough for an issuer to rely on; it does not become the issuer. The accelerator's "regulatory-aware RWA" framing rewards this honesty.
- **Enforcement is legal, not instantaneous.** `enforce_title_transfer` flips an on-chain status and anchors a hash of the off-chain legal transfer. The bars move through the custody instruction and security agreement, under law. The chain removes *factual* dispute about the secured position; it does not bypass the legal process. Say this out loud.
- **Oracle risk is the core financial risk.** Stale or manipulated gold prices break LTV. Mitigate with a SEP-40 feed, staleness gating, and conservative haircuts; name it in the pitch rather than hiding it.
- **Stellar is not (yet) on Mastercard's settlement-chain list.** Argent's Stellar leg is borrower↔bank repayment, independent of Mastercard issuer↔acquirer settlement. Keep the two settlement domains clearly separate in every description.
- **Soroban constraints.** `no_std`; security-critical state in persistent storage with TTL management (never temporary); keep the value-moving surface (`SettlementVault`) minimal for audit.

---

## 13. The sentences to use (positioning, verbatim-safe)

- *"Argent is a Soroban collateral engine for bank-issued secured credit lines backed by vaulted gold. The card transaction stays on card rails. The collateral truth lives on Stellar."*
- *"Network-agnostic core, Mastercard-first reference implementation."*
- *"Argent does not process card payments, custody gold, or lend. It makes the gold-backed collateral position clean, locked, visible, and enforceable enough for a regulated issuer to rely on."*
- *"The one thing that moves over Stellar is the repayment and the collateral release, atomically, which is the part card rails can't do."*

## DFNS integration: institutional signing and approval layer

*This section describes the planned SCF Build Integration-Track work. DFNS is the selected, approved Integration-List building block (Wallets-as-a-Service). The integration replaces the development stand-in signers with policy-governed institutional Stellar wallets, one per market role, and adds the approval workflow that a bank/custodian/refiner-facing product requires. The architecture below already accommodates it: see `service/src/chain/signer.ts`, where the `Signer` interface, the `SignerRegistry`, auth-entry signing, and the `DfnsSigner` placeholder define the exact seam DFNS slots into. No change to the chain layer is required to swap in DFNS, that is the whole point of the interface.*

### Why DFNS

Argent must never let one backend key act for every market participant. The bank, custodian, verifier, and reward sponsor each need separate authority, their own approval rules, and their own audit trail over the Soroban actions assigned to their real-world role. DFNS provides exactly that: a policy-governed Stellar wallet per role, signing/approval policies, pending-approval objects with webhook events, and key management, while the institution retains control (DFNS is infrastructure, not a custodian). This turns Argent's multi-party control model from a claim into an enforced property.

### Role → DFNS wallet mapping

| Market actor | Real-world responsibility | DFNS role in Argent |
|---|---|---|
| **Owner / cardholder** | selects gold, accepts pledge, repays, claims rewards | own wallet (Freighter / Stellar Wallets Kit), not DFNS, the value-bearing repayment leg stays with the user |
| **Bank** | accepts pledge, opens line, suspends, releases, enforces | DFNS bank wallet + credit/enforcement approval policy |
| **Custodian** | confirms bars, immobilizes, releases, confirms realization | DFNS custodian wallet + custody-control policy |
| **Verifier** | confirms eligible posted card spend | DFNS verifier wallet + spend-evidence policy |
| **Sponsor / refiner** | approves reward campaign, claim, voucher, redemption | DFNS sponsor wallet + reward-campaign policy |
| **Operator** | builds/submits tx, pays fees, indexes events | Argent's own operator key (tx source + fee sponsorship) |

### Signing flow

```
Soroban action requested (e.g. bank authorizes release)
        ↓
Argent service builds the invocation, simulates, discovers the auth entries
        ↓
each entry routed to the role's DFNS wallet (via the DfnsSigner)
        ↓
DFNS policy evaluates → approval pending / approved / rejected / expired
        ↓
on approval, DFNS signs the Soroban authorization entry
        ↓
Argent reassembles and submits the transaction to Stellar
        ↓
indexer reflects the resulting events; frontend shows signer/approval state
```

### The one validation item (named honestly)

The chain layer signs Soroban **authorization entries**, not whole transaction envelopes. The first funded step validates the exact DFNS path for this: either DFNS Stellar transaction signing where applicable, or DFNS generic Ed25519/hash signing of the Soroban auth-entry payload, returning a signature plus the role's public key (the `SigningCallback` shape `authorizeEntry()` accepts). Naming this as a Tranche-1 validation item rather than assuming it is deliberate.

### Reusable ecosystem output

A by-product of the integration is a **DFNS + Soroban multi-party authorization reference adapter**: a TypeScript module showing how to build a Soroban invocation, discover required authorizations, route each role signature through a DFNS-managed Stellar wallet, handle pending approvals, submit, and expose signer state to a frontend. This is published for the ecosystem, so the work has utility beyond Argent.

### What does *not* change

The Soroban contracts stay signer-agnostic, no DFNS dependency in the contracts. Argent still does not custody gold, lend, or process payments. DFNS governs *who may authorize which action*; Soroban records the shared lifecycle state; the bank, custodian, and sponsor remain the regulated parties. Off-list integrations (price oracle, card processor, refiner) stay simulated/commercial context and are not part of the funded Integration-Track scope.

## Appendix A: SDF building blocks the build reuses

Argent assembles audited, SDF-maintained, Apache-2.0 primitives rather than hand-rolling infrastructure. This is both faster and a stronger engineering signal for the accelerator: the novel surface is the collateral logic, not the plumbing.

| Building block | What it is | Where Argent uses it |
|---|---|---|
| **`@stellar/typescript-wallet-sdk`** | TS SDK for Stellar wallet apps; SEP-10 auth + SEP-24 anchor flows | `anchor/`, the cardholder funds the settlement asset to repay with (the exact SDK MoneyGram's integration guide prescribes) |
| **`@stellar/stellar-sdk` (js-stellar-sdk)** | Main JS/TS client library | `chain/`, build/sign/submit Soroban tx, read contract state |
| **`stellar-cli` + Scaffold Stellar** | Rust CLI + SDF scaffolding tool | Phase 0, stand up the contract workspace + frontend skeleton |
| **`rs-soroban-sdk`** | Rust SDK for Soroban contracts | the contract suite (`CreditLedger`, `SettlementVault`, …) |
| **OpenZeppelin Contracts for Soroban** | Audited RBAC / token / vault primitives | `AccessControl`; token interactions on the SettlementVault |
| **SDF Anchor Platform** | Production service implementing SEP-10/12/24 etc. | pilot-stage funding rail (replaces the demo's stubbed funding) |
| **`stellar-rpc` + Stellar Lab** | Soroban RPC + in-browser tx/contract tooling | `indexer/` event subscription; manual testnet ops |
| **`wallet-backend`** (reference) | SDF-maintained backend for Stellar wallet apps | reference implementation for the `chain/` + `indexer/` service shape |
| **USDC testnet** (`GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5`) + Circle faucet | Test settlement asset | the repayment-DvP leg on testnet, no self-issued token needed |

Adjacency worth flagging (not a build dependency): SDF now ships **`x402-stellar`** and **`stellar-mpp-sdk`** for HTTP-native / machine-payment settlement on Stellar, a natural "where this goes next" vector if Argent's credit primitive is ever consumed by agentic spenders.

---

## Appendix B: The partner-decomposition pattern (MoneyGram / MGUSD as precedent)

Argent's posture, own the product and orchestration, outsource every regulated function to the licensed party, is not novel; it is exactly how MoneyGram structured MGUSD on Stellar (June 2026). Presenting Argent in this template borrows credibility a Stellar reviewer recognizes instantly:

| Role | MGUSD party | Argent equivalent |
|---|---|---|
| Brand / network owner | MoneyGram | **Argent** (orchestration; lends/custodies/issues nothing) |
| Regulated issuer | Bridge (Stripe) | **Partner bank** (extends credit, issues the card) |
| Smart-contract supply/state layer | M0 | **Argent's Soroban suite** (`CreditLedger` + `SettlementVault`) |
| Custody / float | Fireblocks | **Gold custodian** (bars) + digital-asset custodian (settlement float) |
| Settlement chain | Stellar | **Stellar** |

The narrative arc also transfers: MoneyGram ran on Circle's USDC before graduating to its own issuance. Argent starts on USDC for the settlement leg and can graduate later to a purpose-built settlement asset via the Bridge/M0 issuance-as-a-service path, a Phase-7 expansion vector, not MVP. (Borrow MoneyGram's *structure*, not its remittance *market*: Argent's user owns allocated gold bars, not a cash-kiosk balance.)

---

*Naming: this is **Argent**. The earlier "VaultLine" working name is retired. Repo, Railway service, and the application all read Argent. Aurum stays untouched for Axelra (Canton/Daml).*

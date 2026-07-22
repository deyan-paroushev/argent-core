# Argent Core

**Gold-backed obligation infrastructure on Stellar.**

Argent is a bank-in-the-loop collateral-control protocol. It is designed to let a bank use customer-owned allocated bullion as controlled reserve behind purpose-bound guarantees and other approved obligations. The gold stays in approved custody. The bank underwrites and issues. The company remains liable. Argent coordinates the authorized capacity state.

> One reserve. Many obligations. One authoritative capacity state.

The code currently proves a transparent secured-credit reference on Stellar testnet. The confidential obligation-facility profile is the next product build; it is not represented as deployed.

## Start here

These five documents are the public front door:

| Document | Question it answers |
|---|---|
| [PRODUCT.md](PRODUCT.md) | Who is Argent for, why does it exist, and how does it help? |
| [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) | What is built, what is not, and what are the three delivery gates? |
| [PILOT_PROFILE.md](PILOT_PROFILE.md) | What exactly should the first bank–custodian–company pilot test? |
| [SECURITY_AND_PRIVACY.md](SECURITY_AND_PRIVACY.md) | How can Soroban provide shared integrity without exposing commercial state? |
| [LEGAL_PILOT_CHECKLIST.md](LEGAL_PILOT_CHECKLIST.md) | Which legal conclusions and documents must exist before live collateral is used? |

Detailed prior specifications and design research remain in [docs/reference](docs/reference/README.md). They provide useful depth but do not override the five documents above.

## Product boundary

Argent is:

- a control and evidence layer for bank-approved use of allocated bullion;
- a capacity register for purpose-bound obligations;
- a governed workflow among the company, bank, custodian, and approved evidence providers;
- an integrity layer that can anchor minimized state on Soroban.

Argent is not:

- a bank, guarantor, custodian, surety, insurer, broker, or legal adviser;
- tokenized gold or a transferable capacity token;
- an unsecured credit-underwriting system;
- a public database of bars, customers, beneficiaries, limits, or obligations;
- a mechanism that creates or perfects a security interest merely by recording state on-chain.

## Verify the implementation

```bash
git clone https://github.com/deyan-paroushev/argent-core
cd argent-core
cargo test --manifest-path contracts/Cargo.toml
python3 scripts/check_docs.py
```

Current contract surface: 224 tests across `credit_ledger`, `settlement_vault`, and the non-core `rewards_ledger` reference. The collateral-control and settlement contracts account for 179 of those tests.

The [live demonstrator](https://argent-production-4a3f.up.railway.app) uses synthetic Stellar testnet data.

## Repository map

- `contracts/credit_ledger/` — transparent collateral and secured-exposure reference.
- `contracts/settlement_vault/` — atomic settlement-asset repayment reference.
- `contracts/rewards_ledger/` — historical optional rewards reference; not part of the obligation product.
- `docs/reference/` — detailed architecture, prior specifications, operations, and research.
- `scripts/check_docs.py` — documentation conformance checks against contract source.

## Current public claim

Argent has implemented and tested shared collateral-control primitives. It has not yet deployed a confidential bank-guarantee facility, signed a production bank or custodian pilot, created a legally effective pledge on-chain, or issued a bank instrument.

Apache-2.0 — see [LICENSE](LICENSE).

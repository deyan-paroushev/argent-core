# Deployment and Operations Runbook

> **Implementation scope:** This runbook deploys and exercises the current secured-credit reference contracts. It does not deploy the target typed-obligation extension. The institutional signing and evidence work remains reusable across both profiles.

**Argent Core V5. Soroban testnet and mainnet deployment runbook.**

Status: operational runbook. This document is written for a reviewer, operator, or future maintainer who needs to build, deploy, initialize, verify and exercise the Argent Core contracts.

Argent records signed control state around physical collateral. A deployment is not complete when the contracts are deployed. It is complete only when the contracts are initialized, peer bindings are verified, roles are approved, a reference lifecycle runs end to end, and the evidence pack is regenerated against the deployed contract IDs.

---

## 1. Contract workspace

Current contracts:

| Crate | Purpose |
|---|---|
| `credit_ledger` | Core collateral-control lifecycle and event stream. |
| `settlement_vault` | Settlement-asset repayment bound to exposure reduction. |
| `rewards_ledger` | Optional rewards overlay, separate from pledged collateral. |

The settlement vault imports the compiled `credit_ledger.wasm`, so build `credit_ledger` to WASM before running the full workspace tests.

---

## 2. Required tools

Install and record versions before every release deployment.

```bash
rustc --version
cargo --version
stellar --version
rustup target add wasm32v1-none
```

Expected project baseline:

```text
soroban-sdk = 23.5.3
network = testnet for current prototype
```

For mainnet, repeat the same procedure with mainnet RPC, funded accounts, production signers and a reviewed deployment package.

---

## 3. Environment variables

Use environment variables or a non-committed `.env` file. Never commit secrets.

```bash
export STELLAR_NETWORK=testnet
export STELLAR_RPC_URL="https://soroban-testnet.stellar.org"
export DEPLOYER_SECRET="..."
export ADMIN_ADDRESS="..."
export SETTLEMENT_TOKEN_ID="..."
export BANK_ADDRESS="..."
export CUSTODIAN_ADDRESS="..."
export PROCESSOR_ADDRESS="..."
export VALUER_ADDRESS="..."
export OWNER_ADDRESS="..."
```

For the DFNS-governed build (the current build target), replace local private-key signers with DFNS role wallets. The contract-level addresses remain ordinary Stellar addresses; the signing path changes.

---

## 4. Clean build

Run from the repository root or `contracts/` workspace.

```bash
cd contracts
cargo clean
cargo build --target wasm32v1-none --release -p credit_ledger
cargo build --target wasm32v1-none --release -p settlement_vault
cargo build --target wasm32v1-none --release -p rewards_ledger
```

Optimize WASM artifacts:

```bash
stellar contract optimize \
  --wasm target/wasm32v1-none/release/credit_ledger.wasm

stellar contract optimize \
  --wasm target/wasm32v1-none/release/settlement_vault.wasm

stellar contract optimize \
  --wasm target/wasm32v1-none/release/rewards_ledger.wasm
```

Record hashes:

```bash
shasum -a 256 target/wasm32v1-none/release/*.optimized.wasm
```

Check size before deployment:

```bash
ls -lh target/wasm32v1-none/release/*.optimized.wasm
```

If any optimized WASM exceeds the active Soroban contract-size limit, do not deploy. Split the contract surface before minting.

---

## 5. Test before deployment

Run the full workspace suite:

```bash
cd contracts
cargo build --target wasm32v1-none --release -p credit_ledger
cargo test --workspace
```

Expected baseline:

```text
credit_ledger:    162 passed
rewards_ledger:    45 passed
settlement_vault:  17 passed
Total:            224 passed, 0 failed
```

Save the output:

```bash
cargo test --workspace 2>&1 | tee ../docs/test-summary.txt
```

Do not deploy a commit that does not pass the full suite.

---

## 6. Deploy order

Deploy all contracts first, then initialize. Initialization requires peer contract IDs.

```bash
# Example shape only. Replace with the exact CLI syntax used in the deployment environment.
stellar contract deploy \
  --wasm target/wasm32v1-none/release/credit_ledger.optimized.wasm \
  --source $DEPLOYER_SECRET \
  --network $STELLAR_NETWORK

stellar contract deploy \
  --wasm target/wasm32v1-none/release/settlement_vault.optimized.wasm \
  --source $DEPLOYER_SECRET \
  --network $STELLAR_NETWORK

stellar contract deploy \
  --wasm target/wasm32v1-none/release/rewards_ledger.optimized.wasm \
  --source $DEPLOYER_SECRET \
  --network $STELLAR_NETWORK
```

Record:

```bash
export CREDIT_LEDGER_ID="C..."
export SETTLEMENT_VAULT_ID="C..."
export REWARDS_LEDGER_ID="C..."
```

Sanity check: all contract IDs should be contract addresses beginning with `C`.

---

## 7. Initialization order

Initialize in an order that satisfies peer bindings.

### 7.1 Initialize settlement vault

The settlement vault must know the admin, settlement token and credit ledger.

```bash
stellar contract invoke \
  --id $SETTLEMENT_VAULT_ID \
  --source $DEPLOYER_SECRET \
  --network $STELLAR_NETWORK \
  -- initialize \
  --admin $ADMIN_ADDRESS \
  --settlement_token $SETTLEMENT_TOKEN_ID \
  --credit_ledger $CREDIT_LEDGER_ID
```

### 7.2 Initialize credit ledger

The credit ledger must know the admin and settlement vault.

```bash
stellar contract invoke \
  --id $CREDIT_LEDGER_ID \
  --source $DEPLOYER_SECRET \
  --network $STELLAR_NETWORK \
  -- initialize \
  --admin $ADMIN_ADDRESS \
  --settlement_vault $SETTLEMENT_VAULT_ID
```

### 7.3 Initialize rewards ledger

Use only if the rewards overlay is part of this deployment.

```bash
stellar contract invoke \
  --id $REWARDS_LEDGER_ID \
  --source $DEPLOYER_SECRET \
  --network $STELLAR_NETWORK \
  -- initialize \
  --admin $ADMIN_ADDRESS
```

---

## 8. Post-initialization verification

Verify by getter before any business lifecycle action.

```bash
stellar contract invoke --id $CREDIT_LEDGER_ID --network $STELLAR_NETWORK -- get_admin
stellar contract invoke --id $CREDIT_LEDGER_ID --network $STELLAR_NETWORK -- get_settlement_vault
stellar contract invoke --id $SETTLEMENT_VAULT_ID --network $STELLAR_NETWORK -- get_credit_ledger
stellar contract invoke --id $SETTLEMENT_VAULT_ID --network $STELLAR_NETWORK -- get_settlement_token
```

Acceptance criteria:

- `credit_ledger.get_settlement_vault() == SETTLEMENT_VAULT_ID`
- `settlement_vault.get_credit_ledger() == CREDIT_LEDGER_ID`
- `settlement_vault.get_settlement_token() == SETTLEMENT_TOKEN_ID`
- admin address is the intended admin;
- no initialization call swallowed an error.

---

## 9. Role approvals

Approve only the roles required for the reference lifecycle.

Typical testnet setup:

```bash
# bank
stellar contract invoke --id $CREDIT_LEDGER_ID --source $DEPLOYER_SECRET --network $STELLAR_NETWORK -- approve_party --party $BANK_ADDRESS --role Bank

# custodian
stellar contract invoke --id $CREDIT_LEDGER_ID --source $DEPLOYER_SECRET --network $STELLAR_NETWORK -- approve_party --party $CUSTODIAN_ADDRESS --role Custodian

# processor / verifier
stellar contract invoke --id $CREDIT_LEDGER_ID --source $DEPLOYER_SECRET --network $STELLAR_NETWORK -- approve_party --party $PROCESSOR_ADDRESS --role Processor

# valuation source
stellar contract invoke --id $CREDIT_LEDGER_ID --source $DEPLOYER_SECRET --network $STELLAR_NETWORK -- approve_party --party $VALUER_ADDRESS --role Valuation

# settlement vault role, if the current contract model requires an explicit vault approval in addition to binding
stellar contract invoke --id $CREDIT_LEDGER_ID --source $DEPLOYER_SECRET --network $STELLAR_NETWORK -- approve_party --party $SETTLEMENT_VAULT_ID --role Vault
```

Verify:

```bash
stellar contract invoke --id $CREDIT_LEDGER_ID --network $STELLAR_NETWORK -- is_approved --party $BANK_ADDRESS --role Bank
stellar contract invoke --id $CREDIT_LEDGER_ID --network $STELLAR_NETWORK -- is_approved --party $CUSTODIAN_ADDRESS --role Custodian
```

Do not proceed unless every governed role returns `true`.

---

## 10. Reference lifecycle order

A fresh reference lifecycle should be regenerated after every new deployment.

Recommended order:

1. `register_instrument`
2. `register_framework`
3. `admit_instrument`
4. `register_position` with `LotEvidence`
5. `select_lot_for_collateral`
6. `confirm_and_immobilize`
7. `activate_pledge`
8. `open_credit_line` with valuation reference
9. `record_drawdown`
10. `settle_repayment` through `settlement_vault`
11. `bank_authorize_release`
12. `custodian_confirm_release`

Capture for every step:

- tx hash;
- ledger sequence;
- signer/role;
- emitted `CollateralEventV1` or `GovernanceEventV1`;
- relevant state getter output after the transaction.

Update `docs/evidence-pack-index.md` after the run.

---

## 11. Service update order

Do **not** point the live service at new contract IDs until the reference lifecycle succeeds.

Safe order:

1. Deploy contracts.
2. Initialize contracts.
3. Verify peer bindings and role approvals.
4. Run reference lifecycle through CLI/script.
5. Regenerate evidence pack and certificates.
6. Update service environment variables in a staging environment.
7. Run the service lifecycle against the new deployment.
8. Verify UI, certificates and event indexer show the new IDs.
9. Switch public/live environment.

Failure mode if done out of order:

- stale ABI calls fail;
- lifecycle scripts point to old method names or old argument shapes;
- certificates reference old contract IDs;
- indexer filters the wrong contract;
- demo appears broken even though contracts are correct.

---

## 12. Ambiguous transaction status

If a transaction is submitted but not confirmed before timeout:

1. Do not blindly resubmit the same logical write.
2. Search by transaction hash in RPC/explorer.
3. Query contract state using getters.
4. If the state changed, record the tx hash and continue.
5. If the state did not change and no successful transaction exists, rebuild from current sequence/account state and resubmit.

This is especially important for non-idempotent writes such as registration, drawdown, release and enforcement.

---

## 13. Failure recovery guide

| Symptom | Likely cause | Recovery |
|---|---|---|
| `NotInitialized` | Contract deployed but not initialized, or wrong contract ID. | Run getter checks; initialize correct contract. |
| `PartyNotApproved` | Role not approved or revoked. | Approve role, verify with `is_approved`, rerun action. |
| `InstrumentNotEligible` | Instrument not registered, retired, or not admitted to framework. | Register/admit instrument and verify eligibility record. |
| `InvalidRiskParams` | Haircut, LTV, maintenance, price or limit violates treatment. | Check framework treatment and input scaling. |
| `InvalidDocumentHash` | Required evidence/valuation hash is zero or missing. | Regenerate non-zero evidence hash. |
| Settlement repayment fails | Wrong vault, wrong token, insufficient balance, duplicate payment ref. | Verify vault binding, token balance, payment ref and line state. |
| Release fails | Outstanding balance, missing bank authorization, wrong state. | Query line/pledge/position state before retry. |
| Indexer shows no events | Event filter points to old contract ID or wrong event family. | Update contract IDs and decoder configuration. |

---

## 14. Mainnet gates

Do not launch to mainnet until all gates are satisfied:

- [ ] Fresh deployment from tagged commit.
- [ ] Full test suite passes.
- [ ] Optimized WASM hashes recorded.
- [ ] Peer bindings verified by getters.
- [ ] DFNS role-wallet topology tested, if DFNS is in scope.
- [ ] Reference lifecycle run end to end on target network.
- [ ] Evidence pack regenerated.
- [ ] Certificates regenerated.
- [ ] No secrets in repository or artifacts.
- [ ] Legal/custody/valuation boundary reviewed.
- [ ] Incident response owner identified.

---

## 15. Public evidence update

After deployment, update:

- `docs/evidence-pack-index.md`
- `docs/REVIEWER_QUICKSTART.md`
- live demo environment variables;
- certificate generator contract IDs;
- application/deck transaction examples;
- any README contract-ID table.

The public repo, live service and evidence certificate must all point to the same deployment.

---

## 16. Target reservation and deliverability operations

> **Target profile, not current contract behavior.** These procedures apply when `MasterFacility`, typed obligations, and pre-issuance reservations are implemented.

A production operator should monitor the following queue states:

```text
REQUESTED
PRECHECKED
PROVISIONALLY_RESERVED
BANK_APPROVED
COMMITTED
ISSUE_SUBMITTED
ISSUED
REJECTED
EXPIRED
CANCELLED
RECONCILIATION_REQUIRED
```

For each request, preserve:

- originating-system request ID;
- idempotency key and canonical request digest;
- facility, applicant, product, beneficiary, amount, and expiry;
- reservation ID and reservation expiry;
- policy and evidence version;
- DFNS approval reference;
- Soroban transaction and event reference;
- bank product-system reference;
- current reason code and next action.

### 16.1 Reservation expiry

A provisional reservation may expire only under the configured bank policy. Before release:

1. confirm that no instrument was issued;
2. query the authoritative product system if issue status is uncertain;
3. record the expiry or cancellation reason;
4. release capacity atomically;
5. notify the originating system;
6. include the event in reconciliation output.

Committed capacity must not be released merely because a callback or client connection timed out.

### 16.2 Duplicate commands and callbacks

- reuse of an idempotency key with the same request returns the existing result;
- reuse of the key with different request data is rejected;
- duplicate callbacks do not create duplicate obligations or payments;
- out-of-order callbacks are held or rejected according to lifecycle version;
- manual replay must preserve the original correlation identifiers.

### 16.3 Ambiguous issue status

If the product-system request may have succeeded but no definitive response was received:

```text
keep reservation committed
-> mark RECONCILIATION_REQUIRED
-> query authoritative product system
-> prohibit blind resubmission
-> prohibit release
-> record definitive issued or rejected outcome
```

## 17. External-system reconciliation

Production reconciliation should compare:

- upstream reserve assurance scope, expiry, tolerance status, and authoritative record;
- bank facility and product system;
- custodian or vault control book;
- DFNS approval state;
- Soroban contract state and event archive;
- settlement or reimbursement system;
- evidence-package index.

The daily or intraday report should show:

- unmatched requests and callbacks;
- reservation and obligation status differences;
- stale custody, valuation, policy, or document state;
- duplicated or missing events;
- age of oldest unresolved exception;
- assigned owner and escalation deadline.

A disagreement must be visible. Argent must not silently overwrite the bank or custodian record.

### 17.1 Shared-gold assurance discrepancy procedure

When a facility depends on an external gold platform or product operator, the operator should monitor at least:

```text
RESERVE_DATA_STALE
SOURCE_ASSURANCE_EXPIRED
SOURCE_TOLERANCE_BREACHED
CUSTODY_MISMATCH
OWNERSHIP_OR_ENTITLEMENT_MISMATCH
ENCUMBRANCE_MISMATCH
BACKING_MISMATCH
EXTERNAL_STATUS_UNKNOWN
```

Required response:

1. stop new reservations and issuance;
2. preserve active obligations, reservations, and encumbrances;
3. block release unless the bank and authoritative reserve source confirm the release condition;
4. query the source and custodian using stable correlation identifiers;
5. apply margin, cure, substitution, or enforcement procedures under bank instruction;
6. record the definitive resolution as a new event and evidence item.

Do not automatically delete or free capacity because an upstream assertion disappeared. Loss of evidence is an exception state, not proof that the collateral or obligation ceased to exist.

## 18. Privacy and evidence operations

Apply the data classification and role-view rules in [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md) and the production public/private boundary in [`confidential-control-and-public-integrity.md`](confidential-control-and-public-integrity.md).

The current contracts are a transparent reference profile. They must use synthetic data. A mainnet deployment of those contracts does not become confidential merely because documents are replaced by hashes.

Operational requirements include:

- no raw bar list, KYC file, complete legal agreement, or beneficiary document in public contract state;
- encrypted evidence storage with tenant and role separation;
- purpose-bound access grants and access logging;
- approved redaction or derived statement before external disclosure;
- retention and deletion schedule by evidence class;
- incident response for unauthorized access or public metadata leakage;
- separate keys and environments for development, test, and production;
- recovery plan for evidence-encryption keys and signing-service credentials.
- versioned canonical bar-identity and evidence-encoding profile;
- custodian nullifier service inside an HSM or equivalent controlled boundary;
- nullifier-domain inventory, key continuity, rotation, migration, recovery, and compromise runbook;
- private transition-envelope archive and deterministic public-payload derivation;
- separate state and nullifier-set root reconciliation;
- common relay, fixed-cadence scheduler, uniform event schema, padding, quiet-period, retry, and fee-funding procedures;
- automated inspection of public arguments, storage, events, authorization entries, returns, errors, diagnostic logs, identifiers, accounts, timing, and batch sizes.

A hash is an integrity commitment. It is not permission to publish the underlying data or a guarantee that the committed value cannot be guessed.

A randomly salted evidence commitment is also not a uniqueness control. Duplicate-allocation refusal requires the separate deterministic custodian nullifier defined by the governed asset-identity profile.

## 19. Operational service levels

A production design partner should define measurable targets for:

- preflight response time;
- reservation creation and expiry processing;
- DFNS approval timeout;
- product-system issue callback;
- event indexing and reconciliation lag;
- stale valuation and custody alerts;
- evidence retrieval;
- manual exception acknowledgement and resolution;
- recovery-point and recovery-time objectives.

Ledger availability does not imply that the bank, custodian, document examiner, or settlement rail operates continuously. Service levels must reflect the real operating windows of each authority.

## 20. Target-profile go-live checklist

- [ ] authoritative system identified for every state and document;
- [ ] preflight reason codes tested;
- [ ] provisional and committed reservation paths tested;
- [ ] reservation expiry and cancellation tested;
- [ ] duplicate request and callback behavior tested;
- [ ] ambiguous issue status tested without blind resubmission;
- [ ] product-system and Soroban reconciliation tested;
- [ ] upstream assurance scope, expiry, tolerance breach, and discrepancy paths tested where an external gold source is used;
- [ ] source outage does not release active capacity or collateral;
- [ ] role-specific views and evidence permissions tested;
- [ ] transparent reference contracts contain synthetic data only;
- [ ] canonical identity implementations reproduce the published test vectors;
- [ ] alternate evidence salts still produce the same custodian nullifier;
- [ ] nullifier derivation excludes owner, bank, facility, obligation, and transaction fields;
- [ ] nullifier key rotation or migration preserves every active lock;
- [ ] private transition and public anchor roots reconcile deterministically;
- [ ] epoch replay, rollback, skipped root, duplicate batch, relay retry, and relay compromise tested;
- [ ] public-surface and observer-simulation review covers identities, amounts, action types, graph, timing, and volume;
- [ ] privacy and metadata-leakage review completed;
- [ ] manual exception and disaster-recovery procedures rehearsed;
- [ ] monitoring, alerting, escalation, and evidence export approved.

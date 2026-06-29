#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    token::{StellarAssetClient, TokenClient},
    Address, BytesN, Env, Symbol,
};

// The credit_ledger client + types come from the contractimport! in lib.rs.
// We register its WASM in-test to exercise the real cross-contract path.
use crate::credit_ledger;

fn id(env: &Env) -> BytesN<32> {
    BytesN::random(env)
}

fn setup_drawn_line(
    env: &Env,
) -> (
    credit_ledger::Client<'static>,
    Address,    // ledger_id
    BytesN<32>, // pledge_id
    BytesN<32>, // line_id
    Address,    // bank
    Address,    // cardholder (== owner; the cardholder must be the pledgor)
    Address,    // vault_id (bound to this ledger at initialize)
) {
    let ledger_id = env.register(credit_ledger::WASM, ());
    let ledger = credit_ledger::Client::new(env, &ledger_id);

    let admin = Address::generate(env);
    let owner = Address::generate(env);
    let custodian = Address::generate(env);
    let bank = Address::generate(env);
    let processor = Address::generate(env);
    // The cardholder (borrower) must be the pledgor: owner == cardholder.
    let cardholder = owner.clone();

    // The settlement vault is bound at initialize (it only needs to be deployed,
    // not initialized, for binding). Deploy it first so we can pass it to init.
    let vault_id = env.register(SettlementVault, ());

    ledger.initialize(&admin, &vault_id);
    ledger.approve_party(&custodian, &credit_ledger::Role::Custodian);
    ledger.approve_party(&bank, &credit_ledger::Role::Bank);
    ledger.approve_party(&processor, &credit_ledger::Role::Processor);
    ledger.approve_party(&vault_id, &credit_ledger::Role::Vault);

    // tri-party control framework first
    let framework_id = id(env);
    ledger.register_framework(
        &framework_id, &owner, &bank, &custodian,
        &id(env), &id(env), &id(env), &id(env), &id(env), &id(env),
    );

    let instrument = credit_ledger::InstrumentKey {
        issuer: owner.clone(),
        depository: custodian.clone(),
        id: Symbol::new(env, "XAU_LGD"),
        version: 1,
    };
    ledger.register_instrument(&credit_ledger::Instrument {
        key: instrument.clone(),
        commodity: Symbol::new(env, "gold"),
        unit: Symbol::new(env, "oz"),
        grade_hash: id(env),
        status: credit_ledger::InstrumentStatus::Active,
    });
    ledger.admit_instrument(
        &framework_id, &instrument, &bank, &custodian,
        &id(env), &0u32, &9000u32, &9500u32,
    );

    let position_id = id(env);
    let pledge_id = id(env);
    let line_id = id(env);
    let expiry = env.ledger().sequence() + 100_000;
    ledger.register_position(
        &position_id,
        &framework_id,
        &owner,
        &custodian,
        &instrument,
        &credit_ledger::LotEvidence {
            manifest_hash: id(env),
            uniqueness_hash: id(env),
            quality_cert_hash: id(env),
            quantity_cert_hash: id(env),
            location_hash: id(env),
        },
        &4_011_000_000i128,
        &expiry,
    );
    // owner selects, custodian immobilizes, bank pledges
    ledger.select_lot_for_collateral(&position_id, &owner, &id(env));
    ledger.confirm_and_immobilize(&position_id, &custodian, &id(env));
    ledger.activate_pledge(&pledge_id, &position_id, &owner, &bank, &id(env));
    ledger.open_credit_line(
        &line_id, &pledge_id, &bank, &cardholder,
        &719_000i128, &6000u32, &7500u32, &29_900_000_000i128, &id(env),
    );
    ledger.record_drawdown(&line_id, &processor, &id(env), &25_000i128);

    (ledger, ledger_id, pledge_id, line_id, bank, cardholder, vault_id)
}

#[test]
fn settle_repayment_moves_token_and_reduces_debt() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();

    // SAC test asset (USDC-like settlement token)
    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let token_addr = sac.address();
    let token = TokenClient::new(&env, &token_addr);
    let token_admin = StellarAssetClient::new(&env, &token_addr);

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder, vault_id) = setup_drawn_line(&env);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);

    // deploy SettlementVault, bind token + ledger, approve as Vault
    // setup_drawn_line already deployed this vault and bound it to the ledger at
    // init; initialize it with the settlement token for the transfer.
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&Address::generate(&env), &token_addr, &ledger_id);

    token_admin.mint(&cardholder, &25_000i128);
    assert_eq!(token.balance(&cardholder), 25_000);
    assert_eq!(token.balance(&bank), 0);

    // the headline transaction: settle the repayment (no release; that is a
    // separate bank-authorized, custodian-confirmed act on the ledger)
    vault.settle_repayment(&line_id, &cardholder, &bank, &id(&env), &25_000i128);

    assert_eq!(token.balance(&cardholder), 0);
    assert_eq!(token.balance(&bank), 25_000);

    // debt cleared, but the line stays Active and the collateral is NOT released
    let line = ledger.get_line(&line_id);
    assert_eq!(line.drawn_balance, 0);
    assert_eq!(line.status, credit_ledger::LineStatus::Active);
}

#[test]
fn partial_settlement_reduces_debt() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let token_addr = sac.address();
    let token_admin = StellarAssetClient::new(&env, &token_addr);

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder, vault_id) = setup_drawn_line(&env);

    // setup_drawn_line already deployed this vault and bound it to the ledger at
    // init; initialize it with the settlement token for the transfer.
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&Address::generate(&env), &token_addr, &ledger_id);
    token_admin.mint(&cardholder, &10_000i128);

    // repay only 10,000 of 25,000
    vault.settle_repayment(&line_id, &cardholder, &bank, &id(&env), &10_000i128);

    let line = ledger.get_line(&line_id);
    assert_eq!(line.drawn_balance, 15_000);
    assert_eq!(line.status, credit_ledger::LineStatus::Active);
}

#[test]
fn settlement_is_idempotent_on_payment_ref() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let token_addr = sac.address();
    let token_admin = StellarAssetClient::new(&env, &token_addr);

    let (_ledger, ledger_id, _pledge_id, line_id, bank, cardholder, vault_id) = setup_drawn_line(&env);

    // setup_drawn_line already deployed this vault and bound it to the ledger at
    // init; initialize it with the settlement token for the transfer.
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&Address::generate(&env), &token_addr, &ledger_id);
    token_admin.mint(&cardholder, &20_000i128);

    let pay = id(&env);
    vault.settle_repayment(&line_id, &cardholder, &bank, &pay, &10_000i128);
    // same payment_ref again must fail on the ledger's idempotency guard
    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &pay, &10_000i128);
    assert!(res.is_err());
}

// ===========================================================================
// ADVERSARIAL TEST SUITE. settlement_vault
//
// settlement_vault is the only contract that MOVES TOKENS, so it gets the most
// adversarial attention. It performs a two-step atomic action: (1) SEP-41
// transfer cardholder -> bank, then (2) cross-contract apply_repayment on the
// credit ledger. Its headline safety claim is "if the token transfer fails,
// nothing commits" and "only a bound, approved vault can reduce the debt."
// These tests stress exactly those claims, grouped by risk class:
//
//   A. Atomicity: a FAILED token transfer (insufficient balance) must leave the
//      debt unchanged. This is the contract's central promise and had no test.
//   B. Initialization guards: double-init refused; settle-before-init refused.
//   C. Host-level authorization: only the cardholder can authorize moving their
//      own tokens.
//   D. Approval coupling: an UNAPPROVED vault cannot reduce the debt, and the
//      whole tx (including the token move) reverts.
//   E. Boundary / accounting: over-settlement behavior is DOCUMENTED (the bank
//      receives the full amount while the debt clamps to drawn_balance);
//      non-positive amounts refused; idempotency already covered upstream.
// ===========================================================================

use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use soroban_sdk::IntoVal;
// NOTE: StellarAssetClient and TokenClient are already imported at the top of
// this file (the existing test module), so they are NOT re-imported here.

// Build a vault bound to a fresh SAC token and a drawn credit line, approved as
// a Vault on the ledger. Returns everything a test needs to attack it.
fn setup_vault(
    env: &Env,
) -> (
    credit_ledger::Client<'static>,
    SettlementVaultClient<'static>,
    TokenClient<'static>,
    StellarAssetClient<'static>,
    BytesN<32>, // line_id
    Address,    // bank
    Address,    // cardholder
    Address,    // vault_id
) {
    let issuer = Address::generate(env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let token_addr = sac.address();
    let token = TokenClient::new(env, &token_addr);
    let token_admin = StellarAssetClient::new(env, &token_addr);

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder, vault_id) = setup_drawn_line(env);

    // setup_drawn_line already bound this vault to the ledger at init; initialize
    // it with the settlement token.
    let vault = SettlementVaultClient::new(env, &vault_id);
    vault.initialize(&Address::generate(&env), &token_addr, &ledger_id);

    (ledger, vault, token, token_admin, line_id, bank, cardholder, vault_id)
}

// ---- A. Atomicity: failed transfer must not reduce debt ----------------------

#[test]
fn insufficient_balance_leaves_debt_unchanged() {
    // THE HEADLINE CLAIM. The cardholder is minted LESS than they try to repay.
    // The SEP-41 transfer (step 1) must fail, and because it precedes the debt
    // reduction (step 2), the drawn balance must be untouched and no tokens move.
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (ledger, vault, token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);

    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);
    token_admin.mint(&cardholder, &5_000i128); // only 5k, but will try 25k

    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &25_000i128);
    assert!(res.is_err(), "settle must fail when the cardholder cannot fund the transfer");

    // Nothing moved, nothing reduced: full atomicity.
    assert_eq!(token.balance(&cardholder), 5_000, "tokens must not move on a failed settle");
    assert_eq!(token.balance(&bank), 0);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000, "debt must be unchanged");
}

// ---- B. Initialization guards -----------------------------------------------

#[test]
fn double_initialize_is_refused() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let token_addr = sac.address();
    let (_ledger, ledger_id, _p, _l, _b, _c, _vault_id) = setup_drawn_line(&env);

    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&Address::generate(&env), &token_addr, &ledger_id);
    let res = vault.try_initialize(&Address::generate(&env), &token_addr, &ledger_id);
    assert_eq!(res, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn settle_before_initialize_is_refused() {
    // A vault that was never initialized has no token or ledger binding, so a
    // settle attempt must fail with NotInitialized rather than moving anything.
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (_ledger, _ledger_id, _p, line_id, bank, cardholder, _vault_id) = setup_drawn_line(&env);

    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    // NOTE: no vault.initialize(...) call.
    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &1_000i128);
    assert_eq!(res, Err(Ok(Error::NotInitialized)));
}

// ---- C. Host-level authorization --------------------------------------------

#[test]
fn host_auth_only_cardholder_can_authorize_transfer() {
    // settle_repayment calls cardholder.require_auth(). Prove that a transaction
    // signed by someone OTHER than the cardholder cannot drain the cardholder's
    // tokens, even with the cardholder's address passed as the arg.
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (_ledger, vault, _token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);
    token_admin.mint(&cardholder, &25_000i128);

    let attacker = Address::generate(&env);
    let res = vault
        .mock_auths(&[MockAuth {
            address: &attacker,
            invoke: &MockAuthInvoke {
                contract: &vault.address,
                fn_name: "settle_repayment",
                args: (
                    line_id.clone(),
                    cardholder.clone(),
                    bank.clone(),
                    id(&env),
                    25_000i128,
                )
                    .into_val(&env),
                sub_invokes: &[],
            },
        }])
        .try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &25_000i128);
    assert!(res.is_err(), "only the cardholder may authorize moving their tokens");
}

// ---- D. Approval coupling: unapproved vault cannot reduce debt ---------------

#[test]
fn unapproved_vault_cannot_settle_and_reverts_cleanly() {
    // A vault NOT approved as Role::Vault on the ledger must fail at
    // apply_repayment (step 2). Because step 2 fails, the whole transaction
    // reverts, so the token transfer (step 1) must roll back too: no tokens move
    // and no debt changes. This proves the cross-contract approval gate protects
    // the bank even if a rogue vault is deployed and bound to the token.
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let token_addr = sac.address();
    let token = TokenClient::new(&env, &token_addr);
    let token_admin = StellarAssetClient::new(&env, &token_addr);

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder, _vault_id) = setup_drawn_line(&env);

    // deploy a vault and bind it, but DO NOT approve it on the ledger
    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&Address::generate(&env), &token_addr, &ledger_id);
    // (intentionally NO ledger.approve_party(vault_id, Vault))

    token_admin.mint(&cardholder, &25_000i128);

    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &25_000i128);
    assert!(res.is_err(), "an unapproved vault must not be able to settle");

    // Full revert: no tokens moved, debt unchanged.
    assert_eq!(token.balance(&cardholder), 25_000, "tokens must roll back when step 2 fails");
    assert_eq!(token.balance(&bank), 0);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);
}

// ---- E. Boundary / accounting -----------------------------------------------

#[test]
fn settle_nonpositive_amount_refused() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (_ledger, vault, _token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);
    token_admin.mint(&cardholder, &25_000i128);
    assert_eq!(
        vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &0i128),
        Err(Ok(Error::AmountNotPositive))
    );
    assert_eq!(
        vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &(-1_i128)),
        Err(Ok(Error::AmountNotPositive))
    );
}

#[test]
fn over_settlement_documented_bank_receives_full_amount_debt_clamps() {
    // SECURITY POLICY UPDATE. The previous test pinned the old clamping behavior.
    // The hardened contract now refuses amount > drawn_balance before moving any
    // token, so the bank cannot receive an unrecorded overpayment.
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (ledger, vault, token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);

    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);
    token_admin.mint(&cardholder, &40_000i128);

    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &40_000i128);
    assert_eq!(res, Err(Ok(Error::RepaymentExceedsOutstandingBalance)));

    assert_eq!(token.balance(&bank), 0);
    assert_eq!(token.balance(&cardholder), 40_000);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);
}


// ---- F. Repayment pre-checks before any token movement ----------------------

#[test]
fn settlement_refuses_overpayment_above_drawn_balance() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (ledger, vault, token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);

    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);
    token_admin.mint(&cardholder, &30_000i128);

    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &30_000i128);
    assert_eq!(res, Err(Ok(Error::RepaymentExceedsOutstandingBalance)));
    assert_eq!(token.balance(&cardholder), 30_000);
    assert_eq!(token.balance(&bank), 0);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);
}

#[test]
fn settlement_refuses_repayment_on_closed_line() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (ledger, vault, token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);

    token_admin.mint(&cardholder, &26_000i128);
    vault.settle_repayment(&line_id, &cardholder, &bank, &id(&env), &25_000i128);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 0);
    ledger.bank_authorize_release(&line_id, &bank, &id(&env));
    assert_eq!(ledger.get_line(&line_id).status, credit_ledger::LineStatus::Closed);

    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &1i128);
    assert_eq!(res, Err(Ok(Error::LineNotRepayable)));
    assert_eq!(token.balance(&cardholder), 1_000);
    assert_eq!(token.balance(&bank), 25_000);
}

// ---- G. Idempotency rollback with balance assertions ------------------------

#[test]
fn duplicate_payment_ref_does_not_move_tokens_twice() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (ledger, vault, token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);

    token_admin.mint(&cardholder, &25_000i128);
    let pay = id(&env);
    vault.settle_repayment(&line_id, &cardholder, &bank, &pay, &10_000i128);

    assert_eq!(token.balance(&cardholder), 15_000);
    assert_eq!(token.balance(&bank), 10_000);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 15_000);

    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &pay, &10_000i128);
    assert!(res.is_err(), "duplicate payment reference must fail");

    assert_eq!(token.balance(&cardholder), 15_000);
    assert_eq!(token.balance(&bank), 10_000);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 15_000);
}

// ---- H. Snapshot and binding tests for reviewer-grade coverage -------------

#[test]
fn snapshot_partial_settlement_records_exact_balances_and_debt() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (ledger, vault, token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);

    token_admin.mint(&cardholder, &25_000i128);
    vault.settle_repayment(&line_id, &cardholder, &bank, &id(&env), &7_500i128);

    let line = ledger.get_line(&line_id);
    assert_eq!(token.balance(&cardholder), 17_500);
    assert_eq!(token.balance(&bank), 7_500);
    assert_eq!(line.drawn_balance, 17_500);
    assert_eq!(line.available_limit, 701_500);
    assert_eq!(line.status, credit_ledger::LineStatus::Active);
}

#[test]
fn settlement_bound_to_configured_ledger_only() {
    let env = Env::default();
    env.mock_all_auths();
    // The imported credit_ledger WASM is large (unoptimized); lift the test
    // instantiation budget. Production deploys an optimized (wasm-opt) build.
    env.cost_estimate().budget().reset_unlimited();
    let (ledger, _ledger_id, _pledge_id, line_id, bank, cardholder, _vault_id) = setup_drawn_line(&env);
    let (_other_ledger, other_ledger_id, _other_pledge, _other_line, _other_bank, _other_cardholder, _other_vault_id) = setup_drawn_line(&env);

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let token_addr = sac.address();
    let token = TokenClient::new(&env, &token_addr);
    let token_admin = StellarAssetClient::new(&env, &token_addr);
    // A vault that points at a DIFFERENT ledger and is not the vault our ledger was
    // bound to at init. The ledger binds exactly one vault at initialize and accepts
    // repayments only from it, so this stranger vault must not be able to settle this
    // line. (We do not call set_settlement_vault: binding is init-only now, and the
    // ledger is already bound to its own vault.)
    let other_vault_id = env.register(SettlementVault, ());
    let other_vault = SettlementVaultClient::new(&env, &other_vault_id);
    other_vault.initialize(&Address::generate(&env), &token_addr, &other_ledger_id);
    ledger.approve_party(&other_vault_id, &credit_ledger::Role::Vault);

    token_admin.mint(&cardholder, &25_000i128);
    let res = other_vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &10_000i128);
    assert!(res.is_err(), "a vault that is not the ledger's bound vault must not settle this line");
    assert_eq!(token.balance(&cardholder), 25_000);
    assert_eq!(token.balance(&bank), 0);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);
}

// ---- Batch 10: the vault exposes its bindings and refuses getters pre-init ----
// Operators and reviewers must be able to verify what the vault is bound to
// (which ledger, which settlement asset, which admin) without inspecting raw
// storage. These getters make the binding auditable on-chain.
#[test]
fn batch10_vault_getters_return_bound_addresses() {
    let env = Env::default();
    env.mock_all_auths();
    env.cost_estimate().budget().reset_unlimited();

    // A real ledger (for a real address) and a SAC settlement token.
    let ledger_id = env.register(credit_ledger::WASM, ());
    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let token_addr = sac.address();

    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    let admin = Address::generate(&env);
    vault.initialize(&admin, &token_addr, &ledger_id);

    assert_eq!(vault.get_credit_ledger(), ledger_id, "get_credit_ledger returns the bound ledger");
    assert_eq!(vault.get_settlement_token(), token_addr, "get_settlement_token returns the bound asset");
    assert_eq!(vault.get_admin(), admin, "get_admin returns the init admin");
}

#[test]
fn batch10_vault_getters_fail_before_init() {
    let env = Env::default();
    env.mock_all_auths();
    env.cost_estimate().budget().reset_unlimited();

    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);

    // Before initialize, every binding getter reports NotInitialized rather than
    // panicking or returning a junk address.
    assert_eq!(vault.try_get_credit_ledger(), Err(Ok(Error::NotInitialized)));
    assert_eq!(vault.try_get_settlement_token(), Err(Ok(Error::NotInitialized)));
    assert_eq!(vault.try_get_admin(), Err(Ok(Error::NotInitialized)));
}

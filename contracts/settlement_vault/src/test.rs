#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    token::{StellarAssetClient, TokenClient},
    Address, BytesN, Env,
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
    Address,    // cardholder
) {
    let ledger_id = env.register(credit_ledger::WASM, ());
    let ledger = credit_ledger::Client::new(env, &ledger_id);

    let admin = Address::generate(env);
    let owner = Address::generate(env);
    let custodian = Address::generate(env);
    let bank = Address::generate(env);
    let processor = Address::generate(env);
    let cardholder = Address::generate(env);

    ledger.initialize(&admin);
    ledger.approve_party(&custodian, &credit_ledger::Role::Custodian);
    ledger.approve_party(&bank, &credit_ledger::Role::Bank);
    ledger.approve_party(&processor, &credit_ledger::Role::Processor);

    // tri-party control framework first
    let framework_id = id(env);
    ledger.register_framework(
        &framework_id, &owner, &bank, &custodian,
        &id(env), &id(env), &id(env), &id(env), &id(env), &id(env),
    );

    let position_id = id(env);
    let pledge_id = id(env);
    let line_id = id(env);
    let expiry = env.ledger().sequence() + 100_000;
    ledger.register_position(
        &position_id, &framework_id, &owner, &custodian, &id(env), &id(env), &4_011_000_000i128, &expiry,
    );
    // owner selects, custodian immobilizes, bank pledges
    ledger.select_bars_for_collateral(&position_id, &owner, &id(env));
    ledger.confirm_and_immobilize(&position_id, &custodian, &id(env));
    ledger.activate_pledge(&pledge_id, &position_id, &owner, &bank, &id(env));
    ledger.open_credit_line(
        &line_id, &pledge_id, &bank, &cardholder,
        &719_000i128, &6000u32, &7500u32, &29_900_000_000i128,
    );
    ledger.record_drawdown(&line_id, &processor, &id(env), &25_000i128);

    (ledger, ledger_id, pledge_id, line_id, bank, cardholder)
}

#[test]
fn settle_repayment_moves_token_and_reduces_debt() {
    let env = Env::default();
    env.mock_all_auths();

    // SAC test asset (USDC-like settlement token)
    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let token_addr = sac.address();
    let token = TokenClient::new(&env, &token_addr);
    let token_admin = StellarAssetClient::new(&env, &token_addr);

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder) = setup_drawn_line(&env);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 25_000);

    // deploy SettlementVault, bind token + ledger, approve as Vault
    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&token_addr, &ledger_id);
    ledger.approve_party(&vault_id, &credit_ledger::Role::Vault);

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

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let token_addr = sac.address();
    let token_admin = StellarAssetClient::new(&env, &token_addr);

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder) = setup_drawn_line(&env);

    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&token_addr, &ledger_id);
    ledger.approve_party(&vault_id, &credit_ledger::Role::Vault);
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

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let token_addr = sac.address();
    let token_admin = StellarAssetClient::new(&env, &token_addr);

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder) = setup_drawn_line(&env);

    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&token_addr, &ledger_id);
    ledger.approve_party(&vault_id, &credit_ledger::Role::Vault);
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

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder) = setup_drawn_line(env);

    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(env, &vault_id);
    vault.initialize(&token_addr, &ledger_id);
    ledger.approve_party(&vault_id, &credit_ledger::Role::Vault);

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
    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let token_addr = sac.address();
    let (_ledger, ledger_id, _p, _l, _b, _c) = setup_drawn_line(&env);

    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&token_addr, &ledger_id);
    let res = vault.try_initialize(&token_addr, &ledger_id);
    assert_eq!(res, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn settle_before_initialize_is_refused() {
    // A vault that was never initialized has no token or ledger binding, so a
    // settle attempt must fail with NotInitialized rather than moving anything.
    let env = Env::default();
    env.mock_all_auths();
    let (_ledger, _ledger_id, _p, line_id, bank, cardholder) = setup_drawn_line(&env);

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

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let token_addr = sac.address();
    let token = TokenClient::new(&env, &token_addr);
    let token_admin = StellarAssetClient::new(&env, &token_addr);

    let (ledger, ledger_id, _pledge_id, line_id, bank, cardholder) = setup_drawn_line(&env);

    // deploy a vault and bind it, but DO NOT approve it on the ledger
    let vault_id = env.register(SettlementVault, ());
    let vault = SettlementVaultClient::new(&env, &vault_id);
    vault.initialize(&token_addr, &ledger_id);
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
    let (ledger, vault, token, token_admin, line_id, bank, cardholder, _v) = setup_vault(&env);

    token_admin.mint(&cardholder, &26_000i128);
    vault.settle_repayment(&line_id, &cardholder, &bank, &id(&env), &25_000i128);
    assert_eq!(ledger.get_line(&line_id).drawn_balance, 0);
    ledger.bank_authorize_release(&line_id, &bank);
    assert_eq!(ledger.get_line(&line_id).status, credit_ledger::LineStatus::Closed);

    let res = vault.try_settle_repayment(&line_id, &cardholder, &bank, &id(&env), &1i128);
    assert_eq!(res, Err(Ok(Error::LineNotRepayable)));
    // The refused settle moved no tokens: cardholder keeps the 1,000 left after
    // paying 25,000 of their 26,000, and the bank keeps exactly the 25,000.
    assert_eq!(token.balance(&cardholder), 1_000);
    assert_eq!(token.balance(&bank), 25_000);
}

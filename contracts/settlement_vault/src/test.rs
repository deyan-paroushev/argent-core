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

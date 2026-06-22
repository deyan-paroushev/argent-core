extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    Address, BytesN, Env,
};

fn h(env: &Env, n: u8) -> BytesN<32> {
    BytesN::from_array(env, &[n; 32])
}

#[derive(Clone)]
struct Actors {
    sponsor: Address,
    verifier: Address,
    gold_partner: Address,
    bank: Address,
    user: Address,
    campaign_id: BytesN<32>,
    spend_ref: BytesN<32>,
    mcc: BytesN<32>,
}

// Currency scale: program units are CHF cents (rappen). 1_000_000 = CHF 10,000.00.
const CAP: i128 = 100_000_00; // CHF 100,000.00
const BUDGET: i128 = 250_000_00; // CHF 250,000.00
const CLAIM_WINDOW: u32 = 365 * 17_280; // ~1 year of ledgers

/// Build a standard campaign config: given rate/cap/budget, window 0..10M.
fn cfg(env: &Env, mcc: &BytesN<32>, rate_bps: u32, cap: i128, budget: i128) -> CampaignConfig {
    CampaignConfig {
        reward_rate_bps: rate_bps,
        user_spend_cap: cap,
        total_budget: budget,
        currency_code: 756, // CHF ISO numeric
        start_ledger: 0,
        end_ledger: 10_000_000,
        claim_window_ledgers: CLAIM_WINDOW,
        eligible_mcc_policy_hash: mcc.clone(),
        redemption_terms_hash: h(env, 3),
        sponsor_product_scope_hash: h(env, 4),
        funding_commitment_hash: h(env, 5),
    }
}

/// Build a spend input for a given reference, amount and mcc.
fn spend(env: &Env, spend_ref: &BytesN<32>, amount: i128, mcc: &BytesN<32>) -> SpendInput {
    SpendInput {
        spend_ref_hash: spend_ref.clone(),
        card_line_hash: h(env, 30),
        amount,
        currency_code: 756,
        category_policy_hash: mcc.clone(),
        posted_at_ledger: 1_000,
    }
}

fn setup(env: &Env) -> (RewardsLedgerClient<'_>, Actors) {
    env.mock_all_auths();
    let contract_id = env.register(RewardsLedger, ());
    let client = RewardsLedgerClient::new(env, &contract_id);

    let admin = Address::generate(env);
    let sponsor = Address::generate(env);
    let verifier = Address::generate(env);
    let gold_partner = Address::generate(env);
    let bank = Address::generate(env);
    let user = Address::generate(env);

    client.initialize(&admin);
    client.approve_party(&sponsor, &Role::Sponsor);
    client.approve_party(&verifier, &Role::Verifier);
    client.approve_party(&gold_partner, &Role::GoldPartner);
    client.approve_party(&bank, &Role::Bank);

    let campaign_id = h(env, 1);
    let spend_ref = h(env, 20);
    let mcc = h(env, 2);

    env.ledger().with_mut(|l| l.sequence_number = 1_000);

    client.create_campaign(
        &campaign_id, &sponsor, &verifier, &gold_partner, &cfg(env, &mcc, 100, CAP, BUDGET),
    );

    (client, Actors { sponsor, verifier, gold_partner, bank, user, campaign_id, spend_ref, mcc })
}

/// Drive a spend through to a voucher, returning the reward value.
fn to_voucher(client: &RewardsLedgerClient, a: &Actors, env: &Env) -> i128 {
    let reward = client.record_eligible_spend(
        &a.campaign_id, &a.user, &a.verifier, &spend(env, &a.spend_ref, 1_000_000, &a.mcc),
    );
    client.confirm_spend_final(&a.campaign_id, &a.spend_ref, &a.verifier, &h(env, 31));
    client.submit_claim(&a.campaign_id, &a.spend_ref, &a.user, &h(env, 32));
    client.sponsor_approve_claim(&a.campaign_id, &a.spend_ref, &a.sponsor, &h(env, 33), &h(env, 34));
    reward
}

#[test]
fn full_flow_redeems_to_gold() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let reward = to_voucher(&client, &a, &env);
    assert_eq!(reward, 10_000);

    let price_e7 = 3_000_000_000_000i128;
    client.confirm_redemption(
        &a.campaign_id, &a.spend_ref, &a.gold_partner, &h(&env, 40), &h(&env, 41),
        &reward, &price_e7, &h(&env, 42),
    );

    let r = client.get_redemption(&a.spend_ref);
    assert_eq!(r.fine_weight_oz_e7, 333_333);
    assert_eq!(r.redeemed_value, 10_000);
    assert_eq!(client.get_accrual(&a.spend_ref).status, RewardStatus::Redeemed);
    assert_eq!(client.get_user_usage(&a.campaign_id, &a.user).fine_weight_oz_e7_total, 333_333);

    let camp = client.get_campaign(&a.campaign_id);
    assert_eq!(camp.redeemed_reward_value, 10_000);
    assert_eq!(camp.redeemed_fine_weight_oz_e7, 333_333);
    assert_eq!(camp.reserved_reward_value, 0);
}

#[test]
fn finality_gate_blocks_claim_before_final() {
    let env = Env::default();
    let (client, a) = setup(&env);
    client.record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, 1_000_000, &a.mcc));
    let r = client.try_submit_claim(&a.campaign_id, &a.spend_ref, &a.user, &h(&env, 32));
    assert_eq!(r, Err(Ok(Error::InvalidStatus)));
}

#[test]
fn per_user_cap_truncates_eligible() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let reward = client.record_eligible_spend(
        &a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, CAP + 5_000_000, &a.mcc),
    );
    assert_eq!(reward, CAP / 100);
    assert_eq!(client.get_user_usage(&a.campaign_id, &a.user).eligible_spend_total, CAP);
}

#[test]
fn cap_exhausted_rejects_further_spend() {
    let env = Env::default();
    let (client, a) = setup(&env);
    client.record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, CAP, &a.mcc));
    let r = client.try_record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &h(&env, 21), 1_00, &a.mcc));
    assert_eq!(r, Err(Ok(Error::UserCapExceeded)));
}

#[test]
fn duplicate_spend_ref_rejected() {
    let env = Env::default();
    let (client, a) = setup(&env);
    client.record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, 1_000_000, &a.mcc));
    let r = client.try_record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, 1_000_000, &a.mcc));
    assert_eq!(r, Err(Ok(Error::SpendExists)));
}

#[test]
fn bank_clawback_releases_budget() {
    let env = Env::default();
    let (client, a) = setup(&env);
    client.record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, 1_000_000, &a.mcc));
    client.confirm_spend_final(&a.campaign_id, &a.spend_ref, &a.verifier, &h(&env, 31));
    assert_eq!(client.get_campaign(&a.campaign_id).reserved_reward_value, 10_000);

    client.cancel_reward(&a.campaign_id, &a.spend_ref, &a.bank, &h(&env, 60));
    let after = client.get_campaign(&a.campaign_id);
    assert_eq!(after.reserved_reward_value, 0);
    assert_eq!(after.cancelled_reward_value, 10_000);
    assert_eq!(client.get_accrual(&a.spend_ref).status, RewardStatus::Cancelled);
}

#[test]
fn cannot_cancel_after_redemption() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let reward = to_voucher(&client, &a, &env);
    client.confirm_redemption(
        &a.campaign_id, &a.spend_ref, &a.gold_partner, &h(&env, 40), &h(&env, 41),
        &reward, &3_000_000_000_000i128, &h(&env, 42),
    );
    let r = client.try_cancel_reward(&a.campaign_id, &a.spend_ref, &a.bank, &h(&env, 60));
    assert_eq!(r, Err(Ok(Error::InvalidStatus)));
}

#[test]
fn sponsor_reject_releases_budget() {
    let env = Env::default();
    let (client, a) = setup(&env);
    client.record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, 1_000_000, &a.mcc));
    client.confirm_spend_final(&a.campaign_id, &a.spend_ref, &a.verifier, &h(&env, 31));
    client.submit_claim(&a.campaign_id, &a.spend_ref, &a.user, &h(&env, 32));
    client.sponsor_reject_claim(&a.campaign_id, &a.spend_ref, &a.sponsor, &h(&env, 33));
    assert_eq!(client.get_campaign(&a.campaign_id).reserved_reward_value, 0);
    assert_eq!(client.get_accrual(&a.spend_ref).status, RewardStatus::Rejected);
}

#[test]
fn expiry_after_window_releases_budget() {
    let env = Env::default();
    let (client, a) = setup(&env);
    // Use a dedicated campaign with a SHORT claim window. Jumping millions of
    // ledgers ahead can archive the contract instance in the test host (a
    // test-only artifact: on-network, entries bump their TTL on access). A short
    // window tests the same expiry logic cheaply and deterministically.
    let cid = h(&env, 8);
    let short_window: u32 = 5_000;
    let mut c = cfg(&env, &a.mcc, 100, CAP, BUDGET);
    c.claim_window_ledgers = short_window;
    client.create_campaign(&cid, &a.sponsor, &a.verifier, &a.gold_partner, &c);

    let sref = h(&env, 80);
    client.record_eligible_spend(&cid, &a.user, &a.verifier, &spend(&env, &sref, 1_000_000, &a.mcc));
    client.confirm_spend_final(&cid, &sref, &a.verifier, &h(&env, 31));
    env.ledger().with_mut(|l| l.sequence_number = 1_000 + short_window + 1);
    client.expire_reward(&cid, &sref);
    let camp = client.get_campaign(&cid);
    assert_eq!(camp.reserved_reward_value, 0);
    assert_eq!(camp.expired_reward_value, 10_000);
    assert_eq!(client.get_accrual(&sref).status, RewardStatus::Expired);
}

#[test]
fn expire_before_window_rejected() {
    let env = Env::default();
    let (client, a) = setup(&env);
    client.record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, 1_000_000, &a.mcc));
    client.confirm_spend_final(&a.campaign_id, &a.spend_ref, &a.verifier, &h(&env, 31));
    let r = client.try_expire_reward(&a.campaign_id, &a.spend_ref);
    assert_eq!(r, Err(Ok(Error::NotExpired)));
}

#[test]
fn invalid_rate_rejected() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let r = client.try_create_campaign(
        &h(&env, 9), &a.sponsor, &a.verifier, &a.gold_partner, &cfg(&env, &a.mcc, 1_001, CAP, BUDGET),
    );
    assert_eq!(r, Err(Ok(Error::InvalidRate)));
}

#[test]
fn policy_mismatch_on_wrong_mcc() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let r = client.try_record_eligible_spend(
        &a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, 1_000_000, &h(&env, 99)),
    );
    assert_eq!(r, Err(Ok(Error::PolicyMismatch)));
}

#[test]
fn wrong_gold_partner_cannot_redeem() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let reward = to_voucher(&client, &a, &env);
    let stranger = Address::generate(&env);
    client.approve_party(&stranger, &Role::GoldPartner);
    let r = client.try_confirm_redemption(
        &a.campaign_id, &a.spend_ref, &stranger, &h(&env, 40), &h(&env, 41),
        &reward, &3_000_000_000_000i128, &h(&env, 42),
    );
    assert_eq!(r, Err(Ok(Error::NotAuthorized)));
}

#[test]
fn redeem_value_mismatch_rejected() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let reward = to_voucher(&client, &a, &env);
    let r = client.try_confirm_redemption(
        &a.campaign_id, &a.spend_ref, &a.gold_partner, &h(&env, 40), &h(&env, 41),
        &(reward - 1), &3_000_000_000_000i128, &h(&env, 42),
    );
    assert_eq!(r, Err(Ok(Error::AmountMismatch)));
}

#[test]
fn nonpositive_price_rejected() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let reward = to_voucher(&client, &a, &env);
    let r = client.try_confirm_redemption(
        &a.campaign_id, &a.spend_ref, &a.gold_partner, &h(&env, 40), &h(&env, 41),
        &reward, &0i128, &h(&env, 42),
    );
    assert_eq!(r, Err(Ok(Error::PriceNotPositive)));
}

#[test]
fn paused_campaign_blocks_spend() {
    let env = Env::default();
    let (client, a) = setup(&env);
    client.pause_campaign(&a.campaign_id, &a.sponsor);
    let r = client.try_record_eligible_spend(&a.campaign_id, &a.user, &a.verifier, &spend(&env, &a.spend_ref, 1_000_000, &a.mcc));
    assert_eq!(r, Err(Ok(Error::CampaignNotActive)));
}

#[test]
fn budget_exhaustion_rejects() {
    let env = Env::default();
    let (client, a) = setup(&env);
    let cid = h(&env, 7);
    client.create_campaign(
        &cid, &a.sponsor, &a.verifier, &a.gold_partner, &cfg(&env, &a.mcc, 100, 100_000_000, 50),
    );
    client.record_eligible_spend(&cid, &a.user, &a.verifier, &spend(&env, &h(&env, 50), 4_000, &a.mcc));
    let r = client.try_record_eligible_spend(&cid, &a.user, &a.verifier, &spend(&env, &h(&env, 51), 2_000, &a.mcc));
    assert_eq!(r, Err(Ok(Error::BudgetInsufficient)));
}

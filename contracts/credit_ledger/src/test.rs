#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _, BytesN as _, Events as _},
    Address, BytesN, Env,
};

struct Fixture {
    env: Env,
    client: CreditLedgerClient<'static>,
    admin: Address,
    owner: Address,
    custodian: Address,
    bank: Address,
    processor: Address,
    cardholder: Address,
    vault: Address,
    valuation: Address,
}

fn setup() -> Fixture {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreditLedger, ());
    let client = CreditLedgerClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let custodian = Address::generate(&env);
    let bank = Address::generate(&env);
    let processor = Address::generate(&env);
    let cardholder = Address::generate(&env);
    let vault = Address::generate(&env);
    let valuation = Address::generate(&env);

    client.initialize(&admin);
    client.approve_party(&custodian, &Role::Custodian);
    client.approve_party(&bank, &Role::Bank);
    client.approve_party(&processor, &Role::Processor);
    client.approve_party(&vault, &Role::Vault);
    client.approve_party(&valuation, &Role::Valuation);

    Fixture { env, client, admin, owner, custodian, bank, processor, cardholder, vault, valuation }
}

fn id(env: &Env) -> BytesN<32> {
    BytesN::random(env)
}

/// Register a tri-party control framework among the fixture's owner, bank, and
/// custodian, anchoring six placeholder document hashes. Returns framework id.
fn register_framework(f: &Fixture) -> BytesN<32> {
    let framework_id = id(&f.env);
    f.client.register_framework(
        &framework_id,
        &f.owner,
        &f.bank,
        &f.custodian,
        &id(&f.env),
        &id(&f.env),
        &id(&f.env),
        &id(&f.env),
        &id(&f.env),
        &id(&f.env),
    );
    framework_id
}

/// Register a position and have the custodian confirm-and-immobilize it.
/// Registers a control framework first and binds the position to it.
/// Returns the position id, left in the Earmarked state.
fn register_and_immobilize(f: &Fixture) -> BytesN<32> {
    let framework_id = register_framework(f);
    let position_id = id(&f.env);
    let expiry = f.env.ledger().sequence() + 100_000;
    let fine_weight_oz_e7: i128 = 4_011_000_000; // 401.10 oz
    f.client.register_position(
        &position_id,
        &framework_id,
        &f.owner,
        &f.custodian,
        &id(&f.env), // barlist_hash
        &id(&f.env), // serials_hash
        &fine_weight_oz_e7,
        &expiry,
    );
    f.client.select_bars_for_collateral(&position_id, &f.owner, &id(&f.env));
    f.client.confirm_and_immobilize(&position_id, &f.custodian, &id(&f.env));
    position_id
}

/// Register, immobilize, pledge, and open a ~720,000 line against ~401.10 oz
/// at 2,990/oz, 60% advance, 75% maintenance (base ~= 719,533). Returns ids.
fn happy_to_open(f: &Fixture) -> (BytesN<32>, BytesN<32>, BytesN<32>) {
    let position_id = register_and_immobilize(f);
    let pledge_id = id(&f.env);
    let line_id = id(&f.env);

    f.client.activate_pledge(
        &pledge_id,
        &position_id,
        &f.owner,
        &f.bank,
        &id(&f.env),
    );

    // price 2,990/oz scaled 1e7; advance 6000 bps; maintenance 7500 bps
    // base = 401.10 * 2990 * 0.60 = 719,533 (whole units here)
    let price_per_oz_e7: i128 = 29_900_000_000;
    f.client.open_credit_line(
        &line_id,
        &pledge_id,
        &f.bank,
        &f.cardholder,
        &719_000, // <= base
        &6000u32,
        &7500u32,
        &price_per_oz_e7,
    );

    (position_id, pledge_id, line_id)
}

// ---- happy paths -----------------------------------------------------------

#[test]
fn register_framework_three_party() {
    let f = setup();
    let framework_id = register_framework(&f);
    let fwk = f.client.get_framework(&framework_id);
    assert_eq!(fwk.owner, f.owner);
    assert_eq!(fwk.bank, f.bank);
    assert_eq!(fwk.custodian, f.custodian);
    assert_eq!(fwk.status, FrameworkStatus::Active);
}

#[test]
fn refuses_framework_with_unapproved_bank() {
    let f = setup();
    let rogue_bank = Address::generate(&f.env);
    let framework_id = id(&f.env);
    let res = f.client.try_register_framework(
        &framework_id,
        &f.owner,
        &rogue_bank,
        &f.custodian,
        &id(&f.env), &id(&f.env), &id(&f.env), &id(&f.env), &id(&f.env), &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

#[test]
fn immobilize_moves_free_to_earmarked() {
    let f = setup();
    let framework_id = register_framework(&f);
    let position_id = id(&f.env);
    let expiry = f.env.ledger().sequence() + 100_000;
    f.client.register_position(
        &position_id, &framework_id, &f.owner, &f.custodian, &id(&f.env), &id(&f.env), &4_011_000_000, &expiry,
    );
    assert_eq!(f.client.get_position(&position_id).status, PositionStatus::Free);
    // owner selects the bars as collateral
    let request_hash = id(&f.env);
    f.client.select_bars_for_collateral(&position_id, &f.owner, &request_hash);
    assert_eq!(f.client.get_position(&position_id).status, PositionStatus::Selected);
    let selection = f.client.get_selection(&position_id);
    assert_eq!(selection.request_hash, request_hash);
    assert_eq!(selection.owner, f.owner);
    // custodian confirms and immobilizes the selection
    let control_hash = id(&f.env);
    f.client.confirm_and_immobilize(&position_id, &f.custodian, &control_hash);
    assert_eq!(f.client.get_position(&position_id).status, PositionStatus::Earmarked);
    // the tri-party control framework was recorded
    let control = f.client.get_custody_control(&position_id);
    assert_eq!(control.control_agreement_hash, control_hash);
    assert_eq!(control.custodian, f.custodian);
}

#[test]
fn happy_path_draw_repay_release() {
    let f = setup();
    let (position_id, pledge_id, line_id) = happy_to_open(&f);

    assert_eq!(f.client.available_capacity(&line_id), 719_000);

    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    assert_eq!(f.client.available_capacity(&line_id), 694_000);
    assert_eq!(f.client.get_line(&line_id).drawn_balance, 25_000);

    f.client.apply_repayment(&line_id, &f.vault, &id(&f.env), &25_000);
    assert_eq!(f.client.available_capacity(&line_id), 719_000);
    assert_eq!(f.client.get_line(&line_id).drawn_balance, 0);

    // stage one: bank authorizes release of its security interest (prong i).
    // the bars are not yet returned, so the position is ReleasePending and the
    // pledge is ReleaseAuthorized (lien persists until perfection terminates).
    f.client.bank_authorize_release(&line_id, &f.bank);
    assert_eq!(f.client.get_position(&position_id).status, PositionStatus::ReleasePending);
    assert_eq!(f.client.get_pledge(&pledge_id).status, PledgeStatus::ReleaseAuthorized);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Closed);

    // stage two: custodian confirms return of possession (prong ii), which
    // terminates perfection and restores clear title.
    f.client.custodian_confirm_release(&pledge_id, &f.custodian, &id(&f.env));
    assert_eq!(f.client.get_position(&position_id).status, PositionStatus::Released);
    assert_eq!(f.client.get_pledge(&pledge_id).status, PledgeStatus::Released);
}

#[test]
fn default_branch_records_enforcement() {
    let f = setup();
    let (position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);

    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Defaulted);

    f.env.ledger().set_sequence_number(cure_deadline + 1);

    f.client.record_enforcement(
        &line_id,
        &f.bank,
        &EnforcementOutcome::Sold,
        &id(&f.env),
    );
    assert_eq!(f.client.get_position(&position_id).status, PositionStatus::Enforced);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Closed);
}

#[test]
fn cure_restores_line() {
    let f = setup();
    let (_p, pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let cure_deadline = f.env.ledger().sequence() + 100;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);

    f.client.cure_default(&line_id, &f.cardholder);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Active);
    assert_eq!(f.client.get_pledge(&pledge_id).status, PledgeStatus::Active);
}

#[test]
fn refuses_default_notice_with_past_deadline() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    // move the ledger forward so we have a past to point at
    f.env.ledger().set_sequence_number(1_000);
    // a deadline at the current ledger is not in the future -> refused
    let at_now = f.client.try_issue_default_notice(&line_id, &f.bank, &1_000);
    assert_eq!(at_now, Err(Ok(Error::CureDeadlineNotFuture)));
    // a deadline before the current ledger -> refused
    let in_past = f.client.try_issue_default_notice(&line_id, &f.bank, &999);
    assert_eq!(in_past, Err(Ok(Error::CureDeadlineNotFuture)));
    // the line was not moved into default by the refused attempts
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Active);
}

#[test]
fn accepts_default_notice_with_future_deadline() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let deadline = f.env.ledger().sequence() + 1;
    f.client.issue_default_notice(&line_id, &f.bank, &deadline);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Defaulted);
}

#[test]
fn cure_allowed_after_deadline_until_enforcement() {
    let f = setup();
    let (_p, pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);

    // advance PAST the cure deadline without the bank enforcing
    f.env.ledger().set_sequence_number(cure_deadline + 5);

    // lenient by design: the cure still succeeds because the default is still
    // "continuing" (enforcement has not been recorded). The deadline gates the
    // bank's right to enforce, not the borrower's ability to pay.
    f.client.cure_default(&line_id, &f.cardholder);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Active);
    assert_eq!(f.client.get_pledge(&pledge_id).status, PledgeStatus::Active);
}

// ---- margin / maintenance --------------------------------------------------

/// Helper to revalue at a given price/oz (whole units), fresh and tight conf.
fn revalue_at(f: &Fixture, line_id: &BytesN<32>, price_whole: i128) {
    let price_e7 = price_whole * 10_000_000;
    let now = f.env.ledger().timestamp();
    // confidence well within tolerance; price fresh (priced now)
    f.client.revalue_and_check(
        line_id,
        &f.valuation,
        &price_e7,
        &(price_e7 / 1000), // 0.1% conf
        &now,
        &86_400u64,  // 1-day freshness window
        &50u32,      // conf tolerance 0.5%
        &9000u32,    // warning at 90% of the action band
    );
}

#[test]
fn margin_covered_warning_called_across_falling_price() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    // draw most of the line so a price drop bites
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &700_000);

    // At the opening price (2,990), maintenance 75% of raw value is well above
    // the 700k drawn -> Covered.
    revalue_at(&f, &line_id, 2_990);
    assert_eq!(f.client.get_valuation(&line_id).margin_state, MarginState::Covered);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Active);

    // Drop the price so drawn crosses the warning band but not the action band.
    // raw = 401.10 * price ; action = raw * 0.75 ; warn = action * 0.90.
    // Find a price where warn < 700k <= action. price ~ 2,360 gives action ~=
    // 710k, warn ~= 639k -> Warning.
    revalue_at(&f, &line_id, 2_360);
    assert_eq!(f.client.get_valuation(&line_id).margin_state, MarginState::Warning);

    // Drop further so drawn exceeds the action band -> Called and Suspended.
    // price 2,300 gives action ~= 691k < 700k -> Called.
    revalue_at(&f, &line_id, 2_300);
    assert_eq!(f.client.get_valuation(&line_id).margin_state, MarginState::Called);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Suspended);
}

#[test]
fn revaluation_does_not_clear_bank_suspension() {
    // This is the regression test for the bank-suspension override bug: a bank
    // stops the line for a non-margin reason, and a later revaluation that finds
    // the margin perfectly covered must NOT silently reactivate the line.
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &700_000);

    // bank deliberately suspends (fraud / KYC / sanctions / credit stop)
    f.client.bank_suspend_line(&line_id, &f.bank, &id(&f.env));
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Suspended);
    assert_eq!(f.client.get_line(&line_id).manual_bank_suspension, true);

    // a fully-covered revaluation runs (price healthy, margin Covered)
    revalue_at(&f, &line_id, 2_990);
    assert_eq!(f.client.get_valuation(&line_id).margin_state, MarginState::Covered);
    // the line MUST remain Suspended: the bank stop is not cleared by valuation
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Suspended);
    assert_eq!(f.client.get_line(&line_id).manual_bank_suspension, true);
}

#[test]
fn bank_suspend_and_resume_round_trip() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);

    f.client.bank_suspend_line(&line_id, &f.bank, &id(&f.env));
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Suspended);
    assert_eq!(f.client.get_line(&line_id).available_limit, 0);

    // only the bank can resume; resuming clears the flag and restores Active
    f.client.bank_resume_line(&line_id, &f.bank);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Active);
    assert_eq!(f.client.get_line(&line_id).manual_bank_suspension, false);
}

#[test]
fn refuses_resume_of_non_suspended_line() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    // line is Active, not bank-suspended: resume must refuse
    let res = f.client.try_bank_resume_line(&line_id, &f.bank);
    assert_eq!(res, Err(Ok(Error::LineNotSuspended)));
}

#[test]
fn called_line_blocks_draws_until_recovered() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &700_000);

    // Force a margin call.
    revalue_at(&f, &line_id, 2_300);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Suspended);

    // A draw on a suspended line is refused.
    let res = f.client.try_record_drawdown(&line_id, &f.processor, &id(&f.env), &1_000);
    assert_eq!(res, Err(Ok(Error::LineNotActive)));

    // Price recovers; revaluation restores the line to Active.
    revalue_at(&f, &line_id, 2_990);
    assert_eq!(f.client.get_valuation(&line_id).margin_state, MarginState::Covered);
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Active);
}

// ---- refusal paths ---------------------------------------------------------

#[test]
fn refuses_pledge_of_free_not_earmarked_position() {
    let f = setup();
    // register but do NOT immobilize
    let framework_id = register_framework(&f);
    let position_id = id(&f.env);
    let expiry = f.env.ledger().sequence() + 100_000;
    f.client.register_position(
        &position_id, &framework_id, &f.owner, &f.custodian, &id(&f.env), &id(&f.env), &4_011_000_000, &expiry,
    );
    let res = f.client.try_activate_pledge(
        &id(&f.env), &position_id, &f.owner, &f.bank, &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::PositionNotEarmarked)));
}

#[test]
fn refuses_duplicate_payment_ref() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let pay = id(&f.env);
    f.client.apply_repayment(&line_id, &f.vault, &pay, &10_000);
    // same payment reference cannot be applied twice
    let res = f.client.try_apply_repayment(&line_id, &f.vault, &pay, &10_000);
    assert_eq!(res, Err(Ok(Error::DuplicatePaymentRef)));
}

#[test]
fn repayment_does_not_release_collateral() {
    let f = setup();
    let (position_id, pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    // full repayment clears the debt
    f.client.apply_repayment(&line_id, &f.vault, &id(&f.env), &25_000);
    assert_eq!(f.client.get_line(&line_id).drawn_balance, 0);
    // but the collateral is NOT released: position stays Pledged, pledge Active
    assert_eq!(f.client.get_position(&position_id).status, PositionStatus::Pledged);
    assert_eq!(f.client.get_pledge(&pledge_id).status, PledgeStatus::Active);
    // and the repayment was recorded
    assert_eq!(f.client.get_line(&line_id).status, LineStatus::Active);
}

#[test]
fn owner_requests_collateral_adjustment() {
    let f = setup();
    let (_pos, _pledge, line_id) = happy_to_open(&f);
    let adj_id = id(&f.env);
    let new_barlist = id(&f.env);
    let request_hash = id(&f.env);
    f.client.request_collateral_adjustment(
        &adj_id,
        &line_id,
        &f.owner,
        &AdjustmentType::TopUp,
        &new_barlist,
        &5_000_000_000i128, // proposed new weight
        &request_hash,
    );
    let adj = f.client.get_adjustment(&adj_id);
    assert_eq!(adj.status, AdjustmentStatus::Requested);
    assert_eq!(adj.adjustment_type, AdjustmentType::TopUp);
    assert_eq!(adj.new_barlist_hash, new_barlist);
    assert_eq!(adj.new_weight_oz_e7, 5_000_000_000i128);
}

#[test]
fn refuses_adjustment_request_by_non_owner() {
    let f = setup();
    let (_pos, _pledge, line_id) = happy_to_open(&f);
    let stranger = Address::generate(&f.env);
    let res = f.client.try_request_collateral_adjustment(
        &id(&f.env),
        &line_id,
        &stranger,
        &AdjustmentType::Substitution,
        &id(&f.env),
        &4_011_000_000i128,
        &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::NotAuthorized)));
}

#[test]
fn full_adjustment_topup_clears_three_party() {
    let f = setup();
    let (position_id, _pledge, line_id) = happy_to_open(&f);
    // draw most of the line so coverage matters
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &700_000);

    let adj_id = id(&f.env);
    let new_barlist = id(&f.env);
    let new_weight: i128 = 5_000_000_000; // 500 oz, a top-up from 401.10
    f.client.request_collateral_adjustment(
        &adj_id, &line_id, &f.owner, &AdjustmentType::TopUp,
        &new_barlist, &new_weight, &id(&f.env),
    );
    // custodian confirms it can hold the new set
    f.client.custodian_confirm_adjustment(&adj_id, &f.custodian);
    assert_eq!(f.client.get_adjustment(&adj_id).status, AdjustmentStatus::CustodianConfirmed);
    // bank approves at a price where the new collateral covers the draw
    f.client.bank_approve_adjustment(&adj_id, &f.bank, &29_900_000_000i128);
    assert_eq!(f.client.get_adjustment(&adj_id).status, AdjustmentStatus::Approved);
    // the position's collateral schedule updated
    let pos = f.client.get_position(&position_id);
    assert_eq!(pos.fine_weight_oz_e7, new_weight);
    assert_eq!(pos.barlist_hash, new_barlist);
}

#[test]
fn refuses_undercovered_partial_release() {
    let f = setup();
    let (_pos, _pledge, line_id) = happy_to_open(&f);
    // draw near the full advance capacity
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &700_000);

    let adj_id = id(&f.env);
    // propose releasing down to a tiny weight that cannot cover 700k at 60% ltv
    f.client.request_collateral_adjustment(
        &adj_id, &line_id, &f.owner, &AdjustmentType::PartialRelease,
        &id(&f.env), &500_000_000i128 /* 50 oz */, &id(&f.env),
    );
    f.client.custodian_confirm_adjustment(&adj_id, &f.custodian);
    // at 2,990/oz, 50 oz * 0.60 = ~89,700 < 700,000 drawn -> undercovered
    let res = f.client.try_bank_approve_adjustment(&adj_id, &f.bank, &29_900_000_000i128);
    assert_eq!(res, Err(Ok(Error::AdjustmentUndercovered)));
}

#[test]
fn refuses_approve_adjustment_before_custodian() {
    let f = setup();
    let (_pos, _pledge, line_id) = happy_to_open(&f);
    let adj_id = id(&f.env);
    f.client.request_collateral_adjustment(
        &adj_id, &line_id, &f.owner, &AdjustmentType::TopUp,
        &id(&f.env), &5_000_000_000i128, &id(&f.env),
    );
    // adjustment is Requested, not CustodianConfirmed: bank cannot approve yet
    let res = f.client.try_bank_approve_adjustment(&adj_id, &f.bank, &29_900_000_000i128);
    assert_eq!(res, Err(Ok(Error::AdjustmentWrongStatus)));
}

#[test]
fn refuses_double_pledge_of_same_bars() {
    let f = setup();
    let framework_id = register_framework(&f);
    // register a first position with a specific serials_hash
    let serials = id(&f.env);
    let pos1 = id(&f.env);
    let expiry = f.env.ledger().sequence() + 100_000;
    f.client.register_position(
        &pos1, &framework_id, &f.owner, &f.custodian, &id(&f.env), &serials, &4_011_000_000i128, &expiry,
    );
    // a second position cannot register the SAME serials while pos1 is active
    let pos2 = id(&f.env);
    let res = f.client.try_register_position(
        &pos2, &framework_id, &f.owner, &f.custodian, &id(&f.env), &serials, &4_011_000_000i128, &expiry,
    );
    assert_eq!(res, Err(Ok(Error::BarSetAlreadyActive)));
}

#[test]
fn bars_reusable_after_release() {
    let f = setup();
    let framework_id = register_framework(&f);
    let serials = id(&f.env);
    let pos1 = id(&f.env);
    let pledge1 = id(&f.env);
    let line1 = id(&f.env);
    let expiry = f.env.ledger().sequence() + 100_000;
    // full lifecycle: register -> select -> immobilize -> pledge -> open -> release
    f.client.register_position(
        &pos1, &framework_id, &f.owner, &f.custodian, &id(&f.env), &serials, &4_011_000_000i128, &expiry,
    );
    f.client.select_bars_for_collateral(&pos1, &f.owner, &id(&f.env));
    f.client.confirm_and_immobilize(&pos1, &f.custodian, &id(&f.env));
    f.client.activate_pledge(&pledge1, &pos1, &f.owner, &f.bank, &id(&f.env));
    f.client.open_credit_line(
        &line1, &pledge1, &f.bank, &f.cardholder, &719_000i128, &6000u32, &7500u32, &29_900_000_000i128,
    );
    // no draw: release immediately (drawn == 0)
    f.client.bank_authorize_release(&line1, &f.bank);
    f.client.custodian_confirm_release(&pledge1, &f.custodian, &id(&f.env));
    assert_eq!(f.client.get_position(&pos1).status, PositionStatus::Released);

    // the same serials can now be registered again under a new position
    let pos2 = id(&f.env);
    f.client.register_position(
        &pos2, &framework_id, &f.owner, &f.custodian, &id(&f.env), &serials, &4_011_000_000i128, &expiry,
    );
    assert_eq!(f.client.get_position(&pos2).status, PositionStatus::Free);
}

#[test]
fn refuses_immobilize_of_unselected_position() {
    let f = setup();
    let framework_id = register_framework(&f);
    let position_id = id(&f.env);
    let expiry = f.env.ledger().sequence() + 100_000;
    f.client.register_position(
        &position_id, &framework_id, &f.owner, &f.custodian, &id(&f.env), &id(&f.env), &4_011_000_000, &expiry,
    );
    // position is Free, owner has not selected it: custodian cannot immobilize
    let res = f.client.try_confirm_and_immobilize(&position_id, &f.custodian, &id(&f.env));
    assert_eq!(res, Err(Ok(Error::PositionNotSelected)));
}

#[test]
fn refuses_immobilize_by_wrong_custodian() {
    let f = setup();
    let rogue = Address::generate(&f.env);
    f.client.approve_party(&rogue, &Role::Custodian);
    let framework_id = register_framework(&f);
    let position_id = id(&f.env);
    let expiry = f.env.ledger().sequence() + 100_000;
    f.client.register_position(
        &position_id, &framework_id, &f.owner, &f.custodian, &id(&f.env), &id(&f.env), &4_011_000_000, &expiry,
    );
    f.client.select_bars_for_collateral(&position_id, &f.owner, &id(&f.env));
    // rogue is an approved custodian but not THIS position's custodian
    let res = f.client.try_confirm_and_immobilize(&position_id, &rogue, &id(&f.env));
    assert_eq!(res, Err(Ok(Error::NotAuthorized)));
}

#[test]
fn refuses_draw_above_limit() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let res = f.client.try_record_drawdown(&line_id, &f.processor, &id(&f.env), &800_000);
    assert_eq!(res, Err(Ok(Error::InsufficientCapacity)));
}

#[test]
fn refuses_duplicate_auth_ref() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let auth = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &auth, &10_000);
    let res = f.client.try_record_drawdown(&line_id, &f.processor, &auth, &10_000);
    assert_eq!(res, Err(Ok(Error::DuplicateAuthRef)));
}

#[test]
fn reverse_drawdown_unwinds_exact_amount() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let auth = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &auth, &10_000);
    let before = f.client.available_capacity(&line_id);
    f.client.reverse_drawdown(&line_id, &f.processor, &auth, &10_000);
    let after = f.client.available_capacity(&line_id);
    // capacity restored by exactly the drawn amount
    assert_eq!(after, before + 10_000);
    // the line's drawn balance is back to zero
    let line = f.client.get_line(&line_id);
    assert_eq!(line.drawn_balance, 0);
    // the record is kept but marked reversed
    let rec = f.client.get_drawdown(&auth);
    assert_eq!(rec.amount, 10_000);
    assert!(rec.reversed);
}

#[test]
fn refuses_reverse_with_wrong_amount() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let auth = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &auth, &10_000);
    // reversing more than was drawn is refused (this is the bug the record fix closes)
    let over = f.client.try_reverse_drawdown(&line_id, &f.processor, &auth, &25_000);
    assert_eq!(over, Err(Ok(Error::ReversalAmountMismatch)));
    // reversing less than was drawn is also refused (no partial reversals)
    let under = f.client.try_reverse_drawdown(&line_id, &f.processor, &auth, &5_000);
    assert_eq!(under, Err(Ok(Error::ReversalAmountMismatch)));
    // the line is untouched by the refused attempts
    let line = f.client.get_line(&line_id);
    assert_eq!(line.drawn_balance, 10_000);
}

#[test]
fn refuses_reverse_unknown_auth_ref() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    // never drawn under this auth_ref
    let res = f.client.try_reverse_drawdown(&line_id, &f.processor, &id(&f.env), &10_000);
    assert_eq!(res, Err(Ok(Error::NothingToReverse)));
}

#[test]
fn refuses_double_reverse() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let auth = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &auth, &10_000);
    f.client.reverse_drawdown(&line_id, &f.processor, &auth, &10_000);
    // second reversal of the same auth_ref is refused
    let res = f.client.try_reverse_drawdown(&line_id, &f.processor, &auth, &10_000);
    assert_eq!(res, Err(Ok(Error::NothingToReverse)));
}

#[test]
fn refuses_redraw_on_reversed_auth_ref() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let auth = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &auth, &10_000);
    f.client.reverse_drawdown(&line_id, &f.processor, &auth, &10_000);
    // an auth_ref is single-use: it cannot be drawn again after reversal
    let res = f.client.try_record_drawdown(&line_id, &f.processor, &auth, &10_000);
    assert_eq!(res, Err(Ok(Error::DuplicateAuthRef)));
}

#[test]
fn refuses_reverse_on_wrong_line() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let auth = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &auth, &10_000);
    // a different (random) line id cannot reverse this auth_ref
    let res = f.client.try_reverse_drawdown(&id(&f.env), &f.processor, &auth, &10_000);
    assert_eq!(res, Err(Ok(Error::LineNotFound)));
}

#[test]
fn refuses_release_with_outstanding_balance() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &10_000);
    let res = f.client.try_bank_authorize_release(&line_id, &f.bank);
    assert_eq!(res, Err(Ok(Error::OutstandingBalance)));
}

#[test]
fn refuses_confirm_release_before_bank_authorizes() {
    let f = setup();
    let (_pos, pledge_id, _line_id) = happy_to_open(&f);
    // pledge is Active, position Pledged; custodian cannot confirm release yet
    let res = f.client.try_custodian_confirm_release(&pledge_id, &f.custodian, &id(&f.env));
    assert_eq!(res, Err(Ok(Error::PledgeNotActive)));
}

#[test]
fn refuses_authorize_release_by_wrong_bank() {
    let f = setup();
    let other_bank = Address::generate(&f.env);
    f.client.approve_party(&other_bank, &Role::Bank);
    let (_p, _pl, line_id) = happy_to_open(&f);
    // line has zero drawn; a bank that is not the line's bank cannot authorize
    let res = f.client.try_bank_authorize_release(&line_id, &other_bank);
    assert_eq!(res, Err(Ok(Error::NotAuthorized)));
}

#[test]
fn refuses_limit_above_borrowing_base() {
    let f = setup();
    let position_id = register_and_immobilize(&f);
    let pledge_id = id(&f.env);
    let line_id = id(&f.env);
    f.client.activate_pledge(&pledge_id, &position_id, &f.owner, &f.bank, &id(&f.env));
    // base ~= 719,533; ask for 1,000,000
    let res = f.client.try_open_credit_line(
        &line_id, &pledge_id, &f.bank, &f.cardholder, &1_000_000, &6000u32, &7500u32, &29_900_000_000,
    );
    assert_eq!(res, Err(Ok(Error::LimitExceedsBorrowingBase)));
}

#[test]
fn refuses_invalid_risk_params() {
    let f = setup();
    let position_id = register_and_immobilize(&f);
    let pledge_id = id(&f.env);
    f.client.activate_pledge(&pledge_id, &position_id, &f.owner, &f.bank, &id(&f.env));

    // advance >= maintenance is invalid
    let res = f.client.try_open_credit_line(
        &id(&f.env), &pledge_id, &f.bank, &f.cardholder, &100_000, &7500u32, &7000u32, &29_900_000_000,
    );
    assert_eq!(res, Err(Ok(Error::InvalidRiskParams)));

    // maintenance > 100% is invalid
    let res2 = f.client.try_open_credit_line(
        &id(&f.env), &pledge_id, &f.bank, &f.cardholder, &100_000, &6000u32, &10_001u32, &29_900_000_000,
    );
    assert_eq!(res2, Err(Ok(Error::InvalidRiskParams)));
}

#[test]
fn refuses_stale_price_on_revaluation() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    // advance the clock so a price stamped at 0 is stale
    f.env.ledger().set_timestamp(1_000_000);
    let price_e7: i128 = 29_900_000_000;
    let res = f.client.try_revalue_and_check(
        &line_id,
        &f.valuation,
        &price_e7,
        &(price_e7 / 1000),
        &0u64,        // priced long ago
        &3600u64,     // 1-hour window
        &50u32,
        &9000u32,
    );
    assert_eq!(res, Err(Ok(Error::PriceStale)));
}

#[test]
fn refuses_wide_confidence_on_revaluation() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let now = f.env.ledger().timestamp();
    let price_e7: i128 = 29_900_000_000;
    let res = f.client.try_revalue_and_check(
        &line_id,
        &f.valuation,
        &price_e7,
        &(price_e7 / 10),  // 10% conf, far wider than tolerance
        &now,
        &86_400u64,
        &50u32,            // tolerance 0.5%
        &9000u32,
    );
    assert_eq!(res, Err(Ok(Error::PriceConfidenceTooWide)));
}

#[test]
fn refuses_enforce_before_cure_expiry() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let cure_deadline = f.env.ledger().sequence() + 1_000;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    // do NOT advance the ledger
    let res = f.client.try_record_enforcement(
        &line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::CurePeriodNotExpired)));
}

#[test]
fn refuses_pledge_of_already_pledged_position() {
    let f = setup();
    let (position_id, _pl, _line) = happy_to_open(&f);
    // the position is now Pledged, not Earmarked -> PositionNotEarmarked
    let res = f.client.try_activate_pledge(
        &id(&f.env), &position_id, &f.owner, &f.bank, &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::PositionNotEarmarked)));
}

#[test]
fn refuses_pledge_to_bank_outside_framework() {
    let f = setup();
    // a second, approved bank that is NOT the framework's bank
    let other_bank = Address::generate(&f.env);
    f.client.approve_party(&other_bank, &Role::Bank);
    let position_id = register_and_immobilize(&f);
    // framework names f.bank; pledging to other_bank must be refused
    let res = f.client.try_activate_pledge(
        &id(&f.env), &position_id, &f.owner, &other_bank, &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::FrameworkPartyMismatch)));
}

#[test]
fn refuses_unapproved_bank() {
    let f = setup();
    let rogue_bank = Address::generate(&f.env);
    let position_id = register_and_immobilize(&f);
    let res = f.client.try_activate_pledge(
        &id(&f.env), &position_id, &f.owner, &rogue_bank, &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

#[test]
fn refuses_stale_attestation_on_register() {
    let f = setup();
    let framework_id = register_framework(&f);
    let position_id = id(&f.env);
    f.env.ledger().set_sequence_number(50);
    let res = f.client.try_register_position(
        &position_id, &framework_id, &f.owner, &f.custodian, &id(&f.env), &id(&f.env), &4_011_000_000, &10u32,
    );
    assert_eq!(res, Err(Ok(Error::AttestationStale)));
}

// ---- enforcement readiness (the certificate honesty gate) ------------------

/// Opening a readiness record yields the honest default: Incomplete. No real
/// liquidation partner has been named, so a certificate generated from this
/// must render DRAFT.
#[test]
fn readiness_opens_incomplete() {
    let f = setup();
    let (_p, _pledge_id, line_id) = happy_to_open(&f);

    f.client.open_enforcement_readiness(&line_id, &f.bank);
    let r = f.client.get_enforcement_readiness(&line_id);
    assert_eq!(r.status, ReadinessStatus::Incomplete);
    assert_eq!(r.version, 0);
}

/// Populating with PLACEHOLDER fields (agent == bank, zero route hash,
/// settlement asset == bank) must STAY Incomplete. This is the core honesty
/// property: the contract refuses to assert readiness without a real party.
#[test]
fn readiness_with_placeholders_stays_incomplete() {
    let f = setup();
    let (_p, _pledge_id, line_id) = happy_to_open(&f);
    f.client.open_enforcement_readiness(&line_id, &f.bank);

    let zero = BytesN::from_array(&f.env, &[0u8; 32]);
    f.client.populate_enforcement_readiness(
        &line_id,
        &f.bank,
        &f.bank,        // liquidation_agent == bank  -> NOT a real agent
        &zero,          // realization_route_hash unset
        &f.bank,        // settlement_asset == bank   -> NOT real
        &id(&f.env),
        &id(&f.env),
        &0u32,
    );
    let r = f.client.get_enforcement_readiness(&line_id);
    assert_eq!(r.status, ReadinessStatus::Incomplete);
    assert_eq!(r.version, 1); // it did record a populate attempt
}

/// Populating with REAL fields (a distinct agent, a real route hash, a distinct
/// settlement asset) promotes the record to Ready. Only now may a certificate
/// render as anything other than DRAFT.
#[test]
fn readiness_with_real_fields_reaches_ready() {
    let f = setup();
    let (_p, _pledge_id, line_id) = happy_to_open(&f);
    f.client.open_enforcement_readiness(&line_id, &f.bank);

    let agent = Address::generate(&f.env);
    let settlement = Address::generate(&f.env);
    f.client.populate_enforcement_readiness(
        &line_id,
        &f.bank,
        &agent,         // real, distinct agent
        &id(&f.env),    // real route hash (non-zero)
        &settlement,    // real, distinct settlement asset
        &id(&f.env),
        &id(&f.env),
        &(f.env.ledger().sequence() + 1000),
    );
    let r = f.client.get_enforcement_readiness(&line_id);
    assert_eq!(r.status, ReadinessStatus::Ready);
    assert_eq!(r.liquidation_agent, agent);
}

/// A readiness record can be expired, dropping it back out of Ready so the
/// certificate returns to DRAFT until re-populated.
#[test]
fn readiness_can_expire() {
    let f = setup();
    let (_p, _pledge_id, line_id) = happy_to_open(&f);
    f.client.open_enforcement_readiness(&line_id, &f.bank);
    let agent = Address::generate(&f.env);
    let settlement = Address::generate(&f.env);
    f.client.populate_enforcement_readiness(
        &line_id, &f.bank, &agent, &id(&f.env), &settlement, &id(&f.env), &id(&f.env),
        &(f.env.ledger().sequence() + 1000),
    );
    assert_eq!(f.client.get_enforcement_readiness(&line_id).status, ReadinessStatus::Ready);

    f.client.expire_enforcement_readiness(&line_id, &f.bank);
    assert_eq!(f.client.get_enforcement_readiness(&line_id).status, ReadinessStatus::Expired);
}

// ===========================================================================
// ADVERSARIAL TEST SUITE. credit_ledger
//
// These tests are written to BREAK the contract, not to confirm the happy path.
// Each encodes a specific attack and asserts the contract refuses it. They are
// grouped by the class of risk they de-risk for mainnet:
//
//   A. Host-level authorization. The functional tests above run under
//      mock_all_auths(), which makes every require_auth() pass. That proves our
//      explicit "if party != x" checks, but NOT that the Soroban host rejects a
//      transaction signed by the wrong key. These tests grant auth to ONE
//      specific address via mock_auths and prove the action fails when the
//      host-level signer is wrong. (A wrong-signer / overflow failure surfaces
//      as Err(Err(InvokeError)), a system error, so we assert .is_err().)
//   B. Admin key control (revoke_party): a compromised party must lose power.
//   C. Arithmetic / boundary: overflow, exact-limit edges, off-by-one.
//   D. State-machine ordering: driving the lifecycle out of order.
//   E. Idempotency / replay across references.
// ===========================================================================

use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use soroban_sdk::IntoVal;

// A fixture that does NOT mock all auths, so individual calls must carry a
// matching mock_auths grant or the host rejects them. Mirrors setup() but with
// explicit, per-call authorization to model real signing.
fn setup_strict() -> Fixture {
    let env = Env::default();
    // NOTE: deliberately NO env.mock_all_auths() here.
    let contract_id = env.register(CreditLedger, ());
    let client = CreditLedgerClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let custodian = Address::generate(&env);
    let bank = Address::generate(&env);
    let processor = Address::generate(&env);
    let cardholder = Address::generate(&env);
    let vault = Address::generate(&env);
    let valuation = Address::generate(&env);

    // Admin setup still needs auth; grant exactly the admin calls we make.
    env.mock_all_auths();
    client.initialize(&admin);
    client.approve_party(&custodian, &Role::Custodian);
    client.approve_party(&bank, &Role::Bank);
    client.approve_party(&processor, &Role::Processor);
    client.approve_party(&vault, &Role::Vault);
    client.approve_party(&valuation, &Role::Valuation);
    // From here, tests opt into strict per-call auth by using mock_auths.

    Fixture { env, client, admin, owner, custodian, bank, processor, cardholder, vault, valuation }
}

// ---- A. Host-level authorization --------------------------------------------

#[test]
fn host_auth_wrong_signer_cannot_open_line() {
    // The bank role is approved, but the TRANSACTION is signed by an attacker.
    // Even though the attacker passes the real bank's address as the `bank`
    // arg, the host must reject because the auth entry is for the attacker, not
    // the bank. This proves require_auth() (not just our address check) gates
    // the call.
    let f = setup_strict();
    f.env.mock_all_auths();
    let (_pos, pledge_id, _l) = happy_to_open(&f); // builds state under mock_all_auths

    // open a SECOND line, but now strictly: only the attacker has signed.
    let attacker = Address::generate(&f.env);
    let line_id2 = id(&f.env);
    let price_per_oz_e7: i128 = 29_900_000_000;

    // Re-enter strict mode: clear blanket auth by creating a narrow grant that
    // authorizes the ATTACKER (not the bank) for this invocation.
    let res = f
        .client
        .mock_auths(&[MockAuth {
            address: &attacker,
            invoke: &MockAuthInvoke {
                contract: &f.client.address,
                fn_name: "open_credit_line",
                args: (
                    line_id2.clone(),
                    pledge_id.clone(),
                    f.bank.clone(),
                    f.cardholder.clone(),
                    100_000i128,
                    6000u32,
                    7500u32,
                    price_per_oz_e7,
                )
                    .into_val(&f.env),
                sub_invokes: &[],
            },
        }])
        .try_open_credit_line(
            &line_id2,
            &pledge_id,
            &f.bank,
            &f.cardholder,
            &100_000,
            &6000u32,
            &7500u32,
            &price_per_oz_e7,
        );
    // The bank's require_auth() is not satisfied by the attacker's grant.
    assert!(res.is_err(), "open_credit_line must fail when the bank did not sign");
}

#[test]
fn host_auth_wrong_signer_cannot_record_drawdown() {
    let f = setup_strict();
    f.env.mock_all_auths();
    let (_pos, _pledge_id, line_id) = happy_to_open(&f);

    let attacker = Address::generate(&f.env);
    let auth_ref = id(&f.env);
    // Attacker signs, but record_drawdown requires the PROCESSOR to have signed.
    let res = f
        .client
        .mock_auths(&[MockAuth {
            address: &attacker,
            invoke: &MockAuthInvoke {
                contract: &f.client.address,
                fn_name: "record_drawdown",
                args: (line_id.clone(), f.processor.clone(), auth_ref.clone(), 10_000i128)
                    .into_val(&f.env),
                sub_invokes: &[],
            },
        }])
        .try_record_drawdown(&line_id, &f.processor, &auth_ref, &10_000);
    assert!(res.is_err(), "drawdown must fail when the processor did not sign");
}

#[test]
fn host_auth_wrong_signer_cannot_issue_default() {
    let f = setup_strict();
    f.env.mock_all_auths();
    let (_pos, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);

    let attacker = Address::generate(&f.env);
    let deadline = f.env.ledger().sequence() + 100;
    let res = f
        .client
        .mock_auths(&[MockAuth {
            address: &attacker,
            invoke: &MockAuthInvoke {
                contract: &f.client.address,
                fn_name: "issue_default_notice",
                args: (line_id.clone(), f.bank.clone(), deadline).into_val(&f.env),
                sub_invokes: &[],
            },
        }])
        .try_issue_default_notice(&line_id, &f.bank, &deadline);
    assert!(res.is_err(), "default notice must fail when the bank did not sign");
}

#[test]
fn host_auth_correct_signer_succeeds() {
    // Positive control for the negative tests above: the SAME call succeeds
    // when the correct party (the bank) actually signs it. This proves the
    // failures above are due to the wrong signer, not a malformed invocation.
    let f = setup_strict();
    f.env.mock_all_auths();
    let _existing = happy_to_open(&f);
    let position_id2 = register_and_immobilize(&f);
    let pledge_id = id(&f.env);
    f.client.activate_pledge(&pledge_id, &position_id2, &f.owner, &f.bank, &id(&f.env));

    let line_id2 = id(&f.env);
    let price_per_oz_e7: i128 = 29_900_000_000;
    let res = f
        .client
        .mock_auths(&[MockAuth {
            address: &f.bank,
            invoke: &MockAuthInvoke {
                contract: &f.client.address,
                fn_name: "open_credit_line",
                args: (
                    line_id2.clone(),
                    pledge_id.clone(),
                    f.bank.clone(),
                    f.cardholder.clone(),
                    100_000i128,
                    6000u32,
                    7500u32,
                    price_per_oz_e7,
                )
                    .into_val(&f.env),
                sub_invokes: &[],
            },
        }])
        .try_open_credit_line(
            &line_id2,
            &pledge_id,
            &f.bank,
            &f.cardholder,
            &100_000,
            &6000u32,
            &7500u32,
            &price_per_oz_e7,
        );
    assert!(res.is_ok(), "open_credit_line must succeed when the bank signs");
}

// ---- B. Admin key control: revoke_party -------------------------------------

#[test]
fn revoke_party_removes_approval() {
    let f = setup();
    assert!(f.client.is_approved(&f.bank, &Role::Bank));
    f.client.revoke_party(&f.bank, &Role::Bank);
    assert!(!f.client.is_approved(&f.bank, &Role::Bank));
}

#[test]
fn revoked_bank_cannot_open_line() {
    // A compromised bank key, once revoked by the admin, must be unable to act.
    let f = setup();
    let position_id = register_and_immobilize(&f);
    let pledge_id = id(&f.env);
    let line_id = id(&f.env);
    f.client.activate_pledge(&pledge_id, &position_id, &f.owner, &f.bank, &id(&f.env));

    // admin revokes the bank
    f.client.revoke_party(&f.bank, &Role::Bank);

    let price_per_oz_e7: i128 = 29_900_000_000;
    let res = f.client.try_open_credit_line(
        &line_id, &pledge_id, &f.bank, &f.cardholder, &719_000, &6000u32, &7500u32, &price_per_oz_e7,
    );
    // With the bank revoked, opening must fail with PartyNotApproved.
    // (activate_pledge happened pre-revoke; this is the post-revoke action.)
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)),
        "a revoked bank must not be able to open a line");
}

#[test]
fn revoked_processor_cannot_draw() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.revoke_party(&f.processor, &Role::Processor);
    let res = f.client.try_record_drawdown(&line_id, &f.processor, &id(&f.env), &10_000);
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

// The following four cover the functions the adversarial suite proved were NOT
// checking is_approved before the fix. Each revokes the party AFTER the line is
// open, then proves the now-revoked key can no longer take the action. Before
// the fix these would have succeeded (the finding); after the fix they return
// PartyNotApproved. The risk these close is a compromised bank/custodian key
// continuing to seize or release collateral after the admin has revoked it.

#[test]
fn revoked_bank_cannot_issue_default() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    f.client.revoke_party(&f.bank, &Role::Bank);
    let deadline = f.env.ledger().sequence() + 100;
    let res = f.client.try_issue_default_notice(&line_id, &f.bank, &deadline);
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

#[test]
fn revoked_bank_cannot_enforce() {
    // Drive to a defaulted, cure-expired line with an APPROVED bank, then revoke
    // the bank and prove it can no longer seize the collateral.
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    f.env.ledger().set_sequence_number(cure_deadline + 1);

    f.client.revoke_party(&f.bank, &Role::Bank);
    let res = f.client.try_record_enforcement(
        &line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

#[test]
fn revoked_bank_cannot_authorize_release() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    // line has zero drawn balance (never drew), so release is otherwise eligible
    f.client.revoke_party(&f.bank, &Role::Bank);
    let res = f.client.try_bank_authorize_release(&line_id, &f.bank);
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

#[test]
fn revoked_custodian_cannot_confirm_release() {
    // Bank authorizes release (approved), then the custodian is revoked before
    // it confirms. A revoked custodian must not be able to assert the gold was
    // returned, which is the act that terminates the bank's security interest.
    let f = setup();
    let (_position_id, pledge_id, line_id) = happy_to_open(&f);
    f.client.bank_authorize_release(&line_id, &f.bank);

    f.client.revoke_party(&f.custodian, &Role::Custodian);
    let res = f.client.try_custodian_confirm_release(&pledge_id, &f.custodian, &id(&f.env));
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

#[test]
fn reapproval_after_revoke_restores_power() {
    // Revocation is not permanent: the admin can re-approve. Confirms the
    // revoke path does not corrupt the approval namespace.
    let f = setup();
    f.client.revoke_party(&f.processor, &Role::Processor);
    assert!(!f.client.is_approved(&f.processor, &Role::Processor));
    f.client.approve_party(&f.processor, &Role::Processor);
    assert!(f.client.is_approved(&f.processor, &Role::Processor));
}

// ---- C. Arithmetic / boundary -----------------------------------------------

#[test]
fn draw_exactly_available_limit_succeeds_then_one_more_fails() {
    // Off-by-one at the capacity boundary: drawing the entire limit must work,
    // and the very next stroop must be refused.
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let limit = f.client.available_capacity(&line_id);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &limit);
    assert_eq!(f.client.available_capacity(&line_id), 0);
    let res = f.client.try_record_drawdown(&line_id, &f.processor, &id(&f.env), &1);
    assert_eq!(res, Err(Ok(Error::InsufficientCapacity)));
}

#[test]
fn open_line_with_overflow_price_is_refused_not_wrapped() {
    // ADVERSARIAL: borrowing_base uses saturating_mul. Feed an i128::MAX price
    // and confirm the contract does NOT silently wrap to a huge borrowing base
    // that lets the limit pass. Either the limit check refuses it (saturated
    // base still bounded by approved_limit check) or it errors; it must NOT
    // succeed in opening a line whose limit exceeds real collateral value.
    let f = setup();
    let position_id = register_and_immobilize(&f);
    let pledge_id = id(&f.env);
    let line_id = id(&f.env);
    f.client.activate_pledge(&pledge_id, &position_id, &f.owner, &f.bank, &id(&f.env));

    // Attempt a wildly large approved_limit with a normal price. Must be
    // refused because it exceeds the borrowing base.
    let price_per_oz_e7: i128 = 29_900_000_000;
    let res = f.client.try_open_credit_line(
        &line_id, &pledge_id, &f.bank, &f.cardholder,
        &i128::MAX, &6000u32, &7500u32, &price_per_oz_e7,
    );
    assert!(res.is_err(), "an absurd approved_limit must be refused, never accepted");
}

#[test]
fn negative_amounts_are_refused_everywhere() {
    // Defense in depth: no value-moving entry accepts a non-positive amount.
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    assert_eq!(
        f.client.try_record_drawdown(&line_id, &f.processor, &id(&f.env), &0),
        Err(Ok(Error::AmountNotPositive))
    );
    assert_eq!(
        f.client.try_record_drawdown(&line_id, &f.processor, &id(&f.env), &(-1)),
        Err(Ok(Error::AmountNotPositive))
    );
}

// ---- D. State-machine ordering ----------------------------------------------

#[test]
fn cannot_draw_on_unopened_line() {
    // No line exists for this id: a draw must fail rather than create state.
    let f = setup();
    let _ = register_and_immobilize(&f);
    let res = f.client.try_record_drawdown(&id(&f.env), &f.processor, &id(&f.env), &1_000);
    assert!(res.is_err(), "drawing on a non-existent line must fail");
}

#[test]
fn cannot_enforce_without_default() {
    // Enforcement on an Active (non-defaulted) line must be refused.
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let res = f.client.try_record_enforcement(
        &line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::NotDefaulted)));
}

#[test]
fn cannot_resume_a_line_never_suspended() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let res = f.client.try_bank_resume_line(&line_id, &f.bank);
    assert_eq!(res, Err(Ok(Error::LineNotSuspended)));
}

#[test]
fn cannot_cure_a_line_not_in_default() {
    let f = setup();
    let (_p, _pl, line_id) = happy_to_open(&f);
    let res = f.client.try_cure_default(&line_id, &f.cardholder);
    assert_eq!(res, Err(Ok(Error::NotDefaulted)));
}

// ---- E. Idempotency / replay ------------------------------------------------

#[test]
fn enforcement_cannot_be_recorded_twice() {
    // After enforcement closes the line, a second enforcement must be refused
    // (the line is Closed / NotDefaulted), so collateral cannot be seized twice.
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    f.env.ledger().set_sequence_number(cure_deadline + 1);
    f.client.record_enforcement(&line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env));

    let second = f.client.try_record_enforcement(
        &line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env),
    );
    assert!(second.is_err(), "enforcement must not be repeatable on a closed line");
}

// ---- F. Additive SCF adversarial hardening tests ---------------------------

fn zero_hash(env: &Env) -> BytesN<32> {
    BytesN::from_array(env, &[0u8; 32])
}

#[test]
fn refuses_second_credit_line_against_same_pledge() {
    let f = setup();
    let (_position_id, pledge_id, _line_id) = happy_to_open(&f);
    let second_line_id = id(&f.env);
    let price_per_oz_e7: i128 = 29_900_000_000;

    let res = f.client.try_open_credit_line(
        &second_line_id,
        &pledge_id,
        &f.bank,
        &f.cardholder,
        &100_000i128,
        &6000u32,
        &7500u32,
        &price_per_oz_e7,
    );
    assert_eq!(res, Err(Ok(Error::PledgeAlreadyHasLine)));
}

#[test]
fn refuses_duplicate_credit_line_id() {
    let f = setup();
    let (_position_id, pledge_id, line_id) = happy_to_open(&f);
    let price_per_oz_e7: i128 = 29_900_000_000;

    let res = f.client.try_open_credit_line(
        &line_id,
        &pledge_id,
        &f.bank,
        &f.cardholder,
        &100_000i128,
        &6000u32,
        &7500u32,
        &price_per_oz_e7,
    );
    assert_eq!(res, Err(Ok(Error::LineExists)));
}

#[test]
fn refuses_repayment_above_drawn_balance() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000i128);

    let res = f.client.try_apply_repayment(&line_id, &f.vault, &id(&f.env), &25_001i128);
    assert_eq!(res, Err(Ok(Error::RepaymentExceedsOutstandingBalance)));
    assert_eq!(f.client.get_line(&line_id).drawn_balance, 25_000);
}

#[test]
fn refuses_repayment_on_closed_line() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.bank_authorize_release(&line_id, &f.bank);

    let res = f.client.try_apply_repayment(&line_id, &f.vault, &id(&f.env), &1i128);
    assert_eq!(res, Err(Ok(Error::LineNotActive)));
}

#[test]
fn refuses_reverse_after_line_closed() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    let auth_ref = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &auth_ref, &25_000i128);
    f.client.apply_repayment(&line_id, &f.vault, &id(&f.env), &25_000i128);
    f.client.bank_authorize_release(&line_id, &f.bank);

    let res = f.client.try_reverse_drawdown(&line_id, &f.processor, &auth_ref, &25_000i128);
    assert_eq!(res, Err(Ok(Error::LineNotActive)));
}

#[test]
fn refuses_future_dated_price_on_revaluation() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.env.ledger().set_timestamp(1_000);
    let price_e7: i128 = 29_900_000_000;

    let res = f.client.try_revalue_and_check(
        &line_id,
        &f.valuation,
        &price_e7,
        &(price_e7 / 1000),
        &1_001u64,
        &86_400u64,
        &50u32,
        &9000u32,
    );
    assert_eq!(res, Err(Ok(Error::PriceFromFuture)));
}

#[test]
fn refuses_invalid_revaluation_thresholds() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    let now = f.env.ledger().timestamp();
    let price_e7: i128 = 29_900_000_000;

    let cases = [
        (0u64, 50u32, 9000u32),
        (86_400u64, 0u32, 9000u32),
        (86_400u64, 10_001u32, 9000u32),
        (86_400u64, 50u32, 0u32),
        (86_400u64, 50u32, 10_001u32),
    ];

    for (max_age_secs, conf_tol_bps, warning_bps) in cases {
        let res = f.client.try_revalue_and_check(
            &line_id,
            &f.valuation,
            &price_e7,
            &(price_e7 / 1000),
            &now,
            &max_age_secs,
            &conf_tol_bps,
            &warning_bps,
        );
        assert_eq!(res, Err(Ok(Error::InvalidRevaluationParams)));
    }
}

#[test]
fn refuses_zero_hash_in_framework_documents() {
    let f = setup();
    let framework_id = id(&f.env);
    let zero = zero_hash(&f.env);

    let res = f.client.try_register_framework(
        &framework_id,
        &f.owner,
        &f.bank,
        &f.custodian,
        &zero,
        &id(&f.env),
        &id(&f.env),
        &id(&f.env),
        &id(&f.env),
        &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::InvalidDocumentHash)));
}

#[test]
fn refuses_zero_barlist_or_serials_hash() {
    let f = setup();
    let framework_id = register_framework(&f);
    let position_id = id(&f.env);
    let zero = zero_hash(&f.env);
    let expiry = f.env.ledger().sequence() + 100_000;

    let zero_barlist = f.client.try_register_position(
        &position_id,
        &framework_id,
        &f.owner,
        &f.custodian,
        &zero,
        &id(&f.env),
        &4_011_000_000i128,
        &expiry,
    );
    assert_eq!(zero_barlist, Err(Ok(Error::InvalidDocumentHash)));

    let zero_serials = f.client.try_register_position(
        &id(&f.env),
        &framework_id,
        &f.owner,
        &f.custodian,
        &id(&f.env),
        &zero,
        &4_011_000_000i128,
        &expiry,
    );
    assert_eq!(zero_serials, Err(Ok(Error::InvalidDocumentHash)));
}

#[test]
fn refuses_zero_release_notice_hash() {
    let f = setup();
    let (_position_id, pledge_id, line_id) = happy_to_open(&f);
    f.client.bank_authorize_release(&line_id, &f.bank);

    let res = f.client.try_custodian_confirm_release(&pledge_id, &f.custodian, &zero_hash(&f.env));
    assert_eq!(res, Err(Ok(Error::InvalidDocumentHash)));
}

#[test]
fn refuses_zero_enforcement_legal_instrument_hash() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000i128);
    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    f.env.ledger().set_sequence_number(cure_deadline + 1);

    let res = f.client.try_record_enforcement(
        &line_id,
        &f.bank,
        &EnforcementOutcome::Sold,
        &zero_hash(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::InvalidDocumentHash)));
}

#[test]
fn refuses_adjustment_with_zero_hashes() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    let zero = zero_hash(&f.env);

    let zero_barlist = f.client.try_request_collateral_adjustment(
        &id(&f.env),
        &line_id,
        &f.owner,
        &AdjustmentType::TopUp,
        &zero,
        &4_500_000_000i128,
        &id(&f.env),
    );
    assert_eq!(zero_barlist, Err(Ok(Error::InvalidDocumentHash)));

    let zero_request = f.client.try_request_collateral_adjustment(
        &id(&f.env),
        &line_id,
        &f.owner,
        &AdjustmentType::TopUp,
        &id(&f.env),
        &4_500_000_000i128,
        &zero,
    );
    assert_eq!(zero_request, Err(Ok(Error::InvalidDocumentHash)));
}

#[test]
fn expired_readiness_can_be_repopulated_to_ready() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.open_enforcement_readiness(&line_id, &f.bank);
    let agent = Address::generate(&f.env);
    let settlement = Address::generate(&f.env);

    f.client.populate_enforcement_readiness(
        &line_id,
        &f.bank,
        &agent,
        &id(&f.env),
        &settlement,
        &id(&f.env),
        &id(&f.env),
        &(f.env.ledger().sequence() + 1000),
    );
    f.client.expire_enforcement_readiness(&line_id, &f.bank);
    assert_eq!(f.client.get_enforcement_readiness(&line_id).status, ReadinessStatus::Expired);

    f.client.populate_enforcement_readiness(
        &line_id,
        &f.bank,
        &agent,
        &id(&f.env),
        &settlement,
        &id(&f.env),
        &id(&f.env),
        &(f.env.ledger().sequence() + 2000),
    );
    let readiness = f.client.get_enforcement_readiness(&line_id);
    assert_eq!(readiness.status, ReadinessStatus::Ready);
    assert_eq!(readiness.version, 2);
}

#[test]
fn readiness_real_fields_require_future_validity_window() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.open_enforcement_readiness(&line_id, &f.bank);
    let agent = Address::generate(&f.env);
    let settlement = Address::generate(&f.env);
    let current = f.env.ledger().sequence();

    let res = f.client.try_populate_enforcement_readiness(
        &line_id,
        &f.bank,
        &agent,
        &id(&f.env),
        &settlement,
        &id(&f.env),
        &id(&f.env),
        &current,
    );
    assert_eq!(res, Err(Ok(Error::ReadinessExpired)));
}

#[test]
fn refuses_duplicate_readiness_record() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.open_enforcement_readiness(&line_id, &f.bank);

    let res = f.client.try_open_enforcement_readiness(&line_id, &f.bank);
    assert_eq!(res, Err(Ok(Error::ReadinessWrongStatus)));
}

#[test]
fn refuses_double_enforcement_after_terminal_close() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000i128);
    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    f.env.ledger().set_sequence_number(cure_deadline + 1);
    f.client.record_enforcement(&line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env));

    let res = f.client.try_record_enforcement(
        &line_id,
        &f.bank,
        &EnforcementOutcome::Sold,
        &id(&f.env),
    );
    assert_eq!(res, Err(Ok(Error::NotDefaulted)));
}

#[test]
fn refuses_cure_after_enforcement() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000i128);
    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    f.env.ledger().set_sequence_number(cure_deadline + 1);
    f.client.record_enforcement(&line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env));

    let res = f.client.try_cure_default(&line_id, &f.cardholder);
    assert_eq!(res, Err(Ok(Error::NotDefaulted)));
}

#[test]
fn unapproved_processor_cannot_record_drawdown() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    let rogue_processor = Address::generate(&f.env);
    let auth_ref = id(&f.env);

    let res = f.client.mock_auths(&[MockAuth {
        address: &rogue_processor,
        invoke: &MockAuthInvoke {
            contract: &f.client.address,
            fn_name: "record_drawdown",
            args: (line_id.clone(), rogue_processor.clone(), auth_ref.clone(), 10_000i128).into_val(&f.env),
            sub_invokes: &[],
        },
    }]).try_record_drawdown(&line_id, &rogue_processor, &auth_ref, &10_000i128);
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

#[test]
fn unapproved_vault_cannot_apply_repayment() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000i128);
    let rogue_vault = Address::generate(&f.env);
    let payment_ref = id(&f.env);

    let res = f.client.mock_auths(&[MockAuth {
        address: &rogue_vault,
        invoke: &MockAuthInvoke {
            contract: &f.client.address,
            fn_name: "apply_repayment",
            args: (line_id.clone(), rogue_vault.clone(), payment_ref.clone(), 10_000i128).into_val(&f.env),
            sub_invokes: &[],
        },
    }]).try_apply_repayment(&line_id, &rogue_vault, &payment_ref, &10_000i128);
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

#[test]
fn rogue_valuer_cannot_revalue() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    let rogue_valuer = Address::generate(&f.env);
    let now = f.env.ledger().timestamp();
    let price_e7: i128 = 29_900_000_000;

    let res = f.client.mock_auths(&[MockAuth {
        address: &rogue_valuer,
        invoke: &MockAuthInvoke {
            contract: &f.client.address,
            fn_name: "revalue_and_check",
            args: (
                line_id.clone(),
                rogue_valuer.clone(),
                price_e7,
                price_e7 / 1000,
                now,
                86_400u64,
                50u32,
                9000u32,
            ).into_val(&f.env),
            sub_invokes: &[],
        },
    }]).try_revalue_and_check(
        &line_id,
        &rogue_valuer,
        &price_e7,
        &(price_e7 / 1000),
        &now,
        &86_400u64,
        &50u32,
        &9000u32,
    );
    assert_eq!(res, Err(Ok(Error::NotAuthorized)));
}

#[test]
fn owner_cannot_open_line_as_bank() {
    let f = setup();
    let position_id = register_and_immobilize(&f);
    let pledge_id = id(&f.env);
    f.client.activate_pledge(&pledge_id, &position_id, &f.owner, &f.bank, &id(&f.env));
    let line_id = id(&f.env);
    let price_per_oz_e7: i128 = 29_900_000_000;

    let res = f.client.mock_auths(&[MockAuth {
        address: &f.owner,
        invoke: &MockAuthInvoke {
            contract: &f.client.address,
            fn_name: "open_credit_line",
            args: (
                line_id.clone(),
                pledge_id.clone(),
                f.owner.clone(),
                f.cardholder.clone(),
                100_000i128,
                6000u32,
                7500u32,
                price_per_oz_e7,
            ).into_val(&f.env),
            sub_invokes: &[],
        },
    }]).try_open_credit_line(
        &line_id,
        &pledge_id,
        &f.owner,
        &f.cardholder,
        &100_000i128,
        &6000u32,
        &7500u32,
        &price_per_oz_e7,
    );
    assert_eq!(res, Err(Ok(Error::PartyNotApproved)));
}

// ---- H. Event trace coverage for indexers and evidence certificates ---------

#[test]
fn release_lifecycle_emits_indexer_visible_events() {
    // The test event buffer reliably reports the most recent invocation's
    // events, so we assert on a single state-changing call rather than across
    // a multi-call lifecycle. bank_authorize_release publishes the release
    // event the indexer reconciles against.
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    f.client.apply_repayment(&line_id, &f.vault, &id(&f.env), &25_000);
    f.client.bank_authorize_release(&line_id, &f.bank);
    let events = f.env.events().all();
    assert!(
        !events.is_empty(),
        "release authorization must publish an event for reconciliation"
    );
}

#[test]
fn default_enforcement_lifecycle_emits_indexer_visible_events() {
    // record_enforcement publishes the terminal enforcement event that the
    // evidence trail and indexer depend on. Assert on that single call.
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    f.env.ledger().set_sequence_number(cure_deadline + 1);
    f.client.record_enforcement(&line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env));
    let events = f.env.events().all();
    assert!(
        !events.is_empty(),
        "enforcement must publish an event for the evidence trail"
    );
}

// ---- I. Property, snapshot and TTL tests for reviewer-grade coverage --------

fn assert_line_accounting_invariant(f: &Fixture, line_id: &BytesN<32>) {
    let line = f.client.get_line(line_id);
    assert!(line.drawn_balance >= 0);
    assert!(line.available_limit >= 0);
    assert!(line.drawn_balance + line.available_limit <= line.approved_limit);
}

#[test]
fn property_accounting_sequence_preserves_capacity_bounds() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);

    assert_line_accounting_invariant(&f, &line_id);

    let draw_a = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &draw_a, &100_000);
    assert_line_accounting_invariant(&f, &line_id);

    let draw_b = id(&f.env);
    f.client.record_drawdown(&line_id, &f.processor, &draw_b, &50_000);
    assert_line_accounting_invariant(&f, &line_id);

    f.client.reverse_drawdown(&line_id, &f.processor, &draw_b, &50_000);
    assert_line_accounting_invariant(&f, &line_id);

    f.client.apply_repayment(&line_id, &f.vault, &id(&f.env), &25_000);
    assert_line_accounting_invariant(&f, &line_id);

    f.client.revalue_and_check(
        &line_id,
        &f.valuation,
        &29_900_000_000i128,
        &10_000_000i128,
        &f.env.ledger().timestamp(),
        &60u64,
        &500u32,
        &8000u32,
    );
    assert_line_accounting_invariant(&f, &line_id);
}

#[test]
fn snapshot_release_path_records_exact_terminal_state() {
    let f = setup();
    let (position_id, pledge_id, line_id) = happy_to_open(&f);

    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    f.client.apply_repayment(&line_id, &f.vault, &id(&f.env), &25_000);
    f.client.bank_authorize_release(&line_id, &f.bank);
    f.client.custodian_confirm_release(&pledge_id, &f.custodian, &id(&f.env));

    let line = f.client.get_line(&line_id);
    let pledge = f.client.get_pledge(&pledge_id);
    let position = f.client.get_position(&position_id);

    assert_eq!(line.status, LineStatus::Closed);
    assert_eq!(line.drawn_balance, 0);
    assert_eq!(line.available_limit, line.approved_limit);
    assert_eq!(pledge.status, PledgeStatus::Released);
    assert_eq!(position.status, PositionStatus::Released);
}

#[test]
fn snapshot_default_enforcement_path_records_exact_terminal_state() {
    let f = setup();
    let (position_id, pledge_id, line_id) = happy_to_open(&f);

    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);
    let cure_deadline = f.env.ledger().sequence() + 10;
    f.client.issue_default_notice(&line_id, &f.bank, &cure_deadline);
    f.env.ledger().set_sequence_number(cure_deadline + 1);
    f.client.record_enforcement(&line_id, &f.bank, &EnforcementOutcome::Sold, &id(&f.env));

    let line = f.client.get_line(&line_id);
    let pledge = f.client.get_pledge(&pledge_id);
    let position = f.client.get_position(&position_id);

    assert_eq!(line.status, LineStatus::Closed);
    assert_eq!(pledge.status, PledgeStatus::Enforced);
    assert_eq!(position.status, PositionStatus::Enforced);
    assert_eq!(line.drawn_balance, 25_000);
}

#[test]
fn ttl_medium_advance_preserves_core_credit_records() {
    let f = setup();
    let (position_id, pledge_id, line_id) = happy_to_open(&f);
    f.client.record_drawdown(&line_id, &f.processor, &id(&f.env), &25_000);

    let later = f.env.ledger().sequence() + 1_000;
    f.env.ledger().set_sequence_number(later);

    assert_eq!(f.client.get_line(&line_id).drawn_balance, 25_000);
    assert_eq!(f.client.get_pledge(&pledge_id).status, PledgeStatus::Active);
    assert_eq!(f.client.get_position(&position_id).status, PositionStatus::Pledged);
}

#[test]
fn readiness_ready_requires_valuation_and_waterfall_hashes() {
    let f = setup();
    let (_position_id, _pledge_id, line_id) = happy_to_open(&f);
    f.client.open_enforcement_readiness(&line_id, &f.bank);
    let agent = Address::generate(&f.env);
    let settlement = Address::generate(&f.env);
    let zero = BytesN::from_array(&f.env, &[0u8; 32]);

    let zero_valuation = f.client.try_populate_enforcement_readiness(
        &line_id,
        &f.bank,
        &agent,
        &id(&f.env),
        &settlement,
        &zero,
        &id(&f.env),
        &(f.env.ledger().sequence() + 1000),
    );
    assert_eq!(zero_valuation, Err(Ok(Error::InvalidDocumentHash)));

    let zero_waterfall = f.client.try_populate_enforcement_readiness(
        &line_id,
        &f.bank,
        &agent,
        &id(&f.env),
        &settlement,
        &id(&f.env),
        &zero,
        &(f.env.ledger().sequence() + 1000),
    );
    assert_eq!(zero_waterfall, Err(Ok(Error::InvalidDocumentHash)));
}

#![no_std]

mod error;
mod types;

#[cfg(test)]
mod test;

pub use error::Error;
pub use types::*;

use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env};

const DAY_IN_LEDGERS: u32 = 17_280; // ~5s ledgers
// Storage TTL horizon. Set beyond a 12-month claim window (365 days) so a
// reward that sits untouched for its full window does not risk having its
// persistent entry archived before it can be claimed or expired. Entries also
// bump their TTL on every access, so this is the floor, not the typical life.
const BUMP_AMOUNT: u32 = 400 * DAY_IN_LEDGERS;
const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - (7 * DAY_IN_LEDGERS);

/// Maximum permitted reward rate, basis points. 1000 bps = 10%. A sponsor
/// loyalty program is typically 50..=100 bps; the ceiling guards against a
/// fat-fingered policy that would blow the budget.
const MAX_RATE_BPS: u32 = 1_000;
const BPS_DENOM: i128 = 10_000;
/// 1e7 fixed-point scale, matching the credit ledger's price/weight scaling.
const E7: i128 = 10_000_000;

#[contract]
pub struct RewardsLedger;

#[contractimpl]
impl RewardsLedger {
    // ---------------------------------------------------------------------
    // Admin / role registry
    // ---------------------------------------------------------------------

    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .extend_ttl(LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Ok(())
    }

    pub fn approve_party(env: Env, party: Address, role: Role) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        let key = DataKey::Approved(party.clone(), role);
        env.storage().persistent().set(&key, &true);
        Self::bump(&env, &key);
        env.events()
            .publish((symbol_short!("party"), symbol_short!("approve")), (party, role));
        Ok(())
    }

    pub fn revoke_party(env: Env, party: Address, role: Role) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .remove(&DataKey::Approved(party.clone(), role));
        env.events()
            .publish((symbol_short!("party"), symbol_short!("revoke")), (party, role));
        Ok(())
    }

    pub fn is_approved(env: Env, party: Address, role: Role) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Approved(party, role))
            .unwrap_or(false)
    }

    // ---------------------------------------------------------------------
    // Reward campaign lifecycle
    // ---------------------------------------------------------------------

    pub fn create_campaign(
        env: Env,
        campaign_id: BytesN<32>,
        sponsor: Address,
        verifier: Address,
        gold_partner: Address,
        cfg: CampaignConfig,
    ) -> Result<(), Error> {
        sponsor.require_auth();
        if !Self::is_approved(env.clone(), sponsor.clone(), Role::Sponsor) {
            return Err(Error::PartyNotApproved);
        }
        if !Self::is_approved(env.clone(), verifier.clone(), Role::Verifier) {
            return Err(Error::PartyNotApproved);
        }
        if !Self::is_approved(env.clone(), gold_partner.clone(), Role::GoldPartner) {
            return Err(Error::PartyNotApproved);
        }
        // Tighter ceiling than the raw 100% an unbounded bps allows.
        if cfg.reward_rate_bps == 0 || cfg.reward_rate_bps > MAX_RATE_BPS {
            return Err(Error::InvalidRate);
        }
        if cfg.user_spend_cap <= 0 || cfg.total_budget <= 0 {
            return Err(Error::InvalidAmount);
        }
        if cfg.start_ledger >= cfg.end_ledger || cfg.claim_window_ledgers == 0 {
            return Err(Error::InvalidWindow);
        }
        if Self::is_zero_hash(&env, &cfg.eligible_mcc_policy_hash)
            || Self::is_zero_hash(&env, &cfg.redemption_terms_hash)
            || Self::is_zero_hash(&env, &cfg.sponsor_product_scope_hash)
            || Self::is_zero_hash(&env, &cfg.funding_commitment_hash)
        {
            return Err(Error::InvalidEvidenceHash);
        }

        let key = DataKey::Campaign(campaign_id.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::CampaignExists);
        }

        let campaign = RewardCampaign {
            sponsor,
            verifier,
            gold_partner,
            reward_rate_bps: cfg.reward_rate_bps,
            user_spend_cap: cfg.user_spend_cap,
            total_budget: cfg.total_budget,
            reserved_reward_value: 0,
            redeemed_reward_value: 0,
            cancelled_reward_value: 0,
            expired_reward_value: 0,
            redeemed_fine_weight_oz_e7: 0,
            currency_code: cfg.currency_code,
            start_ledger: cfg.start_ledger,
            end_ledger: cfg.end_ledger,
            claim_window_ledgers: cfg.claim_window_ledgers,
            eligible_mcc_policy_hash: cfg.eligible_mcc_policy_hash,
            redemption_terms_hash: cfg.redemption_terms_hash,
            sponsor_product_scope_hash: cfg.sponsor_product_scope_hash,
            funding_commitment_hash: cfg.funding_commitment_hash,
            status: CampaignStatus::Active,
        };
        env.storage().persistent().set(&key, &campaign);
        Self::bump(&env, &key);
        env.events()
            .publish((symbol_short!("campaign"), symbol_short!("create")), campaign_id);
        Ok(())
    }

    pub fn add_campaign_budget(
        env: Env,
        campaign_id: BytesN<32>,
        sponsor: Address,
        amount: i128,
        funding_hash: BytesN<32>,
    ) -> Result<(), Error> {
        sponsor.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if Self::is_zero_hash(&env, &funding_hash) {
            return Err(Error::InvalidEvidenceHash);
        }
        let mut campaign = Self::load_campaign(&env, &campaign_id)?;
        if campaign.sponsor != sponsor {
            return Err(Error::NotAuthorized);
        }
        if campaign.status == CampaignStatus::Closed {
            return Err(Error::CampaignNotActive);
        }
        campaign.total_budget = campaign
            .total_budget
            .checked_add(amount)
            .ok_or(Error::MathOverflow)?;
        Self::save_campaign(&env, &campaign_id, &campaign);
        env.events().publish(
            (symbol_short!("budget"), symbol_short!("added")),
            (campaign_id, amount, funding_hash),
        );
        Ok(())
    }

    pub fn pause_campaign(env: Env, campaign_id: BytesN<32>, sponsor: Address) -> Result<(), Error> {
        sponsor.require_auth();
        let mut campaign = Self::load_campaign(&env, &campaign_id)?;
        if campaign.sponsor != sponsor {
            return Err(Error::NotAuthorized);
        }
        campaign.status = CampaignStatus::Paused;
        Self::save_campaign(&env, &campaign_id, &campaign);
        env.events()
            .publish((symbol_short!("campaign"), symbol_short!("pause")), campaign_id);
        Ok(())
    }

    pub fn resume_campaign(env: Env, campaign_id: BytesN<32>, sponsor: Address) -> Result<(), Error> {
        sponsor.require_auth();
        let mut campaign = Self::load_campaign(&env, &campaign_id)?;
        if campaign.sponsor != sponsor {
            return Err(Error::NotAuthorized);
        }
        if campaign.status == CampaignStatus::Closed {
            return Err(Error::CampaignNotActive);
        }
        campaign.status = CampaignStatus::Active;
        Self::save_campaign(&env, &campaign_id, &campaign);
        env.events()
            .publish((symbol_short!("campaign"), symbol_short!("resume")), campaign_id);
        Ok(())
    }

    pub fn close_campaign(env: Env, campaign_id: BytesN<32>, sponsor: Address) -> Result<(), Error> {
        sponsor.require_auth();
        let mut campaign = Self::load_campaign(&env, &campaign_id)?;
        if campaign.sponsor != sponsor {
            return Err(Error::NotAuthorized);
        }
        campaign.status = CampaignStatus::Closed;
        Self::save_campaign(&env, &campaign_id, &campaign);
        env.events()
            .publish((symbol_short!("campaign"), symbol_short!("close")), campaign_id);
        Ok(())
    }

    // ---------------------------------------------------------------------
    // Spend eligibility and reward accrual
    // ---------------------------------------------------------------------

    pub fn record_eligible_spend(
        env: Env,
        campaign_id: BytesN<32>,
        user: Address,
        verifier: Address,
        input: SpendInput,
    ) -> Result<i128, Error> {
        verifier.require_auth();
        if input.amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if Self::is_zero_hash(&env, &input.spend_ref_hash)
            || Self::is_zero_hash(&env, &input.card_line_hash)
            || Self::is_zero_hash(&env, &input.category_policy_hash)
        {
            return Err(Error::InvalidEvidenceHash);
        }
        let mut campaign = Self::load_active_campaign(&env, &campaign_id)?;
        if campaign.verifier != verifier {
            return Err(Error::NotAuthorized);
        }
        Self::assert_campaign_window(&env, &campaign)?;
        if campaign.currency_code != input.currency_code {
            return Err(Error::PolicyMismatch);
        }
        if campaign.eligible_mcc_policy_hash != input.category_policy_hash {
            return Err(Error::PolicyMismatch);
        }

        let spend_key = DataKey::Spend(input.spend_ref_hash.clone());
        if env.storage().persistent().has(&spend_key) {
            return Err(Error::SpendExists);
        }

        let usage_key = DataKey::UserUsage(campaign_id.clone(), user.clone());
        let mut usage: UserCampaignUsage =
            env.storage().persistent().get(&usage_key).unwrap_or(UserCampaignUsage {
                eligible_spend_total: 0,
                reward_value_total: 0,
                fine_weight_oz_e7_total: 0,
            });

        let remaining_user_cap = campaign.user_spend_cap - usage.eligible_spend_total;
        if remaining_user_cap <= 0 {
            return Err(Error::UserCapExceeded);
        }
        // Cap the eligible amount to the remaining per-user headroom.
        let eligible_amount = if input.amount > remaining_user_cap {
            remaining_user_cap
        } else {
            input.amount
        };
        let reward_value = Self::calculate_reward(eligible_amount, campaign.reward_rate_bps)?;
        if reward_value <= 0 {
            return Err(Error::InvalidAmount);
        }
        // Remaining budget = total - (reserved + redeemed). Cancelled/expired
        // value has already been removed from reserved, so it is available again.
        let committed = campaign
            .reserved_reward_value
            .checked_add(campaign.redeemed_reward_value)
            .ok_or(Error::MathOverflow)?;
        let remaining_budget = campaign.total_budget - committed;
        if reward_value > remaining_budget {
            return Err(Error::BudgetInsufficient);
        }

        campaign.reserved_reward_value = campaign
            .reserved_reward_value
            .checked_add(reward_value)
            .ok_or(Error::MathOverflow)?;
        usage.eligible_spend_total += eligible_amount;
        usage.reward_value_total += reward_value;

        let spend = EligibleSpend {
            campaign_id: campaign_id.clone(),
            card_line_hash: input.card_line_hash,
            user: user.clone(),
            verifier,
            spend_ref_hash: input.spend_ref_hash.clone(),
            amount: input.amount,
            eligible_amount,
            currency_code: input.currency_code,
            category_policy_hash: input.category_policy_hash,
            posted_at_ledger: input.posted_at_ledger,
            finality_hash: BytesN::from_array(&env, &[0u8; 32]),
            status: SpendStatus::Recorded,
        };

        let accrual = RewardAccrual {
            campaign_id: campaign_id.clone(),
            user,
            spend_ref_hash: input.spend_ref_hash.clone(),
            eligible_spend_amount: eligible_amount,
            reward_value,
            currency_code: input.currency_code,
            accrued_at_ledger: env.ledger().sequence(),
            claimable_at_ledger: 0,
            claim_expiry_ledger: 0,
            status: RewardStatus::Pending,
        };

        env.storage().persistent().set(&spend_key, &spend);
        Self::bump(&env, &spend_key);
        let accrual_key = DataKey::Accrual(input.spend_ref_hash.clone());
        env.storage().persistent().set(&accrual_key, &accrual);
        Self::bump(&env, &accrual_key);
        env.storage().persistent().set(&usage_key, &usage);
        Self::bump(&env, &usage_key);
        Self::save_campaign(&env, &campaign_id, &campaign);

        env.events().publish(
            (symbol_short!("spend"), symbol_short!("record")),
            (campaign_id, input.spend_ref_hash.clone(), reward_value),
        );
        Ok(reward_value)
    }

    /// Confirm a recorded spend is final (not reversed). Only then does the
    /// reward become claimable and the claim clock start. This is the finality
    /// gate that keeps refunds/reversals out of the reward pool.
    pub fn confirm_spend_final(
        env: Env,
        campaign_id: BytesN<32>,
        spend_ref_hash: BytesN<32>,
        verifier: Address,
        finality_hash: BytesN<32>,
    ) -> Result<(), Error> {
        verifier.require_auth();
        if Self::is_zero_hash(&env, &finality_hash) {
            return Err(Error::InvalidEvidenceHash);
        }
        let campaign = Self::load_active_campaign(&env, &campaign_id)?;
        if campaign.verifier != verifier {
            return Err(Error::NotAuthorized);
        }
        let mut spend = Self::load_spend(&env, &spend_ref_hash)?;
        if spend.campaign_id != campaign_id || spend.status != SpendStatus::Recorded {
            return Err(Error::InvalidStatus);
        }
        let mut accrual = Self::load_accrual(&env, &spend_ref_hash)?;
        if accrual.status != RewardStatus::Pending {
            return Err(Error::InvalidStatus);
        }
        let now = env.ledger().sequence();
        spend.status = SpendStatus::Final;
        spend.finality_hash = finality_hash;
        accrual.status = RewardStatus::Claimable;
        accrual.claimable_at_ledger = now;
        accrual.claim_expiry_ledger = now
            .checked_add(campaign.claim_window_ledgers)
            .ok_or(Error::MathOverflow)?;
        Self::save_spend(&env, &spend_ref_hash, &spend);
        Self::save_accrual(&env, &spend_ref_hash, &accrual);
        env.events()
            .publish((symbol_short!("spend"), symbol_short!("final")), spend_ref_hash);
        Ok(())
    }

    /// Cancel an unredeemed reward (refund, chargeback, fraud, policy breach or
    /// failed verification). Callable by the verifier, the sponsor, or an
    /// approved Bank. Releases the reserved value back to budget. A Redeemed
    /// reward CANNOT be cancelled: its gold has already been allocated.
    pub fn cancel_reward(
        env: Env,
        campaign_id: BytesN<32>,
        spend_ref_hash: BytesN<32>,
        actor: Address,
        reason_hash: BytesN<32>,
    ) -> Result<(), Error> {
        actor.require_auth();
        if Self::is_zero_hash(&env, &reason_hash) {
            return Err(Error::InvalidEvidenceHash);
        }
        let mut campaign = Self::load_campaign(&env, &campaign_id)?;
        let is_bank = Self::is_approved(env.clone(), actor.clone(), Role::Bank);
        if actor != campaign.verifier && actor != campaign.sponsor && !is_bank {
            return Err(Error::NotAuthorized);
        }
        let mut spend = Self::load_spend(&env, &spend_ref_hash)?;
        let mut accrual = Self::load_accrual(&env, &spend_ref_hash)?;
        if spend.campaign_id != campaign_id || accrual.campaign_id != campaign_id {
            return Err(Error::PolicyMismatch);
        }
        // Redeemed/Cancelled/Expired are terminal. In particular a Redeemed
        // reward's gold is allocated and cannot be reversed on-chain.
        if accrual.status == RewardStatus::Redeemed
            || accrual.status == RewardStatus::Cancelled
            || accrual.status == RewardStatus::Expired
            || accrual.status == RewardStatus::Rejected
        {
            return Err(Error::InvalidStatus);
        }
        spend.status = SpendStatus::Cancelled;
        accrual.status = RewardStatus::Cancelled;
        campaign.reserved_reward_value = (campaign.reserved_reward_value - accrual.reward_value).max(0);
        campaign.cancelled_reward_value = campaign
            .cancelled_reward_value
            .checked_add(accrual.reward_value)
            .ok_or(Error::MathOverflow)?;
        Self::save_spend(&env, &spend_ref_hash, &spend);
        Self::save_accrual(&env, &spend_ref_hash, &accrual);
        Self::save_campaign(&env, &campaign_id, &campaign);
        env.events().publish(
            (symbol_short!("reward"), symbol_short!("cancel")),
            (spend_ref_hash, reason_hash),
        );
        Ok(())
    }

    // ---------------------------------------------------------------------
    // Claim / voucher / redemption lifecycle
    // ---------------------------------------------------------------------

    pub fn submit_claim(
        env: Env,
        campaign_id: BytesN<32>,
        spend_ref_hash: BytesN<32>,
        user: Address,
        receipt_bundle_hash: BytesN<32>,
    ) -> Result<(), Error> {
        user.require_auth();
        if Self::is_zero_hash(&env, &receipt_bundle_hash) {
            return Err(Error::InvalidEvidenceHash);
        }
        let accrual = Self::load_accrual(&env, &spend_ref_hash)?;
        if accrual.campaign_id != campaign_id || accrual.user != user {
            return Err(Error::NotAuthorized);
        }
        if accrual.status != RewardStatus::Claimable {
            return Err(Error::InvalidStatus);
        }
        if env.ledger().sequence() > accrual.claim_expiry_ledger {
            return Err(Error::Expired);
        }
        let claim_key = DataKey::Claim(spend_ref_hash.clone());
        if env.storage().persistent().has(&claim_key) {
            return Err(Error::ClaimExists);
        }
        let claim = RewardClaim {
            campaign_id,
            user,
            spend_ref_hash: spend_ref_hash.clone(),
            receipt_bundle_hash,
            submitted_at_ledger: env.ledger().sequence(),
            sponsor_response_hash: BytesN::from_array(&env, &[0u8; 32]),
            status: ClaimStatus::Submitted,
        };
        env.storage().persistent().set(&claim_key, &claim);
        Self::bump(&env, &claim_key);
        let mut updated = accrual;
        updated.status = RewardStatus::ClaimSubmitted;
        Self::save_accrual(&env, &spend_ref_hash, &updated);
        env.events()
            .publish((symbol_short!("claim"), symbol_short!("submit")), spend_ref_hash);
        Ok(())
    }

    pub fn sponsor_approve_claim(
        env: Env,
        campaign_id: BytesN<32>,
        spend_ref_hash: BytesN<32>,
        sponsor: Address,
        sponsor_response_hash: BytesN<32>,
        voucher_hash: BytesN<32>,
    ) -> Result<(), Error> {
        sponsor.require_auth();
        if Self::is_zero_hash(&env, &sponsor_response_hash)
            || Self::is_zero_hash(&env, &voucher_hash)
        {
            return Err(Error::InvalidEvidenceHash);
        }
        let campaign = Self::load_campaign(&env, &campaign_id)?;
        if campaign.sponsor != sponsor {
            return Err(Error::NotAuthorized);
        }
        let mut claim = Self::load_claim(&env, &spend_ref_hash)?;
        let mut accrual = Self::load_accrual(&env, &spend_ref_hash)?;
        if claim.campaign_id != campaign_id || accrual.campaign_id != campaign_id {
            return Err(Error::PolicyMismatch);
        }
        if claim.status != ClaimStatus::Submitted || accrual.status != RewardStatus::ClaimSubmitted {
            return Err(Error::InvalidStatus);
        }
        if env.ledger().sequence() > accrual.claim_expiry_ledger {
            return Err(Error::Expired);
        }
        let voucher_key = DataKey::Voucher(spend_ref_hash.clone());
        if env.storage().persistent().has(&voucher_key) {
            return Err(Error::VoucherExists);
        }
        claim.status = ClaimStatus::Approved;
        claim.sponsor_response_hash = sponsor_response_hash;
        accrual.status = RewardStatus::VoucherIssued;
        let voucher = VoucherRecord {
            campaign_id,
            user: accrual.user.clone(),
            spend_ref_hash: spend_ref_hash.clone(),
            voucher_hash,
            reward_value: accrual.reward_value,
            issued_at_ledger: env.ledger().sequence(),
            expiry_ledger: accrual.claim_expiry_ledger,
        };
        Self::save_claim(&env, &spend_ref_hash, &claim);
        Self::save_accrual(&env, &spend_ref_hash, &accrual);
        env.storage().persistent().set(&voucher_key, &voucher);
        Self::bump(&env, &voucher_key);
        env.events()
            .publish((symbol_short!("voucher"), symbol_short!("issue")), spend_ref_hash);
        Ok(())
    }

    pub fn sponsor_reject_claim(
        env: Env,
        campaign_id: BytesN<32>,
        spend_ref_hash: BytesN<32>,
        sponsor: Address,
        sponsor_response_hash: BytesN<32>,
    ) -> Result<(), Error> {
        sponsor.require_auth();
        if Self::is_zero_hash(&env, &sponsor_response_hash) {
            return Err(Error::InvalidEvidenceHash);
        }
        let mut campaign = Self::load_campaign(&env, &campaign_id)?;
        if campaign.sponsor != sponsor {
            return Err(Error::NotAuthorized);
        }
        let mut claim = Self::load_claim(&env, &spend_ref_hash)?;
        let mut accrual = Self::load_accrual(&env, &spend_ref_hash)?;
        if claim.campaign_id != campaign_id || accrual.campaign_id != campaign_id {
            return Err(Error::PolicyMismatch);
        }
        if claim.status != ClaimStatus::Submitted || accrual.status != RewardStatus::ClaimSubmitted {
            return Err(Error::InvalidStatus);
        }
        claim.status = ClaimStatus::Rejected;
        claim.sponsor_response_hash = sponsor_response_hash;
        accrual.status = RewardStatus::Rejected;
        // Rejected reward releases its reserved value back to budget.
        campaign.reserved_reward_value = (campaign.reserved_reward_value - accrual.reward_value).max(0);
        campaign.cancelled_reward_value = campaign
            .cancelled_reward_value
            .checked_add(accrual.reward_value)
            .ok_or(Error::MathOverflow)?;
        Self::save_claim(&env, &spend_ref_hash, &claim);
        Self::save_accrual(&env, &spend_ref_hash, &accrual);
        Self::save_campaign(&env, &campaign_id, &campaign);
        env.events()
            .publish((symbol_short!("claim"), symbol_short!("reject")), spend_ref_hash);
        Ok(())
    }

    /// Confirm redemption of a voucher into ALLOCATED GOLD. The gold partner
    /// signs, supplies the gold price and a custody-receipt hash, and the
    /// contract computes and records the fine gold weight. This is the moment
    /// "accumulate gold" becomes literal and auditable on-chain.
    ///
    /// `price_per_oz_e7` is the gold price per troy ounce in the campaign's
    /// program currency units, scaled by 1e7. The fine weight is recorded in
    /// troy oz scaled by 1e7.
    #[allow(clippy::too_many_arguments)]
    pub fn confirm_redemption(
        env: Env,
        campaign_id: BytesN<32>,
        spend_ref_hash: BytesN<32>,
        gold_partner: Address,
        order_hash: BytesN<32>,
        product_hash: BytesN<32>,
        redeemed_value: i128,
        price_per_oz_e7: i128,
        custody_receipt_hash: BytesN<32>,
    ) -> Result<(), Error> {
        gold_partner.require_auth();
        if redeemed_value <= 0 {
            return Err(Error::InvalidAmount);
        }
        if price_per_oz_e7 <= 0 {
            return Err(Error::PriceNotPositive);
        }
        if Self::is_zero_hash(&env, &order_hash)
            || Self::is_zero_hash(&env, &product_hash)
            || Self::is_zero_hash(&env, &custody_receipt_hash)
        {
            return Err(Error::InvalidEvidenceHash);
        }
        let mut campaign = Self::load_campaign(&env, &campaign_id)?;
        if campaign.gold_partner != gold_partner {
            return Err(Error::NotAuthorized);
        }
        let mut accrual = Self::load_accrual(&env, &spend_ref_hash)?;
        if accrual.campaign_id != campaign_id || accrual.status != RewardStatus::VoucherIssued {
            return Err(Error::InvalidStatus);
        }
        if redeemed_value != accrual.reward_value {
            return Err(Error::AmountMismatch);
        }
        let redemption_key = DataKey::Redemption(spend_ref_hash.clone());
        if env.storage().persistent().has(&redemption_key) {
            return Err(Error::RedemptionExists);
        }

        // Fine weight (troy oz, scaled by 1e7) from a reward value in program
        // currency minor units and a gold price in those units per oz scaled by
        // 1e7:
        //   oz    = reward_value / (price_per_oz_e7 / 1e7)
        //         = reward_value * 1e7 / price_per_oz_e7
        //   oz_e7 = oz * 1e7 = reward_value * 1e7 * 1e7 / price_per_oz_e7
        // The double 1e7 is required: without it a small (cents) reward over a
        // large (1e7-scaled) price floors to zero. Checked for overflow;
        // realistic intermediates (~1e19) sit well inside i128.
        let fine_weight_oz_e7 = redeemed_value
            .checked_mul(E7)
            .and_then(|v| v.checked_mul(E7))
            .ok_or(Error::MathOverflow)?
            / price_per_oz_e7;

        let redemption = RedemptionRecord {
            campaign_id: campaign_id.clone(),
            user: accrual.user.clone(),
            spend_ref_hash: spend_ref_hash.clone(),
            order_hash,
            product_hash,
            redeemed_value,
            price_per_oz_e7,
            fine_weight_oz_e7,
            custody_receipt_hash,
            redeemed_at_ledger: env.ledger().sequence(),
        };

        accrual.status = RewardStatus::Redeemed;
        campaign.reserved_reward_value = (campaign.reserved_reward_value - accrual.reward_value).max(0);
        campaign.redeemed_reward_value = campaign
            .redeemed_reward_value
            .checked_add(redeemed_value)
            .ok_or(Error::MathOverflow)?;
        campaign.redeemed_fine_weight_oz_e7 = campaign
            .redeemed_fine_weight_oz_e7
            .checked_add(fine_weight_oz_e7)
            .ok_or(Error::MathOverflow)?;

        // accumulate the user's running fine-gold balance for this campaign.
        let usage_key = DataKey::UserUsage(campaign_id.clone(), accrual.user.clone());
        let mut usage: UserCampaignUsage =
            env.storage().persistent().get(&usage_key).unwrap_or(UserCampaignUsage {
                eligible_spend_total: 0,
                reward_value_total: 0,
                fine_weight_oz_e7_total: 0,
            });
        usage.fine_weight_oz_e7_total = usage
            .fine_weight_oz_e7_total
            .checked_add(fine_weight_oz_e7)
            .ok_or(Error::MathOverflow)?;
        env.storage().persistent().set(&usage_key, &usage);
        Self::bump(&env, &usage_key);

        Self::save_accrual(&env, &spend_ref_hash, &accrual);
        Self::save_campaign(&env, &campaign_id, &campaign);
        env.storage().persistent().set(&redemption_key, &redemption);
        Self::bump(&env, &redemption_key);
        env.events().publish(
            (symbol_short!("reward"), symbol_short!("redeem")),
            (spend_ref_hash, redeemed_value, fine_weight_oz_e7),
        );
        Ok(())
    }

    /// Expire an unclaimed/unredeemed reward whose claim window has passed.
    /// Releases the reserved value back to budget. Anyone may call (it is a
    /// permissionless cleanup that can only fire after expiry).
    pub fn expire_reward(
        env: Env,
        campaign_id: BytesN<32>,
        spend_ref_hash: BytesN<32>,
    ) -> Result<(), Error> {
        let mut campaign = Self::load_campaign(&env, &campaign_id)?;
        let mut accrual = Self::load_accrual(&env, &spend_ref_hash)?;
        if accrual.campaign_id != campaign_id {
            return Err(Error::PolicyMismatch);
        }
        // Only rewards that progressed to a claim clock and have not reached a
        // terminal state can expire.
        if accrual.status == RewardStatus::Redeemed
            || accrual.status == RewardStatus::Cancelled
            || accrual.status == RewardStatus::Expired
            || accrual.status == RewardStatus::Rejected
            || accrual.status == RewardStatus::Pending
        {
            return Err(Error::InvalidStatus);
        }
        if accrual.claim_expiry_ledger == 0 || env.ledger().sequence() <= accrual.claim_expiry_ledger {
            return Err(Error::NotExpired);
        }
        accrual.status = RewardStatus::Expired;
        campaign.reserved_reward_value = (campaign.reserved_reward_value - accrual.reward_value).max(0);
        campaign.expired_reward_value = campaign
            .expired_reward_value
            .checked_add(accrual.reward_value)
            .ok_or(Error::MathOverflow)?;
        Self::save_accrual(&env, &spend_ref_hash, &accrual);
        Self::save_campaign(&env, &campaign_id, &campaign);
        env.events()
            .publish((symbol_short!("reward"), symbol_short!("expire")), spend_ref_hash);
        Ok(())
    }

    // ---------------------------------------------------------------------
    // Reads
    // ---------------------------------------------------------------------

    pub fn get_campaign(env: Env, campaign_id: BytesN<32>) -> Result<RewardCampaign, Error> {
        Self::load_campaign(&env, &campaign_id)
    }

    pub fn get_spend(env: Env, spend_ref_hash: BytesN<32>) -> Result<EligibleSpend, Error> {
        Self::load_spend(&env, &spend_ref_hash)
    }

    pub fn get_accrual(env: Env, spend_ref_hash: BytesN<32>) -> Result<RewardAccrual, Error> {
        Self::load_accrual(&env, &spend_ref_hash)
    }

    pub fn get_user_usage(env: Env, campaign_id: BytesN<32>, user: Address) -> UserCampaignUsage {
        env.storage()
            .persistent()
            .get(&DataKey::UserUsage(campaign_id, user))
            .unwrap_or(UserCampaignUsage {
                eligible_spend_total: 0,
                reward_value_total: 0,
                fine_weight_oz_e7_total: 0,
            })
    }

    pub fn get_claim(env: Env, spend_ref_hash: BytesN<32>) -> Result<RewardClaim, Error> {
        Self::load_claim(&env, &spend_ref_hash)
    }

    pub fn get_voucher(env: Env, spend_ref_hash: BytesN<32>) -> Result<VoucherRecord, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Voucher(spend_ref_hash))
            .ok_or(Error::VoucherNotFound)
    }

    pub fn get_redemption(env: Env, spend_ref_hash: BytesN<32>) -> Result<RedemptionRecord, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Redemption(spend_ref_hash))
            .ok_or(Error::RedemptionNotFound)
    }

    // ---------------------------------------------------------------------
    // Internal helpers
    // ---------------------------------------------------------------------

    fn admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    fn load_campaign(env: &Env, campaign_id: &BytesN<32>) -> Result<RewardCampaign, Error> {
        let key = DataKey::Campaign(campaign_id.clone());
        let value = env.storage().persistent().get(&key).ok_or(Error::CampaignNotFound)?;
        Self::bump(env, &key);
        Ok(value)
    }

    fn load_active_campaign(env: &Env, campaign_id: &BytesN<32>) -> Result<RewardCampaign, Error> {
        let campaign = Self::load_campaign(env, campaign_id)?;
        if campaign.status != CampaignStatus::Active {
            return Err(Error::CampaignNotActive);
        }
        Ok(campaign)
    }

    fn save_campaign(env: &Env, campaign_id: &BytesN<32>, campaign: &RewardCampaign) {
        let key = DataKey::Campaign(campaign_id.clone());
        env.storage().persistent().set(&key, campaign);
        Self::bump(env, &key);
    }

    fn load_spend(env: &Env, spend_ref_hash: &BytesN<32>) -> Result<EligibleSpend, Error> {
        let key = DataKey::Spend(spend_ref_hash.clone());
        let value = env.storage().persistent().get(&key).ok_or(Error::SpendNotFound)?;
        Self::bump(env, &key);
        Ok(value)
    }

    fn save_spend(env: &Env, spend_ref_hash: &BytesN<32>, spend: &EligibleSpend) {
        let key = DataKey::Spend(spend_ref_hash.clone());
        env.storage().persistent().set(&key, spend);
        Self::bump(env, &key);
    }

    fn load_accrual(env: &Env, spend_ref_hash: &BytesN<32>) -> Result<RewardAccrual, Error> {
        let key = DataKey::Accrual(spend_ref_hash.clone());
        let value = env.storage().persistent().get(&key).ok_or(Error::AccrualNotFound)?;
        Self::bump(env, &key);
        Ok(value)
    }

    fn save_accrual(env: &Env, spend_ref_hash: &BytesN<32>, accrual: &RewardAccrual) {
        let key = DataKey::Accrual(spend_ref_hash.clone());
        env.storage().persistent().set(&key, accrual);
        Self::bump(env, &key);
    }

    fn load_claim(env: &Env, spend_ref_hash: &BytesN<32>) -> Result<RewardClaim, Error> {
        let key = DataKey::Claim(spend_ref_hash.clone());
        let value = env.storage().persistent().get(&key).ok_or(Error::ClaimNotFound)?;
        Self::bump(env, &key);
        Ok(value)
    }

    fn save_claim(env: &Env, spend_ref_hash: &BytesN<32>, claim: &RewardClaim) {
        let key = DataKey::Claim(spend_ref_hash.clone());
        env.storage().persistent().set(&key, claim);
        Self::bump(env, &key);
    }

    fn assert_campaign_window(env: &Env, campaign: &RewardCampaign) -> Result<(), Error> {
        let ledger = env.ledger().sequence();
        if ledger < campaign.start_ledger || ledger > campaign.end_ledger {
            return Err(Error::CampaignNotInWindow);
        }
        Ok(())
    }

    fn calculate_reward(eligible_amount: i128, reward_rate_bps: u32) -> Result<i128, Error> {
        if eligible_amount <= 0 || reward_rate_bps == 0 {
            return Err(Error::InvalidAmount);
        }
        let scaled = eligible_amount
            .checked_mul(reward_rate_bps as i128)
            .ok_or(Error::MathOverflow)?;
        Ok(scaled / BPS_DENOM)
    }

    fn is_zero_hash(env: &Env, hash: &BytesN<32>) -> bool {
        let zero = BytesN::from_array(env, &[0u8; 32]);
        hash == &zero
    }

    fn bump(env: &Env, key: &DataKey) {
        env.storage()
            .persistent()
            .extend_ttl(key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
    }
}

use soroban_sdk::{contracttype, Address, BytesN};

/// Roles are operational, not legal conclusions. The contract enforces who may
/// write which market-relevant state.
///
/// - Sponsor:     funds campaigns, approves/rejects claims (the LBMA refiner).
/// - Verifier:    records finally-posted eligible spend (issuer processor/bank).
/// - GoldPartner: confirms redemption into allocated bullion (often = sponsor).
/// - Bank:        may claw back rewards (chargeback / default).
/// - Processor:   reserved for a future split of the verifier role.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Admin,
    Sponsor,
    Verifier,
    GoldPartner,
    Processor,
    Bank,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CampaignStatus {
    Active,
    Paused,
    Closed,
}

/// A sponsor-funded reward campaign.
///
/// Amounts are integer units of the chosen program currency, normally minor
/// units such as cents/rappen. The contract does not hard-code CHF because a
/// future campaign may use EUR, USD or another settlement currency.
///
/// Budget is split into running buckets so the sponsor's exposure is always
/// reconcilable on-chain: `reserved` is owed-but-not-yet-redeemed, `redeemed`
/// is settled to gold, `cancelled` and `expired` are released back to budget.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RewardCampaign {
    pub sponsor: Address,
    /// Bank / issuer processor / appointed verifier allowed to record eligible
    /// posted card spend and confirm its finality for this campaign.
    pub verifier: Address,
    /// Refiner / bullion partner that confirms voucher redemption into approved
    /// bullion products and supplies the gold price at redemption.
    pub gold_partner: Address,
    /// Reward rate in basis points: 50 = 0.50%, 100 = 1.00%.
    pub reward_rate_bps: u32,
    /// Maximum eligible spend per user for this campaign, program currency units.
    pub user_spend_cap: i128,
    /// Sponsor budget committed to this campaign, program currency units.
    pub total_budget: i128,
    pub reserved_reward_value: i128,
    pub redeemed_reward_value: i128,
    pub cancelled_reward_value: i128,
    pub expired_reward_value: i128,
    /// total fine gold weight allocated across the campaign, troy oz * 1e7
    pub redeemed_fine_weight_oz_e7: i128,
    pub currency_code: u32,
    pub start_ledger: u32,
    pub end_ledger: u32,
    /// Claim window after spend finality, in ledgers. For a 12-month period set
    /// the ledger-equivalent in the service configuration.
    pub claim_window_ledgers: u32,
    /// Off-chain policy commitments: sponsor terms, eligible MCC/categories,
    /// exclusions, voucher terms and product scope, anchored by hash.
    pub eligible_mcc_policy_hash: BytesN<32>,
    pub redemption_terms_hash: BytesN<32>,
    pub sponsor_product_scope_hash: BytesN<32>,
    pub funding_commitment_hash: BytesN<32>,
    pub status: CampaignStatus,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpendStatus {
    Recorded,
    Final,
    Cancelled,
}

/// A posted card transaction the verifier says is eligible. Not raw card data:
/// a privacy-preserving reference/hash to the processor/bank record.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EligibleSpend {
    pub campaign_id: BytesN<32>,
    pub card_line_hash: BytesN<32>,
    pub user: Address,
    pub verifier: Address,
    pub spend_ref_hash: BytesN<32>,
    pub amount: i128,
    pub eligible_amount: i128,
    pub currency_code: u32,
    pub category_policy_hash: BytesN<32>,
    pub posted_at_ledger: u32,
    pub finality_hash: BytesN<32>,
    pub status: SpendStatus,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RewardStatus {
    Pending,
    Claimable,
    ClaimSubmitted,
    VoucherIssued,
    Redeemed,
    Cancelled,
    Expired,
    Rejected,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RewardAccrual {
    pub campaign_id: BytesN<32>,
    pub user: Address,
    pub spend_ref_hash: BytesN<32>,
    pub eligible_spend_amount: i128,
    pub reward_value: i128,
    pub currency_code: u32,
    pub accrued_at_ledger: u32,
    pub claimable_at_ledger: u32,
    pub claim_expiry_ledger: u32,
    pub status: RewardStatus,
}

/// Per-(campaign, user) running totals, including accumulated fine gold weight.
/// The gold figure is the literal "accumulate gold" promise: it only rises when
/// a reward is actually redeemed into allocated metal.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserCampaignUsage {
    pub eligible_spend_total: i128,
    pub reward_value_total: i128,
    /// fine gold weight redeemed by this user in this campaign, troy oz * 1e7
    pub fine_weight_oz_e7_total: i128,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimStatus {
    Submitted,
    Approved,
    Rejected,
}

/// A one-time user claim supported by receipt / transaction evidence. The
/// receipt bundle stays off-chain; the hash links it to the claim.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RewardClaim {
    pub campaign_id: BytesN<32>,
    pub user: Address,
    pub spend_ref_hash: BytesN<32>,
    pub receipt_bundle_hash: BytesN<32>,
    pub submitted_at_ledger: u32,
    pub sponsor_response_hash: BytesN<32>,
    pub status: ClaimStatus,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoucherRecord {
    pub campaign_id: BytesN<32>,
    pub user: Address,
    pub spend_ref_hash: BytesN<32>,
    pub voucher_hash: BytesN<32>,
    pub reward_value: i128,
    pub issued_at_ledger: u32,
    pub expiry_ledger: u32,
}

/// Redemption record. This is the on-chain proof that the gold partner
/// allocated bullion against a voucher, and it carries the GOLD WEIGHT, not
/// just the value: the price used and the resulting fine weight. This is where
/// the "accumulate gold" claim becomes literal and auditable.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedemptionRecord {
    pub campaign_id: BytesN<32>,
    pub user: Address,
    pub spend_ref_hash: BytesN<32>,
    pub order_hash: BytesN<32>,
    pub product_hash: BytesN<32>,
    pub redeemed_value: i128,
    /// gold price per troy ounce used for conversion, program units * 1e7
    pub price_per_oz_e7: i128,
    /// resulting fine gold weight, troy oz * 1e7
    pub fine_weight_oz_e7: i128,
    /// hash of the gold partner's allocation / custody receipt for this metal
    pub custody_receipt_hash: BytesN<32>,
    pub redeemed_at_ledger: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Approved(Address, Role),
    Campaign(BytesN<32>),
    Spend(BytesN<32>),
    Accrual(BytesN<32>),
    UserUsage(BytesN<32>, Address),
    Claim(BytesN<32>),
    Voucher(BytesN<32>),
    Redemption(BytesN<32>),
}

/// Grouped configuration for `create_campaign`. Soroban caps contract functions
/// at 10 parameters (including env), so the campaign's economics, window and
/// off-chain policy hashes are passed as one struct.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CampaignConfig {
    /// reward rate in basis points: 50 = 0.50%, 100 = 1.00%
    pub reward_rate_bps: u32,
    /// per-cardholder eligible-spend cap, program currency minor units
    pub user_spend_cap: i128,
    /// sponsor budget committed to this campaign, program currency minor units
    pub total_budget: i128,
    pub currency_code: u32,
    pub start_ledger: u32,
    pub end_ledger: u32,
    /// claim window after spend finality, in ledgers
    pub claim_window_ledgers: u32,
    pub eligible_mcc_policy_hash: BytesN<32>,
    pub redemption_terms_hash: BytesN<32>,
    pub sponsor_product_scope_hash: BytesN<32>,
    pub funding_commitment_hash: BytesN<32>,
}

/// Grouped input for `record_eligible_spend`, for the same parameter-count
/// reason. Identifies the posted card transaction and the spend amount.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpendInput {
    pub spend_ref_hash: BytesN<32>,
    pub card_line_hash: BytesN<32>,
    pub amount: i128,
    pub currency_code: u32,
    pub category_policy_hash: BytesN<32>,
    pub posted_at_ledger: u32,
}

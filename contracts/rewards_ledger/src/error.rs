use soroban_sdk::contracterror;

/// Contract errors, numbered stably so the off-chain service can map them to
/// HTTP responses.
#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAuthorized = 3,
    PartyNotApproved = 4,
    CampaignExists = 5,
    CampaignNotFound = 6,
    CampaignNotActive = 7,
    CampaignNotInWindow = 8,
    InvalidAmount = 9,
    InvalidRate = 10,
    InvalidWindow = 11,
    BudgetInsufficient = 12,
    UserCapExceeded = 13,
    SpendExists = 14,
    SpendNotFound = 15,
    AccrualNotFound = 16,
    InvalidStatus = 17,
    PolicyMismatch = 18,
    Expired = 19,
    NotExpired = 20,
    ClaimExists = 21,
    VoucherExists = 22,
    RedemptionExists = 23,
    AmountMismatch = 24,
    ClaimNotFound = 25,
    VoucherNotFound = 26,
    RedemptionNotFound = 27,
    /// The gold price supplied at redemption was not positive.
    PriceNotPositive = 28,
    /// An arithmetic operation overflowed i128 (guarded, should not occur with
    /// realistic loyalty-scale figures).
    MathOverflow = 29,
    /// A market-critical evidence or policy hash is all-zero.
    InvalidEvidenceHash = 30,
}

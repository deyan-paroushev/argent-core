use soroban_sdk::contracterror;

/// Contract errors. Numbered stably so the off-chain service can map them to
/// HTTP responses. Each corresponds to a refusal path in the architecture doc.
#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotAuthorized = 3,
    PartyNotApproved = 4,
    /// A framework with this id already exists.
    FrameworkExists = 5,
    /// The referenced framework does not exist or is not active.
    FrameworkNotActive = 6,
    /// The position's owner/custodian, or the pledging bank, does not match the
    /// parties named in the control framework.
    FrameworkPartyMismatch = 7,

    PositionNotFound = 10,
    PositionExists = 11,
    PositionNotFree = 12,
    AttestationStale = 13,
    /// A bank pledge was attempted on a position the custodian has not yet
    /// confirmed and immobilized under the control agreement.
    PositionNotEarmarked = 14,
    /// The custodian tried to immobilize a position the owner has not selected.
    PositionNotSelected = 15,
    /// Custodian tried to confirm release on a position not awaiting release.
    PositionNotReleasePending = 16,
    /// This exact lot (uniqueness_hash) is already active under another
    /// position. The same allocated lot cannot be double-pledged.
    LotAlreadyActive = 17,

    PledgeNotFound = 20,
    PledgeNotActive = 21,

    LineNotFound = 30,
    LineNotActive = 31,
    LimitExceedsBorrowingBase = 32,
    InsufficientCapacity = 33,
    OutstandingBalance = 34,
    /// The bank-set risk parameters are invalid: the rule is
    /// 0 < ltv_bps (advance) < maintenance_bps <= 10000. This prevents a line
    /// from being configured to lend past the value of its collateral.
    InvalidRiskParams = 35,

    DuplicateAuthRef = 40,
    AmountNotPositive = 41,
    /// A repayment with this payment reference was already applied.
    DuplicatePaymentRef = 42,
    /// An adjustment with this id already exists.
    AdjustmentExists = 43,
    /// The adjustment is not in the status this step requires.
    AdjustmentWrongStatus = 44,
    /// Approving the adjustment would leave the line under-covered at the
    /// advance rate (released collateral must still cover the drawn balance).
    AdjustmentUndercovered = 45,
    /// reverse_drawdown referenced an auth_ref that has no recorded drawdown
    /// (or one that was already reversed). There is nothing to unwind.
    NothingToReverse = 46,
    /// reverse_drawdown was called with an amount that does not equal the
    /// amount originally drawn under this auth_ref. A reversal must unwind
    /// exactly what was drawn; partial reversals are not supported.
    ReversalAmountMismatch = 47,

    NotDefaulted = 50,
    CurePeriodNotExpired = 51,
    AlreadyEnforced = 52,
    /// issue_default_notice was given a cure deadline at or before the current
    /// ledger. A default notice must grant a real, forward-looking cure window;
    /// a past deadline is a malformed notice.
    CureDeadlineNotFuture = 53,

    /// The submitted price is older than the allowed freshness window.
    PriceStale = 60,
    /// The submitted price's confidence band is wider than the allowed tolerance.
    PriceConfidenceTooWide = 61,
    /// The submitted price or confidence was not a positive value.
    PriceNotPositive = 62,

    /// No enforcement-readiness record exists for this line.
    ReadinessNotFound = 70,
    /// The readiness record is not in a state that permits this transition.
    ReadinessWrongStatus = 71,

    /// A credit line with this id already exists.
    LineExists = 73,
    /// This pledge already has a bound credit line.
    PledgeAlreadyHasLine = 74,
    /// Repayment amount exceeds the outstanding drawn balance.
    RepaymentExceedsBalance = 75,
    /// Oracle price timestamp is later than the current ledger timestamp.
    PriceFromFuture = 76,
    /// Revaluation thresholds or freshness parameters are malformed.
    InvalidRevaluationParams = 77,
    /// A legally critical evidence/document hash is all-zero.
    InvalidDocumentHash = 78,
    /// Enforcement readiness cannot be promoted because its validity window has expired.
    ReadinessExpired = 79,
    /// A pledge with this id already exists; activation is not idempotent over a live id.
    PledgeExists = 80,
    /// No instrument with this key exists in the registry.
    InstrumentNotFound = 82,
    /// The instrument is not admitted to this framework's eligible set (or is
    /// retired and can no longer back a position).
    InstrumentNotEligible = 84,
}

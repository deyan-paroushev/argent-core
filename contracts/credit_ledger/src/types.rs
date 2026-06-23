use soroban_sdk::{contracttype, Address, BytesN};

/// Lifecycle of a tri-party control framework.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameworkStatus {
    Active,
    Terminated,
}

/// The tri-party legal and control perimeter for a facility, signed by all
/// three parties (owner/pledgor, bank/secured party, custodian) before any
/// pledge. A bank will not rely on a bare on-chain pledge without the facility
/// agreement, the pledge/security agreement, the custody-control agreement, the
/// eligible-collateral schedule, the margin policy, and the enforcement
/// waterfall. This object anchors the hashes of those six off-chain documents
/// and binds the operational state machine to them. The chain does not replace
/// the signed documents; it records that the three parties established this
/// framework and which documents govern it.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlFramework {
    pub owner: Address,
    pub bank: Address,
    pub custodian: Address,
    pub facility_agreement_hash: BytesN<32>,
    pub pledge_agreement_hash: BytesN<32>,
    pub custody_agreement_hash: BytesN<32>,
    pub eligible_schedule_hash: BytesN<32>,
    pub margin_policy_hash: BytesN<32>,
    pub enforcement_waterfall_hash: BytesN<32>,
    pub status: FrameworkStatus,
}

/// Lifecycle of a vaulted gold position.
///
/// Free       -> registered and attested, owner's unencumbered title, idle.
/// Earmarked  -> the custodian has confirmed and immobilized the specific bars
///               under the tri-party control agreement, pending a bank pledge.
///               The owner can no longer withdraw or substitute unilaterally.
/// Pledged    -> a bank has accepted the pledge; the bars secure a credit line.
/// Enforced   -> default enforcement has been carried out against the bars.
/// Released   -> the pledge was lifted; title returns unencumbered to the owner.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionStatus {
    Free,
    Selected,
    Earmarked,
    Pledged,
    Enforced,
    ReleasePending,
    Released,
}

/// The owner's signed instruction to designate a registered position as
/// collateral. The bars themselves are already identified on the position
/// (barlist_hash) and recorded at the custody/account level; this record is the
/// owner's directive that the recorded set be committed, anchoring the hash of
/// the owner's signed collateral-request letter. It is the owner-selects half
/// of the two-sided consent that the custodian then confirms at immobilization.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollateralSelection {
    pub position_id: BytesN<32>,
    pub owner: Address,
    /// hash of the owner's signed collateral-request instruction
    pub request_hash: BytesN<32>,
}

/// A recorded repayment against a credit line, keyed by the off-chain payment
/// reference. Storing it makes repayment idempotent (the same payment_ref
/// cannot be applied twice) and gives each repayment an auditable record.
/// Repayment reduces the drawn balance only. It does not cure margin or
/// default, and it does not release collateral; release is a separate
/// bank-authorized, custodian-confirmed act.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepaymentRecord {
    pub credit_line_id: BytesN<32>,
    pub amount_applied: i128,
    pub applied_at_ledger: u32,
}

/// A recorded card drawdown against a credit line, keyed by the off-chain
/// authorization reference. Storing the full record (not a bare bool) makes the
/// drawdown idempotent (the same auth_ref cannot be drawn twice) AND lets a
/// reversal verify it is unwinding exactly what was drawn: same line, same
/// amount. Without the stored amount a processor could reverse an arbitrary
/// figure, corrupting the line's drawn balance. The record carries a `reversed`
/// flag so a reversed authorization is kept (audit trail) but cannot be
/// reversed twice.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawdownRecord {
    pub credit_line_id: BytesN<32>,
    pub amount: i128,
    pub drawn_at_ledger: u32,
    pub reversed: bool,
}

/// The kind of collateral adjustment an owner can request on a live facility.
/// Real collateral facilities are not static: the owner may add collateral,
/// swap bars, or ask to return excess.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdjustmentType {
    TopUp,
    Substitution,
    PartialRelease,
}

/// State machine of a collateral-adjustment request. All three parties must
/// clear an adjustment: the owner requests, the custodian confirms it can hold
/// and block the proposed set, and the bank approves if coverage holds.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdjustmentStatus {
    Requested,
    CustodianConfirmed,
    Approved,
    Rejected,
}

/// A collateral-adjustment request acting on a live credit line. Modeled as a
/// first-class state-machine object (the agreement stays fixed; only the
/// collateral schedule changes). It carries the proposed new collateral
/// schedule (new barlist and weight) and advances through its own states as the
/// custodian and bank clear it. On approval the position's schedule is updated.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollateralAdjustment {
    pub credit_line_id: BytesN<32>,
    pub adjustment_type: AdjustmentType,
    /// proposed new bar-list commitment if the adjustment is approved
    pub new_barlist_hash: BytesN<32>,
    /// proposed new fine weight (troy oz, scaled 1e7) if approved
    pub new_weight_oz_e7: i128,
    /// hash of the owner's signed adjustment-request instruction
    pub request_hash: BytesN<32>,
    pub status: AdjustmentStatus,
}

/// Lifecycle of the pledge that locks a position to a bank.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PledgeStatus {
    Active,
    /// The bank has released its security interest (payoff-letter act, prong i),
    /// but the custodian has not yet returned possession, so the possessory
    /// perfection is not yet terminated. The lien persists until the custodian
    /// confirms release.
    ReleaseAuthorized,
    Released,
    Defaulted,
    Enforced,
}

/// Lifecycle of the credit line secured by a pledge.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineStatus {
    Active,
    Suspended,
    Defaulted,
    Closed,
}

/// Readiness of a facility's enforcement / realization path. This gates whether
/// an Enforcement Readiness Certificate may render as anything other than DRAFT.
///
/// `Incomplete` is the honest default: the realization route, liquidation agent
/// and approved settlement asset are not yet agreed, so no certificate can
/// truthfully assert the facility is ready to enforce. `Ready` is only reachable
/// once the partner-dependent fields below are populated by the bank. `Expired`
/// marks a readiness record whose agreed validity window has passed and must be
/// re-confirmed. The contract is the single source of truth for this status:
/// the certificate generator reads it and CANNOT print a non-DRAFT certificate
/// while the record is `Incomplete`.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadinessStatus {
    Incomplete,
    Ready,
    Expired,
}

/// The enforcement-readiness record for a credit line. It documents the agreed
/// path from default to cash realization. The HASHES (not the documents) of the
/// real agreements are anchored here; the human-readable detail lives off chain.
///
/// While `liquidation_agent` is unset (all-zero) or any required hash is unset,
/// the record stays `Incomplete` and the certificate is DRAFT. This is the
/// honesty-by-construction property: the machinery refuses to assert a
/// realization path that has not actually been agreed with a real party.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementReadiness {
    pub credit_line_id: BytesN<32>,
    pub status: ReadinessStatus,
    /// The named liquidation agent / realization counterparty. Unset until a
    /// real partner is agreed; an all-zero address means "not yet appointed".
    pub liquidation_agent: Address,
    /// hash of the agreed realization-route document (how collateral becomes
    /// cash: venue, method, timing).
    pub realization_route_hash: BytesN<32>,
    /// the settlement asset the proceeds are paid in (e.g. a USDC SAC address).
    pub settlement_asset: Address,
    /// hash of the approved valuation-source / pricing policy used at realization.
    pub valuation_source_hash: BytesN<32>,
    /// hash of the enforcement waterfall (lender first, surplus to borrower,
    /// shortfall visible). Carried from the framework for certificate convenience.
    pub waterfall_hash: BytesN<32>,
    /// ledger after which this readiness record must be re-confirmed.
    pub valid_until_ledger: u32,
    /// monotonically increasing snapshot counter, bumped on each populate, so a
    /// presented certificate can be tied to an exact readiness version.
    pub version: u32,
}

/// Roles recognised by the access-control registry.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Admin,
    Bank,
    Custodian,
    Processor,
    Valuation,
    /// A SettlementVault contract authorized to apply repayments / release
    /// pledges on behalf of the atomic repay-and-release flow.
    Vault,
}

/// An attested vaulted-gold position. The full bar list is NEVER stored on
/// chain. Only its hash, plus the usable collateral facts.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VaultPosition {
    pub owner: Address,
    pub custodian: Address,
    /// the control framework this position is registered under. The position is
    /// designated as collateral under that framework's eligible-collateral
    /// schedule, with its owner and custodian matching the framework's.
    pub framework_id: BytesN<32>,
    /// keccak/sha hash of the off-chain bar list document (serials, assay,
    /// weights, formatting). The full-document commitment.
    pub barlist_hash: BytesN<32>,
    /// hash of just the bar serial numbers. This is the collateral-uniqueness
    /// key: the same serials cannot be active under two positions at once, which
    /// is what enforces no-double-pledge at the bar-set level (not merely at the
    /// position level).
    pub serials_hash: BytesN<32>,
    /// fine weight in troy ounces, scaled by 1e7 (so 401.10 oz -> 4_011_000_000)
    pub fine_weight_oz_e7: i128,
    /// ledger sequence after which the custody attestation is stale
    pub attestation_expiry: u32,
    pub status: PositionStatus,
}

/// The outcome of enforcing the security after an uncured default. Enforcement
/// follows the agreed waterfall in the off-chain security and control
/// documents. Recording the outcome on chain does NOT itself convey ownership
/// or move metal; it anchors which lawful path was taken so the trail is
/// undisputed. Proceeds are applied to the debt and any surplus returns to the
/// owner under the documents.
///
/// Sold          -> the bars were sold and proceeds applied to the debt.
/// Appropriated  -> the bank took the bars in (partial or full) satisfaction of
///                  the debt at an agreed valuation.
/// Transferred   -> title to the bars was transferred under the security
///                  documents (only where the governing law permits it).
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnforcementOutcome {
    Sold,
    Appropriated,
    Transferred,
}

/// Margin state of a credit line after the latest revaluation.
///
/// Covered  -> the drawn balance sits comfortably within the borrowing base.
/// Warning  -> the drawn balance has crossed the warning band; the buffer is
///             shrinking. A notice is warranted but draws are not yet blocked.
/// Called   -> the drawn balance has crossed the action band; a margin call is
///             issued. The borrower must cure by repayment or additional
///             collateral before further draws.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarginState {
    Covered,
    Warning,
    Called,
}

/// A revaluation record for a credit line, kept as a side-record so the core
/// CreditLine struct stays lean. Written by `revalue_and_check` each time a
/// fresh price is acted on.
///
/// The price and its source timestamp come from an off-chain gold feed (Pyth
/// `Metal.XAU/USD` in the current design) submitted by the valuation role. The
/// contract validates freshness against `priced_at` and rejects a price whose
/// confidence band is too wide, so a stale or low-quality price cannot drive a
/// margin decision.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineValuation {
    /// price per troy ounce, scaled by 1e7, last acted on
    pub price_per_oz_e7: i128,
    /// confidence (half-width) of that price, scaled by 1e7; smaller is better
    pub confidence_e7: i128,
    /// source publish time of the price (unix seconds), for the freshness check
    pub priced_at: u64,
    /// ledger sequence at which this revaluation was recorded on chain
    pub valued_at_ledger: u32,
    /// recomputed borrowing base at the acted-on price
    pub borrowing_base: i128,
    pub margin_state: MarginState,
}

/// The tri-party control framework under which a position is held as
/// collateral. A real control agreement is a separate legal instrument signed
/// by all three parties (owner/pledgor, custodian/intermediary, and
/// bank/secured party). It is what establishes the bank's "control" and lets
/// the custodian act on the bank's instructions. This object anchors the hash
/// of that agreement as the framework the custodian acts under when it
/// immobilizes the bars. Recording it on chain does NOT replace the off-chain
/// signed agreement; it makes the framework reference part of the indisputable
/// trail.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustodyControl {
    pub position_id: BytesN<32>,
    pub custodian: Address,
    /// hash of the off-chain tri-party control / custody-control agreement
    pub control_agreement_hash: BytesN<32>,
    /// the position status at the time control was recorded (Earmarked)
    pub status: PositionStatus,
}

/// A pledge locks a specific position to a specific secured party (bank).
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pledge {
    pub position_id: BytesN<32>,
    pub pledgor: Address,
    pub bank: Address,
    /// hash of the off-chain security agreement / legal terms
    pub legal_terms_hash: BytesN<32>,
    pub status: PledgeStatus,
}

/// The secured-debt position. Holds NO money. Only the facility state.
/// All monetary amounts are minor units of `currency` (e.g. CHF cents),
/// represented as i128.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreditLine {
    pub pledge_id: BytesN<32>,
    pub bank: Address,
    pub cardholder: Address,
    /// ISO-4217-ish 3-char code packed as a Soroban Symbol elsewhere; we keep
    /// the numeric currency exponent out of scope for the MVP and treat all
    /// amounts as already-scaled minor units.
    pub approved_limit: i128,
    pub drawn_balance: i128,
    pub available_limit: i128,
    /// loan-to-value in basis points (e.g. 6000 = 60%). This is the ADVANCE
    /// rate: the fraction of collateral value the bank will lend at
    /// origination. Bank-set per its own credit policy.
    pub ltv_bps: u32,
    /// maintenance threshold in basis points (e.g. 7500 = 75%). The HIGHER
    /// fraction of current collateral value at which a margin call fires: when
    /// the gold price falls enough that the drawn balance exceeds
    /// (collateral_value * maintenance_bps), the line is called. Bank-set, and
    /// the contract enforces ltv_bps < maintenance_bps <= 10000 so a line can
    /// never be configured to lend past the value of its collateral.
    pub maintenance_bps: u32,
    /// ledger sequence after which an open default may be enforced; 0 when not
    /// in default.
    pub cure_expiry_ledger: u32,
    /// true when the bank has deliberately stopped the facility for a reason
    /// other than margin (fraud, KYC, sanctions, documentation, internal credit
    /// stop). A revaluation may move the margin dimension, but it must never
    /// clear a bank stop: only the bank can resume the line.
    pub manual_bank_suspension: bool,
    pub status: LineStatus,
}

/// Storage keys. Persistent for everything security-critical (never Temporary).
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    /// role registry: (Address, Role) -> bool
    Approved(Address, Role),
    /// tri-party control framework: framework_id -> ControlFramework
    Framework(BytesN<32>),
    Position(BytesN<32>),
    /// bar-set uniqueness lock: serials_hash -> position_id holding it active.
    /// Prevents the same physical bars being pledged under two positions.
    BarSet(BytesN<32>),
    /// owner's bar-selection instruction: position_id -> CollateralSelection
    Selection(BytesN<32>),
    Pledge(BytesN<32>),
    Line(BytesN<32>),
    /// tri-party control framework: position_id -> CustodyControl
    Control(BytesN<32>),
    /// revaluation side-record: line_id -> LineValuation
    Valuation(BytesN<32>),
    /// drawdown record / idempotency guard: auth_ref -> DrawdownRecord
    Draw(BytesN<32>),
    /// repayment record / idempotency guard: payment_ref -> RepaymentRecord
    Repayment(BytesN<32>),
    /// collateral-adjustment request: adjustment_id -> CollateralAdjustment
    Adjustment(BytesN<32>),
    /// enforcement-readiness record: line_id -> EnforcementReadiness
    Readiness(BytesN<32>),
}

use soroban_sdk::{contracttype, Address, BytesN, Symbol};

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
/// collateral. The lot itself is already identified on the position
/// (manifest_hash) and recorded at the custody/account level; this record is the
/// owner's directive that the recorded lot be committed, anchoring the hash of
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
    /// proposed new manifest commitment (document for the new lot) if approved
    pub new_manifest_hash: BytesN<32>,
    /// proposed new uniqueness commitment (the collateral-uniqueness key). Must be
    /// maintained in lockstep with new_manifest_hash so the lot uniqueness lock
    /// tracks the real collateral identity after substitution/top-up/release.
    pub new_uniqueness_hash: BytesN<32>,
    /// proposed new quality certificate commitment for the substituted / topped-up
    /// lot, maintained in lockstep with the other new_* commitments.
    pub new_quality_cert_hash: BytesN<32>,
    /// proposed new quantity certificate commitment for the new lot.
    pub new_quantity_cert_hash: BytesN<32>,
    /// proposed new location commitment for the new lot.
    pub new_location_hash: BytesN<32>,
    /// proposed new quantity in the instrument's unit (scaled 1e7) if approved
    pub new_quantity_e7: i128,
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
    /// The asset owner / pledgor: the delegated self-signing party. It
    /// authorizes its own acts (selection, adjustment requests, cure) but is
    /// never policy-governed by the registry and never checked by is_approved.
    /// Present so a CollateralEventV1 can name the true authority on an
    /// owner-delegated act. No approval path grants it.
    Owner,
}

/// Lifecycle of an instrument in the registry.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstrumentStatus {
    Active,
    Retired,
}

/// Lifecycle of a per-framework instrument eligibility record.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EligibilityStatus {
    Active,
    Retired,
}

/// The bank-approved result of a framework eligible-collateral schedule for a
/// single instrument: not merely admitted but admitted under a specific
/// treatment. On-chain compression of the CDM collateral criteria / treatment
/// model. The legal schedule stays off-chain as eligibility_hash; the contract
/// records the treatment the bank applies (haircut, max advance, maintenance).
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameworkInstrumentEligibility {
    pub framework_id: BytesN<32>,
    pub instrument: InstrumentKey,
    /// Commitment to the eligibility schedule clause / CDM CollateralCriteria.
    pub eligibility_hash: BytesN<32>,
    /// Discount applied to gross value before LTV, in bps. 500 = 5 percent.
    pub haircut_bps: u32,
    /// Maximum advance rate permitted for this instrument here, bps.
    pub max_ltv_bps: u32,
    /// Maintenance threshold for this instrument here, bps.
    pub maintenance_bps: u32,
    pub status: EligibilityStatus,
}

/// The identity of an instrument in the registry. Mirrors Daml Finance's
/// InstrumentKey (issuer, depository, id, version). The issuer is the party that
/// defines the asset standard; the depository is the custodian that attests
/// custody of this asset class. Both co-sign instrument registration, so neither
/// can unilaterally define the asset (the two-signatory anti-fraud guarantee).
/// `version` tracks the linear evolution of a standard: a grade redefinition
/// bumps the version, so old holdings keep referencing the standard in force
/// when they were created.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstrumentKey {
    pub issuer: Address,
    pub depository: Address,
    /// short textual id, e.g. "XAU_LGD", "CU_LME_A", "WHEAT_2"
    pub id: Symbol,
    pub version: u32,
}

/// The economic identity of one unit of collateral. Daml Finance "Instrument":
/// it describes WHAT is held, separated from the holding that records HOW MUCH
/// and against which custodian. Defined once in the registry and referenced by
/// every position of this asset, so asset data is never replicated per holding.
/// Gold is one instrument among many; copper, wheat, and crude are others.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instrument {
    pub key: InstrumentKey,
    /// the commodity class, e.g. "gold", "copper", "wheat", "crude"
    pub commodity: Symbol,
    /// the unit the quantity is denominated in, e.g. "oz", "mt", "bbl", "bu"
    pub unit: Symbol,
    /// commitment to the off-chain grade / quality standard document (LBMA Good
    /// Delivery, LME Grade A, No.2 wheat, Brent). First-class because grade
    /// prices the collateral and a substitution to a lower grade is a risk event.
    pub grade_hash: BytesN<32>,
    pub status: InstrumentStatus,
}

/// The set of off-chain document and identity commitments for a single lot. This
/// is the evidence a warehouse-receipt-finance lender underwrites against:
/// the manifest, the lot's unique identity, and the quality, quantity, and
/// location certificates. Bundled into one struct so the position and adjustment
/// entrypoints stay within Soroban's per-function parameter limit, and because
/// these commitments are a cohesive evidence set, not independent arguments.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LotEvidence {
    /// Commitment to the off-chain manifest document for this lot (bar list,
    /// warehouse receipt, lot schedule, with quantities and quality).
    pub manifest_hash: BytesN<32>,
    /// Commitment to the lot's identity (the collateral-uniqueness key): bar
    /// serials, receipt id, parcel id. The same lot cannot be active twice.
    pub uniqueness_hash: BytesN<32>,
    /// Commitment to the lot-level quality / assay / grading certificate.
    pub quality_cert_hash: BytesN<32>,
    /// Commitment to the lot-level weight / quantity certificate.
    pub quantity_cert_hash: BytesN<32>,
    /// Commitment to the warehouse / vault / tank / terminal location.
    pub location_hash: BytesN<32>,
}

/// An attested allocated-collateral position: the Daml Finance "Holding". It
/// records how much of an instrument an owner holds against a custodian under a
/// framework. The economic terms (commodity, unit, grade) live on the referenced
/// instrument, not here. The specific lot's documents and identity stay here.
/// The full manifest is NEVER stored on chain, only its hash.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VaultPosition {
    pub owner: Address,
    pub custodian: Address,
    /// the control framework this position is registered under. The position is
    /// designated as collateral under that framework's eligible-collateral
    /// schedule, with its owner and custodian matching the framework's.
    pub framework_id: BytesN<32>,
    /// the instrument this collateral is an allocation of. The economic identity
    /// (commodity, unit, grade) is read from the registry under this key.
    pub instrument: InstrumentKey,
    /// hash of the off-chain manifest document for this specific lot (the bar
    /// list, the warehouse receipt, the lot schedule, with quantities and
    /// quality). The full-document commitment for this holding.
    pub manifest_hash: BytesN<32>,
    /// hash of the lot's identity (bar serials for gold, receipt id for a
    /// warehouse receipt, lot/parcel id for bulk). This is the collateral-
    /// uniqueness key: the same lot cannot be active under two positions at once,
    /// which enforces no-double-pledge at the lot level (not merely the position
    /// level). The same mechanism that prevents duplicate financing of one
    /// warehouse receipt.
    pub uniqueness_hash: BytesN<32>,
    /// lot-level quality / assay / grading certificate commitment. The bank
    /// underwrites against grade, so quality is a first-class evidence field, not
    /// buried in the manifest. Warehouse-receipt finance evidences stated quality.
    pub quality_cert_hash: BytesN<32>,
    /// lot-level weight / quantity certificate commitment. The collateral manager
    /// certifies quantity independently of the manifest. Warehouse-receipt finance
    /// evidences stated quantity.
    pub quantity_cert_hash: BytesN<32>,
    /// warehouse / vault / tank / terminal location commitment. Existence and
    /// location are part of what the collateral manager monitors.
    pub location_hash: BytesN<32>,
    /// quantity of the instrument's unit, scaled by 1e7 (so 401.10 units ->
    /// 4_011_000_000). The unit meaning comes from the referenced instrument.
    pub quantity_e7: i128,
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
    /// price per the instrument's unit, scaled by 1e7, last acted on
    pub price_per_unit_e7: i128,
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
    /// The admin address proposed by the current admin but not yet accepted. The
    /// rotation is a two-step handshake: the current admin proposes, and the
    /// proposed address must itself accept before it becomes the admin. This
    /// makes it impossible to hand control to a mistyped or uncontrolled address.
    PendingAdmin,
    /// the single SettlementVault contract address authorized to apply repayments
    SettlementVault,
    /// role registry: (Address, Role) -> bool
    Approved(Address, Role),
    /// tri-party control framework: framework_id -> ControlFramework
    Framework(BytesN<32>),
    /// instrument registry: fingerprint(InstrumentKey) -> Instrument. The economic
    /// identity of an asset, defined once and referenced by every position of it.
    /// Keyed by a 32-byte sha256 fingerprint of the InstrumentKey rather than the
    /// key struct itself: the full InstrumentKey serializes to 264 bytes, over the
    /// network's 250-byte ledger-key limit. The full key still travels in the
    /// stored value and in events, so the book of record is unchanged in content.
    Instrument(BytesN<32>),
    /// per-framework instrument eligibility (the CDM "GC basket"):
    /// (framework_id, fingerprint(InstrumentKey)) -> bool. A framework admits the
    /// instruments it will accept as collateral; register_position checks
    /// membership. Fingerprinted for the same ledger-key-size reason as Instrument.
    EligibleInstrument(BytesN<32>, BytesN<32>),
    /// per-framework instrument treatment (CDM collateral criteria / treatment
    /// result): (framework_id, fingerprint(InstrumentKey)) ->
    /// FrameworkInstrumentEligibility. Supersedes the boolean EligibleInstrument:
    /// records the haircut / LTV / maintenance the bank applies, not just that the
    /// instrument is admitted. Fingerprinted for the same ledger-key-size reason.
    FrameworkInstrument(BytesN<32>, BytesN<32>),
    Position(BytesN<32>),
    /// lot uniqueness lock: uniqueness_hash -> position_id holding it active.
    /// Prevents the same allocated lot (bars, warehouse receipt, parcel) being
    /// pledged under two positions. Generalizes the former bar-set lock.
    LotLock(BytesN<32>),
    /// owner's bar-selection instruction: position_id -> CollateralSelection
    Selection(BytesN<32>),
    Pledge(BytesN<32>),
    Line(BytesN<32>),
    /// pledge-to-line uniqueness guard: pledge_id -> credit_line_id.
    /// Prevents one pledged collateral set from supporting multiple lines.
    LineForPledge(BytesN<32>),
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
    /// CollateralEventV1 ordering: framework_id -> u64 last-emitted sequence.
    /// Gap-free run the indexer can replay.
    FrameworkSeq(BytesN<32>),
    /// GovernanceEventV1 ordering: a single contract-wide u64 last-emitted
    /// sequence. Unlike FrameworkSeq this is not keyed by framework: governance
    /// acts form one global authority stream. Gap-free run the indexer replays.
    GovernanceSeq,
    /// position_id -> framework_id, for cheap event tagging.
    ContextForPosition(BytesN<32>),
    /// pledge_id -> FacilityContext
    ContextForPledge(BytesN<32>),
    /// credit_line_id -> FacilityContext
    ContextForLine(BytesN<32>),
    /// adjustment_id -> FacilityContext
    ContextForAdjustment(BytesN<32>),
}

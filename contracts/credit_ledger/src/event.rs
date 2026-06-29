//! Canonical event layer: the replayable evidence record (CollateralEventV1).
//!
//! The contract keeps stored state as the on-chain enforcement guard. These
//! canonical events are the public, replayable mirror: an off-chain indexer can
//! rebuild every position, pledge, and line from the event stream alone. The
//! events are emitted alongside the existing thin events during migration.

use soroban_sdk::{contractevent, contracttype, Address, BytesN, Symbol};

use crate::types::{
    AdjustmentStatus, AdjustmentType, EnforcementOutcome, InstrumentKey, LineStatus, MarginState,
    PledgeStatus, PositionStatus, ReadinessStatus, Role,
};

/// The entity kind a CollateralEventV1 acts on. Second event topic, so an
/// indexer can filter the stream by object kind without decoding bodies.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityKind {
    Framework,
    Position,
    Pledge,
    Line,
    Drawdown,
    Repayment,
    Valuation,
    Adjustment,
    Release,
    Default,
    Enforcement,
    Readiness,
}

/// The specific control act a CollateralEventV1 records. Third event topic, and
/// the discriminant the replay fold switches on.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollateralAction {
    FrameworkRegistered,
    PositionRegistered,
    CollateralSelected,
    CollateralImmobilized,
    PledgeActivated,
    LineOpened,
    DrawdownRecorded,
    DrawdownReversed,
    RepaymentApplied,
    LineRevalued,
    LineSuspendedByBank,
    LineResumedByBank,
    AdjustmentRequested,
    AdjustmentCustodianConfirmed,
    AdjustmentApproved,
    ReleaseAuthorized,
    ReleaseConfirmed,
    DefaultNoticeIssued,
    DefaultCured,
    EnforcementRecorded,
    ReadinessOpened,
    ReadinessPopulated,
    ReadinessExpired,
}

/// One uniform label for the previous_state / new_state dimension, spanning
/// every entity's state machine, so a transition is expressed homogeneously.
/// `Null` is the "no prior state" / "not applicable" label.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateLabel {
    Null,
    FrameworkActive,
    PositionFree,
    PositionSelected,
    PositionEarmarked,
    PositionPledged,
    PositionReleasePending,
    PositionReleased,
    PositionEnforced,
    PledgeActive,
    PledgeReleaseAuthorized,
    PledgeReleased,
    PledgeDefaulted,
    PledgeEnforced,
    LineActive,
    LineSuspended,
    LineDefaulted,
    LineClosed,
    MarginCovered,
    MarginWarning,
    MarginCalled,
    AdjustmentRequested,
    AdjustmentCustodianConfirmed,
    AdjustmentApproved,
    AdjustmentRejected,
    ReadinessIncomplete,
    ReadinessReady,
    ReadinessExpired,
}

impl StateLabel {
    pub fn from_position(s: PositionStatus) -> StateLabel {
        match s {
            PositionStatus::Free => StateLabel::PositionFree,
            PositionStatus::Selected => StateLabel::PositionSelected,
            PositionStatus::Earmarked => StateLabel::PositionEarmarked,
            PositionStatus::Pledged => StateLabel::PositionPledged,
            PositionStatus::Enforced => StateLabel::PositionEnforced,
            PositionStatus::ReleasePending => StateLabel::PositionReleasePending,
            PositionStatus::Released => StateLabel::PositionReleased,
        }
    }
    pub fn from_pledge(s: PledgeStatus) -> StateLabel {
        match s {
            PledgeStatus::Active => StateLabel::PledgeActive,
            PledgeStatus::ReleaseAuthorized => StateLabel::PledgeReleaseAuthorized,
            PledgeStatus::Released => StateLabel::PledgeReleased,
            PledgeStatus::Defaulted => StateLabel::PledgeDefaulted,
            PledgeStatus::Enforced => StateLabel::PledgeEnforced,
        }
    }
    pub fn from_line(s: LineStatus) -> StateLabel {
        match s {
            LineStatus::Active => StateLabel::LineActive,
            LineStatus::Suspended => StateLabel::LineSuspended,
            LineStatus::Defaulted => StateLabel::LineDefaulted,
            LineStatus::Closed => StateLabel::LineClosed,
        }
    }
    pub fn from_margin(s: MarginState) -> StateLabel {
        match s {
            MarginState::Covered => StateLabel::MarginCovered,
            MarginState::Warning => StateLabel::MarginWarning,
            MarginState::Called => StateLabel::MarginCalled,
        }
    }
    pub fn from_adjustment(s: AdjustmentStatus) -> StateLabel {
        match s {
            AdjustmentStatus::Requested => StateLabel::AdjustmentRequested,
            AdjustmentStatus::CustodianConfirmed => StateLabel::AdjustmentCustodianConfirmed,
            AdjustmentStatus::Approved => StateLabel::AdjustmentApproved,
            AdjustmentStatus::Rejected => StateLabel::AdjustmentRejected,
        }
    }
    pub fn from_readiness(s: ReadinessStatus) -> StateLabel {
        match s {
            ReadinessStatus::Incomplete => StateLabel::ReadinessIncomplete,
            ReadinessStatus::Ready => StateLabel::ReadinessReady,
            ReadinessStatus::Expired => StateLabel::ReadinessExpired,
        }
    }
}

/// Resolves a facility-level id (pledge, line, adjustment) to its aggregate
/// root and the objects above it, in one storage read. This is what lets a
/// line-scoped call emit a framework-scoped CollateralEventV1 without walking
/// line -> pledge -> position -> framework on every event.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FacilityContext {
    pub framework_id: BytesN<32>,
    pub position_id: BytesN<32>,
    pub pledge_id: BytesN<32>,
}

/// Payload for FrameworkRegistered: the three parties and the six document
/// hashes that define the control perimeter.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameworkRegisteredData {
    /// The asset owner / pledgor party for this framework.
    pub owner: Address,
    /// The lending bank party for this framework.
    pub bank: Address,
    /// The custodian holding the physical collateral for this framework.
    pub custodian: Address,
    /// Commitment to the facility (master credit) agreement.
    pub facility_agreement_hash: BytesN<32>,
    /// Commitment to the pledge (security) agreement.
    pub pledge_agreement_hash: BytesN<32>,
    /// Commitment to the custody agreement governing the held asset.
    pub custody_agreement_hash: BytesN<32>,
    /// Commitment to the eligible-collateral schedule.
    pub eligible_schedule_hash: BytesN<32>,
    /// Commitment to the margin policy (advance rate, maintenance, calls).
    pub margin_policy_hash: BytesN<32>,
    /// Commitment to the enforcement waterfall (realization order on default).
    pub enforcement_waterfall_hash: BytesN<32>,
}

/// Payload for DefaultNoticeIssued: the cure deadline the bank set and the
/// commitment to the default notice document. A replay reconstructs the
/// line's cure-expiry deadline (when enforcement becomes available) from this
/// alone, rather than only seeing the notice hash. The deadline is a material
/// projection field the LineDefaulted state label does not carry, so it must
/// travel in the event for a chain-only reducer to render "cure by ledger N".
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultNoticeData {
    /// Ledger sequence by which the cure must be made before the bank may
    /// enforce. Unlocks the bank's enforcement right once passed.
    pub cure_deadline_ledger: u32,
    /// Commitment to the default notice document.
    pub notice_hash: BytesN<32>,
}

/// A single-hash payload, for actions whose only extra datum is one document
/// hash. The meaning of the hash is fixed by the event's `action`. Replaces an
/// empty payload (v23 rejects zero-field contracttype structs as event map data).
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashData {
    /// The single document/evidence commitment for this act. Its meaning is
    /// fixed by the event's `action` (e.g. a control agreement, a release
    /// notice, a default notice, a cure evidence document).
    pub hash: BytesN<32>,
}

/// Payload for PositionRegistered: the attested asset identity.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionRegisteredData {
    /// The asset owner / pledgor whose collateral this position attests.
    pub owner: Address,
    /// The custodian attesting custody of the position's lot.
    pub custodian: Address,
    /// The instrument this collateral is an allocation of (commodity, unit,
    /// grade are read from the registry under this key).
    pub instrument: InstrumentKey,
    /// Commitment to the manifest document for this lot (bar list, warehouse
    /// receipt, lot schedule, with quantities and quality).
    pub manifest_hash: BytesN<32>,
    /// Commitment to the lot's identity (the collateral-uniqueness key).
    pub uniqueness_hash: BytesN<32>,
    /// Commitment to the lot-level quality / assay / grading certificate.
    pub quality_cert_hash: BytesN<32>,
    /// Commitment to the lot-level weight / quantity certificate.
    pub quantity_cert_hash: BytesN<32>,
    /// Commitment to the warehouse / vault / tank / terminal location.
    pub location_hash: BytesN<32>,
    /// Attested quantity in the instrument's unit, scaled by 1e7.
    pub quantity_e7: i128,
    /// The instrument's unit, denormalized so an indexer can interpret the
    /// quantity from the event alone (e.g. "oz", "mt", "bbl", "bu").
    pub unit: Symbol,
    /// Commitment to the instrument's grade / quality standard, denormalized for
    /// the same reason.
    pub grade_hash: BytesN<32>,
    /// Ledger sequence after which the custody attestation expires.
    pub attestation_expiry: u32,
}

/// Payload for LineOpened: the facility's commercial and risk terms.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineOpenedData {
    /// Approved credit limit for the line, in settlement-asset minor units.
    pub approved_limit: i128,
    /// Loan-to-value at opening, in basis points (advance rate).
    pub ltv_bps: u32,
    /// Haircut applied to gross collateral value before LTV at opening, in bps.
    /// Read from the instrument's framework eligibility treatment.
    pub haircut_bps: u32,
    /// Maintenance threshold in basis points; breaching it triggers a margin
    /// warning or call on revaluation.
    pub maintenance_bps: u32,
    /// Collateral price per unit at opening, scaled by 1e7.
    pub price_per_unit_e7: i128,
}

/// Payload for balance-moving acts (drawdown, reversal, repayment): the amount
/// that moved and the line balances after it. A replay reconstructs line
/// utilization from these without re-deriving from every prior event.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BalanceMoveData {
    /// The amount that moved on this act, in settlement-asset minor units
    /// (drawdown adds, reversal and repayment subtract).
    pub amount: i128,
    /// The line's drawn balance after this act.
    pub drawn_after: i128,
    /// The line's available capacity after this act.
    pub available_after: i128,
}

/// Payload for LineRevalued: the price acted on, the resulting margin state,
/// and the balances it was judged against. A replay reconstructs the margin
/// decision and the line's post-revaluation capacity from this alone.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RevaluedData {
    /// Collateral price per unit acted on, scaled by 1e7.
    pub price_per_unit_e7: i128,
    /// Confidence of the submitted price, scaled by 1e7.
    pub confidence_e7: i128,
    /// Resulting margin state after revaluation (Covered, Warning, Called).
    pub margin_state: MarginState,
    /// The line's drawn balance the revaluation was judged against.
    pub drawn_balance: i128,
    /// The advance base (collateral value times advance rate) after revaluation.
    pub advance_base: i128,
    /// The line's available capacity after revaluation.
    pub available_after: i128,
}

/// Payload for EnforcementRecorded: the realization outcome and the legal
/// instrument that effected it. Terminal for the pledge.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementData {
    /// The realization outcome recorded for the enforcement.
    pub outcome: EnforcementOutcome,
    /// Commitment to the legal instrument that effected the enforcement.
    pub legal_instrument_hash: BytesN<32>,
}

/// Payload for AdjustmentApproved: the new attested collateral identity after a
/// bank-approved adjustment. A replay reconstructs the position's post-adjustment
/// the manifest, lot identity, and quantity from this alone, rather than only seeing a
/// single manifest hash.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdjustmentApprovedData {
    /// The kind of collateral adjustment approved.
    pub adjustment_type: AdjustmentType,
    /// Commitment to the new manifest after the adjustment.
    pub new_manifest_hash: BytesN<32>,
    /// Commitment to the new lot identity after the adjustment.
    pub new_uniqueness_hash: BytesN<32>,
    /// Commitment to the new lot-level quality certificate after adjustment.
    pub new_quality_cert_hash: BytesN<32>,
    /// Commitment to the new lot-level quantity certificate after adjustment.
    pub new_quantity_cert_hash: BytesN<32>,
    /// Commitment to the new lot location after adjustment.
    pub new_location_hash: BytesN<32>,
    /// New attested quantity in the instrument's unit, scaled by 1e7.
    pub new_quantity_e7: i128,
    /// Collateral price per unit at approval, scaled by 1e7.
    pub price_per_unit_e7: i128,
}

/// Payload for ReadinessPopulated: the full enforcement-readiness record set by
/// the bank. A replay reconstructs the realization route, settlement asset,
/// valuation source, waterfall, validity window, and version from this, rather
/// than only the route hash.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadinessPopulatedData {
    /// The liquidation agent authorized to realize collateral on enforcement.
    pub liquidation_agent: Address,
    /// Commitment to the realization route (how collateral is liquidated).
    pub realization_route_hash: BytesN<32>,
    /// The settlement asset proceeds are realized into.
    pub settlement_asset: Address,
    /// Commitment to the valuation source used for realization pricing.
    pub valuation_source_hash: BytesN<32>,
    /// Commitment to the proceeds waterfall (distribution order).
    pub waterfall_hash: BytesN<32>,
    /// Ledger sequence until which this readiness record is valid.
    pub valid_until_ledger: u32,
    /// Version of the readiness record (bumped on each repopulation).
    pub version: u32,
    /// Status of the readiness record (Incomplete, Ready, Expired).
    pub status: ReadinessStatus,
}

/// Typed payload of a CollateralEventV1. Tuple enum (v23-legal): each variant
/// carries one payload struct, chosen by the event's `action`.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CollateralPayloadV1 {
    /// Carries the parties and document commitments defining a new framework.
    FrameworkRegistered(FrameworkRegisteredData),
    /// Carries the attested asset identity of a newly registered position.
    PositionRegistered(PositionRegisteredData),
    /// Carries the commercial and risk terms of a newly opened credit line.
    LineOpened(LineOpenedData),
    /// Carries the amount moved and resulting line balances (drawdown,
    /// reversal, repayment, suspend/resume capacity snapshot).
    BalanceMove(BalanceMoveData),
    /// Carries the price, margin state, and balances of a revaluation.
    Revalued(RevaluedData),
    /// Carries the outcome and legal instrument of an enforcement.
    Enforcement(EnforcementData),
    /// Carries the new attested collateral identity after a bank-approved
    /// adjustment.
    AdjustmentApproved(AdjustmentApprovedData),
    /// Carries the full enforcement-readiness record populated by the bank.
    ReadinessPopulated(ReadinessPopulatedData),
    /// Carries the cure deadline and notice commitment of a default notice.
    DefaultNotice(DefaultNoticeData),
    /// Carries a single document/evidence commitment for acts whose only extra
    /// datum is one hash; the hash's meaning is fixed by the event's `action`.
    Hash(HashData),
}

/// The canonical, replayable evidence record for one committed control act.
///
/// This single declaration is both the write-model commitment the contract
/// emits after a valid state transition, and the read-model contract exposed
/// through SEP-48 in the WASM contract spec. The same field set produces the
/// runtime topics and map body AND the inspectable schema, so the wire event
/// and the published spec cannot diverge.
///
/// Map data format: non-topic fields are emitted as a self-describing
/// `Map<Symbol, Val>` (keys sorted by field name), registered in the contract
/// spec so indexers and forkers discover and decode the event by field name
/// without any Argent-specific code.
///
/// The first topic is pinned explicitly to `collateral_event_v1` rather than
/// relying on the SDK's snake_case derivation of the struct name, so a future
/// rename of this struct cannot silently shift the marker every consumer
/// matches on. The pinned value equals the snake_case of `CollateralEventV1`,
/// so this is byte-identical to the previous default emission.
#[contractevent(topics = ["collateral_event_v1"], data_format = "map")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollateralEventV1 {
    /// Framework aggregate id. Every deal-lifecycle event is sequenced under
    /// this id; it is the causal spine an indexer folds the stream along.
    #[topic]
    pub framework_id: BytesN<32>,
    /// Entity kind this act affects (Position, Pledge, Line, Adjustment, etc).
    /// Second topic, so a consumer can filter the stream by object kind.
    #[topic]
    pub entity: EntityKind,
    /// The specific control act committed. Third topic, and the discriminant a
    /// replay fold switches on to apply the event to the projection.
    #[topic]
    pub action: CollateralAction,
    /// Monotonic, framework-scoped sequence number, starting at 1. A gap means
    /// a missing or uningested event; the stream is complete iff contiguous.
    pub sequence: u64,
    /// Address that authorized this act. With `role`, this reconciles an
    /// off-chain DFNS approval to the on-chain act via actor + role + tx hash.
    pub actor: Address,
    /// Role under which the actor was permitted to act. Names the true
    /// authority even for the self-signing owner-delegated acts.
    pub role: Role,
    /// Affected position id, or the all-zero hash when not applicable.
    pub position_id: BytesN<32>,
    /// Affected pledge id, or the all-zero hash when not applicable.
    pub pledge_id: BytesN<32>,
    /// Affected credit line id, or the all-zero hash when not applicable.
    pub credit_line_id: BytesN<32>,
    /// Affected adjustment id, or the all-zero hash when not applicable.
    pub adjustment_id: BytesN<32>,
    /// State label of the affected entity before this act. `Null` when there
    /// was no prior state (the entity is being created by this act).
    pub previous_state: StateLabel,
    /// State label of the affected entity after this act.
    pub new_state: StateLabel,
    /// Commitment to the evidence document for this act, or the all-zero hash
    /// only for acts that genuinely have no associated document.
    pub evidence_hash: BytesN<32>,
    /// Commitment to the policy, condition, or rule checked for this act, or
    /// the all-zero hash when no condition applied.
    pub condition_hash: BytesN<32>,
    /// Valuation reference for acts that acted on a price, or the all-zero
    /// hash when no valuation was involved.
    pub valuation_ref: BytesN<32>,
    /// Ledger sequence the contract observed when it emitted this event.
    pub occurred_ledger: u32,
    /// Typed action payload carrying the fields needed to reconstruct the
    /// changed projection state from this event plus prior events.
    pub payload: CollateralPayloadV1,
}
// ---- Governance event layer (GovernanceEventV1) ---------------------------
//
// The deal-lifecycle stream (CollateralEventV1) is sequenced per framework and
// records acts on positions, pledges, lines, and adjustments. Governance acts
// are different in kind: they change the contract's authority structure itself
// (who may act as bank / custodian / processor, which instruments are eligible,
// and who holds the admin key). These are not framework-scoped deal events; they
// form their own contract-wide stream so the authority history is as auditable
// and replayable as the deal history. An indexer folds this stream to answer
// "who was authorized to do what, as of sequence N".

/// The governance act a GovernanceEventV1 records. First non-pinned topic, so a
/// consumer can filter the governance stream by act kind without decoding the
/// body.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GovernanceAction {
    /// The admin proposed a successor admin (step 1 of the two-step rotation).
    AdminProposed,
    /// The proposed admin accepted control (step 2; admin key changed).
    AdminAccepted,
    /// A party was approved for a role (bank, custodian, processor, etc).
    PartyApproved,
    /// A party's role approval was revoked.
    PartyRevoked,
    /// An instrument was registered in the global registry.
    InstrumentRegistered,
    /// An instrument was retired from the global registry.
    InstrumentRetired,
    /// An instrument was admitted to a framework under a treatment (the CDM
    /// collateral-criteria / treatment result: haircut, max LTV, maintenance).
    InstrumentAdmitted,
}

/// Payload for an admin-rotation governance act (proposed or accepted). Carries
/// the address the act concerns.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdminChangeData {
    /// The proposed (AdminProposed) or newly active (AdminAccepted) admin.
    pub new_admin: Address,
}

/// Payload for a party-approval governance act (approved or revoked). Carries
/// the party and the role the act concerns.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartyChangeData {
    /// The party whose role authorization changed.
    pub party: Address,
    /// The role granted or revoked.
    pub role: Role,
}

/// Payload for an instrument-registry governance act (registered or retired).
/// Carries the instrument key. Retirement and registration are global registry
/// acts, not framework-scoped, so no framework id travels here.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstrumentRegistryData {
    /// The instrument the registry act concerns.
    pub instrument: InstrumentKey,
}

/// Payload for an instrument-admission governance act. Records that an
/// instrument was admitted to a specific framework under a specific treatment.
/// This is the governance-stream counterpart of the FrameworkInstrumentEligibility
/// record written to storage: the authority event that granted the treatment.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstrumentAdmittedData {
    /// The framework the instrument was admitted to.
    pub framework_id: BytesN<32>,
    /// The instrument admitted.
    pub instrument: InstrumentKey,
    /// Commitment to the off-chain eligibility schedule clause / CDM criteria.
    pub eligibility_hash: BytesN<32>,
    /// Discount applied to gross collateral value before LTV, in basis points.
    pub haircut_bps: u32,
    /// Maximum advance rate permitted for this instrument here, in bps.
    pub max_ltv_bps: u32,
    /// Maintenance threshold for this instrument here, in bps.
    pub maintenance_bps: u32,
}

/// Typed governance payload. The variant fixes which identities the act carries,
/// mirroring how CollateralPayloadV1 carries deal-act specifics.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GovernancePayloadV1 {
    /// Admin proposed or accepted: carries the address.
    AdminChange(AdminChangeData),
    /// Party approved or revoked: carries party + role.
    PartyChange(PartyChangeData),
    /// Instrument registered or retired: carries the instrument key.
    InstrumentRegistry(InstrumentRegistryData),
    /// Instrument admitted to a framework: carries framework + treatment.
    InstrumentAdmitted(InstrumentAdmittedData),
}

/// A contract-wide governance event: the replayable record of an authority
/// change. Sequenced under a single global counter (not per framework), so the
/// governance history is one contiguous, gap-checkable stream. The first topic
/// is pinned to `governance_event_v1` for the same forward-compatibility reason
/// CollateralEventV1 pins its marker: a struct rename must not shift the topic
/// consumers match on.
#[contractevent(topics = ["governance_event_v1"], data_format = "map")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceEventV1 {
    /// The governance act. A topic, so the stream is filterable by act kind.
    #[topic]
    pub action: GovernanceAction,
    /// Monotonic, contract-wide sequence number, starting at 1. A gap means a
    /// missing or uningested governance event; the stream is complete iff
    /// contiguous.
    pub sequence: u64,
    /// The address that authorized this governance act (the admin for most acts;
    /// the accepting admin for AdminAccepted; the co-authorizing bank for an
    /// instrument admission).
    pub actor: Address,
    /// Commitment to an evidence document for this act, or the all-zero hash for
    /// acts that have no associated document.
    pub evidence_hash: BytesN<32>,
    /// Ledger sequence the contract observed when it emitted this event.
    pub occurred_ledger: u32,
    /// Typed governance payload carrying the act-specific identities.
    pub payload: GovernancePayloadV1,
}

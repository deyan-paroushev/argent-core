#![no_std]

mod error;
mod types;
mod event;

#[cfg(test)]
mod test;

pub use error::Error;
pub use types::*;
pub use event::*;
use soroban_sdk::{
    contract, contractimpl, contractmeta, symbol_short, xdr::ToXdr, Address, BytesN, Env,
};

const DAY_IN_LEDGERS: u32 = 17_280; // ~5s ledgers
const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

// Self-identifying protocol metadata, written into the WASM custom section
// `contractmetav0` and visible via `stellar contract inspect`. Marks this as a
// deliberate, versioned protocol artifact rather than an anonymous contract.
contractmeta!(key = "name", val = "Argent CreditLedger");
contractmeta!(key = "proto", val = "argent.collateral.v1");
contractmeta!(key = "events", val = "CollateralEventV1");
contractmeta!(key = "sdk", val = "soroban-sdk-23.5.3");
// Upgrade stance: this contract exposes NO code-upgrade entrypoint. Its WASM is
// immutable once deployed; the book of record cannot be rewritten by a later
// implementation swap. Operational control is limited to admin rotation (a
// two-step propose/accept handshake) and the role registry. Stated explicitly
// so a reviewer can confirm the immutability claim against the entrypoints.
contractmeta!(key = "upgrade", val = "none-immutable");
contractmeta!(key = "admin", val = "rotatable-2step");

#[contract]
pub struct CreditLedger;

#[contractimpl]
impl CreditLedger {
    // ---- lifecycle: admin & access control -------------------------------

    /// One-time initialization. Sets the admin who can manage the role registry.
    /// Initialize the ledger. The settlement vault is bound here so a deployment
    /// cannot leave a ledger that silently refuses every repayment (apply_repayment
    /// requires the bound vault). The vault contract only needs to be DEPLOYED, not
    /// initialized, before this call, since binding stores its address without
    /// cross-calling it.
    pub fn initialize(env: Env, admin: Address, settlement_vault: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::SettlementVault, &settlement_vault);
        env.storage()
            .instance()
            .extend_ttl(LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Ok(())
    }

    /// Step 1 of admin rotation. The current admin nominates a successor. This
    /// does NOT change the active admin: the nominee is recorded as pending and
    /// must call accept_admin to take control. A nomination can be overwritten by
    /// proposing a different address, or effectively cancelled by proposing the
    /// current admin. Admin only.
    pub fn propose_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::PendingAdmin, &new_admin);
        env.storage()
            .instance()
            .extend_ttl(LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Self::emit_governance_event(
            &env,
            GovernanceAction::AdminProposed,
            admin,
            Self::zero_hash(&env),
            GovernancePayloadV1::AdminChange(AdminChangeData {
                new_admin,
            }),
        );
        Ok(())
    }

    /// Step 2 of admin rotation. The pending admin accepts control. The caller
    /// must be the exact address the current admin proposed, and must authorize
    /// the call itself, so control can never pass to a mistyped or uncontrolled
    /// address. On success the active admin is replaced and the pending slot is
    /// cleared. Fails if there is no pending proposal or the caller is not it.
    pub fn accept_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        new_admin.require_auth();
        let pending: Address = env
            .storage()
            .instance()
            .get(&DataKey::PendingAdmin)
            .ok_or(Error::NotAuthorized)?;
        if pending != new_admin {
            return Err(Error::NotAuthorized);
        }
        env.storage().instance().set(&DataKey::Admin, &new_admin);
        env.storage().instance().remove(&DataKey::PendingAdmin);
        env.storage()
            .instance()
            .extend_ttl(LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Self::emit_governance_event(
            &env,
            GovernanceAction::AdminAccepted,
            new_admin.clone(),
            Self::zero_hash(&env),
            GovernancePayloadV1::AdminChange(AdminChangeData {
                new_admin,
            }),
        );
        Ok(())
    }

    /// The current admin address. Read-only.
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        Self::admin(&env)
    }

    /// Approve a party for a role. Admin only.
    pub fn approve_party(
        env: Env,
        party: Address,
        role: Role,
    ) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        Self::assert_approvable_role(role)?;
        let key = DataKey::Approved(party.clone(), role);
        env.storage().persistent().set(&key, &true);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        env.events()
            .publish((symbol_short!("party"), symbol_short!("approved")), (party.clone(), role));
        Self::emit_governance_event(
            &env,
            GovernanceAction::PartyApproved,
            admin,
            Self::zero_hash(&env),
            GovernancePayloadV1::PartyChange(PartyChangeData { party, role }),
        );
        Ok(())
    }

    /// Revoke a party's role. Admin only.
    pub fn revoke_party(
        env: Env,
        party: Address,
        role: Role,
    ) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        // Only roles that can be approved can be revoked. Owner and Admin are
        // never approval-granted (Owner is self-delegated, Admin is the rotation
        // key), so revoking them is meaningless and would emit a misleading
        // PartyRevoked event. Mirrors the guard approve_party already applies.
        Self::assert_approvable_role(role)?;
        env.storage()
            .persistent()
            .remove(&DataKey::Approved(party.clone(), role));
        env.events()
            .publish((symbol_short!("party"), symbol_short!("revoked")), (party.clone(), role));
        Self::emit_governance_event(
            &env,
            GovernanceAction::PartyRevoked,
            admin,
            Self::zero_hash(&env),
            GovernancePayloadV1::PartyChange(PartyChangeData { party, role }),
        );
        Ok(())
    }

    pub fn is_approved(env: Env, party: Address, role: Role) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Approved(party, role))
            .unwrap_or(false)
    }

    /// Refresh the TTL on a party's approval entry. Admin only. Lets a long-lived
    /// credit facility keep its approved bank / custodian / processor entries from
    /// archiving over a multi-year term.
    pub fn bump_approval_ttl(env: Env, party: Address, role: Role) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        let key = DataKey::Approved(party, role);
        if !env.storage().persistent().has(&key) {
            return Err(Error::PartyNotApproved);
        }
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Ok(())
    }

    /// Bind the single SettlementVault contract authorized to apply repayments.
    /// Admin only. apply_repayment rejects any other caller, even one that holds
    /// Role::Vault in the registry.
    /// Read the bound settlement vault (provisioning/verification aid).
    pub fn get_settlement_vault(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::SettlementVault)
            .ok_or(Error::NotInitialized)
    }

    // ---- lifecycle: control framework ------------------------------------

    /// Establish the tri-party control framework. Owner, bank, and custodian all
    /// sign, binding the operational state machine to the six governing
    /// documents (facility agreement, pledge/security agreement, custody-control
    /// agreement, eligible-collateral schedule, margin policy, enforcement
    /// waterfall). This is the legal and control perimeter that comes before any
    /// pledge. The bank and custodian must be approved in the registry. The
    /// chain records the framework and which documents govern it; it does not
    /// replace the signed off-chain agreements.
    // ---- canonical event layer (CollateralEventV1) -----------------------
    // The contract keeps stored state as the enforcement guard AND emits a
    // canonical, replayable event so an off-chain indexer can rebuild the full
    // collateral record from the event stream alone. The sequence is
    // framework-scoped and gap-free: bumped inside the same call that writes
    // state, so a failed call reverts the bump with the state.

    /// The all-zero hash, used for "unset" id / evidence fields on an event.
    fn zero_hash(env: &Env) -> BytesN<32> {
        BytesN::from_array(env, &[0u8; 32])
    }

    /// 32-byte fingerprint of an InstrumentKey, used as the storage-key component
    /// wherever an instrument is indexed (Instrument, EligibleInstrument,
    /// FrameworkInstrument). The full InstrumentKey struct serializes to 264 bytes,
    /// over the network's 250-byte ledger-key limit, so it cannot be a storage key
    /// directly. The fingerprint is sha256 of the key's XDR serialization: stable,
    /// collision-resistant, and fixed at 32 bytes. The full InstrumentKey is still
    /// carried in stored values and events, so no information is lost.
    fn instrument_fingerprint(env: &Env, key: &InstrumentKey) -> BytesN<32> {
        env.crypto().sha256(&key.clone().to_xdr(env)).to_bytes()
    }

    /// Read-and-increment the framework-scoped sequence. Returns the sequence to
    /// stamp on the event being emitted (1 for the first event under a framework).
    fn next_framework_sequence(env: &Env, framework_id: &BytesN<32>) -> u64 {
        let key = DataKey::FrameworkSeq(framework_id.clone());
        let last: u64 = env.storage().persistent().get(&key).unwrap_or(0);
        let next = last + 1;
        env.storage().persistent().set(&key, &next);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        next
    }

    /// Next contract-wide governance sequence number. Mirrors
    /// next_framework_sequence but for the single global GovernanceSeq counter:
    /// governance acts form one authority stream, not a per-framework one.
    fn next_governance_sequence(env: &Env) -> u64 {
        let key = DataKey::GovernanceSeq;
        let last: u64 = env.storage().persistent().get(&key).unwrap_or(0);
        let next = last + 1;
        env.storage().persistent().set(&key, &next);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        next
    }

    /// Emit a governance event onto the contract-wide authority stream. Assigns
    /// the next global sequence, stamps the observed ledger, and publishes. The
    /// caller supplies the act, the authorizing actor, an optional evidence
    /// commitment (zero hash when none), and the typed payload.
    fn emit_governance_event(
        env: &Env,
        action: GovernanceAction,
        actor: Address,
        evidence_hash: BytesN<32>,
        payload: GovernancePayloadV1,
    ) {
        let sequence = Self::next_governance_sequence(env);
        let event = GovernanceEventV1 {
            action,
            sequence,
            actor,
            evidence_hash,
            occurred_ledger: env.ledger().sequence(),
            payload,
        };
        event.publish(env);
    }

    /// Public read of the current last-emitted sequence for a framework (0 if
    /// none), so an indexer can check completeness against the chain.
    pub fn framework_sequence(env: Env, framework_id: BytesN<32>) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::FrameworkSeq(framework_id))
            .unwrap_or(0)
    }

    /// The current contract-wide governance sequence: the number of governance
    /// events emitted so far. Read-only. Advances by one on each governance act
    /// (admin rotation, party approval/revocation, instrument lifecycle). A
    /// consumer reads this to bound a replay of the governance stream, or to
    /// confirm a governance act published (the value advancing is the observable
    /// proof emit_governance_event ran to completion).
    pub fn governance_sequence(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::GovernanceSeq)
            .unwrap_or(0)
    }

    /// Resolve the framework id for a line-scoped event via ContextForLine,
    /// falling back to walking line -> pledge -> position if the context map is
    /// somehow absent. Returns (framework_id, position_id, pledge_id).
    fn line_context(
        env: &Env,
        credit_line_id: &BytesN<32>,
    ) -> Result<(BytesN<32>, BytesN<32>, BytesN<32>), Error> {
        if let Some(ctx) = env
            .storage()
            .persistent()
            .get::<DataKey, FacilityContext>(&DataKey::ContextForLine(credit_line_id.clone()))
        {
            return Ok((ctx.framework_id, ctx.position_id, ctx.pledge_id));
        }
        // Fallback: derive from stored records. A canonical event must never be
        // emitted under a zero framework id, so if the line/pledge/position chain
        // cannot be resolved we fail loudly instead of tagging the event to a
        // fake framework.
        let line = env
            .storage()
            .persistent()
            .get::<DataKey, CreditLine>(&DataKey::Line(credit_line_id.clone()))
            .ok_or(Error::LineNotFound)?;
        let pledge = env
            .storage()
            .persistent()
            .get::<DataKey, Pledge>(&DataKey::Pledge(line.pledge_id.clone()))
            .ok_or(Error::PledgeNotFound)?;
        let pos = env
            .storage()
            .persistent()
            .get::<DataKey, VaultPosition>(&DataKey::Position(pledge.position_id.clone()))
            .ok_or(Error::PositionNotFound)?;
        Ok((pos.framework_id, pledge.position_id, line.pledge_id))
    }

    /// Construct and publish a CollateralEventV1. Bumps the framework sequence,
    /// stamps the current ledger, and publishes via the generated .publish().
    #[allow(clippy::too_many_arguments)]
    /// Map a lifecycle action to the framework document whose terms condition
    /// it. Returns the zero hash when the framework cannot be loaded or when the
    /// act has no prior conditioning document (framework registration is itself
    /// the anchoring act, so it conditions nothing). This is a pure read used
    /// only to enrich emitted events; it never affects control flow.
    fn condition_for(
        env: &Env,
        framework_id: &BytesN<32>,
        action: &CollateralAction,
    ) -> BytesN<32> {
        let fwk: Option<ControlFramework> = env
            .storage()
            .persistent()
            .get(&DataKey::Framework(framework_id.clone()));
        let fwk = match fwk {
            Some(f) => f,
            None => return Self::zero_hash(env),
        };
        match action {
            // The position lifecycle is governed by the custody agreement: the
            // custodian's control of the asset is what these acts assert.
            CollateralAction::PositionRegistered
            | CollateralAction::CollateralSelected
            | CollateralAction::CollateralImmobilized => fwk.custody_agreement_hash,

            // Pledge creation, collateral substitution, and release are governed
            // by the pledge agreement (the security interest itself).
            CollateralAction::PledgeActivated
            | CollateralAction::AdjustmentRequested
            | CollateralAction::AdjustmentCustodianConfirmed
            | CollateralAction::AdjustmentApproved
            | CollateralAction::ReleaseAuthorized
            | CollateralAction::ReleaseConfirmed => fwk.pledge_agreement_hash,

            // The facility (loan) terms govern opening, drawing, repaying, and the
            // bank's discretionary stop/resume of the line.
            CollateralAction::LineOpened
            | CollateralAction::DrawdownRecorded
            | CollateralAction::DrawdownReversed
            | CollateralAction::RepaymentApplied
            | CollateralAction::LineSuspendedByBank
            | CollateralAction::LineResumedByBank => fwk.facility_agreement_hash,

            // Revaluation is governed by the margin policy.
            CollateralAction::LineRevalued => fwk.margin_policy_hash,

            // Default, cure, enforcement, and readiness are governed by the
            // enforcement waterfall.
            CollateralAction::DefaultNoticeIssued
            | CollateralAction::DefaultCured
            | CollateralAction::EnforcementRecorded
            | CollateralAction::ReadinessOpened
            | CollateralAction::ReadinessPopulated
            | CollateralAction::ReadinessExpired => fwk.enforcement_waterfall_hash,

            // Framework registration is the anchoring act; it conditions nothing.
            CollateralAction::FrameworkRegistered => Self::zero_hash(env),
        }
    }

    fn emit_event(
        env: &Env,
        framework_id: &BytesN<32>,
        entity: EntityKind,
        action: CollateralAction,
        actor: Address,
        role: Role,
        position_id: BytesN<32>,
        pledge_id: BytesN<32>,
        credit_line_id: BytesN<32>,
        adjustment_id: BytesN<32>,
        previous_state: StateLabel,
        new_state: StateLabel,
        evidence_hash: BytesN<32>,
        condition_hash: BytesN<32>,
        valuation_ref: BytesN<32>,
        payload: CollateralPayloadV1,
    ) {
        // Evidence channel: bind each act to the specific framework document
        // whose terms condition it. If the caller passed an explicit non-zero
        // condition_hash, honor it; otherwise derive the conditioning document
        // from the action. This makes every emitted event self-describing about
        // which leg of the tri-party agreement authorized the act, so an indexer
        // or a forking team can verify the act against the right off-chain terms.
        let condition_hash = if Self::is_zero_hash(env, &condition_hash) {
            Self::condition_for(env, framework_id, &action)
        } else {
            condition_hash
        };
        let sequence = Self::next_framework_sequence(env, framework_id);
        let event = CollateralEventV1 {
            framework_id: framework_id.clone(),
            entity,
            action,
            sequence,
            actor,
            role,
            position_id,
            pledge_id,
            credit_line_id,
            adjustment_id,
            previous_state,
            new_state,
            evidence_hash,
            condition_hash,
            valuation_ref,
            occurred_ledger: env.ledger().sequence(),
            payload,
        };
        event.publish(env);
    }

    pub fn register_framework(
        env: Env,
        framework_id: BytesN<32>,
        owner: Address,
        bank: Address,
        custodian: Address,
        facility_agreement_hash: BytesN<32>,
        pledge_agreement_hash: BytesN<32>,
        custody_agreement_hash: BytesN<32>,
        eligible_schedule_hash: BytesN<32>,
        margin_policy_hash: BytesN<32>,
        enforcement_waterfall_hash: BytesN<32>,
    ) -> Result<(), Error> {
        owner.require_auth();
        bank.require_auth();
        custodian.require_auth();

        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if !Self::is_approved(env.clone(), custodian.clone(), Role::Custodian) {
            return Err(Error::PartyNotApproved);
        }
        if env.storage().persistent().has(&DataKey::Framework(framework_id.clone())) {
            return Err(Error::FrameworkExists);
        }
        if Self::is_zero_hash(&env, &facility_agreement_hash)
            || Self::is_zero_hash(&env, &pledge_agreement_hash)
            || Self::is_zero_hash(&env, &custody_agreement_hash)
            || Self::is_zero_hash(&env, &eligible_schedule_hash)
            || Self::is_zero_hash(&env, &margin_policy_hash)
            || Self::is_zero_hash(&env, &enforcement_waterfall_hash)
        {
            return Err(Error::InvalidDocumentHash);
        }

        let framework = ControlFramework {
            owner: owner.clone(),
            bank: bank.clone(),
            custodian: custodian.clone(),
            facility_agreement_hash: facility_agreement_hash.clone(),
            pledge_agreement_hash: pledge_agreement_hash.clone(),
            custody_agreement_hash: custody_agreement_hash.clone(),
            eligible_schedule_hash: eligible_schedule_hash.clone(),
            margin_policy_hash: margin_policy_hash.clone(),
            enforcement_waterfall_hash: enforcement_waterfall_hash.clone(),
            status: FrameworkStatus::Active,
        };
        let key = DataKey::Framework(framework_id.clone());
        env.storage().persistent().set(&key, &framework);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        env.events()
            .publish((symbol_short!("framework"), symbol_short!("active")), framework_id.clone());

        // canonical event: the framework projection is born here.
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Framework,
            CollateralAction::FrameworkRegistered,
            owner.clone(),
            Role::Owner,
            zero.clone(),
            zero.clone(),
            zero.clone(),
            zero.clone(),
            StateLabel::Null,
            StateLabel::FrameworkActive,
            facility_agreement_hash.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::FrameworkRegistered(FrameworkRegisteredData {
                owner,
                bank,
                custodian,
                facility_agreement_hash,
                pledge_agreement_hash,
                custody_agreement_hash,
                eligible_schedule_hash,
                margin_policy_hash,
                enforcement_waterfall_hash,
            }),
        );
        Ok(())
    }

    /// Read a control framework.
    pub fn get_framework(
        env: Env,
        framework_id: BytesN<32>,
    ) -> Result<ControlFramework, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Framework(framework_id))
            .ok_or(Error::FrameworkNotActive)
    }

    // ---- instrument registry (Daml Finance "Instrument") -----------------

    /// Register an instrument: the economic identity of one unit of collateral
    /// (commodity, unit, grade), defined once and referenced by every position
    /// of it. The issuer (who defines the asset standard) and the depository
    /// (the custodian attesting custody of this asset class) both sign, so
    /// neither can unilaterally define the asset. Idempotent over a live key.
    pub fn register_instrument(env: Env, instrument: Instrument) -> Result<(), Error> {
        instrument.key.issuer.require_auth();
        instrument.key.depository.require_auth();

        if Self::is_zero_hash(&env, &instrument.grade_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        let key = DataKey::Instrument(Self::instrument_fingerprint(&env, &instrument.key));
        if env.storage().persistent().has(&key) {
            return Err(Error::NotAuthorized);
        }
        // Normalize status on registration: a freshly registered instrument is
        // Active regardless of what the caller passed.
        let mut stored = instrument.clone();
        stored.status = InstrumentStatus::Active;
        env.storage().persistent().set(&key, &stored);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Self::emit_governance_event(
            &env,
            GovernanceAction::InstrumentRegistered,
            instrument.key.issuer.clone(),
            Self::zero_hash(&env),
            GovernancePayloadV1::InstrumentRegistry(InstrumentRegistryData {
                instrument: instrument.key,
            }),
        );
        Ok(())
    }

    /// Retire an instrument. New positions can no longer reference it; existing
    /// holdings are unaffected. Issuer and depository both sign, as at creation.
    pub fn retire_instrument(
        env: Env,
        key: InstrumentKey,
        issuer: Address,
        depository: Address,
    ) -> Result<(), Error> {
        issuer.require_auth();
        depository.require_auth();

        let storage_key = DataKey::Instrument(Self::instrument_fingerprint(&env, &key));
        let mut instrument: Instrument = env
            .storage()
            .persistent()
            .get(&storage_key)
            .ok_or(Error::InstrumentNotFound)?;
        // Only the instrument's own issuer and depository may retire it.
        if instrument.key.issuer != issuer || instrument.key.depository != depository {
            return Err(Error::NotAuthorized);
        }
        instrument.status = InstrumentStatus::Retired;
        env.storage().persistent().set(&storage_key, &instrument);
        Self::emit_governance_event(
            &env,
            GovernanceAction::InstrumentRetired,
            issuer,
            Self::zero_hash(&env),
            GovernancePayloadV1::InstrumentRegistry(InstrumentRegistryData {
                instrument: key,
            }),
        );
        Ok(())
    }

    pub fn get_instrument(env: Env, key: InstrumentKey) -> Result<Instrument, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Instrument(Self::instrument_fingerprint(&env, &key)))
            .ok_or(Error::InstrumentNotFound)
    }

    /// Admit an instrument to a framework's eligible-collateral set (the CDM "GC
    /// basket" made enforceable). The framework's bank and custodian both sign:
    /// they jointly decide what collateral the facility will accept. Only an
    /// active, registered instrument can be admitted. register_position then
    /// checks membership here.
    pub fn admit_instrument(
        env: Env,
        framework_id: BytesN<32>,
        instrument: InstrumentKey,
        bank: Address,
        custodian: Address,
        eligibility_hash: BytesN<32>,
        haircut_bps: u32,
        max_ltv_bps: u32,
        maintenance_bps: u32,
    ) -> Result<(), Error> {
        bank.require_auth();
        custodian.require_auth();

        let framework: ControlFramework = env
            .storage()
            .persistent()
            .get(&DataKey::Framework(framework_id.clone()))
            .ok_or(Error::FrameworkNotActive)?;
        if framework.status != FrameworkStatus::Active {
            return Err(Error::FrameworkNotActive);
        }
        if framework.bank != bank || framework.custodian != custodian {
            return Err(Error::NotAuthorized);
        }
        // Re-check current approval. require_auth proves the key signed, and the
        // framework match proves these are the framework's named parties, but
        // neither proves the party is STILL approved. A bank or custodian whose
        // approval was revoked after the framework was created must not be able
        // to admit new instruments. This mirrors the approval re-check every
        // other governed act performs.
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if !Self::is_approved(env.clone(), custodian.clone(), Role::Custodian) {
            return Err(Error::PartyNotApproved);
        }

        let instrument_rec: Instrument = env
            .storage()
            .persistent()
            .get(&DataKey::Instrument(Self::instrument_fingerprint(&env, &instrument)))
            .ok_or(Error::InstrumentNotFound)?;
        if instrument_rec.status != InstrumentStatus::Active {
            return Err(Error::InstrumentNotEligible);
        }

        // The eligibility schedule clause must be a real commitment, and the
        // treatment must be coherent: a haircut cannot exceed 100%, and the
        // advance rate must be positive and strictly below the maintenance
        // threshold, which itself cannot exceed 100%. This is the on-chain
        // record of the CDM collateral criteria / treatment the bank applies to
        // this instrument under this framework.
        if Self::is_zero_hash(&env, &eligibility_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        if haircut_bps > 10_000
            || max_ltv_bps == 0
            || max_ltv_bps >= maintenance_bps
            || maintenance_bps > 10_000
        {
            return Err(Error::InvalidRiskParams);
        }

        let eligibility = FrameworkInstrumentEligibility {
            framework_id: framework_id.clone(),
            instrument: instrument.clone(),
            eligibility_hash: eligibility_hash.clone(),
            haircut_bps,
            max_ltv_bps,
            maintenance_bps,
            status: EligibilityStatus::Active,
        };
        // Commit the eligibility record first, then emit. Soroban rolls the
        // whole transaction back on a later panic, so an event could never be
        // committed without its state, but writing before emitting is the
        // cleaner ordering: the event always follows the state it announces.
        let key = DataKey::FrameworkInstrument(
            framework_id.clone(),
            Self::instrument_fingerprint(&env, &instrument),
        );
        env.storage().persistent().set(&key, &eligibility);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Self::emit_governance_event(
            &env,
            GovernanceAction::InstrumentAdmitted,
            bank,
            Self::zero_hash(&env),
            GovernancePayloadV1::InstrumentAdmitted(InstrumentAdmittedData {
                framework_id,
                instrument,
                eligibility_hash,
                haircut_bps,
                max_ltv_bps,
                maintenance_bps,
            }),
        );
        Ok(())
    }

    // ---- lifecycle: collateral & facility --------------------------------

    /// Register an attested allocated-collateral position (a Daml "Holding").
    /// Both owner and custodian sign; the custodian must be an approved attestor;
    /// the attestation must not already be expired. The position references an
    /// instrument that must be registered, Active, and admitted to this
    /// framework's eligible set. The position id is supplied by the caller
    /// (derived off-chain from owner + manifest_hash) so the service controls
    /// referencing.
    pub fn register_position(
        env: Env,
        position_id: BytesN<32>,
        framework_id: BytesN<32>,
        owner: Address,
        custodian: Address,
        instrument: InstrumentKey,
        evidence: LotEvidence,
        quantity_e7: i128,
        attestation_expiry: u32,
    ) -> Result<(), Error> {
        owner.require_auth();
        custodian.require_auth();
        // Destructure the lot evidence bundle into the individual commitments.
        let LotEvidence {
            manifest_hash,
            uniqueness_hash,
            quality_cert_hash,
            quantity_cert_hash,
            location_hash,
        } = evidence;

        if !Self::is_approved(env.clone(), custodian.clone(), Role::Custodian) {
            return Err(Error::PartyNotApproved);
        }
        // The position must be registered under an active control framework, and
        // the position's owner and custodian must be the parties named in it.
        // This is the on-chain designation of the lot as collateral under the
        // framework's eligible-collateral schedule.
        let framework = Self::load_active_framework(&env, &framework_id)?;
        if framework.owner != owner || framework.custodian != custodian {
            return Err(Error::FrameworkPartyMismatch);
        }
        // The instrument must exist, be Active, and be admitted to this framework.
        // This is the on-chain enforcement of the eligible-collateral schedule:
        // a facility only accepts the instruments its bank and custodian admitted.
        let instrument_rec: Instrument = env
            .storage()
            .persistent()
            .get(&DataKey::Instrument(Self::instrument_fingerprint(&env, &instrument)))
            .ok_or(Error::InstrumentNotFound)?;
        if instrument_rec.status != InstrumentStatus::Active {
            return Err(Error::InstrumentNotEligible);
        }
        // The instrument must be admitted to this framework with an active
        // treatment record (the CDM eligibility result). This both proves
        // eligibility and is where the haircut / LTV ceiling come from at line
        // opening.
        Self::load_active_eligibility(&env, &framework_id, &instrument)?;
        if env.storage().persistent().has(&DataKey::Position(position_id.clone())) {
            return Err(Error::PositionExists);
        }
        if Self::is_zero_hash(&env, &manifest_hash)
            || Self::is_zero_hash(&env, &uniqueness_hash)
            || Self::is_zero_hash(&env, &quality_cert_hash)
            || Self::is_zero_hash(&env, &quantity_cert_hash)
            || Self::is_zero_hash(&env, &location_hash)
        {
            return Err(Error::InvalidDocumentHash);
        }
        // Lot uniqueness: this exact lot must not already be active under another
        // position. This enforces no-double-pledge at the lot level, which is the
        // core promise of the instrument (and prevents duplicate financing of one
        // warehouse receipt).
        if env.storage().persistent().has(&DataKey::LotLock(uniqueness_hash.clone())) {
            return Err(Error::LotAlreadyActive);
        }
        if attestation_expiry <= env.ledger().sequence() {
            return Err(Error::AttestationStale);
        }
        if quantity_e7 <= 0 {
            return Err(Error::AmountNotPositive);
        }

        let pos = VaultPosition {
            owner: owner.clone(),
            custodian: custodian.clone(),
            framework_id: framework_id.clone(),
            instrument: instrument.clone(),
            manifest_hash: manifest_hash.clone(),
            uniqueness_hash: uniqueness_hash.clone(),
            quality_cert_hash: quality_cert_hash.clone(),
            quantity_cert_hash: quantity_cert_hash.clone(),
            location_hash: location_hash.clone(),
            quantity_e7,
            attestation_expiry,
            status: PositionStatus::Free,
        };
        Self::save_position(&env, &position_id, &pos);
        // Lock the lot to this position until it reaches a terminal state.
        let lot_key = DataKey::LotLock(uniqueness_hash.clone());
        env.storage().persistent().set(&lot_key, &position_id);
        env.storage()
            .persistent()
            .extend_ttl(&lot_key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        // context map: position_id -> framework_id, for cheap event tagging.
        let ctx_key = DataKey::ContextForPosition(position_id.clone());
        env.storage().persistent().set(&ctx_key, &framework_id);
        env.storage()
            .persistent()
            .extend_ttl(&ctx_key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        env.events()
            .publish((symbol_short!("position"), symbol_short!("created")), position_id.clone());

        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Position,
            CollateralAction::PositionRegistered,
            custodian.clone(),
            Role::Custodian,
            position_id,
            zero.clone(),
            zero.clone(),
            zero.clone(),
            StateLabel::Null,
            StateLabel::PositionFree,
            manifest_hash.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::PositionRegistered(PositionRegisteredData {
                owner,
                custodian,
                instrument: instrument.clone(),
                manifest_hash,
                uniqueness_hash,
                quality_cert_hash,
                quantity_cert_hash,
                location_hash,
                quantity_e7,
                unit: instrument_rec.unit,
                grade_hash: instrument_rec.grade_hash,
                attestation_expiry,
            }),
        );
        Ok(())
    }

    /// Owner selects a registered position to be designated as collateral,
    /// signing a collateral-request instruction. This is the owner-selects half
    /// of the two-sided consent: the owner directs that the lot (already
    /// identified on the position) be committed. The custodian then confirms it
    /// can hold and block that selection at immobilization. Owner signs; the
    /// owner must be the position's owner; the position must be Free. Moves
    /// Free -> Selected.
    pub fn select_lot_for_collateral(
        env: Env,
        position_id: BytesN<32>,
        owner: Address,
        request_hash: BytesN<32>,
    ) -> Result<(), Error> {
        owner.require_auth();

        let mut pos = Self::load_position(&env, &position_id)?;
        if pos.owner != owner {
            return Err(Error::NotAuthorized);
        }
        if pos.status != PositionStatus::Free {
            return Err(Error::PositionNotFree);
        }
        if pos.attestation_expiry <= env.ledger().sequence() {
            return Err(Error::AttestationStale);
        }
        if Self::is_zero_hash(&env, &request_hash) {
            return Err(Error::InvalidDocumentHash);
        }

        pos.status = PositionStatus::Selected;
        Self::save_position(&env, &position_id, &pos);

        let selection = CollateralSelection {
            position_id: position_id.clone(),
            owner: owner.clone(),
            request_hash: request_hash.clone(),
        };
        let key = DataKey::Selection(position_id.clone());
        env.storage().persistent().set(&key, &selection);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        env.events()
            .publish((symbol_short!("position"), symbol_short!("selected")), position_id.clone());

        let framework_id = env
            .storage()
            .persistent()
            .get(&DataKey::ContextForPosition(position_id.clone()))
            .unwrap_or_else(|| pos.framework_id.clone());
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Position,
            CollateralAction::CollateralSelected,
            owner,
            Role::Owner,
            position_id,
            zero.clone(),
            zero.clone(),
            zero.clone(),
            StateLabel::PositionFree,
            StateLabel::PositionSelected,
            request_hash.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::Hash(HashData { hash: request_hash }),
        );
        Ok(())
    }

    /// Read the owner's lot-selection instruction for a position.
    pub fn get_selection(
        env: Env,
        position_id: BytesN<32>,
    ) -> Result<CollateralSelection, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Selection(position_id))
            .ok_or(Error::PositionNotFound)
    }

    /// Custodian confirms the owner's selected lot and immobilizes it under
    /// the tri-party control agreement. This is the control point that converts
    /// a free holding into bankable collateral: the custodian cryptographically
    /// attests this exact manifest (the position's manifest_hash) and accepts the
    /// block, so the owner can no longer withdraw or substitute unilaterally and
    /// a bank can rely on the lot being there to pledge.
    ///
    /// The custodian acts under the tri-party control framework. A real control
    /// agreement is a separate instrument signed by owner, custodian, and bank;
    /// here its hash (`control_agreement_hash`) is anchored as a CustodyControl
    /// record so the framework reference is part of the trail. The chain does
    /// not replace the off-chain signed agreement.
    ///
    /// Custodian signs. The custodian must be the position's own custodian and
    /// approved in the registry. The position must be Selected (the owner has
    /// signed its collateral-request) and its attestation fresh. The custodian
    /// confirms it can hold and block that selection. Moves Selected -> Earmarked.
    pub fn confirm_and_immobilize(
        env: Env,
        position_id: BytesN<32>,
        custodian: Address,
        control_agreement_hash: BytesN<32>,
    ) -> Result<(), Error> {
        custodian.require_auth();

        if !Self::is_approved(env.clone(), custodian.clone(), Role::Custodian) {
            return Err(Error::PartyNotApproved);
        }
        let mut pos = Self::load_position(&env, &position_id)?;
        if pos.custodian != custodian {
            return Err(Error::NotAuthorized);
        }
        if pos.status != PositionStatus::Selected {
            return Err(Error::PositionNotSelected);
        }
        if pos.attestation_expiry <= env.ledger().sequence() {
            return Err(Error::AttestationStale);
        }
        if Self::is_zero_hash(&env, &control_agreement_hash) {
            return Err(Error::InvalidDocumentHash);
        }

        pos.status = PositionStatus::Earmarked;
        Self::save_position(&env, &position_id, &pos);

        // Record the tri-party control framework the custodian is acting under.
        let control = CustodyControl {
            position_id: position_id.clone(),
            custodian: custodian.clone(),
            control_agreement_hash: control_agreement_hash.clone(),
            status: PositionStatus::Earmarked,
        };
        let key = DataKey::Control(position_id.clone());
        env.storage().persistent().set(&key, &control);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        env.events()
            .publish((symbol_short!("position"), symbol_short!("earmarkd")), position_id.clone());

        let framework_id = env
            .storage()
            .persistent()
            .get(&DataKey::ContextForPosition(position_id.clone()))
            .unwrap_or_else(|| pos.framework_id.clone());
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Position,
            CollateralAction::CollateralImmobilized,
            custodian,
            Role::Custodian,
            position_id,
            zero.clone(),
            zero.clone(),
            zero.clone(),
            StateLabel::PositionSelected,
            StateLabel::PositionEarmarked,
            control_agreement_hash.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::Hash(HashData { hash: control_agreement_hash }),
        );
        Ok(())
    }

    /// Read the tri-party control framework recorded for a position.
    pub fn get_custody_control(
        env: Env,
        position_id: BytesN<32>,
    ) -> Result<CustodyControl, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Control(position_id))
            .ok_or(Error::PositionNotFound)
    }

    /// Lock an earmarked position to a bank. Owner and bank both sign. Bank must
    /// be approved and named in the framework; position must be Earmarked;
    /// attestation must be fresh.
    pub fn activate_pledge(
        env: Env,
        pledge_id: BytesN<32>,
        position_id: BytesN<32>,
        owner: Address,
        bank: Address,
        legal_terms_hash: BytesN<32>,
    ) -> Result<(), Error> {
        owner.require_auth();
        bank.require_auth();

        // A pledge id is single-use. Re-activating an existing id would overwrite
        // a live pledge record; the position-state checks below narrow but do not
        // close this, so guard it explicitly.
        if env.storage().persistent().has(&DataKey::Pledge(pledge_id.clone())) {
            return Err(Error::PledgeExists);
        }

        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        let mut pos = Self::load_position(&env, &position_id)?;
        if pos.owner != owner {
            return Err(Error::NotAuthorized);
        }
        // The bank must be the secured party named in the position's control
        // framework. A position cannot be pledged to a bank outside its
        // governing tri-party agreement.
        let framework = Self::load_active_framework(&env, &pos.framework_id)?;
        if framework.bank != bank {
            return Err(Error::FrameworkPartyMismatch);
        }
        // The custodian must have confirmed and immobilized the lot first. A
        // bank cannot pledge a position the custodian has not earmarked under
        // the control agreement.
        if pos.status != PositionStatus::Earmarked {
            return Err(Error::PositionNotEarmarked);
        }
        if pos.attestation_expiry <= env.ledger().sequence() {
            return Err(Error::AttestationStale);
        }

        pos.status = PositionStatus::Pledged;
        Self::save_position(&env, &position_id, &pos);

        let framework_id = pos.framework_id.clone();
        let pledge = Pledge {
            position_id: position_id.clone(),
            pledgor: owner,
            bank: bank.clone(),
            legal_terms_hash: legal_terms_hash.clone(),
            status: PledgeStatus::Active,
        };
        Self::save_pledge(&env, &pledge_id, &pledge);

        // context map: pledge_id -> (framework, position) for later pledge/line events.
        let ctx_key = DataKey::ContextForPledge(pledge_id.clone());
        let ctx = FacilityContext {
            framework_id: framework_id.clone(),
            position_id: position_id.clone(),
            pledge_id: pledge_id.clone(),
        };
        env.storage().persistent().set(&ctx_key, &ctx);
        env.storage()
            .persistent()
            .extend_ttl(&ctx_key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        env.events()
            .publish((symbol_short!("pledge"), symbol_short!("active")), pledge_id.clone());

        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Pledge,
            CollateralAction::PledgeActivated,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            zero.clone(),
            zero.clone(),
            StateLabel::PositionEarmarked,
            StateLabel::PledgeActive,
            legal_terms_hash.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::Hash(HashData { hash: legal_terms_hash }),
        );
        Ok(())
    }

    /// Open a credit line against an active pledge. Bank signs. The limit must
    /// not exceed the borrowing base (quantity * price * ltv). The price is
    /// passed in by the bank for the MVP (an OracleAdapter replaces this in
    /// Phase 5); it is the price per unit in the line's minor units,
    /// scaled by 1e7.
    pub fn open_credit_line(
        env: Env,
        credit_line_id: BytesN<32>,
        pledge_id: BytesN<32>,
        bank: Address,
        cardholder: Address,
        approved_limit: i128,
        ltv_bps: u32,
        maintenance_bps: u32,
        price_per_unit_e7: i128,
        valuation_ref: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        // A line cannot be opened without pointing at the valuation that justifies
        // the price and limit. valuation_ref commits to the off-chain valuation
        // record, so the opening figure is auditable against a real source rather
        // than an unanchored number. Mirrors the requirement on revalue.
        if Self::is_zero_hash(&env, &valuation_ref) {
            return Err(Error::InvalidDocumentHash);
        }
        if env.storage().persistent().has(&DataKey::Line(credit_line_id.clone())) {
            return Err(Error::LineExists);
        }
        if env.storage().persistent().has(&DataKey::LineForPledge(pledge_id.clone())) {
            return Err(Error::PledgeAlreadyHasLine);
        }

        let pledge = Self::load_pledge(&env, &pledge_id)?;
        if pledge.status != PledgeStatus::Active {
            return Err(Error::PledgeNotActive);
        }
        if pledge.bank != bank {
            return Err(Error::NotAuthorized);
        }
        // The cardholder (borrower) must be the pledgor: the entity whose collateral is
        // pledged is the entity that may draw against it. Argent does not support
        // a bank unilaterally designating a third-party borrower against someone
        // else's collateral; that would require an explicit third-party-collateral
        // consent model, which is deliberately out of scope. The cardholder also
        // signs the opening, so the line terms carry the borrower's own consent in
        // the authorization tree, not just the bank's.
        if cardholder != pledge.pledgor {
            return Err(Error::NotAuthorized);
        }
        cardholder.require_auth();
        if approved_limit <= 0 || price_per_unit_e7 <= 0 {
            return Err(Error::AmountNotPositive);
        }
        // Risk-parameter invariant: the advance rate must be positive and
        // strictly below the maintenance threshold, and the maintenance
        // threshold may not exceed 100% of collateral value. The bank chooses
        // the actual levels per its own policy; the contract only enforces that
        // a line can never be set to lend past the value of its collateral.
        if ltv_bps == 0 || ltv_bps >= maintenance_bps || maintenance_bps > 10_000 {
            return Err(Error::InvalidRiskParams);
        }

        let pos = Self::load_position(&env, &pledge.position_id)?;
        // Load the instrument's framework eligibility treatment. The bank's
        // pre-approved terms bind the line: the requested advance rate may not
        // exceed the instrument's max LTV ceiling, and the haircut recorded in
        // the treatment is applied to the borrowing base. Maintenance stays at
        // the bank's per-line discretion (the treatment carries a framework
        // maintenance level, but the contract does not pin each line to it).
        let eligibility =
            Self::load_active_eligibility(&env, &pos.framework_id, &pos.instrument)?;
        if ltv_bps > eligibility.max_ltv_bps {
            return Err(Error::InvalidRiskParams);
        }
        // Maintenance floor: in Argent's convention the margin call fires when
        // the drawn balance exceeds maintenance_bps of collateral value, so a
        // HIGHER maintenance_bps fires LATER and is WEAKER protection. The
        // framework's admitted maintenance_bps is therefore the weakest the line
        // may be: a line can set maintenance lower (call sooner, stricter) but
        // not higher (call later, weaker) than the treatment allows.
        if maintenance_bps > eligibility.maintenance_bps {
            return Err(Error::InvalidRiskParams);
        }
        let base = Self::borrowing_base(
            pos.quantity_e7,
            price_per_unit_e7,
            eligibility.haircut_bps,
            ltv_bps,
        )?;
        if approved_limit > base {
            return Err(Error::LimitExceedsBorrowingBase);
        }

        let line = CreditLine {
            pledge_id: pledge_id.clone(),
            bank: bank.clone(),
            cardholder,
            approved_limit,
            drawn_balance: 0,
            available_limit: approved_limit,
            ltv_bps,
            maintenance_bps,
            cure_expiry_ledger: 0,
            manual_bank_suspension: false,
            status: LineStatus::Active,
        };
        Self::save_line(&env, &credit_line_id, &line);
        let pledge_line_key = DataKey::LineForPledge(line.pledge_id.clone());
        env.storage().persistent().set(&pledge_line_key, &credit_line_id);
        env.storage()
            .persistent()
            .extend_ttl(&pledge_line_key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        // Resolve the facility context from the pledge, then re-key it to the
        // line so every later line-scoped event resolves in one read.
        let ctx = env
            .storage()
            .persistent()
            .get::<DataKey, FacilityContext>(&DataKey::ContextForPledge(pledge_id.clone()))
            .unwrap_or_else(|| FacilityContext {
                framework_id: pos.framework_id.clone(),
                position_id: pledge.position_id.clone(),
                pledge_id: pledge_id.clone(),
            });
        let framework_id = ctx.framework_id.clone();
        let line_ctx_key = DataKey::ContextForLine(credit_line_id.clone());
        env.storage().persistent().set(&line_ctx_key, &ctx);
        env.storage()
            .persistent()
            .extend_ttl(&line_ctx_key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        env.events()
            .publish((symbol_short!("line"), symbol_short!("opened")), credit_line_id.clone());

        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Line,
            CollateralAction::LineOpened,
            bank,
            Role::Bank,
            ctx.position_id.clone(),
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::Null,
            StateLabel::LineActive,
            zero.clone(),
            zero.clone(),
            valuation_ref,
            CollateralPayloadV1::LineOpened(LineOpenedData {
                approved_limit,
                ltv_bps,
                haircut_bps: eligibility.haircut_bps,
                maintenance_bps,
                price_per_unit_e7,
            }),
        );
        Ok(())
    }

    // ---- lifecycle: utilization (mirrors the card world, moves no money) --

    /// Record a card drawdown. Called by an approved processor after an
    /// off-chain authorization. Idempotent on `auth_ref`.
    pub fn record_drawdown(
        env: Env,
        credit_line_id: BytesN<32>,
        processor: Address,
        auth_ref: BytesN<32>,
        amount: i128,
    ) -> Result<(), Error> {
        processor.require_auth();
        if !Self::is_approved(env.clone(), processor.clone(), Role::Processor) {
            return Err(Error::PartyNotApproved);
        }
        if amount <= 0 {
            return Err(Error::AmountNotPositive);
        }
        if env.storage().persistent().has(&DataKey::Draw(auth_ref.clone())) {
            return Err(Error::DuplicateAuthRef);
        }

        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.status != LineStatus::Active {
            return Err(Error::LineNotActive);
        }
        let pledge = Self::load_pledge(&env, &line.pledge_id)?;
        if pledge.status != PledgeStatus::Active {
            return Err(Error::PledgeNotActive);
        }
        if line.available_limit < amount {
            return Err(Error::InsufficientCapacity);
        }

        line.drawn_balance += amount;
        line.available_limit -= amount;
        Self::save_line(&env, &credit_line_id, &line);
        let record = DrawdownRecord {
            credit_line_id: credit_line_id.clone(),
            amount,
            drawn_at_ledger: env.ledger().sequence(),
            reversed: false,
        };
        let key = DataKey::Draw(auth_ref.clone());
        env.storage().persistent().set(&key, &record);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        env.events().publish(
            (symbol_short!("card"), symbol_short!("draw")),
            (credit_line_id.clone(), auth_ref.clone(), amount),
        );

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Drawdown,
            CollateralAction::DrawdownRecorded,
            processor,
            Role::Processor,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::LineActive,
            StateLabel::LineActive,
            auth_ref,
            zero.clone(),
            zero,
            CollateralPayloadV1::BalanceMove(BalanceMoveData {
                amount,
                drawn_after: line.drawn_balance,
                available_after: line.available_limit,
            }),
        );
        Ok(())
    }

    /// Reverse a previously recorded drawdown (auth expiry / reversal).
    pub fn reverse_drawdown(
        env: Env,
        credit_line_id: BytesN<32>,
        processor: Address,
        auth_ref: BytesN<32>,
        amount: i128,
    ) -> Result<(), Error> {
        processor.require_auth();
        if !Self::is_approved(env.clone(), processor.clone(), Role::Processor) {
            return Err(Error::PartyNotApproved);
        }
        if amount <= 0 {
            return Err(Error::AmountNotPositive);
        }
        // The drawdown must exist and not have been reversed already.
        let key = DataKey::Draw(auth_ref.clone());
        let mut record: DrawdownRecord = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::NothingToReverse)?;
        if record.reversed {
            return Err(Error::NothingToReverse);
        }
        // A reversal must unwind exactly what was drawn under this auth_ref, on
        // the same line. This is the accounting guard: without it a processor
        // could reverse an arbitrary amount and corrupt the drawn balance.
        if record.credit_line_id != credit_line_id {
            return Err(Error::LineNotFound);
        }
        if amount != record.amount {
            return Err(Error::ReversalAmountMismatch);
        }

        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.status == LineStatus::Closed {
            return Err(Error::LineNotActive);
        }
        line.drawn_balance -= amount;
        line.available_limit += amount;
        Self::save_line(&env, &credit_line_id, &line);

        // Keep the record for audit, marked reversed, so the same auth_ref can
        // be neither drawn again nor reversed again.
        record.reversed = true;
        env.storage().persistent().set(&key, &record);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        env.events().publish(
            (symbol_short!("card"), symbol_short!("reverse")),
            (credit_line_id.clone(), auth_ref.clone(), amount),
        );

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Drawdown,
            CollateralAction::DrawdownReversed,
            processor,
            Role::Processor,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::from_line(line.status),
            StateLabel::from_line(line.status),
            auth_ref,
            zero.clone(),
            zero,
            CollateralPayloadV1::BalanceMove(BalanceMoveData {
                amount,
                drawn_after: line.drawn_balance,
                available_after: line.available_limit,
            }),
        );
        Ok(())
    }

    /// Read a drawdown record by its authorization reference. Returns the stored
    /// amount, line, ledger, and whether it has been reversed. Lets the
    /// off-chain layer and evidence certificates reconcile a reversal against the
    /// exact drawdown it unwound.
    pub fn get_drawdown(
        env: Env,
        auth_ref: BytesN<32>,
    ) -> Result<DrawdownRecord, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Draw(auth_ref))
            .ok_or(Error::NothingToReverse)
    }

    /// Apply a repayment to a line, reducing the drawn balance and restoring
    /// capacity. Invoked by an approved SettlementVault as part of the atomic
    /// repay-and-release. The `vault` is the calling contract; it authorizes
    /// the sub-invocation and must be an approved Vault. The bank's interest is
    /// protected because only a vault bound to this ledger and approved by the
    /// admin can reach this entry point.
    pub fn apply_repayment(
        env: Env,
        credit_line_id: BytesN<32>,
        vault: Address,
        payment_ref: BytesN<32>,
        amount: i128,
    ) -> Result<(), Error> {
        vault.require_auth();
        // The repayment may only be applied by the single SettlementVault
        // contract bound at initialize. Holding Role::Vault in the
        // registry is necessary but not sufficient: a debt reduction must come
        // from the bound vault that proved a real settlement transfer, not from
        // any address that was ever approved as a Vault.
        if vault != Self::settlement_vault(&env)? {
            return Err(Error::PartyNotApproved);
        }
        if !Self::is_approved(env.clone(), vault.clone(), Role::Vault) {
            return Err(Error::PartyNotApproved);
        }
        if amount <= 0 {
            return Err(Error::AmountNotPositive);
        }
        // Idempotency: the same off-chain payment cannot be applied twice.
        if env.storage().persistent().has(&DataKey::Repayment(payment_ref.clone())) {
            return Err(Error::DuplicatePaymentRef);
        }
        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.status == LineStatus::Closed {
            return Err(Error::LineNotActive);
        }
        if amount > line.drawn_balance {
            return Err(Error::RepaymentExceedsBalance);
        }
        let applied = amount;
        line.drawn_balance -= applied;
        line.available_limit += applied;
        Self::save_line(&env, &credit_line_id, &line);

        // Record the repayment (also serves as the idempotency guard). Repayment
        // reduces exposure only; it does not release collateral.
        let record = RepaymentRecord {
            credit_line_id: credit_line_id.clone(),
            amount_applied: applied,
            applied_at_ledger: env.ledger().sequence(),
        };
        let key = DataKey::Repayment(payment_ref.clone());
        env.storage().persistent().set(&key, &record);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        env.events().publish(
            (symbol_short!("repay"), symbol_short!("applied")),
            (credit_line_id.clone(), applied),
        );

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Repayment,
            CollateralAction::RepaymentApplied,
            vault,
            Role::Vault,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::from_line(line.status),
            StateLabel::from_line(line.status),
            payment_ref,
            zero.clone(),
            zero,
            CollateralPayloadV1::BalanceMove(BalanceMoveData {
                amount: applied,
                drawn_after: line.drawn_balance,
                available_after: line.available_limit,
            }),
        );
        Ok(())
    }

    /// Read a repayment record by its payment reference.
    pub fn get_repayment(
        env: Env,
        payment_ref: BytesN<32>,
    ) -> Result<RepaymentRecord, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Repayment(payment_ref))
            .ok_or(Error::LineNotFound)
    }

    // ---- collateral adjustment -------------------------------------------

    /// Owner requests an adjustment to the collateral on a live credit line:
    /// TopUp (add to the lot), Substitution (swap the lot), or PartialRelease (return
    /// some). The owner signs and must be the line's pledgor; the line must be
    /// Active. This is the owner's leg of a three-party clearing: the custodian
    /// must then confirm it can hold and block the proposed set, and the bank
    /// must approve if coverage holds. The agreement stays fixed; this proposes
    /// a change to the collateral schedule only. Records the adjustment in
    /// Requested status.
    pub fn request_collateral_adjustment(
        env: Env,
        adjustment_id: BytesN<32>,
        credit_line_id: BytesN<32>,
        owner: Address,
        adjustment_type: AdjustmentType,
        new_evidence: LotEvidence,
        new_quantity_e7: i128,
        request_hash: BytesN<32>,
    ) -> Result<(), Error> {
        owner.require_auth();
        // Destructure the proposed new lot evidence into its commitments.
        let LotEvidence {
            manifest_hash: new_manifest_hash,
            uniqueness_hash: new_uniqueness_hash,
            quality_cert_hash: new_quality_cert_hash,
            quantity_cert_hash: new_quantity_cert_hash,
            location_hash: new_location_hash,
        } = new_evidence;

        if env.storage().persistent().has(&DataKey::Adjustment(adjustment_id.clone())) {
            return Err(Error::AdjustmentExists);
        }
        let line = Self::load_line(&env, &credit_line_id)?;
        if line.status != LineStatus::Active {
            return Err(Error::LineNotActive);
        }
        let pledge = Self::load_pledge(&env, &line.pledge_id)?;
        if pledge.pledgor != owner {
            return Err(Error::NotAuthorized);
        }
        if new_quantity_e7 <= 0 {
            return Err(Error::AmountNotPositive);
        }
        if Self::is_zero_hash(&env, &new_manifest_hash)
            || Self::is_zero_hash(&env, &new_uniqueness_hash)
            || Self::is_zero_hash(&env, &new_quality_cert_hash)
            || Self::is_zero_hash(&env, &new_quantity_cert_hash)
            || Self::is_zero_hash(&env, &new_location_hash)
            || Self::is_zero_hash(&env, &request_hash)
        {
            return Err(Error::InvalidDocumentHash);
        }

        let adjustment = CollateralAdjustment {
            credit_line_id: credit_line_id.clone(),
            adjustment_type,
            new_manifest_hash,
            new_uniqueness_hash,
            new_quality_cert_hash,
            new_quantity_cert_hash,
            new_location_hash,
            new_quantity_e7,
            request_hash: request_hash.clone(),
            status: AdjustmentStatus::Requested,
        };
        let key = DataKey::Adjustment(adjustment_id.clone());
        env.storage().persistent().set(&key, &adjustment);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        // context map: adjustment_id -> facility context, so the custodian and
        // bank clearing legs resolve framework in one read.
        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let adj_ctx_key = DataKey::ContextForAdjustment(adjustment_id.clone());
        let adj_ctx = FacilityContext {
            framework_id: framework_id.clone(),
            position_id: position_id.clone(),
            pledge_id: pledge_id.clone(),
        };
        env.storage().persistent().set(&adj_ctx_key, &adj_ctx);
        env.storage()
            .persistent()
            .extend_ttl(&adj_ctx_key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        env.events()
            .publish((symbol_short!("adjust"), symbol_short!("requestd")), adjustment_id.clone());

        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Adjustment,
            CollateralAction::AdjustmentRequested,
            owner,
            Role::Owner,
            position_id,
            pledge_id,
            credit_line_id,
            adjustment_id,
            StateLabel::Null,
            StateLabel::AdjustmentRequested,
            request_hash.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::Hash(HashData { hash: request_hash }),
        );
        Ok(())
    }

    /// Read a collateral-adjustment request.
    pub fn get_adjustment(
        env: Env,
        adjustment_id: BytesN<32>,
    ) -> Result<CollateralAdjustment, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Adjustment(adjustment_id))
            .ok_or(Error::LineNotFound)
    }

    /// Custodian leg of the adjustment clearing: the custodian confirms it can
    /// hold and block the proposed new lot. The custodian signs and must be
    /// the position's custodian (found via the line's pledge). The adjustment
    /// must be in Requested status. Moves Requested -> CustodianConfirmed. The
    /// bank can then approve if coverage holds.
    pub fn custodian_confirm_adjustment(
        env: Env,
        adjustment_id: BytesN<32>,
        custodian: Address,
        custody_evidence_hash: BytesN<32>,
    ) -> Result<(), Error> {
        custodian.require_auth();
        if Self::is_zero_hash(&env, &custody_evidence_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        let mut adj = Self::load_adjustment(&env, &adjustment_id)?;
        if adj.status != AdjustmentStatus::Requested {
            return Err(Error::AdjustmentWrongStatus);
        }
        let line = Self::load_line(&env, &adj.credit_line_id)?;
        let pledge = Self::load_pledge(&env, &line.pledge_id)?;
        let pos = Self::load_position(&env, &pledge.position_id)?;
        if pos.custodian != custodian {
            return Err(Error::NotAuthorized);
        }
        if !Self::is_approved(env.clone(), custodian.clone(), Role::Custodian) {
            return Err(Error::PartyNotApproved);
        }

        adj.status = AdjustmentStatus::CustodianConfirmed;
        Self::save_adjustment(&env, &adjustment_id, &adj);
        env.events()
            .publish((symbol_short!("adjust"), symbol_short!("custconf")), adjustment_id.clone());

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &adj.credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Adjustment,
            CollateralAction::AdjustmentCustodianConfirmed,
            custodian,
            Role::Custodian,
            position_id,
            pledge_id,
            adj.credit_line_id.clone(),
            adjustment_id,
            StateLabel::AdjustmentRequested,
            StateLabel::AdjustmentCustodianConfirmed,
            custody_evidence_hash.clone(),
            zero.clone(),
            zero.clone(),
            CollateralPayloadV1::Hash(HashData { hash: custody_evidence_hash }),
        );
        Ok(())
    }

    /// Bank leg of the adjustment clearing: the bank approves only if the line
    /// stays covered at the proposed new collateral. The bank signs and must be
    /// the line's bank. The adjustment must be CustodianConfirmed. Coverage is
    /// tested at the advance rate: the proposed new collateral's lendable value
    /// (new_weight * price * ltv_bps) must still cover the drawn balance. This
    /// is the real lendable-collateral-value rule (collateral must cover 100% of
    /// advances when discounted to its advance value). On approval the position's
    /// schedule (manifest and quantity) is updated. `price_per_unit_e7` is the bank's
    /// supplied current price, scaled 1e7. Moves CustodianConfirmed -> Approved.
    pub fn bank_approve_adjustment(
        env: Env,
        adjustment_id: BytesN<32>,
        bank: Address,
        price_per_unit_e7: i128,
        valuation_ref: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        // Approving a collateral substitution re-prices the borrowing base, so it
        // must point at the valuation that justifies the new price. valuation_ref
        // commits to that off-chain record, keeping the re-priced base auditable.
        if Self::is_zero_hash(&env, &valuation_ref) {
            return Err(Error::InvalidDocumentHash);
        }
        if price_per_unit_e7 <= 0 {
            return Err(Error::PriceNotPositive);
        }
        let mut adj = Self::load_adjustment(&env, &adjustment_id)?;
        if adj.status != AdjustmentStatus::CustodianConfirmed {
            return Err(Error::AdjustmentWrongStatus);
        }
        let mut line = Self::load_line(&env, &adj.credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }

        // Approved: update the position's collateral schedule to the new set.
        let pledge = Self::load_pledge(&env, &line.pledge_id)?;
        let mut pos = Self::load_position(&env, &pledge.position_id)?;

        // Re-price the borrowing base under the instrument's eligibility
        // treatment (same haircut and instrument as at line opening; an
        // adjustment cannot change the instrument, only the lot). The proposed
        // new collateral's lendable value must still cover the drawn balance.
        let eligibility =
            Self::load_active_eligibility(&env, &pos.framework_id, &pos.instrument)?;
        let new_base = Self::borrowing_base(
            adj.new_quantity_e7,
            price_per_unit_e7,
            eligibility.haircut_bps,
            line.ltv_bps,
        )?;
        if new_base < line.drawn_balance {
            return Err(Error::AdjustmentUndercovered);
        }

        // Maintain the lot uniqueness lock in lockstep with the collateral
        // identity. If the lot identity changes (substitution / top-up / partial
        // release), the new lot must not already be active under another
        // position, the old lock must be released, and the new lock taken. Without
        // this, an adjustment could swap in a lot pledged elsewhere, or
        // leave the old lot phantom-locked forever.
        if adj.new_uniqueness_hash != pos.uniqueness_hash {
            let new_bar_key = DataKey::LotLock(adj.new_uniqueness_hash.clone());
            if env.storage().persistent().has(&new_bar_key) {
                return Err(Error::LotAlreadyActive);
            }
            env.storage()
                .persistent()
                .remove(&DataKey::LotLock(pos.uniqueness_hash.clone()));
            env.storage().persistent().set(&new_bar_key, &pledge.position_id);
            env.storage()
                .persistent()
                .extend_ttl(&new_bar_key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
            pos.uniqueness_hash = adj.new_uniqueness_hash.clone();
        }

        pos.manifest_hash = adj.new_manifest_hash.clone();
        pos.quality_cert_hash = adj.new_quality_cert_hash.clone();
        pos.quantity_cert_hash = adj.new_quantity_cert_hash.clone();
        pos.location_hash = adj.new_location_hash.clone();
        pos.quantity_e7 = adj.new_quantity_e7;
        Self::save_position(&env, &pledge.position_id, &pos);

        // Recalculate spendable capacity against the substituted collateral. The
        // effective limit is the lower of the bank's approved ceiling and what
        // the new collateral can back (a top-up beyond the approved limit does
        // not auto-raise it; a partial release that lowers the base must reduce
        // available capacity in lockstep, never leaving stale headroom against
        // collateral that is gone). Floored at zero; the coverage test above
        // already guarantees new_base >= drawn_balance.
        let effective_limit = if new_base < line.approved_limit {
            new_base
        } else {
            line.approved_limit
        };
        let recalculated = effective_limit - line.drawn_balance;
        line.available_limit = if recalculated > 0 { recalculated } else { 0 };
        Self::save_line(&env, &adj.credit_line_id, &line);

        adj.status = AdjustmentStatus::Approved;
        Self::save_adjustment(&env, &adjustment_id, &adj);
        env.events()
            .publish((symbol_short!("adjust"), symbol_short!("approved")), adjustment_id.clone());

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &adj.credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Adjustment,
            CollateralAction::AdjustmentApproved,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            adj.credit_line_id.clone(),
            adjustment_id,
            StateLabel::AdjustmentCustodianConfirmed,
            StateLabel::AdjustmentApproved,
            adj.new_manifest_hash.clone(),
            zero.clone(),
            valuation_ref,
            CollateralPayloadV1::AdjustmentApproved(AdjustmentApprovedData {
                adjustment_type: adj.adjustment_type.clone(),
                new_manifest_hash: adj.new_manifest_hash.clone(),
                new_uniqueness_hash: adj.new_uniqueness_hash.clone(),
                new_quality_cert_hash: adj.new_quality_cert_hash.clone(),
                new_quantity_cert_hash: adj.new_quantity_cert_hash.clone(),
                new_location_hash: adj.new_location_hash.clone(),
                new_quantity_e7: adj.new_quantity_e7,
                price_per_unit_e7,
            }),
        );
        Ok(())
    }

    /// Stage one of release: the bank authorizes release of its security
    /// interest (the payoff-letter act, prong i). Valid only when the drawn
    /// balance is zero. The bank signs and must be the line's bank. This
    /// releases the bank's claim and closes the credit line, but the lot remains
    /// in the custodian's possessory block, so perfection is not yet terminated.
    /// Position Pledged -> ReleasePending; pledge Active -> ReleaseAuthorized;
    /// line -> Closed. The lien persists until the custodian confirms.
    pub fn bank_authorize_release(
        env: Env,
        credit_line_id: BytesN<32>,
        bank: Address,
        payoff_letter_hash: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if Self::is_zero_hash(&env, &payoff_letter_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if line.drawn_balance != 0 {
            return Err(Error::OutstandingBalance);
        }
        let mut pledge = Self::load_pledge(&env, &line.pledge_id)?;
        if pledge.status != PledgeStatus::Active {
            return Err(Error::PledgeNotActive);
        }
        let mut pos = Self::load_position(&env, &pledge.position_id)?;

        pos.status = PositionStatus::ReleasePending;
        pledge.status = PledgeStatus::ReleaseAuthorized;
        line.status = LineStatus::Closed;

        Self::save_position(&env, &pledge.position_id, &pos);
        Self::save_pledge(&env, &line.pledge_id, &pledge);
        Self::save_line(&env, &credit_line_id, &line);
        env.events()
            .publish((symbol_short!("release"), symbol_short!("authd")), credit_line_id.clone());

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Release,
            CollateralAction::ReleaseAuthorized,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::PledgeActive,
            StateLabel::PledgeReleaseAuthorized,
            payoff_letter_hash.clone(),
            zero.clone(),
            zero.clone(),
            CollateralPayloadV1::Hash(HashData { hash: payoff_letter_hash }),
        );
        Ok(())
    }

    /// Stage two of release: the custodian confirms it has lifted the book block
    /// and returned possession (termination of perfection, prong ii). This is
    /// what terminates a possessory security interest. The custodian signs and
    /// must be the position's custodian; the position must be ReleasePending and
    /// the pledge must reference this position. Position ReleasePending ->
    /// Released; pledge ReleaseAuthorized -> Released. Clear title is restored.
    pub fn custodian_confirm_release(
        env: Env,
        pledge_id: BytesN<32>,
        custodian: Address,
        release_notice_hash: BytesN<32>,
    ) -> Result<(), Error> {
        custodian.require_auth();
        if !Self::is_approved(env.clone(), custodian.clone(), Role::Custodian) {
            return Err(Error::PartyNotApproved);
        }
        let mut pledge = Self::load_pledge(&env, &pledge_id)?;
        if pledge.status != PledgeStatus::ReleaseAuthorized {
            return Err(Error::PledgeNotActive);
        }
        let mut pos = Self::load_position(&env, &pledge.position_id)?;
        if pos.custodian != custodian {
            return Err(Error::NotAuthorized);
        }
        if pos.status != PositionStatus::ReleasePending {
            return Err(Error::PositionNotReleasePending);
        }
        if Self::is_zero_hash(&env, &release_notice_hash) {
            return Err(Error::InvalidDocumentHash);
        }

        pos.status = PositionStatus::Released;
        pledge.status = PledgeStatus::Released;
        // The lot is returned to clean title: free the lot lock so they
        // can be registered again in future.
        env.storage().persistent().remove(&DataKey::LotLock(pos.uniqueness_hash.clone()));
        Self::save_position(&env, &pledge.position_id, &pos);
        Self::save_pledge(&env, &pledge_id, &pledge);

        // The release_notice_hash records the custodian's return notice.
        env.events().publish(
            (symbol_short!("release"), symbol_short!("confirmd")),
            (pledge_id.clone(), release_notice_hash.clone()),
        );

        // Resolve framework via the pledge context map, falling back to the
        // position record already loaded above.
        let framework_id = env
            .storage()
            .persistent()
            .get::<DataKey, FacilityContext>(&DataKey::ContextForPledge(pledge_id.clone()))
            .map(|c| c.framework_id)
            .unwrap_or_else(|| pos.framework_id.clone());
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Release,
            CollateralAction::ReleaseConfirmed,
            custodian,
            Role::Custodian,
            pledge.position_id.clone(),
            pledge_id,
            zero.clone(),
            zero.clone(),
            StateLabel::PledgeReleaseAuthorized,
            StateLabel::PledgeReleased,
            release_notice_hash.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::Hash(HashData { hash: release_notice_hash }),
        );
        Ok(())
    }

    // ---- lifecycle: default & enforcement (records; does not bypass law) --

    /// Bank issues a default notice and sets the cure deadline. The deadline
    /// must be in the future: a default notice exists to grant the borrower a
    /// real cure window, so a deadline at or before the current ledger is a
    /// malformed notice and is refused. A bank that wants to proceed without a
    /// cure period does not issue a zero-window notice; it lets the existing
    /// window run and then enforces.
    pub fn issue_default_notice(
        env: Env,
        credit_line_id: BytesN<32>,
        bank: Address,
        cure_deadline_ledger: u32,
        notice_hash: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if Self::is_zero_hash(&env, &notice_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        if cure_deadline_ledger <= env.ledger().sequence() {
            return Err(Error::CureDeadlineNotFuture);
        }
        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        // Only a live facility can be defaulted. A closed or already-defaulted
        // line is not defaultable; defaulting it again would be a meaningless or
        // misleading state transition.
        if line.status != LineStatus::Active && line.status != LineStatus::Suspended {
            return Err(Error::LineNotActive);
        }
        let mut pledge = Self::load_pledge(&env, &line.pledge_id)?;
        // The pledge must still be active (not released/already-defaulted) for the
        // collateral to be enforceable under this default.
        if pledge.status != PledgeStatus::Active {
            return Err(Error::PledgeNotActive);
        }
        let prev_line_status = line.status;

        line.status = LineStatus::Defaulted;
        line.cure_expiry_ledger = cure_deadline_ledger;
        pledge.status = PledgeStatus::Defaulted;

        Self::save_line(&env, &credit_line_id, &line);
        Self::save_pledge(&env, &line.pledge_id, &pledge);
        env.events()
            .publish((symbol_short!("default"), symbol_short!("notice")), credit_line_id.clone());

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Default,
            CollateralAction::DefaultNoticeIssued,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::from_line(prev_line_status),
            StateLabel::LineDefaulted,
            notice_hash.clone(),
            zero.clone(),
            zero.clone(),
            CollateralPayloadV1::DefaultNotice(DefaultNoticeData {
                cure_deadline_ledger,
                notice_hash,
            }),
        );
        Ok(())
    }

    /// Cardholder cures a default, restoring the line. Deliberately lenient on
    /// timing: a cure is accepted as long as the line is still Defaulted, even
    /// after the cure deadline has passed, right up until the bank records
    /// enforcement (which closes the line and makes a cure impossible via the
    /// status check below). This mirrors real secured-lending practice, where a
    /// default may be acted on only while it "is continuing": the deadline
    /// unlocks the bank's right to enforce, but the borrower can still cure by
    /// paying until the bank actually enforces, because enforcing collateral is
    /// the costly last resort and repayment is the preferred outcome. The
    /// enforcement gate lives in record_enforcement (cure period must have
    /// expired); this side stays open until enforcement lands.
    pub fn cure_default(
        env: Env,
        credit_line_id: BytesN<32>,
        cardholder: Address,
        bank: Address,
        cure_evidence_hash: BytesN<32>,
    ) -> Result<(), Error> {
        cardholder.require_auth();
        bank.require_auth();
        if Self::is_zero_hash(&env, &cure_evidence_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.cardholder != cardholder {
            return Err(Error::NotAuthorized);
        }
        // The bank that issued the default must countersign the cure. A default
        // is the bank's declared act; the borrower cannot reverse it alone by
        // submitting an evidence hash. Both the remedying party (cardholder) and
        // the declaring party (bank) must authorize for the cure to take effect.
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if line.status != LineStatus::Defaulted {
            return Err(Error::NotDefaulted);
        }
        let mut pledge = Self::load_pledge(&env, &line.pledge_id)?;

        line.status = LineStatus::Active;
        line.cure_expiry_ledger = 0;
        pledge.status = PledgeStatus::Active;

        Self::save_line(&env, &credit_line_id, &line);
        Self::save_pledge(&env, &line.pledge_id, &pledge);
        env.events()
            .publish((symbol_short!("default"), symbol_short!("cured")), credit_line_id.clone());

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Default,
            CollateralAction::DefaultCured,
            cardholder,
            Role::Owner,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::LineDefaulted,
            StateLabel::LineActive,
            cure_evidence_hash.clone(),
            zero.clone(),
            zero.clone(),
            CollateralPayloadV1::Hash(HashData { hash: cure_evidence_hash }),
        );
        Ok(())
    }

    /// Bank deliberately stops the facility for a reason other than margin
    /// (fraud, KYC, sanctions, documentation breach, internal credit stop). The
    /// bank signs and must be the line's bank. Sets the manual-suspension flag
    /// and suspends the line. A revaluation cannot clear this; only the bank can
    /// resume. `reason_hash` anchors the off-chain suspension notice. Valid on an
    /// Active or (margin-)Suspended line, not a Closed or Defaulted one.
    pub fn bank_suspend_line(
        env: Env,
        credit_line_id: BytesN<32>,
        bank: Address,
        reason_hash: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if Self::is_zero_hash(&env, &reason_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if line.status == LineStatus::Closed || line.status == LineStatus::Defaulted {
            return Err(Error::LineNotActive);
        }
        let prev_status = line.status;
        line.manual_bank_suspension = true;
        line.status = LineStatus::Suspended;
        line.available_limit = 0;
        let drawn_after = line.drawn_balance;
        Self::save_line(&env, &credit_line_id, &line);
        env.events().publish(
            (symbol_short!("line"), symbol_short!("bksuspnd")),
            (credit_line_id.clone(), reason_hash.clone()),
        );

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Line,
            CollateralAction::LineSuspendedByBank,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::from_line(prev_status),
            StateLabel::LineSuspended,
            reason_hash,
            zero.clone(),
            zero,
            CollateralPayloadV1::BalanceMove(BalanceMoveData {
                amount: 0,
                drawn_after,
                available_after: 0,
            }),
        );
        Ok(())
    }

    /// Bank lifts its own deliberate stop. The bank signs and must be the line's
    /// bank; the line must be under a bank suspension. Clears the flag and
    /// restores the line to Active. Any margin condition is re-evaluated on the
    /// next revaluation; resuming is a deliberate bank act.
    pub fn bank_resume_line(
        env: Env,
        credit_line_id: BytesN<32>,
        bank: Address,
        resume_evidence_hash: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if Self::is_zero_hash(&env, &resume_evidence_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if !line.manual_bank_suspension {
            return Err(Error::LineNotActive);
        }
        let prev_status = line.status;
        line.manual_bank_suspension = false;
        // Restore capacity. Suspension zeroed available_limit; resume must put it
        // back, or the line is Active-but-unusable until the next revaluation.
        // Use the latest stored valuation's advance base (held in borrowing_base),
        // capped at the approved limit, less drawn. Fall back to the approved
        // limit if no valuation has been recorded yet.
        let cover = match Self::get_valuation(env.clone(), credit_line_id.clone()) {
            Ok(v) => {
                if v.borrowing_base < line.approved_limit {
                    v.borrowing_base
                } else {
                    line.approved_limit
                }
            }
            Err(_) => line.approved_limit,
        };
        line.available_limit = (cover - line.drawn_balance).max(0);
        line.status = LineStatus::Active;
        let available_after = line.available_limit;
        let drawn_after = line.drawn_balance;
        Self::save_line(&env, &credit_line_id, &line);
        env.events()
            .publish((symbol_short!("line"), symbol_short!("bkresume")), credit_line_id.clone());

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Line,
            CollateralAction::LineResumedByBank,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::from_line(prev_status),
            StateLabel::LineActive,
            resume_evidence_hash,
            zero.clone(),
            zero,
            CollateralPayloadV1::BalanceMove(BalanceMoveData {
                amount: 0,
                drawn_after,
                available_after,
            }),
        );
        Ok(())
    }

    /// Revalue a line against a fresh price and check it against the
    /// borrowing base. Records the result as a LineValuation side-record and
    /// moves the line's margin state across two bands:
    ///
    ///   Covered  -> drawn balance well within the borrowing base
    ///   Warning  -> drawn balance crossed the warning band (buffer shrinking)
    ///   Called   -> drawn balance crossed the action band (margin call; the
    ///               line is suspended so no further draws are allowed until the
    ///               borrower cures by repayment or additional collateral)
    ///
    /// The price comes from an off-chain price feed (a metals or commodity oracle in the
    /// current design) submitted by the valuation role (or the bank). The
    /// contract refuses to act on a price that is stale or whose confidence band
    /// is too wide, so a bad price cannot trigger or mask a margin call. Signed
    /// by the valuation role or the bank.
    ///
    /// `price_per_unit_e7` and `confidence_e7` are scaled by 1e7. `priced_at` is
    /// the source publish time (unix seconds). `max_age_secs` is the freshness
    /// window. `conf_tol_bps` is the maximum allowed confidence as a fraction of
    /// price, in basis points (e.g. 50 = confidence may be at most 0.5% of
    /// price). `warning_bps` is the fraction of the maintenance action band at
    /// which the warning band begins (e.g. 9000 = warn at 90% of the call
    /// threshold). The action band itself is the stored maintenance_bps fraction
    /// of current raw collateral value.
    pub fn revalue_and_check(
        env: Env,
        credit_line_id: BytesN<32>,
        valuer: Address,
        price_per_unit_e7: i128,
        confidence_e7: i128,
        priced_at: u64,
        max_age_secs: u64,
        conf_tol_bps: u32,
        warning_bps: u32,
        valuation_ref: BytesN<32>,
    ) -> Result<(), Error> {
        valuer.require_auth();
        // A revaluation is a valuation-bearing act: it must reference the source
        // it acted on (feed / attestation / oracle reference). No valuation
        // source, no valuation state transition. This keeps valuation_ref a real
        // event dimension rather than a decorative zero.
        if Self::is_zero_hash(&env, &valuation_ref) {
            return Err(Error::InvalidDocumentHash);
        }
        if max_age_secs == 0
            || conf_tol_bps == 0
            || conf_tol_bps > 10_000
            || warning_bps == 0
            || warning_bps > 10_000
        {
            return Err(Error::InvalidRevaluationParams);
        }
        // Either the valuation role or the line's own bank may submit a price.
        let is_valuer = Self::is_approved(env.clone(), valuer.clone(), Role::Valuation);

        let mut line = Self::load_line(&env, &credit_line_id)?;
        // Either an approved Valuation party, or the line's bank while still an
        // approved Bank. A revoked bank can no longer revalue its own facility.
        let is_line_bank = line.bank == valuer
            && Self::is_approved(env.clone(), valuer.clone(), Role::Bank);
        if !is_valuer && !is_line_bank {
            return Err(Error::NotAuthorized);
        }
        // The event must name the authority the actor acted under, because
        // actor + role is the reconciliation surface against the off-chain
        // approval. If the actor is this line's own bank, it acts as Bank even
        // if it also happens to hold a Valuation grant; a separate approved
        // valuer acts as Valuation. Emitting Role::Valuation unconditionally
        // would misstate the authority when the bank revalues.
        let actor_role = if is_line_bank {
            Role::Bank
        } else {
            Role::Valuation
        };
        // Only meaningful on a live or already-called line, not a closed one.
        if line.status == LineStatus::Closed || line.status == LineStatus::Defaulted {
            return Err(Error::LineNotActive);
        }
        if price_per_unit_e7 <= 0 || confidence_e7 < 0 {
            return Err(Error::PriceNotPositive);
        }

        // Freshness: the price must not be older than the allowed window.
        let now = env.ledger().timestamp();
        if priced_at > now {
            return Err(Error::PriceFromFuture);
        }
        if now > priced_at && now - priced_at > max_age_secs {
            return Err(Error::PriceStale);
        }

        // Confidence: the band (half-width) must be within tolerance of price.
        // confidence_e7 <= price_per_unit_e7 * conf_tol_bps / 1e4
        let max_conf = price_per_unit_e7
            .checked_mul(conf_tol_bps as i128)
            .ok_or(Error::InvalidRiskParams)?
            / 10_000i128;
        if confidence_e7 > max_conf {
            return Err(Error::PriceConfidenceTooWide);
        }

        // Compute the raw collateral value at the fresh price (no LTV), and the
        // advance base (LTV-adjusted) for the available-limit calculation.
        let pledge = Self::load_pledge(&env, &line.pledge_id)?;
        let pos = Self::load_position(&env, &pledge.position_id)?;
        // The instrument's eligibility haircut is applied continuously, not only
        // at origination. Real collateralized lending marks the haircut-discounted
        // value daily, and the maintenance test fires against that discounted
        // value. So raw_value (the maintenance denominator) is taken at full
        // advance but after the haircut, and advance_base matches the origination
        // formula exactly (haircut then LTV).
        let eligibility =
            Self::load_active_eligibility(&env, &pos.framework_id, &pos.instrument)?;
        let raw_value = Self::borrowing_base(
            pos.quantity_e7,
            price_per_unit_e7,
            eligibility.haircut_bps,
            10_000,
        )?;
        let advance_base = Self::borrowing_base(
            pos.quantity_e7,
            price_per_unit_e7,
            eligibility.haircut_bps,
            line.ltv_bps,
        )?;

        // Two-threshold check, Schwab-style, against RAW collateral value.
        // The margin call fires when the drawn balance exceeds the maintenance
        // fraction of current collateral value:
        //   action band  = raw_value * maintenance_bps / 1e4
        //   warning band = action band * warning_bps / 1e4 (sits below it)
        let action_band = raw_value
            .checked_mul(line.maintenance_bps as i128)
            .ok_or(Error::InvalidRiskParams)?
            / 10_000i128;
        let warning_band = action_band
            .checked_mul(warning_bps as i128)
            .ok_or(Error::InvalidRiskParams)?
            / 10_000i128;
        let margin_state = if line.drawn_balance > action_band {
            MarginState::Called
        } else if line.drawn_balance > warning_band {
            MarginState::Warning
        } else {
            MarginState::Covered
        };

        // A margin call suspends the line (blocks new draws). Recovery from a
        // prior MARGIN suspension restores it to Active. But a bank suspension
        // is a separate dimension: a revaluation must never clear a deliberate
        // bank stop. Available limit follows the lesser of the approved limit
        // and the current advance base, less drawn.
        let prev_line_status = line.status;
        let cover = if advance_base < line.approved_limit { advance_base } else { line.approved_limit };
        line.available_limit = (cover - line.drawn_balance).max(0);
        line.status = if line.manual_bank_suspension {
            // bank stop holds regardless of margin recovery
            LineStatus::Suspended
        } else if margin_state == MarginState::Called {
            LineStatus::Suspended
        } else {
            LineStatus::Active
        };
        Self::save_line(&env, &credit_line_id, &line);

        let valuation = LineValuation {
            price_per_unit_e7,
            confidence_e7,
            priced_at,
            valued_at_ledger: env.ledger().sequence(),
            borrowing_base: advance_base,
            margin_state,
        };
        let key = DataKey::Valuation(credit_line_id.clone());
        env.storage().persistent().set(&key, &valuation);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        let topic = match margin_state {
            MarginState::Called => symbol_short!("called"),
            MarginState::Warning => symbol_short!("warning"),
            MarginState::Covered => symbol_short!("covered"),
        };
        env.events()
            .publish((symbol_short!("margin"), topic), credit_line_id.clone());

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Valuation,
            CollateralAction::LineRevalued,
            valuer,
            actor_role,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::from_line(prev_line_status),
            StateLabel::from_line(line.status),
            valuation_ref.clone(),
            zero.clone(),
            valuation_ref.clone(),
            CollateralPayloadV1::Revalued(RevaluedData {
                price_per_unit_e7,
                confidence_e7,
                margin_state,
                drawn_balance: line.drawn_balance,
                advance_base,
                available_after: line.available_limit,
            }),
        );
        Ok(())
    }

    /// Read the latest revaluation record for a line.
    pub fn get_valuation(
        env: Env,
        credit_line_id: BytesN<32>,
    ) -> Result<LineValuation, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Valuation(credit_line_id))
            .ok_or(Error::LineNotFound)
    }

    /// Record the enforcement outcome after an uncured default. Bank signs.
    /// Refuses before the cure period expires, if not defaulted, or if already
    /// enforced.
    ///
    /// This records WHICH lawful enforcement path was taken under the off-chain
    /// security and control documents (sale, appropriation, or transfer) and
    /// anchors a hash of that legal instrument. It does NOT decree ownership and
    /// does NOT move the physical collateral. Default does not automatically vest
    /// title in the bank; enforcement happens under the governing law, proceeds
    /// are applied to the debt, and any surplus returns to the owner. The chain
    /// records the outcome so the trail is undisputed.
    pub fn record_enforcement(
        env: Env,
        credit_line_id: BytesN<32>,
        bank: Address,
        outcome: EnforcementOutcome,
        legal_instrument_hash: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        let mut line = Self::load_line(&env, &credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if line.status != LineStatus::Defaulted {
            return Err(Error::NotDefaulted);
        }
        if env.ledger().sequence() < line.cure_expiry_ledger {
            return Err(Error::CurePeriodNotExpired);
        }
        if Self::is_zero_hash(&env, &legal_instrument_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        let mut pledge = Self::load_pledge(&env, &line.pledge_id)?;
        if pledge.status == PledgeStatus::Enforced {
            return Err(Error::AlreadyEnforced);
        }
        let mut pos = Self::load_position(&env, &pledge.position_id)?;

        pledge.status = PledgeStatus::Enforced;
        pos.status = PositionStatus::Enforced;
        line.status = LineStatus::Closed;
        // Enforcement is terminal for this lot under this position: free the
        // lot lock.
        env.storage().persistent().remove(&DataKey::LotLock(pos.uniqueness_hash.clone()));

        Self::save_pledge(&env, &line.pledge_id, &pledge);
        Self::save_position(&env, &pledge.position_id, &pos);
        Self::save_line(&env, &credit_line_id, &line);
        env.events().publish(
            (symbol_short!("enforce"), symbol_short!("recorded")),
            (credit_line_id.clone(), outcome, legal_instrument_hash.clone()),
        );

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Enforcement,
            CollateralAction::EnforcementRecorded,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::PledgeDefaulted,
            StateLabel::PledgeEnforced,
            legal_instrument_hash.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::Enforcement(EnforcementData {
                outcome,
                legal_instrument_hash,
            }),
        );
        Ok(())
    }

    // ---- enforcement readiness (gates the certificate; asserts no falsehood) -
    //
    // These functions do NOT change the default / enforcement lifecycle above.
    // They maintain a separate, honest record of whether the facility's
    // realization PATH has actually been agreed with real parties. An
    // Enforcement Readiness Certificate may render as "ready" ONLY when this
    // record is Ready; otherwise it is DRAFT. The contract is the source of
    // truth, so the certificate cannot assert a realization path that does not
    // exist on chain.

    /// Open an (Incomplete) enforcement-readiness record for a line. Bank signs.
    /// Starts deliberately empty: no agent, no route, no settlement asset. This
    /// is the honest default state and yields a DRAFT certificate.
    pub fn open_enforcement_readiness(
        env: Env,
        credit_line_id: BytesN<32>,
        bank: Address,
        readiness_evidence_hash: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        let line = Self::load_line(&env, &credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if Self::is_zero_hash(&env, &readiness_evidence_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        if env.storage().persistent().has(&DataKey::Readiness(credit_line_id.clone())) {
            return Err(Error::ReadinessWrongStatus);
        }
        let zero = BytesN::from_array(&env, &[0u8; 32]);
        let readiness = EnforcementReadiness {
            credit_line_id: credit_line_id.clone(),
            status: ReadinessStatus::Incomplete,
            liquidation_agent: bank.clone(), // placeholder owner; not a real agent
            realization_route_hash: zero.clone(),
            settlement_asset: bank.clone(),
            valuation_source_hash: zero.clone(),
            waterfall_hash: zero,
            valid_until_ledger: 0,
            version: 0,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Readiness(credit_line_id.clone()), &readiness);
        env.events().publish(
            (symbol_short!("readines"), symbol_short!("opened")),
            credit_line_id.clone(),
        );

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Readiness,
            CollateralAction::ReadinessOpened,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::Null,
            StateLabel::ReadinessIncomplete,
            readiness_evidence_hash.clone(),
            zero.clone(),
            zero.clone(),
            CollateralPayloadV1::Hash(HashData { hash: readiness_evidence_hash }),
        );
        Ok(())
    }

    /// Populate the partner-dependent realization fields. Bank signs. The record
    /// becomes `Ready` ONLY when the liquidation agent, realization-route hash
    /// and settlement asset are all genuinely set (non-self, non-zero). Until a
    /// real liquidation partner exists, the bank cannot truthfully reach Ready,
    /// which is the entire point.
    pub fn populate_enforcement_readiness(
        env: Env,
        credit_line_id: BytesN<32>,
        bank: Address,
        liquidation_agent: Address,
        realization_route_hash: BytesN<32>,
        settlement_asset: Address,
        valuation_source_hash: BytesN<32>,
        waterfall_hash: BytesN<32>,
        valid_until_ledger: u32,
    ) -> Result<(), Error> {
        bank.require_auth();
        let line = Self::load_line(&env, &credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        let mut readiness = Self::load_readiness(&env, &credit_line_id)?;

        let zero = BytesN::from_array(&env, &[0u8; 32]);
        // Required-for-Ready: a real agent (not the bank itself, not unset), a
        // realization route, and a settlement asset (not the bank itself).
        let agent_real = liquidation_agent != bank;
        let route_real = realization_route_hash != zero;
        let asset_real = settlement_asset != bank;
        let valuation_real = !Self::is_zero_hash(&env, &valuation_source_hash);
        let waterfall_real = !Self::is_zero_hash(&env, &waterfall_hash);
        let ready_fields_present = agent_real && route_real && asset_real;
        if ready_fields_present && (!valuation_real || !waterfall_real) {
            return Err(Error::InvalidDocumentHash);
        }
        if ready_fields_present && valid_until_ledger <= env.ledger().sequence() {
            return Err(Error::ReadinessExpired);
        }

        readiness.liquidation_agent = liquidation_agent;
        readiness.realization_route_hash = realization_route_hash;
        readiness.settlement_asset = settlement_asset;
        readiness.valuation_source_hash = valuation_source_hash;
        readiness.waterfall_hash = waterfall_hash;
        readiness.valid_until_ledger = valid_until_ledger;
        readiness.version += 1;
        readiness.status = if ready_fields_present && valuation_real && waterfall_real {
            ReadinessStatus::Ready
        } else {
            ReadinessStatus::Incomplete
        };

        let new_status = readiness.status;
        let route_for_event = readiness.realization_route_hash.clone();
        let readiness_payload = ReadinessPopulatedData {
            liquidation_agent: readiness.liquidation_agent.clone(),
            realization_route_hash: readiness.realization_route_hash.clone(),
            settlement_asset: readiness.settlement_asset.clone(),
            valuation_source_hash: readiness.valuation_source_hash.clone(),
            waterfall_hash: readiness.waterfall_hash.clone(),
            valid_until_ledger: readiness.valid_until_ledger,
            version: readiness.version,
            status: readiness.status,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Readiness(credit_line_id.clone()), &readiness);
        env.events().publish(
            (symbol_short!("readines"), symbol_short!("populate")),
            (credit_line_id.clone(), readiness.version, readiness.status),
        );

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Readiness,
            CollateralAction::ReadinessPopulated,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::ReadinessIncomplete,
            StateLabel::from_readiness(new_status),
            route_for_event.clone(),
            zero.clone(),
            zero,
            CollateralPayloadV1::ReadinessPopulated(readiness_payload),
        );
        Ok(())
    }

    /// Mark a readiness record Expired (e.g. its validity window passed, or the
    /// liquidation arrangement lapsed). Bank signs. An Expired record yields a
    /// DRAFT certificate again until re-populated.
    pub fn expire_enforcement_readiness(
        env: Env,
        credit_line_id: BytesN<32>,
        bank: Address,
        expiry_evidence_hash: BytesN<32>,
    ) -> Result<(), Error> {
        bank.require_auth();
        let line = Self::load_line(&env, &credit_line_id)?;
        if line.bank != bank {
            return Err(Error::NotAuthorized);
        }
        if !Self::is_approved(env.clone(), bank.clone(), Role::Bank) {
            return Err(Error::PartyNotApproved);
        }
        if Self::is_zero_hash(&env, &expiry_evidence_hash) {
            return Err(Error::InvalidDocumentHash);
        }
        let mut readiness = Self::load_readiness(&env, &credit_line_id)?;
        readiness.status = ReadinessStatus::Expired;
        env.storage()
            .persistent()
            .set(&DataKey::Readiness(credit_line_id.clone()), &readiness);
        env.events().publish(
            (symbol_short!("readines"), symbol_short!("expired")),
            credit_line_id.clone(),
        );

        let (framework_id, position_id, pledge_id) = Self::line_context(&env, &credit_line_id)?;
        let zero = Self::zero_hash(&env);
        Self::emit_event(
            &env,
            &framework_id,
            EntityKind::Readiness,
            CollateralAction::ReadinessExpired,
            bank,
            Role::Bank,
            position_id,
            pledge_id,
            credit_line_id,
            zero.clone(),
            StateLabel::ReadinessReady,
            StateLabel::ReadinessExpired,
            expiry_evidence_hash.clone(),
            zero.clone(),
            zero.clone(),
            CollateralPayloadV1::Hash(HashData { hash: expiry_evidence_hash }),
        );
        Ok(())
    }

    // ---- reads -----------------------------------------------------------

    /// Read the enforcement-readiness record. The certificate generator uses the
    /// `status` field to decide DRAFT vs ready.
    pub fn get_enforcement_readiness(
        env: Env,
        credit_line_id: BytesN<32>,
    ) -> Result<EnforcementReadiness, Error> {
        Self::load_readiness(&env, &credit_line_id)
    }

    pub fn available_capacity(env: Env, credit_line_id: BytesN<32>) -> Result<i128, Error> {
        Ok(Self::load_line(&env, &credit_line_id)?.available_limit)
    }

    pub fn get_line(env: Env, credit_line_id: BytesN<32>) -> Result<CreditLine, Error> {
        Self::load_line(&env, &credit_line_id)
    }

    pub fn get_position(env: Env, position_id: BytesN<32>) -> Result<VaultPosition, Error> {
        Self::load_position(&env, &position_id)
    }

    pub fn get_pledge(env: Env, pledge_id: BytesN<32>) -> Result<Pledge, Error> {
        Self::load_pledge(&env, &pledge_id)
    }

    // ---- internal helpers ------------------------------------------------

    /// borrowing_base = quantity * price_per_unit * (1 - haircut) * ltv
    /// inputs are scaled by 1e7 (weight, price); ltv is in bps.
    /// result is in the line's minor units (already-scaled).
    fn borrowing_base(
        quantity_e7: i128,
        price_per_unit_e7: i128,
        haircut_bps: u32,
        ltv_bps: u32,
    ) -> Result<i128, Error> {
        // Gross value, then haircut, then advance rate. The haircut is applied
        // before LTV, per ISDA practice (treatment discounts value before the
        // advance fraction is taken):
        //   (qty_e7/1e7) * (price_e7/1e7) * ((1e4 - haircut_bps)/1e4) * (ltv_bps/1e4)
        // = qty_e7 * price_e7 * (1e4 - haircut_bps) * ltv_bps / 1e22
        // Checked, not saturating: a value that overflows i128 is a malformed
        // risk input, not something to silently clamp to a misleading base.
        if haircut_bps > 10_000 {
            return Err(Error::InvalidRiskParams);
        }
        let value = quantity_e7
            .checked_mul(price_per_unit_e7)
            .and_then(|v| v.checked_mul((10_000 - haircut_bps) as i128))
            .and_then(|v| v.checked_mul(ltv_bps as i128))
            .ok_or(Error::InvalidRiskParams)?;
        Ok(value / 10_000_000_000_000_000_000_000i128)
    }

    fn admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    /// Only these roles are grantable through the approval registry. Owner is an
    /// event-semantics role (the self-signing pledgor) and Admin is the contract
    /// owner; neither is a registry-approvable counterparty, so an attempt to
    /// grant them is treated as an unauthorized action.
    fn assert_approvable_role(role: Role) -> Result<(), Error> {
        match role {
            Role::Bank
            | Role::Custodian
            | Role::Processor
            | Role::Valuation
            | Role::Vault => Ok(()),
            Role::Admin | Role::Owner => Err(Error::NotAuthorized),
        }
    }

    /// The single SettlementVault contract bound to this ledger. Repayments may
    /// only be applied by this exact address (see apply_repayment).
    fn settlement_vault(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::SettlementVault)
            .ok_or(Error::NotInitialized)
    }

    fn load_active_framework(
        env: &Env,
        id: &BytesN<32>,
    ) -> Result<ControlFramework, Error> {
        let fwk: ControlFramework = env
            .storage()
            .persistent()
            .get(&DataKey::Framework(id.clone()))
            .ok_or(Error::FrameworkNotActive)?;
        if fwk.status != FrameworkStatus::Active {
            return Err(Error::FrameworkNotActive);
        }
        Ok(fwk)
    }

    /// Load the active eligibility treatment for an instrument under a framework.
    /// This is the CDM collateral-criteria / treatment result: it both proves the
    /// instrument is admitted and carries the haircut / LTV ceiling the bank
    /// applies. A missing or retired record means the instrument is not eligible.
    fn load_active_eligibility(
        env: &Env,
        framework_id: &BytesN<32>,
        instrument: &InstrumentKey,
    ) -> Result<FrameworkInstrumentEligibility, Error> {
        let elig: FrameworkInstrumentEligibility = env
            .storage()
            .persistent()
            .get(&DataKey::FrameworkInstrument(
                framework_id.clone(),
                Self::instrument_fingerprint(env, instrument),
            ))
            .ok_or(Error::InstrumentNotEligible)?;
        if elig.status != EligibilityStatus::Active {
            return Err(Error::InstrumentNotEligible);
        }
        Ok(elig)
    }

    fn load_position(env: &Env, id: &BytesN<32>) -> Result<VaultPosition, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Position(id.clone()))
            .ok_or(Error::PositionNotFound)
    }
    fn save_position(env: &Env, id: &BytesN<32>, p: &VaultPosition) {
        let key = DataKey::Position(id.clone());
        env.storage().persistent().set(&key, p);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
    }

    fn load_pledge(env: &Env, id: &BytesN<32>) -> Result<Pledge, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Pledge(id.clone()))
            .ok_or(Error::PledgeNotFound)
    }
    fn save_pledge(env: &Env, id: &BytesN<32>, p: &Pledge) {
        let key = DataKey::Pledge(id.clone());
        env.storage().persistent().set(&key, p);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
    }

    fn load_line(env: &Env, id: &BytesN<32>) -> Result<CreditLine, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Line(id.clone()))
            .ok_or(Error::LineNotFound)
    }
    fn save_line(env: &Env, id: &BytesN<32>, l: &CreditLine) {
        let key = DataKey::Line(id.clone());
        env.storage().persistent().set(&key, l);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
    }

    fn load_adjustment(env: &Env, id: &BytesN<32>) -> Result<CollateralAdjustment, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Adjustment(id.clone()))
            .ok_or(Error::LineNotFound)
    }

    fn is_zero_hash(env: &Env, hash: &BytesN<32>) -> bool {
        let zero = BytesN::from_array(env, &[0u8; 32]);
        hash == &zero
    }

    fn load_readiness(env: &Env, id: &BytesN<32>) -> Result<EnforcementReadiness, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Readiness(id.clone()))
            .ok_or(Error::ReadinessNotFound)
    }
    fn save_adjustment(env: &Env, id: &BytesN<32>, a: &CollateralAdjustment) {
        let key = DataKey::Adjustment(id.clone());
        env.storage().persistent().set(&key, a);
        env.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
    }
}

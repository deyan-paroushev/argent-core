# Bank Integration and Adapter Strategy

> **Positioning status:** The integration target is a bank-operated reserve obligation facility. Credit, loan-servicing, and settlement adapters remain necessary because an obligation may crystallize into funded reimbursement exposure, but unrestricted cash draw is outside the target product.

**How Argent connects one controlled reserve to bank guarantees, documentary credits, treasury exposure, secured credit, custody, identity, signing, and evidence systems without replacing any of them.**

**Status:** integration strategy and design-partner framework; not all target obligation functions are implemented  
**Canonical product direction:** non-cash-drawable reserve obligation facility  
**Implemented reference:** secured-credit lifecycle on Soroban  
**Boundary:** Argent is a reserve-control sidecar and evidence layer. It does not replace trade-finance platforms, collateral and limits systems, loan servicing, treasury, custody, accounting, payments, or legal enforcement.  
**Companion documents:** `reserve-obligation-infrastructure.md`, `obligation-facility-profile.md`, `integration-and-interoperability.md`, `argent-dfns-signing-sequence.md`, `collateral-eligibility-and-risk-policy.md`, `evidence-pack-index.md`, `deployment-and-runbook.md`  
**Last updated:** 2026-07-18

*This is an integration and adoption note, not legal, banking, regulatory, outsourcing, cyber, or implementation advice. Product and vendor names map the normal institutional technology landscape. No partnership, certification, compatibility, or endorsement is implied. Production use requires institution-specific architecture, security, legal, vendor-risk, data-protection, resilience, accounting, and audit review.*

---

## 1. The one thing this document establishes

Banks already have product, credit, collateral, custody, treasury, accounting, and settlement infrastructure. Argent should not replace it.

The mature product is not a standalone lending system. It is a reserve-control layer beneath purpose-bound bank obligations:

```text
one controlled reserve
-> bank-approved master capacity
-> product and group sublimits
-> guarantees / documentary credits / undertakings / treasury exposure
-> claims, payment, reimbursement, discharge, release, or enforcement
```

The bank remains authoritative for:

- customer and group approval;
- facility and product limits;
- instrument issuance and amendment;
- documentary compliance and claim decisions;
- pricing, accounting, capital, liquidity, and reporting;
- reimbursement and legal enforcement decisions.

The custodian remains authoritative for physical possession, segregation, immobilization, movement, release, and realization.

Argent's institutional route is narrower:

> **Argent records and proves the shared reserve-capacity state that existing bank and custody systems depend on but do not hold in one place.**

The target product prohibits an unrestricted customer cash draw. Capacity is allocated only to a bank-approved product, purpose, amount, beneficiary, and term. A bank payment may still occur under an issued instrument, at which point the exposure becomes reimbursement due.

The current contracts prove the same foundation through a secured-credit reference branch. Loan-origination and servicing adapters remain relevant for that branch and for any crystallized reimbursement exposure, but the first production integration priority shifts toward:

1. limits and collateral management;
2. guarantees and trade-finance platforms;
3. treasury and margin systems;
4. custody and valuation;
5. institutional signing and approvals;
6. accounting, evidence, and reconciliation.

The safe adoption model remains staged:

```text
evidence-only mirror
-> shadow reserve-capacity ledger
-> governed reservation and release control
-> one typed bank obligation
-> broader product integration
```

No stage should require a bank to replace its system of record or accept protocol state as a substitute for legal or product authority.

---

## 2. The infrastructure banks already use

A bank's credit environment is not one system. It is a stack of systems, many of them old, heavily governed, deeply integrated, and not available for direct modification in a pilot. Argent must assume coexistence with that stack from day one.

### 2.1 Loan origination and credit workflow

Loan origination and credit workflow systems manage the application, borrower information, credit analysis, underwriting, approval workflow, document collection, portfolio monitoring, and handoff to servicing. Examples include nCino, Temenos lending workflows, TCS BaNCS for Corporate Loan Origination, Finastra origination components, Oracle workflows, and internal bank systems.

Public vendor descriptions show the direction. nCino describes commercial lending as a single platform for origination, underwriting, and portfolio management [1]. TCS BaNCS describes corporate loan origination as end-to-end digitisation of wholesale, commercial credit, and loan origination through credit approval processes and workflows [2]. Temenos describes corporate and commercial banking as a consistent operating model for lending and financing, with standardised products and processes, straight-through processing, and real-time decisions [3].

**Argent implication:** loan approval remains outside Argent. Argent should supply evidence into the credit workflow: eligible collateral certificate, borrowing-base report, policy hash, custody status, exceptions, and release constraints.

### 2.2 Loan servicing and facility administration

Loan servicing systems administer the facility after approval: limits, drawdowns, repayments, interest, fees, amendments, maturities, closures, notices, and compliance with deal terms. Examples include Finastra Loan IQ, FIS Commercial Loan Servicing, Oracle Banking Corporate Lending, Temenos, Finacle, and internal mainframe or custom servicing systems.

Finastra describes Loan IQ as a commercial loan servicing platform used by 21 of the top 25 global banks and able to support syndicated, bilateral, SME, CRE, SBA, export finance, and other lending types [4]. FIS describes Commercial Loan Servicing as part of a front-to-back commercial lending suite with workflow, analytics, and reporting across the commercial loan process [5]. Oracle's corporate lending material describes drawdowns, repayments, amendments, rollovers, settlements, and closures as normal corporate-lending lifecycle functions [6].

**Argent implication:** the bank's servicing system remains the book of record for the loan. Argent is the book of record for the collateral-control lifecycle around the physical collateral.

### 2.3 Limits, collateral, and covenant systems

Banks manage credit exposure through limits, collateral values, covenants, concentration controls, and borrower groups. Examples include Oracle Banking Enterprise Limits and Collateral Management, Finacle Limits, Collaterals and Covenants Management Suite, and internal enterprise exposure systems.

Oracle describes enterprise limits and collateral management as supporting multiple collateral types, haircut margins, collateral pools linked to credit facilities, order of utilisation, collateral revaluation at defined frequency, and covenant linkage [7]. Finacle describes limits, collateral, and covenant management as an enterprise-level exposure lifecycle solution for multi-entity operations [8].

**Argent implication:** Argent should export exactly the fields those systems understand: collateral identifier, eligibility status, policy version, haircut-adjusted value, borrowing base, utilisation, margin state, exception reason, concentration bucket, release block, and evidence reference.

### 2.4 Collateral and margin platforms

For derivatives, securities finance, repo, treasury, and institutional collateral operations, banks use systems such as Murex MX.3, Adenza Calypso, CloudMargin, LSEG Acadia Margin Manager and Collateral Manager, and triparty collateral services.

Murex describes MX.3 for Collateral Management as an enterprise-wide framework for margining, optimisation, regulatory compliance, and collateral trading, with a real-time view of asset inventory [9]. CloudMargin describes a cloud-native platform for workflow automation, asset optimisation, and real-time reporting across the collateral lifecycle [10]. LSEG Acadia Margin Manager describes margin messaging, dispute resolution, audit tracking, and integration with internal and vendor collateral-management systems [11].

**Argent implication:** Argent should not compete with these platforms. It should provide the physical-collateral control feed they often lack: proof that a specific physical lot was attested, immobilised, pledged, revalued, margin-checked, blocked for release, released, or enforced under a signed policy.

### 2.5 Trade-finance and commodity-finance systems

Commodity-collateral lending often touches trade-finance operations: letters of credit, guarantees, documentary credits, trade loans, commodity documentation, bank guarantees, warehouse receipts, and multi-bank workflows. Banks and corporates use platforms such as Finastra Trade Innovation, Surecomp, Komgo, SWIFT MT 798, and internal trade-finance systems.

Finastra describes Trade Innovation as providing real-time visibility into trade-finance transactions and exposures to monitor and mitigate risk [12]. Surecomp describes itself as a global trade-finance software provider for banks and corporates, with trade instruments including letters of credit, guarantees, documentary collections, and trade loans [13]. Komgo describes itself as a multi-bank trade-finance platform that digitises and automates trade-finance operations for corporates and financial institutions [14]. SWIFT MT 798 covers import letters of credit, export letters of credit, and guarantees or standby letters of credit [15].

**Argent implication:** for commodity finance, the adapter must support documentary evidence and trade-finance references, not only credit-line fields. A pledged physical lot may be linked to a warehouse receipt, bill of lading, storage certificate, inspection certificate, insurance reference, or trade-finance instrument.

### 2.6 Messaging, identity, and authority standards

Banks prefer standards because bespoke integrations are expensive to govern. ISO 20022 provides a common methodology, process, and repository for financial messages [16]. The ISO 20022 repository contains a Data Dictionary and Business Process Catalogue under release control [17]. GLEIF's LEI and vLEI direction matters because the vLEI is designed to let counterparties computationally verify the identity, authority, and role of people acting on behalf of legal entities [18].

**Argent implication:** role authority should not be modelled as ad hoc wallet addresses forever. The bank, custodian, owner, verifier, processor, and signer should eventually map to legal-entity identifiers, role claims, wallet policies, and evidence of authority.

### 2.7 Signing, wallet governance, and policy engines

The current repo keeps the Soroban contracts signer-agnostic. In production, institutional signing should be governed by an external signing layer. DFNS is the currently documented direction in this repository.

DFNS describes wallet operation as user-driven, policy-driven, service-driven, or autonomous within configured limits [19]. Its policy documentation describes policies as gates over signing, transfers, permission changes, and policy edits, with block or approval flows before execution [20]. DFNS also documents programmable approval policies for decoding pending smart-contract calls and approving or denying them with custom business logic [21].

**Argent implication:** DFNS should not be treated only as key custody. It is the policy and authorization perimeter around who is allowed to sign a collateral act, under which role, for which amount, policy version, collateral lot, and evidence hash.

---

## 3. The market direction: interoperability before replacement

The institutional collateral market is not moving toward a single new universal ledger. It is moving toward interoperability, collateral mobility, policy-bound automation, and stronger reconciliation between legacy systems and digital-asset infrastructure.

DTCC's 2026 collateral research frames tokenized collateral around collateral mobility, lower liquidity buffers, reduced capital pressure, and resilience during stress [22]. The Bank of England's Project Meridian Securities tested synchronisation of tokenised securities and central bank money, including intraday repo execution with smart contracts automating eligibility checks, collateral allocation, and settlement [23]. The ECB announced that marketable assets issued in CSDs using DLT-based services would be accepted as Eurosystem collateral from 30 March 2026, but only if they meet the existing collateral eligibility and settlement-system requirements [24]. The ISDA and FINOS Common Domain Model represents eligible collateral through data standards for criteria, include and exclude logic, haircuts, and concentration limits [25].

The pattern is consistent:

```text
Existing legal and accounting systems remain authoritative.
Digital systems make collateral state more structured, mobile, and automatable.
Eligibility and policy still govern the collateral.
Interoperability matters more than replacement.
```

Argent should adopt the same pattern for physical collateral. Physical assets do not need to become free-floating public tokens. Their control state needs to become structured, role-signed, policy-bound, and easy for existing bank systems to consume.

---

## 4. Argent's integration thesis

Argent should be presented to banks as a reserve-capacity and collateral-control sidecar.

A sidecar is not the system of record for everything. It is a bounded service that records and proves one part of the workflow better than the systems around it, then reconciles with them.

For Argent, the bounded domain is:

```text
physical reserve identity
custody attestation
eligibility and treatment
policy version
pledge activation
master capacity and sublimits
purpose-bound reservation
bank-obligation reference
contingent and crystallized exposure
reimbursement evidence
margin state
release authorization
custodian release confirmation
default, cure, readiness, and enforcement evidence
role authority and revocation
replayable event history
```

The systems around Argent keep their authority:

| Domain | Existing bank or market authority | Argent role |
|---|---|---|
| Facility and product decision | bank credit, trade-finance, treasury, limits, and committee workflow | provide reserve-control and capacity evidence |
| Product and exposure books | trade-finance, guarantee, treasury, and loan systems | mirror capacity and exposure-relevant events and reconcile |
| Accounting | core banking, general ledger, loan servicing | no accounting authority |
| Collateral policy | bank risk and collateral function | enforce signed policy version |
| Physical custody | custodian, warehouse, vault, terminal | record custodian-signed control acts |
| Legal documents | facility, pledge, custody, warehouse, enforcement documents | hash and reference documents |
| Price source | bank-approved valuation source | enforce freshness and haircut policy |
| Signing authority | DFNS or other institutional signing layer | require correct role signatures on Soroban |
| Audit and review | bank audit, risk, legal, regulator, external auditor | provide event replay and evidence pack |

The most important adoption sentence is:

> Argent is not the bank's product system. Argent is the shared reserve-capacity ledger beneath the bank obligations that existing product systems issue and administer.

---

## 5. The Bank Adapter Gateway

The integration component should be named conservatively:

```text
Argent Bank Adapter Gateway
```

It is not a core-banking system. It is not a middleware platform for every possible message. It is a controlled translation, reconciliation, and evidence service between bank systems, custodian systems, DFNS, and Soroban.

### 5.1 Reference architecture

```text
Bank product, credit, and facility workflow
nCino / Temenos / TCS BaNCS / internal workflow
        |
        | facility data, policy pack, approval status
        v
Bank facility, trade-finance, treasury, and servicing systems
Loan IQ / FIS / Oracle / Temenos / Finacle / internal servicing
        |
        | facility id, limit, exposure, drawdown, repayment
        v
Bank limits, collateral, and covenant systems
Oracle / Finacle / internal exposure engine
        |
        | eligibility, haircut, borrowing base, exceptions
        v
Argent Bank Adapter Gateway
        |
        | canonical collateral-control model
        | validation, reconciliation, idempotency, evidence hashing
        v
Argent Soroban contracts
credit_ledger / settlement_vault / rewards_ledger
        |
        | typed events, state, transaction hashes
        v
Argent indexer and evidence pack
certificates / reports / replay files / audit bundle
        |
        v
Bank systems, custodian systems, credit ops, risk, audit, legal
```

In parallel:

```text
Custodian / warehouse / vault
        |
        | position statement, immobilisation, release, substitution, dispute
        v
Argent Bank Adapter Gateway

DFNS or institutional signing layer
        |
        | decoded Soroban call, approval policy, signer role, pending state
        v
Argent Bank Adapter Gateway

Stellar RPC / Horizon / indexer archive
        |
        | contract events, transaction status, settlement asset events
        v
Argent evidence pack and reconciliation database
```

### 5.2 Gateway responsibilities

The gateway has seven responsibilities.

1. **Normalize inbound data.** Convert bank, custodian, valuation, document, and signing data into the canonical Argent model.
2. **Validate before broadcast.** Refuse incomplete, stale, inconsistent, unsigned, or policy-mismatched input before any Soroban call is assembled.
3. **Preserve idempotency.** Every inbound request has a stable external reference and idempotency key, so retries do not create duplicate control events.
4. **Bind evidence.** Every off-chain document, statement, data file, approval, valuation, and exception is hashed and referenced.
5. **Reconcile.** Compare bank state, custodian state, DFNS pending state, Soroban state, and indexed event state.
6. **Export certificates.** Produce bank-readable reports, evidence packs, and status messages.
7. **Fail closed.** If policy, custody, valuation, authorization, or indexer state is missing or inconsistent, risk-increasing acts are blocked.

### 5.3 What the gateway should not do

The gateway must not become a hidden credit engine.

It should not approve credit, change bank limits, post accounting entries, move cash, override a custodian, set haircuts, assert legal perfection, mark a loan as repaid, release collateral without role signatures, or approve a Soroban transaction that the bank's own policy would block.

The gateway's power comes from being narrow.

---

## 6. Canonical collateral-control model

The adapter should expose one canonical model, even if each bank source system has a different internal format. The canonical model is not a replacement for ISDA CDM, ISO 20022, or a bank data model. It is the minimum shared shape required to make physical collateral control reliable.

### 6.1 Required objects

```text
Party
Role
Signer
Facility
CollateralRiskPolicy
Instrument
EligibilityTreatment
CollateralPosition
CustodyStatement
Pledge
CreditLine
Valuation
BorrowingBase
Drawdown
Repayment
MarginState
CollateralAdjustment
ReleaseInstruction
DefaultNotice
CureRecord
EnforcementReadiness
EnforcementRecord
ExceptionRecord
EvidenceReference
SorobanEventReference
DFNSApprovalReference
```

### 6.2 Minimal object definitions

A pilot implementation can start with JSON schemas for the following objects.

```json
{
  "facility_id": "BANK-FAC-0001",
  "external_system": "loan_servicing",
  "borrower_party_id": "PARTY-OWNER-001",
  "bank_party_id": "PARTY-BANK-001",
  "currency": "CHF",
  "approved_limit_minor": "16000000",
  "maturity_date": "2027-07-09",
  "policy_id": "POLICY-2026-001",
  "policy_version": 1,
  "facility_doc_hash": "sha256:...",
  "status": "approved_for_shadow_control"
}
```

```json
{
  "position_id": "CUST-BARSET-0001",
  "owner_party_id": "PARTY-OWNER-001",
  "custodian_party_id": "PARTY-CUST-001",
  "instrument_id": "ALLOCATED-GOLD-BARSET",
  "location_code": "VAULT-ZH-001",
  "identity_hash": "sha256:...",
  "uniqueness_hash": "sha256:...",
  "statement_hash": "sha256:...",
  "custody_status": "attested",
  "dispute_status": "none",
  "statement_as_of": "2026-07-09T07:00:00Z"
}
```

```json
{
  "valuation_id": "VAL-0001",
  "position_id": "CUST-BARSET-0001",
  "price_source": "bank_approved_source",
  "valuation_currency": "CHF",
  "raw_value_minor": "33000000",
  "haircut_bps": 1500,
  "haircut_adjusted_value_minor": "28050000",
  "valuation_as_of": "2026-07-09T07:00:00Z",
  "stale_after": "2026-07-10T07:00:00Z",
  "evidence_hash": "sha256:..."
}
```

```json
{
  "borrowing_base_id": "BB-0001",
  "facility_id": "BANK-FAC-0001",
  "policy_id": "POLICY-2026-001",
  "policy_version": 1,
  "total_attested_value_minor": "33000000",
  "eligible_value_minor": "33000000",
  "haircut_adjusted_value_minor": "28050000",
  "max_advance_rate_bps": 5000,
  "borrowing_base_minor": "14025000",
  "outstanding_exposure_minor": "1000000",
  "available_capacity_minor": "13025000",
  "margin_state": "covered",
  "exception_count": 0,
  "computed_at": "2026-07-09T07:05:00Z"
}
```

### 6.3 Identifier discipline

The gateway must distinguish external identifiers from Argent identifiers.

```text
bank_facility_id        external bank reference
servicing_account_id    external loan-servicing reference
custodian_position_id   external custodian reference
argent_framework_id     Soroban framework reference
argent_position_id      Soroban position reference
argent_line_id          Soroban credit-line reference
soroban_tx_hash         Stellar transaction hash
event_sequence          internal indexed event sequence
policy_version          signed bank policy version
```

No adapter should collapse these identifiers into one field. Collapsing identifiers is how reconciliation becomes ambiguous.

---

### 6.4 Preflight, reservation, and definitive outcome model

The gateway must distinguish a collateral position that appears free from capacity that is usable for a particular bank product. The originating system should be able to request a preflight decision containing:

```text
request id and idempotency key
facility and applicant
product type
beneficiary
amount and currency
tenor or expiry
commercial purpose
required evidence references
requested response deadline
```

The gateway then evaluates or retrieves the authoritative result for:

```text
facility and product eligibility
applicant and beneficiary permission
current eligible and available capacity
product and group sublimits
currency, jurisdiction, tenor, and concentration rules
evidence and custody freshness
approval route availability
bank product-system availability
settlement and reimbursement route
```

A positive decision may create a provisional reservation atomically with the capacity reduction. The response should include:

```text
decision and reason codes
reservation id and expiry
reserved capacity
policy version
authoritative external references
required next action
```

The bank product platform remains authoritative for legal and operational issue. Its callback or status response must be correlated to the originating request and reservation. A timeout, lost callback, or ambiguous network response must not trigger blind resubmission or capacity release. The detailed state model is defined in [`capacity-reservation-and-deliverability.md`](capacity-reservation-and-deliverability.md).

## 7. Adapter classes

The gateway should be built as adapter classes, not one-off integrations. The first version should deliberately support low-tech bank pilots before attempting deep API integrations.

### 7.1 File and SFTP adapter

This is the most important first adapter because banks can often export files before they can expose APIs.

Supported formats:

```text
CSV
JSON
XML
XLSX converted to canonical CSV
signed PDF hash references
SFTP drop folder
manual upload with checksum
```

Inbound files:

```text
facility.csv
party.csv
role.csv
collateral_policy.csv
collateral_position.csv
custody_statement.csv
valuation.csv
pledge.csv
drawdown.csv
repayment.csv
release_request.csv
exception.csv
```

Outbound files:

```text
policy_certificate.json
position_eligibility_certificate.json
borrowing_base_report.csv
margin_state_report.csv
release_status_report.csv
exception_report.csv
event_replay.ndjson
evidence_pack_manifest.json
reconciliation_report.csv
```

This adapter makes pilots possible even where the bank's Loan IQ, nCino, Temenos, Oracle, FIS, Murex, Calypso, CloudMargin, or internal system cannot yet be connected.

### 7.2 REST and webhook adapter

A modern API layer should sit beside file exchange.

Inbound endpoints:

```text
POST /v1/parties
POST /v1/facilities
POST /v1/collateral-policies
POST /v1/collateral-positions
POST /v1/custody-statements
POST /v1/valuations
POST /v1/pledges
POST /v1/drawdowns
POST /v1/repayments
POST /v1/release-requests
POST /v1/exceptions
```

Read endpoints:

```text
GET /v1/facilities/{facility_id}/borrowing-base
GET /v1/facilities/{facility_id}/pool-risk-report
GET /v1/positions/{position_id}/eligibility-certificate
GET /v1/releases/{release_id}/status
GET /v1/events?facility_id={facility_id}
GET /v1/evidence-packs/{evidence_pack_id}
GET /v1/reconciliation/{facility_id}
```

Outbound webhooks:

```text
collateral.position_registered
collateral.custody_attested
collateral.immobilized
pledge.activated
line.opened
valuation.updated
margin.warning
margin.called
release.requested
release.authorized
release.confirmed
default.notice_issued
cure.recorded
enforcement.readiness_opened
enforcement.recorded
exception.opened
exception.closed
reconciliation.divergence_detected
```

### 7.3 Loan origination adapter

Purpose: supply the credit workflow with collateral evidence before approval.

Inputs from origination:

```text
borrower identity
facility request
proposed collateral type
proposed custodian
proposed policy version
required documents
approval status
conditions precedent
```

Outputs to origination:

```text
collateral readiness certificate
missing evidence list
eligibility status
policy match result
estimated borrowing base
custodian readiness state
risk and exception flags
```

No loan approval decision is made by Argent.

### 7.4 Loan servicing adapter

Purpose: reconcile collateral-control state with the active facility.

Inputs from servicing:

```text
facility id
limit
outstanding exposure
drawdown request
repayment confirmation
maturity or amendment state
fees or charges relevant to release
servicing status
```

Outputs to servicing:

```text
pledge activated
current borrowing base
available collateral-supported capacity
release blocked or release permitted
default readiness state
collateral exception flag
policy version used
Soroban transaction reference
evidence pack reference
```

The initial mode should be read-only or recommendation-only. Direct servicing updates are a later phase.

### 7.5 Limits, collateral, and covenant adapter

Purpose: export Argent's collateral state into the bank's exposure management environment.

Mapped fields:

```text
collateral type
collateral identifier
collateral owner
custodian
vault or location
eligibility status
policy id and version
haircut
haircut-adjusted value
advance rate
borrowing base
concentration bucket
wrong-way-risk flag
covenant or margin state
open exception count
release block
last valuation time
last custody statement time
```

This is the adapter that makes `collateral-eligibility-and-risk-policy.md` operational.

### 7.6 Collateral and margin platform adapter

Purpose: make physical collateral visible to institutional collateral operations.

Mapped concepts:

```text
inventory lot
eligible collateral schedule
treatment
haircut
valuation
call threshold
margin state
substitution ticket
release ticket
dispute
exception
concentration limit
```

For systems like Murex, Calypso, CloudMargin, and Acadia, Argent's value is a physical-collateral control feed, not a derivatives margin calculator.

### 7.7 Trade-finance and document adapter

Purpose: connect warehouse, shipping, and trade-finance documents to collateral-control state.

For bank obligations, the adapter should support two directions:

1. **preflight and reservation** - receive the requested product, applicant, beneficiary, amount, currency, tenor, purpose, and evidence requirements;
2. **authoritative lifecycle callback** - receive issue, rejection, amendment, cancellation, presentation, claim, payment, expiry, and discharge outcomes from the bank product system.

Each request and callback must carry a stable originating-system reference, idempotency key, facility or reservation reference, status timestamp, and reason code. The adapter must query the authoritative product system before deciding that an ambiguous issue attempt failed.

Mapped references:

```text
letter of credit reference
guarantee reference
standby LC reference
bill of lading reference
warehouse receipt reference
storage certificate reference
inspection certificate reference
quality certificate reference
quantity certificate reference
insurance certificate reference
invoice reference
purchase order reference
trade loan reference
```

For each document, Argent records:

```text
document type
issuer
holder or beneficiary
operative status
issue date
expiry date
hash
source system
linked position
linked facility
exception state
```

A document hash does not make the document true. It makes later substitution, mismatch, or silent revision visible.

### 7.8 Custodian, warehouse, and vault adapter

Purpose: connect the physical root of trust to the control ledger.

Supported modes:

```text
manual attestation mode
file attestation mode
API attestation mode
signed PDF plus hash mode
```

Custodian events:

```text
position_attested
position_immobilized
position_blocked_for_release
position_substituted
position_released
position_under_dispute
position_under_legal_hold
position_insurance_changed
position_audit_updated
position_statement_refreshed
```

Minimum custodian data:

```text
custodian legal identity
position id
owner identity
asset type
quantity
quality or grade
location
allocation or segregation status
encumbrance statement
release-control agreement reference
statement date
statement hash
signer role
```

The adapter should tolerate a low-tech pilot. A custodian may not expose an API in the first design-partner phase. A signed statement plus hash plus role signature is enough to test the control logic.

### 7.9 Shared gold assurance adapter

A future adapter may consume a signed reserve assertion from a custodian, provenance database, pooled-gold register, digital-gold operator, or shared gold platform.

It should map:

- `reserve_profile`, authoritative record ID, and source product ID;
- owner or entitlement holder;
- custodian or product operator;
- quantity, purity or economic-gold equivalent;
- backing, allocation, redemption, and control capabilities;
- assurance provider, scope, timestamp, expiry, and reconciliation-tolerance status;
- economic, legal, and operational equivalence class where the source defines one;
- known holds or encumbrances within the source system's scope;
- explicit `does_prove` and `does_not_prove` semantics.

The gateway must not treat upstream verification as bank eligibility, legal pledgeability, operational control, or legal perfection. It must not duplicate an upstream token, account balance, or ownership record on Stellar. A stale, expired, tolerance-breached, or discrepant assertion blocks new risk-increasing actions and opens reconciliation while active exposure remains controlled.

The bank adapter should preserve four separate gates: reserve verified, legally pledgeable, operationally controllable, and facility issuable.

### 7.10 Valuation adapter

Purpose: enforce freshness and policy-bound valuation without making Argent a pricing provider.

Inputs:

```text
price source
asset type
valuation time
currency
raw price
FX rate where relevant
confidence indicator
haircut policy reference
staleness threshold
```

Outputs:

```text
valuation accepted
valuation rejected
stale valuation flag
zero-value reason
haircut-adjusted value
margin state
borrowing-base change
```

The bank chooses the price source. Argent enforces the bank's freshness and policy rules.

### 7.11 Identity and authority adapter

Purpose: map real legal entities and authorised signers to Argent roles.

Fields:

```text
legal entity name
LEI
vLEI credential reference, future
bank party id
custodian party id
owner party id
signer id
role
role scope
approval policy id
wallet id
revocation status
valid from
valid until
```

The long-term direction should be compatible with LEI and vLEI because the vLEI is explicitly about computational verification of identity, authority, and role [18].

### 7.12 DFNS signing adapter

Purpose: make every on-chain act decodeable, approvable, and attributable before signature.

Required mapping:

```text
Soroban method
Argent role required
party expected
wallet expected
policy id
facility id
position id
line id
amount
currency
policy version
evidence hash
transaction expiry
approval state
broadcast state
```

The adapter must decode the pending Soroban call and compare it to the bank policy before approval. It must reject opaque payloads. It must reconcile pending approvals with broadcast transactions and indexed events.

### 7.13 Stellar and Soroban indexer adapter

Purpose: archive Soroban events and transaction state beyond the short RPC query window.

Stellar documents that `getEvents` can retrieve filtered contract events, but the RPC query window is limited to recent ledgers and backend components should ingest events into their own database for querying and serving [26]. Stellar also documents contract events as the mechanism off-chain applications use to monitor both value movement and custom contract events [27].

Therefore production Argent requires an indexer archive.

Indexer duties:

```text
ingest contract events
parse CollateralEventV1
parse GovernanceEventV1
link events to bank identifiers
link events to DFNS approvals
link events to evidence hashes
track ledger sequence and transaction hash
detect gaps
replay facility state
export event replay file
reconcile with current contract state
```

Horizon remains useful for Stellar account and payment streams, including account payment streaming with cursors [28]. The Soroban event archive remains mandatory for the contract lifecycle.

---

## 8. Integration modes

The adoption sequence should be conservative.

### 8.1 Mode 1: evidence-only mirror

No on-chain enforcement against real credit. No write-back to bank systems.

The bank provides sample or pilot facility data, collateral documents, custodian statements, valuation data, and policy rules. Argent produces:

```text
collateral policy certificate
position eligibility certificate
borrowing-base report
margin-state report
release-state report
evidence pack
reconciliation report
```

Success metric:

```text
Bank credit, risk, operations, legal, and audit can reconcile Argent's outputs against their existing files and spreadsheets.
```

### 8.2 Mode 2: shadow ledger

Argent runs in parallel with the bank's real process. The bank's existing systems remain authoritative. Argent records the collateral-control lifecycle as a shadow ledger.

Allowed:

```text
register framework
register position
record custodian attestation
activate shadow pledge
mirror drawdown
mirror repayment
mirror valuation
mirror release request
mirror release confirmation
record exceptions
```

Not allowed:

```text
block real credit
trigger real release
book accounting
move cash
replace legal notice
```

Success metric:

```text
No unexplained divergence between bank servicing state, custodian state, DFNS approvals, Soroban state, and indexed event replay.
```

### 8.3 Mode 3: controlled decision gate

Argent becomes authoritative for one narrow decision, agreed in advance.

Recommended first gates:

```text
release permitted or blocked
additional drawdown capacity available or blocked
margin warning or call state
collateral exception open or closed
custodian immobilisation confirmed or not confirmed
```

The gate is narrow because that is how a bank can adopt safely. It does not outsource credit approval. It asks Argent for a specific control answer.

### 8.4 Mode 4: limited write-back

Only after successful reconciliation should the adapter write structured statuses back into bank systems.

Low-risk write-backs:

```text
status update
certificate link
evidence hash
exception flag
release block
borrowing-base report
margin-state report
Soroban transaction reference
```

High-risk write-backs, deferred:

```text
loan booking
accounting entry
cash movement
limit amendment
automatic legal notice
automatic collateral release
```

### 8.5 Mode 5: governed production integration

In production, Argent may become a required control record for an agreed product perimeter: one asset type, one bank, one custodian, one facility class, one policy pack, one release path, and one evidence pack. Expansion happens by adding policy packs and adapters, not by making the first pilot broad.

---

## 9. Write policy and fail-closed controls

The adapter must implement a strict write policy.

| Action type | V1 treatment |
|---|---|
| Read bank facility data | allowed |
| Read custodian statement | allowed |
| Hash document evidence | allowed |
| Produce certificate | allowed |
| Mirror drawdown or repayment | allowed with external reference |
| Register position on Soroban | allowed after validation |
| Confirm immobilisation | custodian role only |
| Activate pledge | owner, bank, and custodian workflow only |
| Authorize release | bank role only |
| Confirm release | custodian role only |
| Update bank servicing system | status write-back only, later phase |
| Book accounting entry | prohibited |
| Move bank cash | prohibited |
| Override bank policy | prohibited |
| Override custodian physical control | prohibited |

Fail-closed conditions:

```text
missing policy version
inactive policy
stale valuation
stale custody statement
missing custodian attestation
duplicate uniqueness hash
open legal hold
open dispute
sanctions or AML block flag
wrong signer role
revoked party
DFNS approval mismatch
Soroban transaction expired
indexer event gap
bank state and Argent state diverge
custodian state and Argent state diverge
```

In any fail-closed state, risk-reducing actions may still be allowed under policy: repayment, top-up, documentation cure, dispute closure, signer revocation, exception recording. Risk-increasing actions are blocked: new drawdown, release, substitution that reduces coverage, and policy-unsafe mobilisation.

---

## 10. Security, resilience, and vendor-risk posture

A bank will judge integration through operational resilience and third-party-risk lenses, not only through smart-contract correctness.

DORA applies from 17 January 2025 and is designed to ensure that banks and other financial entities can withstand, respond to, and recover from ICT disruptions such as cyberattacks or system failures [29]. DORA also sets rules for ICT third-party risk management, including contractual and oversight considerations [30]. The EBA's outsourcing guidelines state the governance and third-party-risk framework banks expect around outsourcing arrangements [31]. In 2025, the EBA also consulted on broader third-party-risk guidelines for non-ICT services, making clear that management-body responsibility cannot be delegated [32].

Argent's adoption posture should therefore be:

```text
read-only first
least privilege
bank-owned policy
custodian-owned physical attestation
external signing policy
no shared production keys
no unilateral release authority
no unilateral credit authority
full event replay
certificate exports
reconciliation reports
contractual exit and data export path
```

### 10.1 Controls the adapter should expose

```text
mTLS or bank-approved API security
OAuth or bank identity integration where required
IP allowlisting where required
tenant isolation
environment separation
data minimisation
encryption in transit and at rest
immutable audit log
idempotency keys
request signing
checksum verification
role-based access control
segregation of duties
four-eyes approval for high-risk operations
break-glass procedure
incident logging
operational runbook
backup and restore plan
indexer gap detection
evidence export
exit package
```

### 10.2 Data minimisation

The adapter should not ingest more bank data than the control decision needs.

Required for control:

```text
party identifiers
role identifiers
facility reference
policy version
collateral identity
custody statement hash
valuation and haircut data
exposure amount
release request
exception status
evidence hashes
```

Usually not required:

```text
full borrower financial statements
full credit memo
full KYC file
full customer PII
bank account details unless settlement integration is in scope
internal risk rating unless policy needs it
```

The principle is simple: Argent should not become a sensitive-data sink just to prove collateral control. Shared state, adapter payloads, evidence exports, and user views should be minimized by role and purpose. Exact reserve values, bar serials, beneficiary terms, and group exposure should remain restricted unless required for the receiving party's function. Hashes do not make low-entropy or commercially identifying data confidential. See [`selective-disclosure-and-institutional-privacy.md`](selective-disclosure-and-institutional-privacy.md).

---

## 11. External-service bridge model

The integration strategy should treat external services as bridges into controlled facts, not as authorities Argent blindly trusts.

### 11.1 Facility bridge

Source systems: loan origination, loan servicing, internal facility spreadsheets.

Bridge output:

```text
FacilityApproved
FacilityAmended
ExposureUpdated
DrawdownRequested
RepaymentConfirmed
FacilityClosed
```

Validation:

```text
facility exists
bank party matches
borrower party matches
policy version active
amounts and currency match expected scale
idempotency key unique
```

### 11.2 Collateral bridge

Source systems: collateral engine, custodian system, warehouse statement, vault report.

Bridge output:

```text
PositionAttested
PositionImmobilized
PositionReleased
PositionDisputed
PositionFrozen
StatementRefreshed
```

Validation:

```text
custodian party authorized
position identity stable
statement hash present
statement date fresh
dispute state not contradictory
no release without bank authorization
```

### 11.3 Valuation bridge

Source systems: bank price source, market data provider, internal valuation desk.

Bridge output:

```text
ValuationAccepted
ValuationRejected
BorrowingBaseComputed
MarginStateChanged
```

Validation:

```text
price source approved under policy
valuation timestamp within freshness window
haircut policy matches policy version
FX source approved if needed
value cannot be negative
zero-value reason required where eligible value is zero
```

### 11.4 Document bridge

Source systems: document management system, trade-finance system, custodian statement, legal repository.

Bridge output:

```text
EvidenceReferenceRegistered
EvidenceReferenceSuperseded
EvidenceReferenceDisputed
EvidenceReferenceExpired
```

Validation:

```text
document type expected under policy
hash present
issuer present
linked facility or position present
expiry checked where relevant
superseded document cannot remain active
```

### 11.5 Authorization bridge

Source systems: DFNS, IAM, bank approval workflow, custodian approval workflow.

Bridge output:

```text
ApprovalRequested
ApprovalGranted
ApprovalDenied
ApprovalExpired
SignerRevoked
PolicyChanged
```

Validation:

```text
Soroban method decoded
role required matches wallet role
party expected matches signer party
amount and collateral match approval payload
evidence hash matches pending transaction
policy version active
approval timestamp valid
```

---

## 12. Evidence outputs for bank onboarding

The adapter's main product is the evidence it gives the bank back.

### 12.1 Integration certificate

A bank-readable certificate showing:

```text
connected source systems
adapter mode
read-only or write-back status
supported objects
supported events
current policy version
DFNS policy mapping
Soroban contract ids
evidence archive location
last reconciliation time
known open divergences
```

### 12.2 Facility control report

```text
facility id
borrower
bank
policy version
approved limit
outstanding exposure
pledged collateral
haircut-adjusted value
available borrowing base
margin state
open exceptions
release blocks
last valuation
last custodian statement
last Soroban event
last bank reconciliation
```

### 12.3 Position eligibility certificate

Already defined in `collateral-eligibility-and-risk-policy.md`, but the adapter makes it deliverable to bank systems.

```text
position identity
custodian
owner
asset type
eligibility status
policy version
negative eligibility flags
valuation
haircut
advance value
custody status
immobilisation status
evidence hashes
exceptions
```

### 12.4 Release decision certificate

This should become one of the first controlled pilot outputs.

```text
release request id
facility id
position id
requested by
bank authorization status
custodian confirmation status
current exposure
post-release coverage
policy version
valuation freshness
open exceptions
release decision: permitted or blocked
reason code
Soroban transaction reference
```

### 12.5 Reconciliation report

```text
bank exposure versus Argent exposure
custodian position versus Argent position
DFNS approval versus Soroban transaction
Soroban event stream versus indexer archive
current contract state versus replayed state
open gaps
age of oldest unresolved divergence
```

If Argent cannot produce this report, it is not ready for a bank pilot.

---

## 13. Implementation sequence

The integration build should be staged in the order that reduces adoption risk.

### Phase 1: schemas and examples

Deliver:

```text
canonical JSON schemas
CSV templates
example files
OpenAPI draft
example evidence pack
example reconciliation report
```

No bank integration required.

### Phase 2: file adapter

Deliver:

```text
inbound CSV and JSON loader
checksum and schema validation
idempotency handling
evidence hash registry
certificate generator
reconciliation report
```

This is enough for a design partner to test with anonymised or sample data.

### Phase 3: Soroban event indexer

Deliver:

```text
CollateralEventV1 parser
GovernanceEventV1 parser
event archive database
facility replay
position replay
gap detection
state divergence detection
```

This is mandatory because Stellar RPC event querying is recent-ledger bounded [26].

### Phase 4: DFNS signing adapter

Deliver:

```text
method decoder
role and wallet mapping
policy-id mapping
pending-state tracker
approval reconciliation
expired approval handling
mismatched payload rejection
```

This is the institutional authorization layer described in `argent-dfns-signing-sequence.md`.

### Phase 5: REST and webhook API

Deliver:

```text
OpenAPI v1
inbound endpoints
read endpoints
outbound webhook events
request signing
idempotency keys
error taxonomy
```

### Phase 6: preflight, reservation, and callback API

Deliver:

```text
authenticated preflight endpoint
available-versus-issuable capacity decision
provisional reservation and expiry
stable correlation and idempotency rules
deterministic reason-code taxonomy
issue and lifecycle callback endpoints
ambiguous-outcome reconciliation
definitive response to the originating system
```

### Phase 7: bank-system mapping packs

Deliver mapping notes, not proprietary connectors:

```text
argent-to-loan-servicing.md
argent-to-collateral-management.md
argent-to-trade-finance.md
argent-to-custodian.md
argent-to-dfns.md
argent-to-stellar-indexer.md
```

### Phase 8: design-partner pilot

Deliver:

```text
one bank or finance provider
one custodian
one owner
one collateral pool
one obligation type
one policy pack
one preflight and reservation path
one authoritative issue callback
one release decision gate
one capacity and reconciliation report
```

The pilot should not begin with many assets, many custodians, and many systems.

---

## 14. Design-partner questionnaire

A bank pilot should start by answering these questions.

**Current systems.** Which systems originate the credit, service the facility, manage collateral, manage trade documents, store legal documents, manage signatures, and report exposure? Which system is authoritative for each object?

**Data extraction.** Can the bank provide sample facility, exposure, collateral, valuation, and release data by file export before any API work? What format, cadence, and approval process?

**Collateral policy.** Which collateral policy version should Argent enforce? Which fields are mandatory: eligibility, haircut, valuation frequency, concentration, release, substitution, cure, and exception treatment?

**Custodian workflow.** Can the custodian produce an attestation, immobilisation confirmation, release confirmation, and dispute flag? By API, file, email, signed PDF, or manual workflow?

**Authorization.** Which party signs each act? Which signing platform is used? Which role wallet maps to bank, custodian, owner, verifier, processor, or operator? What is the revocation procedure?

**Read-only pilot.** Which reports would be useful without changing any bank system: eligibility certificate, borrowing-base report, release decision certificate, evidence pack, or reconciliation report?

**Controlled gate.** Which single control decision could Argent safely own first: release permitted, drawdown capacity, margin call, or exception state?

**Risk review.** What information-security, outsourcing, data-protection, operational-resilience, and audit evidence does the bank require before a pilot?

The output should be a signed integration pack, not an informal email thread.

---

### Additional reservation, deliverability, and privacy questions

1. At what point does a quote or application consume capacity?
2. How long may provisional capacity remain reserved before issue?
3. Which system is authoritative when an issue request times out or a callback is lost?
4. Which identifiers make retries and callbacks idempotent across the bank, Argent, DFNS, and Soroban?
5. Which beneficiary, product, jurisdiction, document, and operating-window checks determine issuability?
6. What exact status must be returned to the originating system before capacity may be reused?
7. Which reserve, obligation, and evidence fields may each role see?
8. Which data must never be submitted to a public ledger, even as a direct hash?
9. What retention, deletion, disclosure, and audit rules apply to evidence packages?
10. What manual exception process applies when custody, bank-product, and Argent states disagree?

## 15. Test surface

The adapter layer should be tested like a bank system, not a demo script.

**Schema validation.** Invalid facility, position, valuation, release, exception, or evidence records are refused with stable error codes.

**Idempotency.** Replayed inbound files or API calls do not create duplicate Soroban events.

**Mapping.** Bank facility id, custodian position id, Argent position id, Soroban transaction hash, DFNS approval id, and evidence hash stay distinct and traceable.

**Freshness.** Stale valuation, stale custody statement, stale policy, and stale DFNS approval block risk-increasing actions.

**Authorization.** A bank release request cannot be signed by an owner wallet; a custodian release confirmation cannot be signed by a bank wallet; a revoked signer cannot sign anything.

**Payload decoding.** DFNS pending actions are decoded and compared against method, role, amount, facility, position, policy version, and evidence hash before approval.

**Event ingestion.** Missing Soroban events, duplicate events, ledger gaps, and replay divergence are detected.

**Reconciliation.** Bank exposure versus Argent exposure, custodian position versus Argent position, DFNS approval versus Soroban transaction, and current contract state versus replayed state are compared.

**Fail-closed behaviour.** When state is inconsistent, release, drawdown, substitution, and mobilisation are blocked, while repayment, top-up, cure, and exception recording remain possible where policy allows.

**Evidence export.** The evidence pack can be regenerated from archived data without relying on a mutable dashboard.

**Operational recovery.** Adapter restart, duplicate file delivery, partial webhook failure, RPC outage, indexer catch-up, and failed broadcast are handled without losing state.

---

## 16. What this strategy is not

- **It is not a claim that Argent integrates today with Loan IQ, nCino, Temenos, Oracle, FIS, Finacle, Murex, Calypso, CloudMargin, Acadia, Surecomp, Komgo, SWIFT, DFNS, or any custodian platform.** This document defines the adapter model and mapping strategy.
- **It is not a replacement for loan origination.** The bank still approves credit.
- **It is not a replacement for loan servicing.** The bank still books, services, and accounts for the loan.
- **It is not a replacement for collateral management.** The bank still owns collateral policy and enterprise exposure management.
- **It is not a replacement for custody.** The custodian remains the physical root of trust.
- **It is not an outsourcing bypass.** The bank must still perform vendor-risk, operational-resilience, data-protection, and audit reviews.
- **It is not a universal integration claim.** Each bank's stack and control requirements differ.
- **It is not a promise of straight-through production use.** The safe route is evidence-only, shadow ledger, controlled gate, limited write-back, then governed production.

The principle is simple:

> Argent should meet banks where they are. It should not ask them to abandon the systems that already run credit. It should give those systems a stronger, shared, role-signed, replayable control record for physical collateral.

---

## References

Independent sources, cited to evidence the bank infrastructure and integration discipline described above. No partnership, compatibility, certification, or endorsement by any named organization is implied.

[1] nCino, "Commercial Lending," describes commercial lending across origination, underwriting, and portfolio management. https://www.ncino.com/solutions/commercial-lending

[2] TCS, "TCS BaNCS for Corporate Loan Origination," describes end-to-end digitisation of wholesale, commercial credit, and loan origination. https://www.tcs.com/what-we-do/products-platforms/tcs-bancs/solution/corporate-loan-origination

[3] Temenos, "Core for Corporate and Commercial Banking," describes a consistent operating model for corporate lending and commercial finance. https://www.temenos.com/products/core-banking/core-for-corporate/

[4] Finastra, "Loan IQ," describes Loan IQ as a commercial loan servicing platform used by 21 of the top 25 global banks and supporting multiple lending types. https://www.finastra.com/lending/solutions/loan-iq

[5] FIS, "Commercial Loan Servicing," describes an integrated front-to-back commercial lending and servicing solution. https://www.fisglobal.com/products/fis-commercial-lending-suite/fis-commercial-loan-servicing

[6] Oracle, "Oracle Banking Corporate Lending," product materials describe corporate lending lifecycle functions including drawdowns, repayments, amendments, rollovers, settlements, and closures. https://www.oracle.com/financial-services/banking/corporate-lending/

[7] Oracle, "Banking Enterprise Limits and Collateral Management," describes collateral pools, haircut margins, facility links, order of utilisation, revaluation frequency, and covenants. https://www.oracle.com/asean/financial-services/banking/enterprise-limits-collateral-management/

[8] Infosys Finacle, "Limits, Collaterals and Covenants Management Suite," describes enterprise-level limits, collateral, and covenant management. https://www.finacle.com/solution/limits-collaterals-and-covenants-management-suite/

[9] Murex, "MX.3 for Collateral Management," describes enterprise-wide margining, optimisation, regulatory compliance, collateral trading, and real-time inventory view. https://www.murex.com/en/insights/brochure/mx3-collateral-management

[10] CloudMargin, "Real-time collateral management," describes workflow automation, asset optimisation, and real-time reporting across the collateral lifecycle. https://www.cloudmargin.com/

[11] LSEG Acadia, "Margin Manager," describes margin messaging, dispute resolution, audit tracking, and integration with collateral systems. https://www.lseg.com/en/post-trade/solutions/streamline/margin-manager

[12] Finastra, "Trade Innovation," describes real-time visibility into trade-finance transactions and exposures. https://www.finastra.com/lending/solutions/trade-innovation

[13] Surecomp, "Trade Finance Software and Platform," describes trade-finance software for banks and corporates. https://surecomp.com/

[14] Komgo, "About Komgo," describes digital and automated trade-finance operations for corporates and financial institutions. https://www.komgo.io/about

[15] SWIFT, "MT 798," describes import letters of credit, export letters of credit, and guarantees or standby letters of credit. https://www.swift.com/trade-finance/mt-798

[16] ISO 20022, "ISO 20022," describes the common standardisation approach, methodology, process, and repository. https://www.iso20022.org/iso-20022

[17] ISO 20022, "The ISO 20022 Repository," describes the Data Dictionary and Business Process Catalogue. https://www.iso20022.org/financial-repository

[18] GLEIF, "The verifiable LEI (vLEI)," describes computational verification of identity, authority, and role. https://www.gleif.org/en/organizational-identity/introducing-the-verifiable-lei-vlei

[19] DFNS, "Wallet-as-a-Service," describes user-driven, policy-driven, service-driven, and autonomous wallet operation within limits. https://dfns.co/wallet-as-a-service/

[20] DFNS Docs, "Policies," describes policies as gates over transfers, signing, permission changes, and policy edits. https://docs.dfns.co/core-concepts/policies

[21] DFNS Docs, "Build programmable approval policies," describes decoding pending smart-contract calls and approving or denying them with custom business logic. https://docs.dfns.co/solutions/build-programmable-approval-policies

[22] DTCC, "Tokenized Collateral Could Unlock Billions in Capital and Transform Liquidity Management," 13 May 2026. https://www.dtcc.com/news/2026/may/13/tokenized-collateral-could-unlock-billions-in-capital-and-transform-liquidity-management

[23] Bank of England, "Project Meridian Securities," describes intraday repo execution with smart contracts automating eligibility checks, collateral allocation, and settlement. https://www.bankofengland.co.uk/payment-and-settlement/rtgs-future-roadmap/project-meridian-securities

[24] European Central Bank, "ECB paves way for acceptance of DLT-based assets as eligible Eurosystem collateral," 27 January 2026. https://www.ecb.europa.eu/press/pr/date/2026/html/ecb.pr260127_1~a946167ce1.en.html

[25] FINOS / ISDA, "Eligible Collateral Representation," Common Domain Model. https://cdm.finos.org/docs/eligible-collateral-representation/

[26] Stellar Developers, "getEvents," notes that clients can request filtered events and that backend components should ingest events into their own database for querying and serving. https://developers.stellar.org/docs/data/apis/rpc/api-reference/methods/getEvents

[27] Stellar Developers, "Events," describes events as the mechanism off-chain applications use to monitor value movement and custom contract events. https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/events

[28] Stellar Developers, "Retrieve an Account's Payments," describes streaming successful payments for an account with cursor support. https://developers.stellar.org/docs/data/apis/horizon/api-reference/get-payments-by-account-id

[29] EIOPA, "Digital Operational Resilience Act (DORA)," describes DORA's purpose and application date. https://www.eiopa.europa.eu/digital-operational-resilience-act-dora_en

[30] CSSF, "ICT and cyber risk for DORA entities," describes DORA third-party risk management rules and contractual considerations. https://www.cssf.lu/en/ict-and-cyber-risk-for-dora-entities/

[31] European Banking Authority, "Guidelines on outsourcing arrangements," 25 February 2019. https://www.eba.europa.eu/sites/default/files/documents/10180/2551996/38c80601-f5d7-4855-8ba3-702423665479/EBA%20revised%20Guidelines%20on%20outsourcing%20arrangements.pdf

[32] European Banking Authority, "Draft Guidelines on the sound management of third-party risk," consultation paper, 8 July 2025. https://www.eba.europa.eu/sites/default/files/2025-07/33a0ee15-9601-4c2b-828e-1b09201a6e9f/CP%20on%20Draft%20Guidelines%20on%20sound%20management%20of%20third%20party%20risk.pdf

[33] Quant, "Unlocking collateral mobility: How tokenisation transforms settlement infrastructure," 2026. https://quant.network/perspectives/unlocking-collateral-mobility-how-tokenisation-transforms-settlement-infrastructure/

[34] Digital Asset, "Canton ledger privacy model." https://docs.digitalasset.com/overview/3.5/explanations/ledger-model/ledger-privacy.html

[35] W3C, "Verifiable Credentials Data Model v2.0." https://www.w3.org/TR/vc-data-model-2.0/

[36] OpenID Foundation, "OpenID for Verifiable Presentations 1.0." https://openid.net/specs/openid-4-verifiable-presentations-1_0.html

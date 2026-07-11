# Bullion collateral: system design and build plan

> **Status:** Draft for design-partner review · **Implementation baseline:** `argent-core` @ `646878f` · **Last reviewed:** 2026-07-11
> Claims about contract behaviour in this document were verified against `contracts/credit_ledger/` at the baseline above. Contract code evolves; re-verify before relying on any specific claim.


**Status:** Public product and integration architecture  
**Audience:** banks, secured lenders, bullion desks, custodians, vault operators, collateral managers, implementation partners, legal and risk reviewers, and technical architects  
**Research basis:** public sources reviewed through 11 July 2026  
**Companion documents:** `argent-architecture.md`, `protocol.md`, `why-gold-secured-operational-credit.md`, `credit-control-extension-points.md`, `threat-model-and-security-boundaries.md`

---

## Executive proposition

Banks already offer gold accounts, certificates, physical bullion, custody, financing, leases, and repurchase transactions. Emirates NBD's bullion service is particularly instructive because it brings purchase, storage, financing, certificates, lease rates, repurchase terms, and international delivery into one bank-led product family [1], [2]. HSBC describes the mature bullion-bank model more generally as four connected capabilities: trading, financing, vaulting, and physical services [13].

Those services validate the market. They do not remove the infrastructure problem.

A vertically integrated bank can keep bullion, client records, credit policy, control instructions, and settlement inside one institutional perimeter. The harder and more general case is open-system bullion finance:

```text
borrower or investor
    holds a bullion position with
one or more custodians or account providers
    and seeks liquidity from
a bank or secured lender
    while relying on
a valuation source, dealer, insurer, and settlement bank
```

Each participant holds a different part of the truth. The custodian knows what is held and whether it is blocked. The lender knows the facility, eligibility policy, exposure, margin status, and enforcement rights. The borrower knows the commercial purpose and repayment source. A dealer knows the executable liquidation route. Legal documents define rights that none of those systems can infer from a bar list alone.

Argent's role is to connect those systems without pretending to replace them:

> **Argent is a bullion collateral control plane: a shared, role-authorised layer that classifies the bullion right, binds custody evidence to credit policy, calculates the authorised borrowing base, governs lifecycle transitions, and produces a verifiable record across borrower, lender, and custodian.**

The first production profile remains narrow:

> **A revolving credit facility secured by identified, allocated bullion held with an approved custodian under a tri-party control arrangement.**

The architecture is broader than that first profile. It is designed to distinguish allocated bars from unallocated account claims, certificates, pooled interests, leases, repos, and tokenised representations. Those instruments may all be described commercially as “gold,” but they create different ownership, counterparty, transfer, control, settlement, and enforcement consequences. The architecture must never treat them as interchangeable.

The central design rule is therefore:

> **Classify the legal and custody representation first. Apply the product lifecycle second. Calculate credit only after both are known.**

---

## 1. Purpose

This document defines the reference architecture through which an existing bullion holding can become eligible, controlled collateral for a bank credit product.

It answers six questions:

1. What type of bullion right does the client actually hold?
2. What evidence makes that right eligible for a specific lender?
3. How does the custodian or account provider place the holding under effective control?
4. How is borrowing capacity calculated, monitored, and changed?
5. How do release, substitution, movement, default, and liquidation operate without breaking coverage?
6. How does Argent connect to existing bank, custody, market-data, settlement, and document systems?

The document also separates three product classes that must not be collapsed into one state machine:

- **secured credit:** the borrower retains normal-course ownership and grants security or control;
- **repo:** the transaction is structured around sale and repurchase obligations;
- **lease or metal loan:** metal is delivered or credited and equivalent metal must be returned.

Argent may eventually support all three. The current contract core and the first design-partner implementation are for secured credit against allocated collateral.

---

## 2. Market signal: what Emirates NBD teaches

Emirates NBD matters because it shows how a bank can extend from gold access into a complete bullion product stack.

Its public materials describe several distinct offerings:

- a digital XAU account funded from AED or USD, starting from 0.05 XAU, designed to provide gold exposure without physical delivery or storage [3];
- gold certificates issued in defined denominations, priced from live international gold prices, held at the bank, and redeemable in cash or physical bars [4];
- branded 999.9 bars in 10 g, 50 g, and 100 g denominations, with certificates of authenticity and ownership, available for custody or delivery [6];
- a wholesale physical bullion service covering acquisition, storage, gold-backed financing, precious-metals repo, certificates, international delivery, repurchase terms, leases, and customised financing [1], [2].

The bank's product terms also demonstrate why names are insufficient. The Precious Metal – Gold terms state that the relevant investment is backed by **unallocated** physical bars, that the digital confirmation identifies weight, fineness, and investment type, and that the investment or confirmation cannot be assigned or pledged to a third party without the bank's prior written consent [5]. The wholesale service, by contrast, publicly describes fully backed certificates that may be pledged as collateral [2].

That apparent difference is not a contradiction. It is a product-architecture lesson:

> **“Gold certificate” is a commercial label, not a complete legal classification. Pledgeability, transferability, allocation, redemption rights, issuer consent, and control must be read from the governing product terms.**

Emirates NBD therefore provides a roadmap at two levels.

### 2.1 The bank product roadmap

A mature bullion franchise can develop in this sequence:

```text
access to gold
→ storage and custody
→ digital confirmation or certificate
→ cash and physical redemption
→ secured liquidity
→ repo and lease products
→ cross-border movement
→ customised institutional financing
```

### 2.2 The Argent infrastructure roadmap

Argent should not reproduce that product suite as a bank, dealer, or custodian. It should provide the reusable control infrastructure beneath the credit and collateral parts of the suite:

```text
holding classification
→ evidence validation
→ lender eligibility
→ custody control
→ borrowing base
→ draw and repayment
→ margin and substitution
→ release or enforcement
→ reconciliation and evidence
```

The difference is strategic. Emirates NBD is an example of a bank offering bullion products. Argent should enable banks and custodians to launch or interoperate such products, especially where the bullion, lender, and custodian do not sit inside one organisation.

---

## 3. Scope and non-goals

### 3.1 In scope

This reference architecture covers:

- institutional and business bullion holdings;
- gold first, with silver and other precious metals as later instrument adapters;
- allocated bars held with professional custodians;
- account-based and certificate-based holdings as classified future profiles;
- revolving and term credit facilities;
- bar-level eligibility and whole-bar operations;
- dynamic valuation, haircut, advance rate, and margin state;
- tri-party control and multi-party authorisation;
- repayment and release;
- substitution and cross-vault movement;
- enforcement readiness and evidence;
- adapters to bank, custody, market-data, settlement, and document systems;
- future repo and lease profiles using distinct lifecycle logic.

### 3.2 Explicit non-goals

Argent does not:

1. buy or sell bullion as principal;
2. custody physical metal;
3. issue credit or take borrower credit risk;
4. issue gold certificates, account balances, or tokens;
5. determine that a legal security interest is valid;
6. replace a bank's lending, collateral, treasury, accounting, or general-ledger systems;
7. replace a custodian's authoritative inventory and custody books;
8. replace KYC, AML, sanctions, responsible-sourcing, insurance, assay, audit, or legal review;
9. execute physical delivery or liquidation;
10. assume that a digital record conveys title;
11. treat all gold representations as equivalent;
12. allow the operator to authorise borrower, bank, or custodian actions on their behalf.

The architecture records and governs the **authorised control state**. Legal rights remain grounded in the relevant agreement, account terms, custody record, and governing law.

---

## 4. Design principles

### 4.1 Representation before valuation

A weight and price are not enough to calculate bankable collateral. The system must know what the holder owns or is owed.

One kilogram may represent:

- a specific allocated bar;
- a beneficial interest in a pool;
- an unallocated account claim against a bank;
- a certificate governed by issuer terms;
- a metal loan receivable;
- a token issued against vaulted metal;
- metal subject to an existing lien or delivery obligation.

The same quoted metal value can therefore produce materially different eligible values.

### 4.2 Control without false consolidation

Argent maintains a shared control record but does not declare itself the sole source of truth for every domain.

There are three authoritative domains:

| Domain | Authoritative system | Examples |
|---|---|---|
| Physical and custody truth | Custodian, vault, account provider | bar list, allocation, location, immobilisation, release |
| Credit and legal truth | Lender and legal documents | facility limit, exposure, security rights, covenants, default |
| Shared control truth | Argent | accepted evidence, authorised transitions, cross-party sequence, evidence hashes |

A production design reconciles those truths. It does not overwrite one with another.

### 4.3 Product profiles, not one universal gold workflow

Secured credit, certificate collateral, repo, and lease require different lifecycle and accounting semantics. Shared building blocks should be reused, but legal differences must remain explicit.

### 4.4 Minimum shared data

Sensitive data stays off-chain and, where possible, outside Argent's shared operational store. The shared layer should contain:

- opaque identifiers;
- role addresses and authority references;
- commitments to documents and evidence;
- quantities and policy outputs needed for control;
- lifecycle state;
- event sequence and timestamps;
- transaction and reconciliation references.

It should not contain client identity documents, full bar lists, vault addresses, personal data, account credentials, or complete legal agreements.

### 4.5 Evidence before automation

No workflow should become automatic merely because it can be automated. A transition is eligible for straight-through processing only when:

- the relevant party is authenticated;
- the input is current and attributable;
- the applicable policy version is known;
- the legal and operational prerequisites are established;
- failure and exception handling are agreed;
- the transition can be reconciled with the authoritative external system.

### 4.6 No temporary collateral gaps

A substitution, release, or movement must not produce a moment in which the lender assumes coverage that does not exist. Delivery-versus-release logic, pre-positioning, or explicit temporary haircuts are required.

### 4.7 Policy is attributable and versioned

Eligibility, haircut, advance rate, concentration, valuation, margin, cure, and enforcement decisions must be tied to the policy version under which they were made. A changed policy does not silently rewrite the historical basis of earlier actions.

### 4.8 Open-system first

The architecture must work when:

- the lender did not sell the bullion;
- the bullion is held by an independent custodian;
- holdings span vaults or jurisdictions;
- a borrower has more than one lender;
- a custodian serves more than one bank;
- the bank uses legacy or manual interfaces;
- DLT is not used by every participant.

This is the defensible problem. A closed bank product can always solve more of the workflow internally.

---

## 5. Bullion representation taxonomy

The architecture classifies each holding along three separate dimensions:

1. **holding representation:** what record describes the client's interest;
2. **rights profile:** what the client legally owns or is owed;
3. **transaction profile:** what financing structure is being applied.

### 5.1 Holding representations

| Representation | Typical record | Core risk | Argent treatment |
|---|---|---|---|
| Allocated bar holding | bar list and allocated account | bar identity, title, custody, prior lien | First production profile |
| Pooled allocated interest | units or percentage of a segregated pool | beneficial ownership, pool rules, allocation and redemption | Future rights adapter |
| Unallocated account | XAU balance | account-provider credit exposure; no specific bar | Separate account-claim profile |
| Bank certificate | certificate or digital confirmation | issuer terms, transferability, consent, underlying allocation | Certificate adapter only after legal classification |
| Branded physical bar in custody | certificate plus custody record | specific ownership, storage, delivery and pledge control | Can map to allocated profile if bar-specific evidence exists |
| Tokenised allocated gold | token plus issuer/custody/redemption framework | issuer, token control, custody, redemption and technology | External representation adapter; not required for core |
| Metal receivable or lease position | contract and account record | counterparty delivery and return obligation | Lease profile, not bar pledge |
| Repo position | sale and repurchase confirmations | title transfer, margin, repurchase and close-out | Repo profile, not pledge profile |

### 5.2 Rights profiles

A `RightsProfile` should state, at minimum:

```text
legal holder
beneficial owner
recorded owner, if different
issuer or account provider
custodian or depository
specific-bar ownership: yes / no
segregation: specific / pooled / unallocated
counterparty claim: yes / no
transferable: yes / no / consent required
assignable: yes / no / consent required
pledgeable: yes / no / consent required
redeemable: cash / physical / both / neither
redemption denomination and minimum
normal-course title-transfer permitted: yes / no
existing encumbrance status
applicable governing documents
jurisdiction and governing law references
```

The product should fail closed if those fields are unknown. “Backed by gold” is not a sufficient eligibility statement.

### 5.3 Why the taxonomy matters

LBMA distinguishes allocated accounts, where specific bars are identified on a weight list, from unallocated accounts, where the account holder has a contractual claim against the account provider [7], [8]. The World Gold Council and Linklaters similarly describe the trade-off: allocated ownership provides direct title certainty but introduces whole-bar and operational complexity, while unallocated structures provide easier settlement but create a claim against the institution [10].

The architecture preserves this distinction at the data-model level so a product manager cannot accidentally apply allocated-bar controls to an unallocated claim.

---

## 6. Actors and responsibilities

| Actor | Authoritative responsibility | Argent interaction |
|---|---|---|
| Borrower / owner | ownership evidence, facility obligations, use of proceeds, repayment, adjustment requests | authorises pledge, draw-related instructions where applicable, repayment, top-up, substitution, release request |
| Lender / secured party | underwriting, facility, eligibility, advance rate, margin, release, default and enforcement decisions | approves framework, admits collateral treatment, opens line, records or accepts exposure, controls release and enforcement |
| Custodian / account provider | inventory or account truth, allocation, segregation, location, immobilisation, release and movement | attests holding, acknowledges control, freezes or earmarks, confirms adjustments, releases and enforcement outcome |
| Bullion dealer / liquidation agent | executable purchase or transfer route, settlement and market access | provides quote or execution evidence; receives authorised sale instruction |
| Valuation provider | reference price, timestamp, confidence, fallback data | supplies signed or attributable valuation observation |
| Settlement bank / payment system | cash movement and finality | confirms draw funding, repayment, fees, liquidation proceeds and surplus |
| Refiner / assay or verifier | bar standard, quality, assay, responsible-sourcing evidence | supplies evidence used by eligibility adapter |
| Insurer | insured location, coverage scope, limits and exclusions | supplies coverage evidence and change notifications |
| Argent operator | workflow construction, policy evaluation, indexing, evidence rendering, integration operations | cannot authorise another party's business decision |
| Auditor / regulator / risk reviewer | independent review | receives read-only state, evidence and reconciliation reports subject to permission |

The initial design-partner model requires at least borrower, lender, and custodian. A real production launch also needs an identified valuation source, settlement route, and enforcement counterparty.

---

## 7. Legal and control architecture

### 7.1 The legal perimeter

Before any collateral is activated, the parties establish a `ControlFramework`. In the current reference contracts, it anchors commitments to:

- facility agreement;
- pledge or security agreement;
- custody-control agreement;
- eligible-collateral schedule;
- margin policy;
- enforcement waterfall.

The framework does not create those documents. It states which documents govern the digital lifecycle and which parties accepted that perimeter.

### 7.2 Control is a two-step fact

For allocated bullion, effective operational control requires both:

1. **owner designation:** the owner instructs that identified bars be committed to the facility; and
2. **custodian immobilisation:** the custodian confirms that the bars are blocked from unilateral withdrawal, transfer, substitution, or re-use.

The lender then activates the pledge or opens credit under the agreed legal framework.

A self-declared on-chain pledge without custodian acknowledgement is not sufficient.

### 7.3 Consent and product terms

Certificate and account-based holdings require an additional issuer or account-provider control. Emirates NBD's product terms illustrate why: a digital confirmation may be non-transferable and non-pledgeable to third parties without prior bank consent [5].

A certificate adapter must therefore support:

```text
issuer identifies certificate and holder
issuer confirms governing product version
issuer confirms whether assignment or pledge is permitted
issuer records any required consent
issuer blocks transfer, redemption, or duplicate encumbrance
lender accepts the right as collateral
issuer confirms release or enforcement outcome
```

The adapter cannot infer pledgeability from the existence of a certificate.

### 7.4 No hidden rehypothecation assumption

The allocated-pledge profile assumes that pledged bars remain allocated and are not used, transferred, leased, or re-pledged during the normal life of the facility unless the governing documents and lender policy explicitly allow it.

If collateral reuse is permitted, it must be represented as a different rights and transaction profile with:

- explicit consent;
- reuse authority;
- resulting claim against the reuser;
- substitution and return obligations;
- reporting and reconciliation requirements;
- insolvency and close-out analysis.

Argent must not describe reused metal as continuously immobilised allocated collateral.

---

## 8. High-level system architecture

```text
                                CLIENT AND PARTNER CHANNELS
             borrower portal · bank cockpit · custodian console · auditor view
                                             |
                                             v
                                ARGENT APPLICATION LAYER
       onboarding · holding classifier · evidence pack · facility workflow · exceptions
                                             |
          +----------------------+-----------+-----------+----------------------+
          |                      |                       |                      |
          v                      v                       v                      v
  POLICY / RIGHTS ENGINE   WORKFLOW ORCHESTRATOR   RECONCILIATION ENGINE   EVIDENCE SERVICE
  representation rules    action construction      three-way comparison   certificates/reports
  eligibility treatment   approvals and quorum     breaks and ageing       document commitments
  valuation and margins   retries/idempotency      exception resolution    export and verification
          |                      |                       |                      |
          +----------------------+-----------+-----------+----------------------+
                                             |
                                             v
                               AUTHORISATION AND SIGNING
            institutional role wallets · deny-by-default policy · quorum · audit trail
                                             |
                                             v
                                ARGENT CONTROL LEDGER
       framework · instrument · position · custody control · pledge · line · valuation
       draw · repayment · margin · adjustment · release · default · enforcement evidence
                                             |
          +----------------------+-----------+-----------+----------------------+
          |                      |                       |                      |
          v                      v                       v                      v
  CUSTODY ADAPTERS       BANK CREDIT ADAPTERS     MARKET / FX ADAPTERS    SETTLEMENT ADAPTERS
  bar lists/accounts     limits/exposure/status   prices/confidence       cash/token movement
  freeze/release         draw/repayment refs      calendars/fallback      proceeds/waterfall
          |                      |                       |                      |
          +----------------------+-----------+-----------+----------------------+
                                             |
                                             v
                              EXTERNAL AUTHORITATIVE SYSTEMS
 vault/custody books · loan systems · collateral systems · treasury · payments · documents
```

The on-chain contract is one component. The commercial product is the combined architecture: classification, policy, connectors, authorisation, reconciliation, and evidence.

---

## 9. Canonical domain model

The domain model is deliberately separated into reusable objects. Product profiles compose these objects differently.

### 9.1 `InstrumentDefinition`

Defines the economic standard of the metal rather than a specific holding.

```text
instrument_id
metal: gold / silver / platinum / palladium
unit: gram / kilogram / troy ounce
format: kilobar / London Good Delivery / minted bar / other
minimum_fineness
approved_refiner_scheme
quality_standard_hash
version
issuer_of_standard
depository or custody class
status
```

The current Soroban core already represents this separation through `Instrument` and `InstrumentKey`. The LBMA/SGE kilobar specification is an example of why a versioned instrument standard matters: it defines the critical marks and traceability expectations for four-nines kilobars rather than leaving format and identity to free text [9].

### 9.2 `HoldingRepresentation`

Classifies the form in which the client holds the metal.

```text
representation_id
type: allocated_bar / pooled_interest / unallocated_account /
      certificate / token / lease_receivable / repo_position
provider_id
account_or_certificate_reference_hash
rights_profile_id
quantity
unit
currency_or_metal_code
status
```

This is the principal extension required to move from an allocated-only reference implementation to a broader bullion control plane.

### 9.3 `RightsProfile`

Records the lender-reviewed legal and operational interpretation of the holding. It should be versioned and attributable to the legal review or product terms on which it relies.

```text
rights_profile_id
representation_type
specific_title
beneficial_ownership
counterparty_claim
segregation_type
transfer_rule
pledge_rule
issuer_consent_rule
redemption_rule
reuse_rule
governing_document_hashes
legal_opinion_or_review_hash
valid_from / valid_until
status
```

The shared ledger may store only hashes and compressed policy outcomes; the full interpretation remains off-chain.

### 9.4 `LotEvidence`

The current core already models the evidence bundle for a specific physical lot:

- manifest or bar-list commitment;
- uniqueness commitment;
- quality or assay certificate;
- quantity or weight certificate;
- location commitment.

For gold, the off-chain manifest should contain at least:

```text
refiner or brand
bar serial
format
gross weight
fineness
fine weight
custody account
vault or location code
allocation status
encumbrance status
source timestamp
```

LBMA's allocated-account description uses the same essential bar-list fields: unique number, gross weight, assay or fineness, and fine weight for gold [7].

### 9.5 `Position`

A position binds a holding to an owner, custodian, framework, instrument, quantity, and evidence set. The current `VaultPosition` is the allocated-bar implementation.

A future generic position model should add or reference:

```text
holding_representation_id
rights_profile_id
provider_account_reference_hash
control_capability: immobilise / redemption_block / transfer_block / none
attestation_expiry
```

### 9.6 `EligibilityTreatment`

The lender-approved treatment for one instrument and rights profile under one facility:

```text
eligibility_id
framework_id
instrument_id
rights_profile_id
eligible: yes / no / conditional
haircut_bps
max_advance_bps
maintenance_bps
concentration_limit
minimum_lot or denomination
approved_locations
approved custodians
approved refiners
maturity or expiry constraints
policy_hash
status
```

The current `FrameworkInstrumentEligibility` already records the core haircut, maximum LTV, maintenance threshold, and eligibility commitment. FINOS CDM's eligible-collateral representation provides an industry-aligned model for expressing collateral criteria and treatment in machine-readable form [19]. Argent should map to it where practical rather than inventing a closed vocabulary.

### 9.7 `CustodyControl`

Records the custodian's operational commitment:

```text
position_id
control_type: earmark / freeze / transfer_block / redemption_block
control_reference_hash
confirmed_by
confirmed_at
valid_until
release_authority
substitution_authority
movement_authority
status
```

For allocated bars, this is immobilisation. For certificates, it may be redemption and transfer blocking. For account claims, it may be an account control or assignment acknowledgement.

### 9.8 `Facility`, `Pledge`, and `CreditLine`

These remain separate:

- the `Facility` establishes the contractual and participant perimeter;
- the `Pledge` binds collateral control to the secured party;
- the `CreditLine` records approved limit, drawn exposure, availability, status, and margin state.

One facility may eventually have multiple positions, instruments, custodians, or lines. The first design-partner profile may remain one pool and one line for simplicity.

### 9.9 `ValuationObservation` and `BorrowingBaseSnapshot`

A valuation observation records external data. A borrowing-base snapshot records the lender-policy result.

```text
ValuationObservation
  instrument_id
  source
  price
  quote_currency
  confidence_or_spread
  published_at
  received_at
  fallback_status
  source_reference

BorrowingBaseSnapshot
  facility_id
  valuation_observation_id
  eligible_quantity
  gross_value
  haircut_value
  concentration_adjustments
  eligibility_reserves
  net_eligible_value
  advance_rate
  borrowing_base
  drawn_exposure
  available_capacity
  margin_state
  policy_version
```

Keeping these objects separate prevents the system from presenting an external market price as if it were itself a credit decision.

### 9.10 `Adjustment`

The current core supports top-up, substitution, and partial release as a first-class state machine. The generic object should carry:

- old schedule commitment;
- proposed new schedule commitment;
- reason and type;
- pre-change and post-change borrowing-base calculations;
- custodian confirmation;
- lender approval;
- completion evidence;
- failure or expiry state.

### 9.11 `EnforcementReadiness`

A facility must not claim to be enforceable merely because default functions exist. The current core's `EnforcementReadiness` is the correct pattern: readiness remains incomplete until the liquidation agent, realisation route, settlement asset, valuation source, waterfall, and validity period are populated.

---

## 10. Product profiles

### 10.1 Profile A — allocated-bullion secured credit

**Status:** first production profile; supported by the current control core.

```text
owner designates identified bars
→ custodian verifies and immobilises them
→ lender accepts eligibility and pledge
→ line opens against calculated borrowing base
→ utilisation and repayment change exposure
→ revaluation changes availability and margin state
→ whole-bar top-up, substitution, or release follows controlled workflow
→ default leads to agreed sale, appropriation, or transfer path
```

Normal-course title remains with the owner, subject to the security interest. The bars remain allocated and custodied. The system rejects unilateral release.

### 10.2 Profile B — issuer-controlled certificate collateral

**Status:** next logical representation adapter; not a default extension of allocated-bar logic.

Prerequisites:

- certificate issuer and underlying metal provider identified;
- underlying allocation model known;
- certificate holder and legal rights verified;
- assignment or pledge rule confirmed;
- required issuer consent obtained;
- transfer and redemption blocked while pledged;
- duplicate pledge prevented;
- enforcement and redemption route agreed.

The certificate may represent an unallocated claim, pooled interest, or specific bar entitlement. Its eligibility treatment should depend on the rights profile, not on the word “certificate.”

### 10.3 Profile C — unallocated account claim

**Status:** possible later product; materially different credit risk.

The collateral is a claim against the account provider rather than title to specific bars [7], [8]. The lender must underwrite:

- account-provider credit risk;
- set-off and insolvency treatment;
- assignment or account-control enforceability;
- withdrawal and transfer blocking;
- valuation and settlement mechanics;
- concentration to the account provider.

Argent could control the claim, but it must not display it as allocated metal.

### 10.4 Profile D — gold repo

**Status:** future transaction profile requiring a separate lifecycle model.

```text
trade agreed
→ seller transfers title or account interest
→ buyer transfers cash
→ collateral is valued and margined
→ substitutions or margin transfers occur
→ seller repurchases at maturity or transaction closes out
```

A repo is not simply a pledge with different labels. Product data must include purchase price, repurchase price or rate, purchase date, repurchase date, margin terms, income treatment, substitution rights, events of default, and close-out. FINOS CDM's repo representation is the preferred interoperability reference [20].

### 10.5 Profile E — gold lease or metal loan

**Status:** future transaction profile requiring quantity and return-obligation accounting.

```text
lender delivers or credits metal
→ borrower uses, fabricates, hedges, or sells it as permitted
→ lease rate accrues
→ margin or other credit support is monitored
→ equivalent metal is returned at maturity
```

LBMA states that precious metals may be deposited, borrowed, leased, or lent on allocated or unallocated terms [11], [12]. A lease therefore requires fields that the pledge profile does not have: metal principal, delivery location, lease rate, return date, permitted use, equivalent-metal standard, and return settlement.

### 10.6 Profile F — pooled or tokenised beneficial interest

**Status:** external or future rights profile; not required for Argent's first market.

The World Gold Council's Gold247 work and the Euroclear/Digital Asset pilot show the market's interest in fractional, transferable interests and collateral mobility [10], [17]. DBS's physical gold token similarly describes fractional ownership backed by allocated gold in independent vaults [16].

Argent's position is not that such structures are invalid. It is that banks should have a control-plane option when they do not want to issue or rely on a transferable token. If a tokenised position is accepted, Argent should integrate through a rights adapter and record the issuer, custody, token-control, redemption, and legal framework rather than treating the token balance as self-proving collateral.

---

## 11. Lifecycle state machines

### 11.1 Holding activation

```text
DISCOVERED
  external holding located but not classified
      |
      v
CLASSIFIED
  representation and rights profile established
      |
      v
EVIDENCED
  required ownership, custody, quality, quantity and location evidence complete
      |
      v
ELIGIBLE
  lender treatment assigned under a policy version
      |
      v
CONTROL_PENDING
  owner instruction accepted; provider control requested
      |
      v
CONTROLLED
  custodian or issuer confirms immobilisation / transfer block / redemption block
      |
      v
PLEDGED
  lender activates collateral under the facility
```

Failure states include `REJECTED`, `EXPIRED`, `DISPUTED`, and `CONTROL_LOST`.

### 11.2 Credit line

```text
PROPOSED → APPROVED → ACTIVE → SUSPENDED → ACTIVE
                         |          |
                         |          +→ DEFAULTED
                         +→ CLOSED
```

Draw availability depends on both line status and current borrowing capacity.

### 11.3 Margin

```text
COVERED
  → WARNING
  → CALLED
  → CURED
  → COVERED

CALLED
  → UNCURED
  → DEFAULT / ENFORCEMENT DECISION
```

A margin call does not itself transfer title. It blocks draws and releases, records the cure requirement and deadline, and exposes agreed cure options.

### 11.4 Adjustment

```text
REQUESTED
  → CUSTODIAN_CONFIRMED
  → LENDER_APPROVED
  → EXECUTING
  → COMPLETED

Any pre-completion state
  → REJECTED / EXPIRED / CANCELLED
```

No new schedule becomes authoritative until completion is confirmed and reconciled.

### 11.5 Cross-vault movement

```text
MOVEMENT_PROPOSED
→ SOURCE_FROZEN
→ DISPATCHED
→ IN_TRANSIT
→ DESTINATION_RECEIVED
→ REATTESTED
→ ELIGIBLE_AT_DESTINATION
→ CONTROL_REESTABLISHED
```

While in transit, the lender may apply a zero value, special transit haircut, insurance-based value, or another approved treatment. The system should not assume normal vault eligibility continues.

### 11.6 Release

```text
RELEASE_REQUESTED
→ COVERAGE_TESTED
→ LENDER_AUTHORISED
→ CUSTODIAN_RELEASE_PENDING
→ CUSTODIAN_CONFIRMED
→ RELEASED
```

The lender's release authorisation and the custodian's physical confirmation remain separate acts.

### 11.7 Enforcement

```text
DEFAULT_DECLARED
→ CONTROL_RECONFIRMED
→ FINAL_EXPOSURE_AND_VALUATION
→ ENFORCEMENT_AUTHORISED
→ DEALER_OR_TRANSFER_ROUTE_EXECUTED
→ PROCEEDS_CONFIRMED
→ WATERFALL_APPLIED
→ SURPLUS_OR_SHORTFALL_RECORDED
→ CLOSED
```

An enforcement record mirrors the lawful outcome; it does not itself move metal or convey title.

---

## 12. Core product modules

### 12.1 Holding discovery and activation

This should be the centrepiece of the next demonstrator.

The module imports an existing holding from a bank, custodian, certificate issuer, or client statement and guides it through:

1. provider identification;
2. representation classification;
3. rights-profile determination;
4. evidence collection;
5. lender eligibility screening;
6. control request;
7. provider acknowledgement;
8. initial borrowing-base calculation;
9. lender approval and facility activation.

The output is not merely an uploaded bar list. It is an eligibility decision and a controlled position.

### 12.2 Rights and eligibility engine

The engine combines:

```text
holding representation
+ rights profile
+ instrument and bar attributes
+ custodian / issuer attributes
+ jurisdiction and location
+ facility policy
= eligibility treatment
```

Its output must be explainable. A rejection should identify whether the cause is:

- representation not accepted;
- pledgeability unknown;
- consent missing;
- unapproved custodian, refiner, format, location, or jurisdiction;
- stale evidence;
- prior encumbrance;
- minimum denomination or concentration breach;
- sanctions or compliance block;
- unavailable enforcement route.

### 12.3 Custody-control module

The custody adapter should support at least five operations:

```text
attest holding
place control / immobilise
confirm current controlled state
apply approved adjustment
release or enforce under authorised instruction
```

For institutions without APIs, a controlled manual console with four-eyes approval and signed evidence is an acceptable first adapter. The interface can evolve from manual to file to API without changing the control model.

### 12.4 Borrowing-base and margin engine

The engine recalculates capacity whenever a relevant input changes:

- price;
- FX rate;
- quantity;
- bar eligibility;
- custody-control state;
- concentration;
- haircut or policy;
- utilisation;
- reserve or fee treatment.

It produces a snapshot rather than mutating an unexplained number.

### 12.5 Whole-bar allocator

Allocated bullion is discrete. The allocator maps a credit requirement to actual bars and determines which bars may be released or substituted.

Selection objectives may include:

- minimum number of bars;
- lowest excess collateral;
- preferred refiner or location;
- concentration limits;
- oldest or newest inventory;
- delivery denomination;
- lowest transfer cost;
- preserving a minimum liquidity buffer.

The algorithm proposes a schedule. The bank and custodian still authorise it.

### 12.6 Reconciliation and exception management

At minimum, the system compares:

```text
custodian controlled quantity and schedule
vs
Argent controlled position and lifecycle state
vs
lender collateral and exposure record
```

Break types include:

- quantity mismatch;
- bar-list mismatch;
- state mismatch;
- stale attestation;
- missing transaction acknowledgement;
- duplicate or unknown external reference;
- valuation divergence;
- exposure mismatch;
- settlement not final;
- release or movement not confirmed.

Every break requires an owner, age, severity, and resolution record.

### 12.7 Evidence service

The service renders human-readable and machine-verifiable records for:

- framework establishment;
- holding classification;
- eligibility decision;
- immobilisation;
- pledge activation;
- line opening;
- borrowing-base snapshot;
- draw and repayment;
- margin and cure;
- substitution or release;
- enforcement readiness;
- final release or enforcement.

The evidence package should state what is proven, what source was relied upon, and what is explicitly outside the proof.

---

## 13. Integration architecture

### 13.1 Adapter principle

Banks should not be required to replace systems that already perform their authoritative function. Argent adapters translate between external system facts and canonical control events.

An adapter has four responsibilities:

1. authenticate the source;
2. map the external message to the canonical model;
3. validate completeness, freshness, sequence, and idempotency;
4. return an acknowledgement or exception to the source system.

### 13.2 Custody adapter

**Inbound facts**

- account or holding snapshot;
- bar list or position statement;
- allocation and segregation status;
- location and movement status;
- control or immobilisation acknowledgement;
- adjustment completion;
- release or enforcement completion.

**Outbound instructions**

- request attestation;
- request freeze or earmark;
- request substitution or movement;
- lender-authorised release;
- enforcement instruction reference.

**Supported channels**

- API and webhook;
- signed JSON or XML;
- SFTP file;
- custody report ingestion;
- controlled manual dual-entry for pilot use.

### 13.3 Bank credit adapter

**Inbound facts**

- facility approval;
- approved limit and policy;
- draw or utilisation confirmation;
- repayment and fee application;
- suspension, default, cure, or closure;
- exposure statement.

**Outbound facts**

- eligible value and available capacity;
- margin and evidence status;
- collateral exceptions;
- release or substitution request package;
- control-state and audit report.

Argent should integrate with the bank's existing lending or collateral system rather than become the bank's accounting subledger.

### 13.4 Market-data adapter

Required capabilities:

- approved instrument identifier;
- price and quote currency;
- publication and receipt timestamps;
- confidence, bid/offer, or source-quality measure;
- market calendar;
- FX rate where required;
- stale-price rejection;
- fallback hierarchy;
- manual override with named authority and reason.

The lender owns the valuation policy. Argent applies and evidences it.

### 13.5 Settlement adapter

The reference implementation can atomically bind a recognised Stellar settlement asset transfer to exposure reduction through `settlement_vault`. Production deployments may also use conventional payment rails.

The adapter should represent:

- payment instruction reference;
- payer and beneficiary account aliases;
- amount and currency;
- value date;
- settlement status;
- finality reference;
- reversal or return status;
- allocation to principal, interest, fees, costs, and surplus.

A pending payment must not reduce exposure as if it were final.

### 13.6 Dealer and enforcement adapter

The adapter connects an authorised enforcement decision to an approved realisation route:

- request for executable quote;
- quote expiry and terms;
- bar schedule;
- transfer or delivery instruction;
- sale confirmation;
- settlement amount;
- fees and deductions;
- final proceeds and waterfall evidence.

The first pilot may use a tabletop or controlled manual process, but the counterparty and route must be named before the facility claims enforcement readiness.

### 13.7 Document and evidence adapter

The system should integrate with the bank or partner document repository. Argent stores:

- document identifier;
- content hash;
- document type;
- version;
- effective and expiry dates;
- signer or issuer reference;
- access-control pointer.

The document itself remains in the approved repository.

### 13.8 Canonical event envelope

Every cross-system event should carry a common envelope:

```text
event_id
framework_id
entity_type and entity_id
event_type
source_system
source_reference
actor and role
effective_time
received_time
sequence or version
policy_version
document or evidence commitments
idempotency_key
correlation_id
result: accepted / rejected / pending / exception
reason_code
```

This envelope aligns with the existing `CollateralEventV1` objective: a stable, replayable, per-framework event stream.

---

## 14. Valuation, borrowing base, and risk treatment

### 14.1 Calculation sequence

```text
gross market value
= verified eligible quantity × approved reference price × approved FX rate

haircut-adjusted value
= gross market value × (1 − haircut)

concentration-adjusted value
= haircut-adjusted value − concentration and eligibility reserves

borrowing base
= concentration-adjusted value × approved advance rate

available capacity
= min(contractual facility limit, borrowing base)
  − drawn exposure
  − lender reserves
```

A lender may combine haircut and advance rate differently. Argent should preserve the lender's formulation and disclose each component rather than force one universal formula.

### 14.2 Inputs that can reduce eligible value to zero

- control acknowledgement expired or withdrawn;
- bar or holding cannot be reconciled;
- title or pledgeability disputed;
- custodian, issuer, refiner, location, or representation no longer approved;
- sanctions or legal restriction;
- insurance lapse where policy requires coverage;
- price unavailable beyond fallback tolerance;
- bar in transit without approved transit treatment;
- certificate redemption or transfer block not effective;
- account provider default or credit downgrade where relevant;
- evidence age exceeds policy.

### 14.3 Margin and cure

The policy should distinguish:

- warning threshold;
- draw-suspension threshold;
- margin-call threshold;
- enforcement threshold;
- cure deadline;
- permitted cure methods;
- severe-event override.

Permitted cure methods may include:

- cash repayment;
- additional eligible bars;
- approved substitution;
- cash collateral;
- lender-approved facility reduction;
- other agreed credit support.

### 14.4 Prudential treatment

Basel CRE22 includes gold among recognised forms of eligible financial collateral under specified credit-risk-mitigation methods, subject to legal certainty, valuation, haircuts, and the applicable prudential framework [18]. This does not mean every gold-secured exposure receives the same regulatory benefit. The bank must determine local implementation, exposure class, approach, collateral eligibility, haircut, currency mismatch, maturity mismatch, and operational requirements.

Argent should expose the data and evidence needed for that determination. It should not calculate regulatory capital unless separately implemented and approved by the lender.

---

## 15. Whole-bar operations

The World Gold Council and Linklaters identify the central trade-off of allocated gold: direct ownership certainty comes with operational complexity and the inability to transact in arbitrary fractions of a bar [10]. That constraint should be visible in the product, not hidden behind a smooth interface.

### 15.1 Partial repayment

A repayment reduces exposure immediately when settlement is final. It does not automatically release a fraction of a bar.

### 15.2 Partial release

A release proposal must select complete bars and demonstrate that the remaining pool satisfies:

- current borrowing-base coverage;
- required buffer;
- refiner, location, and concentration rules;
- minimum-lot rules;
- current custody evidence;
- no unresolved margin or default;
- lender and custodian approval.

### 15.3 Substitution

The safe sequence is:

```text
replacement bars identified
→ replacement eligibility verified
→ custodian controls replacement bars
→ aggregate pre-release coverage confirmed
→ lender approves substitution
→ outgoing bars released
→ final schedules reconciled
```

The outgoing bars should not be released first unless an explicit temporary collateral arrangement covers the gap.

### 15.4 Pool design

A production facility should generally use a collateral pool rather than one monolithic position. Each bar or lot remains identifiable, while the borrowing base is calculated at pool level. This supports:

- repeated top-ups and releases;
- concentration control;
- multiple vaults;
- replacement of ineligible bars;
- efficient release selection;
- partial facility reduction.

The current core can evolve from one `VaultPosition` schedule to a pool abstraction without discarding its instrument, evidence, eligibility, adjustment, and event models.

---

## 16. Cross-vault and cross-border mobility

Emirates NBD's wholesale service includes domestic and international vaulting, international delivery, and cross-border bullion operations [1], [2]. UOB's physical bullion offering similarly demonstrates that bank-operated bullion products depend on defined bar formats, buy/sell quotes, physical conversion, and operational procedures [14].

Cross-vault mobility is therefore a later but necessary module.

### 16.1 Movement package

A movement request should include:

- source and destination custodian;
- source and destination account or vault aliases;
- bar schedule;
- transport provider;
- insurance evidence;
- customs, export, import, tax, and responsible-sourcing documents where applicable;
- departure and expected arrival;
- valuation and haircut treatment during transit;
- destination acceptance standard;
- re-assay or inspection rule;
- lender authorisation;
- failure and return route.

### 16.2 Eligibility during transit

The system must record one of the lender-approved treatments:

```text
zero eligible value
reduced transit value
insured value subject to policy limit
continuous eligibility under named control arrangement
```

Silence is not a treatment.

### 16.3 Re-attestation

Arrival alone does not restore eligibility. The destination custodian must confirm receipt, identity, condition, allocation, location, and control. Any assay or weight discrepancy becomes an exception before the borrowing base is restored.

---

## 17. Evidence and audit architecture

### 17.1 Evidence classes

**Identity and rights**

- account, certificate, or position reference;
- rights profile and governing terms;
- ownership or entitlement evidence;
- consent and pledgeability evidence;
- prior-lien or encumbrance evidence.

**Physical and custody**

- bar list;
- assay and weight;
- refiner and standard;
- location and allocation;
- immobilisation or control acknowledgement;
- insurance and audit evidence;
- movement and release confirmation.

**Credit and policy**

- facility and security-document commitments;
- eligible-collateral treatment;
- valuation source and policy;
- advance, margin, cure, and enforcement parameters;
- line approval and status.

**Operational**

- draw, repayment, fee, and settlement references;
- valuation and borrowing-base snapshots;
- margin notices and cures;
- adjustments and exceptions;
- release and enforcement actions.

### 17.2 Proof boundary

Every certificate should include a boundary statement. For example:

> This evidence confirms that the named custodian signed the specified immobilisation acknowledgement and that Argent recorded the corresponding authorised control transition. It does not independently prove physical existence, legal title, absence of undisclosed liens, or enforceability outside the referenced agreements.

This prevents cryptographic evidence from being presented as a substitute for real-world verification.

### 17.3 Replay and reconciliation

The event stream should be replayable into the current control state. The replayed state must reconcile with contract state and external-system snapshots. Divergence is an operational incident, not a cosmetic reporting issue.

---

## 18. Security, privacy, and trust boundaries

### 18.1 Threats addressed

The architecture reduces the risk of:

- unauthorised release;
- unilateral state changes;
- duplicate processing;
- stale valuation driving a margin decision;
- mismatched bank and custody records;
- undocumented substitution;
- hidden policy changes;
- missing enforcement prerequisites;
- after-the-fact rewriting of the control sequence.

### 18.2 Threats not eliminated

It does not eliminate:

- dishonest or compromised custodians;
- forged source documents accepted by authorised users;
- undisclosed off-system liens;
- legal invalidity;
- sanctions or fraud outside the recorded perimeter;
- market gaps and liquidation slippage;
- insolvency of a bank, custodian, issuer, dealer, or account provider;
- physical loss where insurance or operational controls fail;
- collusion among the parties whose signatures are required.

### 18.3 Separation of duties

Production roles should be separable within institutions:

- credit approval;
- collateral eligibility;
- release authority;
- default declaration;
- enforcement authority;
- custodian operations;
- operator and support access.

The same service account should not be able to approve a facility, change policy, release collateral, and record enforcement.

### 18.4 Data minimisation

On-chain data should avoid:

- personal names and identifiers;
- full account numbers;
- precise vault addresses;
- full bar serial lists where commercial sensitivity requires privacy;
- legal-document content;
- KYC or sanctions data;
- trading strategy and customer pricing.

Opaque commitments should remain linkable to authorised off-chain records.

### 18.5 Resilience

The product needs:

- idempotent commands;
- per-framework sequencing;
- retry-safe adapters;
- key and role rotation;
- approval expiry;
- stale-data cut-offs;
- disaster recovery for indexers and evidence stores;
- offline and manual continuity procedures;
- incident-response and reconciliation runbooks.

---

## 19. Deployment patterns

### 19.1 Bank-internal bullion activation

```text
bank bullion product + bank custody + bank lending
```

Argent links internal product silos and provides a shared event and evidence layer. Integration is simplest, but differentiation is lower because the bank already controls the perimeter.

### 19.2 Tri-party external custody

```text
borrower + independent custodian + lender
```

This is the recommended first commercial deployment. Argent provides the shared control state that no party's private system naturally owns.

### 19.3 Custodian-led multi-lender service

```text
one vault or custodian + several approved lenders
```

The custodian exposes standard attestation, control, adjustment, and release workflows through Argent. This can reduce bespoke lender integrations and make custody-held bullion more financeable.

### 19.4 Multi-custodian, multi-lender network

```text
borrower portfolio across custodians
+ several facilities
+ common rights, eligibility, and evidence layer
```

This is the long-term infrastructure opportunity. It requires lender-specific policy partitions, collateral-allocation rules, privacy controls, and strict prevention of duplicate encumbrance.

---

## 20. Mapping to the current Argent implementation

The current Soroban core already provides more of this architecture than the initial gold-card presentation makes visible.

| Reference-architecture concept | Current implementation |
|---|---|
| Legal and control perimeter | `ControlFramework` with six governing-document commitments |
| Instrument standard | `Instrument` and `InstrumentKey` |
| Lender eligibility treatment | `FrameworkInstrumentEligibility` |
| Allocated holding | `VaultPosition` |
| Lot evidence | `LotEvidence` fields embedded in position and adjustments |
| Owner designation | `select_lot_for_collateral` |
| Custodian control | `confirm_and_immobilize` and custody-control record |
| Pledge activation | `activate_pledge` |
| Credit facility | `open_credit_line`, draw, reversal, repayment, suspension and resume |
| Valuation and margin | `revalue_and_check`, `LineValuation`, `MarginState` |
| Whole-bar adjustments | `CollateralAdjustment`: top-up, substitution, partial release |
| Release | bank authorisation plus custodian confirmation |
| Default and cure | `issue_default_notice`, `cure_default` |
| Enforcement readiness | incomplete/ready/expired readiness record |
| Enforcement outcome | sold, appropriated, or transferred evidence |
| Settlement binding | `settlement_vault` atomic settlement-asset repayment |
| Audit stream | `CollateralEventV1` and per-framework sequence |

### 20.1 The principal missing conceptual object

The current core assumes a physical allocated position. The next architecture layer should introduce `HoldingRepresentation` and `RightsProfile` in the service and evidence model before adding new on-chain product states.

That allows Argent to ingest a bank gold account, certificate, or external custody statement and determine:

- whether it maps to the existing allocated profile;
- whether issuer consent or a different control is required;
- whether it is ineligible;
- whether a future product adapter is needed.

### 20.2 What should remain unchanged

The following should stay stable:

- multi-party role separation;
- document commitments;
- instrument / holding separation;
- lender-owned eligibility treatment;
- custodian-confirmed control;
- independent release acts;
- event-sourced evidence;
- operator non-authority;
- off-chain legal and physical execution boundary.

### 20.3 What should not be added prematurely

Do not add repo, lease, unallocated, certificate, and token state machines to the public contract merely because the market contains those products. First implement classification and the allocated-holding activation flow. New contract profiles should be added only with a design partner and reviewed governing documents.

---

## 21. Roadmap

### Phase 0 — Current reference core

**Objective:** demonstrate controlled credit against allocated physical collateral.

Already represented:

- framework and roles;
- instrument and holding;
- evidence commitments;
- immobilisation and pledge;
- line, draw, repayment, valuation, margin, adjustment, release, default, cure, and enforcement evidence.

### Phase 1 — Existing-holding activation

**Objective:** answer the bank's most important onboarding question: “How does a holding already in our vault, account, or client portfolio become eligible credit collateral?”

Deliverables:

1. `HoldingRepresentation` and `RightsProfile` schema;
2. import of a custodian bar list or account/certificate statement;
3. classification workflow;
4. eligibility explanation and rejection reasons;
5. ownership, control, and evidence checklist;
6. custodian control request and acknowledgement;
7. initial borrowing-base snapshot;
8. lender activation approval;
9. complete evidence pack.

This is the strongest immediate next step.

### Phase 2 — Production-relevant tri-party pilot

**Objective:** prove operational interoperability rather than only contract logic.

Deliverables:

- one borrower, custodian, and lender;
- one production-relevant custody adapter;
- one bank credit or collateral adapter;
- approved valuation feed and fallback;
- daily reconciliation;
- draw, repayment, margin, top-up, substitution, and release;
- tabletop enforcement route;
- operating model, support, incident, and audit procedures;
- measured implementation and servicing economics.

### Phase 3 — Certificate and account-control adapter

**Objective:** support bank-issued gold rights where product terms permit collateralisation.

Deliverables:

- issuer/product version registry;
- pledgeability and consent rules;
- transfer and redemption blocking;
- certificate/account control acknowledgement;
- counterparty and concentration treatment;
- release and enforcement workflow.

This phase should start with one named product and governing terms, not a generic “certificate” type.

### Phase 4 — Collateral pool and cross-vault mobility

**Objective:** operate multi-bar, multi-location collateral efficiently.

Deliverables:

- pool-level borrowing base;
- bar-selection optimiser;
- cross-vault movement state;
- transit treatment;
- destination re-attestation;
- multi-custodian reconciliation;
- lender-specific allocation and duplicate-encumbrance controls.

### Phase 5 — Repo and lease product profiles

**Objective:** extend shared bullion evidence and control into distinct wholesale transaction lifecycles.

Deliverables:

- CDM-aligned repo representation and events;
- purchase and repurchase settlement;
- repo margin and substitution;
- metal-loan principal and lease-rate accrual;
- delivery and equivalent-metal return;
- close-out and reporting adapters.

Repo and lease should remain separate modules even where they reuse instrument, holding, valuation, party, document, and settlement components.

### Phase 6 — Multi-bank bullion collateral network

**Objective:** let eligible bullion remain portable across approved custodians and lenders without requiring one bank to own the full product stack.

Deliverables:

- lender-policy partitions;
- privacy-preserving portfolio views;
- collateral allocation across facilities;
- custodian multi-lender interface;
- portable evidence packages;
- common adapter certification;
- network governance and dispute rules.

---

## 22. First design-partner reference implementation

The first design partner should not be asked to adopt the entire roadmap. The reference implementation should be deliberately narrow.

### 22.1 Ideal partner profile

A bank or secured lender that:

- already serves customers holding physical bullion;
- has an approved custodian or vault relationship;
- does not yet have a scalable digital credit-control workflow for those holdings;
- is willing to define one facility policy and one enforcement route;
- can provide credit and operations stakeholders;
- sees value in external-custody or multi-party interoperability.

An equally credible lead partner is a vault operator or bullion custodian that wants to make holdings financeable across multiple lenders.

### 22.2 Reference facility

Use the existing design-partner brief:

- one borrower;
- eight allocated one-kilogram bars;
- one approved custodian;
- one revolving credit line;
- lender-defined advance and margin policy;
- daily or event-driven valuation;
- one partial repayment;
- one whole-bar substitution or release;
- one simulated margin cure;
- one tabletop enforcement test.

### 22.3 Reference workflow

```text
1. Import existing bar list
2. Classify as allocated specific-bar ownership
3. Validate rights and evidence
4. Apply lender eligibility policy
5. Request owner designation
6. Request custodian immobilisation
7. Reconcile controlled schedule
8. Calculate opening borrowing base
9. Lender activates pledge and line
10. Record draw and settlement reference
11. Revalue and monitor margin
12. Repay and execute whole-bar adjustment
13. Release or perform enforcement tabletop
14. Export final evidence and reconciliation report
```

### 22.4 Pilot acceptance criteria

The pilot should not be judged only by transaction success. It succeeds when:

1. the bank and custodian agree the representation and rights classification;
2. the control acknowledgement is legally and operationally usable;
3. all parties reconcile the same controlled bar schedule;
4. the bank can explain every eligibility and borrowing-base output;
5. stale or conflicting evidence blocks the relevant action;
6. draw and repayment references reconcile with the bank's cash records;
7. whole-bar substitution or release completes without a coverage gap;
8. a margin cure works under the approved policy;
9. the enforcement counterparty and cash waterfall are identified and tested;
10. operations can process exceptions and recover from adapter failure;
11. the bank can quantify yield, capital treatment, operating cost, expected loss, and Argent fees;
12. the partners can decide whether the model is replicable for a second borrower or custodian.

---

## 23. What not to build next

The market evidence can tempt Argent to expand too quickly. The following are not the strongest immediate step:

- a consumer gold account;
- a branded bar product;
- a bullion marketplace;
- a gold token;
- a direct custody business;
- lending from Argent's balance sheet;
- a full repo and lease stack before the pledge workflow has a real design partner;
- support for every certificate without reviewing product terms;
- a generic “gold API” that hides legal and custody differences;
- automatic liquidation without a named dealer, legal authority, and settlement route.

The next product milestone is not breadth. It is proof that an existing external holding can be classified, controlled, financed, monitored, adjusted, and released or enforced through production-relevant institutional interfaces.

---

## 24. Strategic position

Emirates NBD demonstrates the value of a bank bringing bullion products under one roof [1], [2]. HSBC demonstrates the mature connection between trading, financing, vaulting, and physical services [13]. Standard Chartered demonstrates the broader corporate and investment-banking context in which precious metals sit alongside cash, derivatives, structured products, and cross-border commodities activity [21]. UOB demonstrates the continuing importance of formats, denominations, physical conversion, and bank buy/sell procedures [14]. Swissquote demonstrates the client expectation for online management, real-time margin visibility, multi-currency credit, and interest charged on utilisation [15]. DBS, the World Gold Council, and Euroclear demonstrate that tokenisation is one industry answer to fractional ownership and collateral mobility [10], [16], [17].

Argent's strongest position is the open-system alternative:

> **Argent lets a bank activate bullion as controlled collateral without requiring the bank to custody every bar, issue every gold product, or convert the underlying asset into a freely transferable token.**

Its durable advantage does not come from the statement that gold can secure credit. Banks already know that. It comes from making the cross-institution control sequence:

- representation-aware;
- policy-attributable;
- custody-confirmed;
- continuously valued;
- whole-bar aware;
- integration-friendly;
- exception-managed;
- and independently evidential.

That is the architecture through which a gold-credit demonstrator can become reusable bank infrastructure.

---

## 25. Architecture decisions to resolve with a design partner

The following questions should be resolved before production implementation:

### Rights and legal

- Which exact holding representation is in scope?
- Does the borrower have specific title, beneficial ownership, or a claim against an issuer?
- What creates and perfects the security interest?
- Is issuer or account-provider consent required?
- Are reuse, substitution, movement, appropriation, or private sale permitted?
- Which jurisdiction and governing law apply?

### Custody

- Who is the authoritative custodian or account provider?
- What bar-list or account fields are available?
- How is control acknowledged and enforced operationally?
- How quickly are changes reported?
- How are insurance, audits, and movements evidenced?

### Credit and risk

- Which refiners, formats, locations, custodians, and representations are eligible?
- What haircut and advance methodology applies?
- What are the margin thresholds, cure periods, and severe-event rules?
- How are FX, concentration, and transit treated?
- What is the primary repayment source?

### Integration

- Which systems own facility, exposure, collateral, custody, price, settlement, and documents?
- Which interfaces are available now?
- What is the minimum safe manual fallback?
- What are the reconciliation frequency and break tolerances?
- Which events must be synchronous and which may be eventual?

### Enforcement

- Who is the liquidation or transfer counterparty?
- What bars can it accept and where?
- What is the expected time and cost to realise?
- Which settlement account and currency receive proceeds?
- How are principal, interest, fees, costs, surplus, and shortfall applied?

### Commercial

- Who pays custody, valuation, legal, integration, and Argent fees?
- What facility size makes the workflow economical?
- What operational cost is removed or reduced?
- What new lending revenue or client retention does the product create?
- What evidence is required to approve replication?

---

## References

[1] Emirates NBD, **“Physical Gold & Silver Bullion Transaction Service FAQ for Clients.”** https://cdn.emiratesnbd.com/en/assets/file/physical_gold_silver_bullion_transaction_service_clients_faqs.pdf

[2] Emirates NBD, **“Emirates NBD Pioneers Physical Cross-Border Gold and Silver Bullion Transaction Service,”** 19 May 2025. https://www.emiratesnbd.com/en/media-center/emirates-nbd-pioneers-physical-cross-border-gold

[3] Emirates NBD, **“Gold Account.”** https://www.emiratesnbd.com/en/accounts/current-accounts/gold-account

[4] Emirates NBD, **“Gold Saving Certificates.”** https://www.emiratesnbd.com/en/wealth/gold-savings-certificate

[5] Emirates NBD, **“Terms and Conditions for Precious Metal – Gold.”** https://cdn.emiratesnbd.com/enbd/files/pdf/precious_metal_tnc.pdf

[6] Emirates NBD, **“Emirates NBD pioneers first branded gold bar ‘Emirates NBD Gold’ in the UAE.”** https://www.emiratesnbd.com/en/media-center/emirates-nbd-pioneers-first-branded-gold-bar

[7] London Bullion Market Association, **“A Guide to the Loco London Precious Metals Market: Precious Metal Accounts.”** https://www.lbma.org.uk/publications/the-otc-guide/precious-metal-accounts

[8] London Bullion Market Association, **“About Loco London”** and **“Glossary.”** https://www.lbma.org.uk/market-standards/about-loco-london ; https://www.lbma.org.uk/resources/glossary

[9] London Bullion Market Association, **“The Guide”** and the LBMA/SGE kilobar specification. https://cdn.lbma.org.uk/downloads/Publications/LBMA-The-Guide-2017-v1.pdf

[10] World Gold Council and Linklaters, **“Pooled Gold Interests and Wholesale Digital Gold – A New Vision for the Gold Market,”** 2 September 2025. https://www.gold.org/what-we-do/gold247/linklaters-white-paper

[11] London Bullion Market Association, **“A Guide to the Loco London Precious Metals Market: Lending and Borrowing Metal.”** https://www.lbma.org.uk/publications/the-otc-guide/lending-and-borrowing-metal

[12] London Bullion Market Association, **“Global Precious Metals Code,”** 2022. https://cdn.lbma.org.uk/downloads/GPMC/Global-Precious-Metals-Code-2022.pdf

[13] HSBC, **“Commodities – Precious Metals.”** https://www.business.hsbc.com/en-gb/products/commodities

[14] UOB, **“Gold & Silver – Business”** and **“Gold & Silver – Personal.”** https://www.uob.com.sg/business/invest/gold-silver.page ; https://www.uob.com.sg/personal/invest/gold-and-silver.page

[15] Swissquote, **“Lombard Loan: Flexible Financing.”** https://www.swissquote.com/en-lu/private/trade/products/securities/lombard-loan

[16] DBS, **“Physical Gold Token.”** https://www.dbs.com.sg/global-financial-markets/forex-and-commodities/physical-gold-token

[17] Euroclear, Digital Asset, and World Gold Council, **“Using DLT to enhance collateral mobility,”** 2 October 2024. https://www.euroclear.com/newsandinsights/en/Format/Whitepapers-Reports/dlt-to-enhance-collateral-mobility.html

[18] Basel Committee on Banking Supervision, **“CRE22 – Standardised approach: credit risk mitigation.”** https://www.bis.org/basel_framework/chapter/CRE/22.htm

[19] FINOS Common Domain Model, **“Eligible Collateral Representation.”** https://cdm.finos.org/docs/eligible-collateral-representation/

[20] FINOS Common Domain Model, **“Repurchase Transaction Representation”** and **“Event Model.”** https://cdm.finos.org/docs/repurchase-agreement-representation/ ; https://cdm.finos.org/docs/event-model/

[21] Standard Chartered, **“Commodities”** and **“Metals and Mining.”** https://www.sc.com/en/corporate-investment-banking/global-markets/commodities/ ; https://www.sc.com/en/corporate-investment-banking/industries/metals-mining/

---

*This document is a product and systems reference architecture. It is not legal, regulatory, accounting, prudential, tax, credit, investment, custody, or operational advice. Each implementation must be reviewed and approved by the participating lender, custodian or account provider, legal advisers, risk and compliance functions, and relevant authorities.*

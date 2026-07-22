# Pilot profile

## Pilot objective

Determine whether one bank can safely and operationally use one company's allocated bullion, held by one approved custodian, as collateral for one purpose-bound bank obligation.

The pilot is a design and control exercise before it is a technology deployment. Its first output is an institutionally accepted operating model—not transaction volume.

## Recommended first use case

**UAE precious-metals company · customs standing guarantee · allocated kilobars**

Why this wedge:

- the company is more likely to own eligible bullion already;
- the obligation recurs and directly affects operating capacity;
- a bank instrument may be specified by the beneficiary;
- the asset, custodian, purpose, and discharge event can be narrowly defined;
- capacity return after discharge is easy to demonstrate.

The final obligation and beneficiary must be selected by the bank and company during Gate 1. No claim is made that every customs authority or account accepts the same instrument or collateral structure.

## Participants

| Participant | Remains authoritative for |
|---|---|
| Company / applicant | Ownership, application, commercial purpose, reimbursement, and permitted requests. |
| Issuing bank | Customer underwriting, facility approval, collateral policy, instrument issuance, claim decision, release, and enforcement instruction. |
| Custodian / control agent | Bar identity, custody, segregation, immobilization, substitution, release, and physical realization. |
| Beneficiary | Instrument acceptance, presentation, claim, cancellation, or discharge according to its rules. |
| Argent | Authorized capacity state, workflow consistency, evidence references, and integrity anchoring. |

Legal counsel, compliance, auditors, valuation providers, institutional signing providers, and integration teams participate as control functions, not substitutes for the four authorities above.

## Asset profile

The initial reserve should be:

- company-owned rather than customer-owned;
- allocated and individually identifiable;
- investment-grade kilobars of an agreed fineness and refiner profile;
- held in one approved vault and custodian namespace;
- segregated or otherwise legally and operationally ring-fenced;
- supported by ownership, assay, insurance, location, and custody evidence;
- free of undisclosed liens or adverse claims;
- eligible under the bank's valuation, haircut, concentration, and margin policy.

Fast-moving inventory, work in progress, pooled interests, unallocated metal, customer assets, cross-custodian holdings, and tokenized claims are excluded from the first pilot.

## Obligation profile

The initial instrument should define:

- applicant and beneficiary;
- issuing bank and product owner;
- instrument type and governing rules;
- maximum amount and currency;
- issue, amendment, expiry, cancellation, claim, and discharge events;
- reimbursement obligation and settlement route;
- collateral amount or capacity treatment;
- permitted substitution and partial-release rules;
- external bank and beneficiary references;
- evidence required before each state transition.

Available facility capacity cannot be withdrawn by the company as cash. Argent records a reservation only after bank approval. The bank product system remains authoritative for the instrument itself.

## End-to-end pilot flow

1. Company, bank, and custodian complete legal, credit, compliance, asset, and authority onboarding.
2. Custodian canonicalizes the private bar identity, creates evidence commitments and derives uniqueness nullifiers within its governed namespace.
3. Bank approves the eligible value, haircut, facility limit, sublimit, and permitted obligation.
4. Company requests the defined obligation.
5. Bank approves the applicant, beneficiary, product, amount, tenor, and evidence package.
6. Argent reserves capacity and records the authorized private transition.
7. Bank issues the instrument in its authoritative product system.
8. Argent records the issuance reference and anchors minimized integrity evidence.
9. Bank confirms expiry, cancellation, reimbursement, claim, or another terminal outcome.
10. Argent returns capacity or records crystallized exposure and the governed enforcement path.
11. Custodian releases, substitutes, or realizes bars only after the required bank-authorized workflow.

## Evidence package

The pilot must reconcile:

- asset and ownership evidence;
- custodian control acknowledgement;
- security agreement and any registration evidence;
- bank facility and collateral-policy approval;
- company and institutional authority records;
- obligation request, approval, issue, amendment, and terminal status;
- valuation and margin evidence;
- private state transition and public integrity anchor;
- reimbursement, release, substitution, or enforcement evidence;
- exception, incident, and recovery records.

## Success measures

| Measure | Pilot question |
|---|---|
| Acceptance | Will the bank, custodian, company, beneficiary, and counsel accept the operating model? |
| Integrity | Can any participant create excess allocation, bypass sequence, replay an act, or release early? |
| Privacy | Can an outside observer infer customer, bar, amount, beneficiary, graph, or lifecycle information? |
| Reconciliation | Do bank, custodian, Argent, and the public anchor converge after every valid and failed event? |
| Operations | Can staff resolve stale evidence, valuation shocks, amendments, outages, and disputed status? |
| Economics | Does the facility add useful headroom or preserve cash after all fees, haircuts, margin risk, and encumbrance costs? |
| Legal effect | Do counsel and the bank confirm creation, perfection, priority, insolvency treatment, and enforcement? |

## Deliberate exclusions

- No surety or insurance risk.
- No unsecured credit decision by Argent.
- No gold brokerage, lending, leasing, or tokenization.
- No multi-bank or cross-custodian allocation.
- No multi-party project assurance network.
- No live customer data on the transparent reference contracts.
- No zero-knowledge proof dependency.
- No claim that Soroban creates the legal pledge.

## Pilot decision

The pilot ends with one of three explicit outcomes:

1. **Proceed** to a limited-value controlled production pilot.
2. **Revise** the asset, obligation, control, privacy, integration, or legal structure and repeat the gate.
3. **Stop** because beneficiary acceptance, economics, legal effectiveness, risk treatment, or operations do not justify production.

See [LEGAL_PILOT_CHECKLIST.md](LEGAL_PILOT_CHECKLIST.md) and [SECURITY_AND_PRIVACY.md](SECURITY_AND_PRIVACY.md) before treating the profile as production-ready.

# Working with Argent: an invitation to design partners

**For institutions that feel the constraints of the current physical-collateral-to-credit relationship, and see room for something better.**

---

## Who this is for

If your institution lends against physical collateral, or holds physical assets it would like to finance, and you find the current relationship more constrained than it should be, this is an open door.

The specific frictions Argent exists to address are well documented and probably familiar: collateral operations run on manual, spreadsheet-bound processes; releasing or substituting collateral is slow and error-prone; a static borrowing base lags the real value of the asset; and the only "modern" option on offer is usually to tokenize the asset, which is not what a bank holding physical gold, metal, or warehouse stock actually wants. Argent takes a different position: the asset stays in custody and is never tokenized, and only the control over it becomes programmable, signed, and auditable. The thesis and mechanism are set out in `collateral-control.md`.

## What this is, and what it is not

This is an invitation to explore, share ideas, and solve real operational problems together. It is a design-partner relationship: an institution with a genuine pain point, working with a builder addressing it, to shape the product against how the work is actually done.

This is not a procurement process, a sales funnel, or a request for unpaid implementation work (though the core is open-source and forks are welcome). It is an early design conversation with institutions that recognize the same operational problem, between people who see it from two sides: the institution that lives with the blocker, and the team building the control layer meant to remove it.

## Where a conversation is most useful

We would value talking with:

- **Banks and lenders** financing physical collateral (precious and base metals, commodities, warehouse inventory, serialised industrial assets) who feel the operational cost of manual collateral control, improper-release risk, and slow substitution.
- **Custodians and collateral managers** who hold the assets and sign for their state, and who see where a signed, on-chain control record would reduce reconciliation and dispute.
- **Commodity traders and producers** whose inventory is working capital, and who want the pledged asset to stay operationally useful (partial release, substitution, sale under repayment control) rather than frozen.
- **Anyone working on the same problem** from a different angle, who wants to compare notes on making physical collateral controllable without tokenizing it.

## What we bring to the conversation

A working, tested prototype on Stellar testnet (not a whitepaper alone): the full pledge, valuation, two-step release, and settlement lifecycle recorded as role-signed events, with the contract core open-source under Apache-2.0. A clear thesis grounded in how secured and commodity finance actually work, and in the collateral-modernization direction that institutions from central banks to market infrastructures have named for themselves. And a conservative, boundaries-first approach: the asset never leaves custody, the lender is never left under-collateralized, and the chain records control, not title.

## What a first conversation looks like

No commitment, no procurement process, no pitch deck required from your side. A conversation about where the current physical-collateral-to-credit relationship costs your institution time, risk, or opportunity, and whether a programmable control layer would help. If it is useful to both sides, we explore a concrete pilot shaped around your workflow. If it is not, we have both learned something and shared ideas worth having.

## What we want to learn

A first conversation is most useful when it is concrete. The questions we are trying to answer:

- Where collateral release or substitution gets stuck today.
- Which documents actually govern custody, pledge, valuation, and release.
- Which parties must sign before a bank considers the collateral controlled.
- How borrowing-base updates are reviewed, challenged, and approved.
- What a bank, custodian, or borrower would need to see before relying on a shared control record.

## Confidentiality

Please do not send confidential client, facility, custody, or transaction data before an appropriate confidentiality arrangement is in place. A first conversation can be productive using anonymized workflows, representative documents, and generic examples.

## How to reach us

Argent is built by Advisa EOOD. To start a conversation, reach us at **deyan@advisa.tech**.

For the technical foundations, see `argent-architecture.md` and `protocol.md`. For the collateral-control thesis and the institutional grounding, see `collateral-control.md`. For where the product goes next, see `product-roadmap.md`.

Repository: https://github.com/deyan-paroushev/argent-core

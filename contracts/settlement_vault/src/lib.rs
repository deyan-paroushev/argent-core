#![no_std]

#[cfg(test)]
mod test;

use soroban_sdk::{
    contract, contracterror, contractimpl, contractmeta, contracttype, symbol_short, token,
    Address, BytesN, Env,
};

// Instance-storage TTL management, matching the credit_ledger convention so the
// two contracts age consistently. ~5s ledgers: a day is 17_280 ledgers; we bump
// the instance lifetime by 30 days whenever it drops within ~29 days of expiry.
const DAY_IN_LEDGERS: u32 = 17_280;
const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

/// Import the CreditLedger's client + types from its built wasm. This generates
/// `credit_ledger::Client` and the contract's types WITHOUT pulling its
/// #[contractimpl] exports into this wasm (which would collide on `initialize`
/// at link time). The path is relative to the workspace root; credit_ledger
/// must be built first (stellar contract build orders this correctly).
mod credit_ledger {
    soroban_sdk::contractimport!(
        file = "../target/wasm32v1-none/release/credit_ledger.wasm"
    );
}

// Self-identifying protocol metadata (WASM `contractmetav0` section).
contractmeta!(key = "name", val = "Argent SettlementVault");
contractmeta!(key = "proto", val = "argent.settlement.v1");
contractmeta!(key = "sdk", val = "soroban-sdk-23.5.3");

#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    AmountNotPositive = 3,
    LineNotRepayable = 4,
    RepaymentExceedsOutstandingBalance = 5,
    NotAuthorizedBankDestination = 6,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    /// the contract admin, authorized at initialize
    Admin,
    /// the SEP-41 / SAC settlement asset (e.g. USDC) contract address
    SettlementToken,
    /// the deployed CreditLedger contract address
    CreditLedger,
}

#[contract]
pub struct SettlementVault;

#[contractimpl]
impl SettlementVault {
    /// Bind the vault to a settlement asset and the credit ledger. One-time.
    pub fn initialize(
        env: Env,
        admin: Address,
        settlement_token: Address,
        credit_ledger: Address,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::CreditLedger) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::SettlementToken, &settlement_token);
        env.storage()
            .instance()
            .set(&DataKey::CreditLedger, &credit_ledger);
        env.storage()
            .instance()
            .extend_ttl(LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Ok(())
    }

    /// Atomic repayment settlement: the cardholder pays the bank in the
    /// settlement asset and the credit ledger's drawn balance is reduced, in one
    /// transaction. If the token transfer fails, nothing commits.
    ///
    /// Repayment reduces exposure only. It does not release collateral: release
    /// is a separate bank-authorized, custodian-confirmed act on the credit
    /// ledger (the vault is the payment rail, not a party to the control
    /// register). `payment_ref` is the off-chain payment reference, threaded to
    /// the ledger so the repayment is idempotent.
    ///
    /// The cardholder signs (authorizing the transfer of their tokens). The
    /// bank is the transfer destination.
    pub fn settle_repayment(
        env: Env,
        credit_line_id: BytesN<32>,
        cardholder: Address,
        bank: Address,
        payment_ref: BytesN<32>,
        amount: i128,
    ) -> Result<(), Error> {
        cardholder.require_auth();
        if amount <= 0 {
            return Err(Error::AmountNotPositive);
        }

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::SettlementToken)
            .ok_or(Error::NotInitialized)?;
        let ledger_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::CreditLedger)
            .ok_or(Error::NotInitialized)?;

        let ledger = credit_ledger::Client::new(&env, &ledger_addr);
        let line = ledger.get_line(&credit_line_id);
        // The repayment destination must be the bank named on the credit line.
        // Without this, a cardholder could pay an arbitrary address while still
        // reducing the on-ledger drawn balance.
        if line.bank != bank {
            return Err(Error::NotAuthorizedBankDestination);
        }
        if line.status == credit_ledger::LineStatus::Closed || line.drawn_balance <= 0 {
            return Err(Error::LineNotRepayable);
        }
        if amount > line.drawn_balance {
            return Err(Error::RepaymentExceedsOutstandingBalance);
        }

        // 1. Move the settlement asset: cardholder -> bank (SEP-41 transfer).
        let token_client = token::TokenClient::new(&env, &token_addr);
        token_client.transfer(&cardholder, &bank, &amount);

        // 2. Reduce the debt atomically via cross-contract call. The vault is a
        //    direct caller of the ledger, so its identity is conveyed via the
        //    invoking-contract address; the ledger checks that the caller is
        //    the vault it was bound to at init. Only a bound vault can mutate
        //    the line, which protects the bank's interest. The payment_ref makes
        //    the repayment idempotent on the ledger side.
        let vault = env.current_contract_address();
        ledger.apply_repayment(&credit_line_id, &vault, &payment_ref, &amount);

        env.events().publish(
            (symbol_short!("repay"), symbol_short!("settled")),
            (credit_line_id, amount),
        );
        env.storage()
            .instance()
            .extend_ttl(LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Ok(())
    }

    /// Read the credit ledger this vault is bound to. Lets any party verify the
    /// vault points at the expected ledger without inspecting raw storage.
    pub fn get_credit_ledger(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::CreditLedger)
            .ok_or(Error::NotInitialized)
    }

    /// Read the settlement asset (SEP-41 / SAC) this vault settles in.
    pub fn get_settlement_token(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::SettlementToken)
            .ok_or(Error::NotInitialized)
    }

    /// Read the vault admin set at initialize.
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }
}

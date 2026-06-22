#![no_std]

#[cfg(test)]
mod test;

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, BytesN,
    Env,
};

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

#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    AmountNotPositive = 3,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
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
        settlement_token: Address,
        credit_ledger: Address,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::CreditLedger) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage()
            .instance()
            .set(&DataKey::SettlementToken, &settlement_token);
        env.storage()
            .instance()
            .set(&DataKey::CreditLedger, &credit_ledger);
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
        let ledger = credit_ledger::Client::new(&env, &ledger_addr);
        ledger.apply_repayment(&credit_line_id, &vault, &payment_ref, &amount);

        env.events().publish(
            (symbol_short!("repay"), symbol_short!("settled")),
            (credit_line_id, amount),
        );
        Ok(())
    }
}

#![allow(clippy::diverging_sub_expression)]

pub mod account;
pub mod instruction_handler;

use anchor_lang::prelude::*;

use instruction_handler::configure_banking_settlement_config::handle_configure_banking_settlement_config_instruction;
use instruction_handler::settle_deposit::handle_settle_deposit_instruction;
use instruction_handler::settle_withdrawal::handle_settle_withdrawal_instruction;

pub use account::{
    BankingSettlementConfig, BankingSettlementConfigVersion, DepositSettlementReceipt,
    DepositSettlementReceiptVersion, PoolAuthority, WithdrawalSettlementReceipt,
    WithdrawalSettlementReceiptVersion,
};
pub use instruction_handler::{
    ConfigureBankingSettlementConfigInstructionAccounts,
    ConfigureBankingSettlementConfigInstructionError, SettleDepositInstructionAccounts,
    SettleDepositInstructionError, SettleWithdrawalInstructionAccounts,
    SettleWithdrawalInstructionError,
};

#[doc(hidden)]
pub(crate) use instruction_handler::configure_banking_settlement_config::configure_banking_settlement_config_instruction_accounts::__client_accounts_configure_banking_settlement_config_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::configure_banking_settlement_config::configure_banking_settlement_config_instruction_accounts::__cpi_client_accounts_configure_banking_settlement_config_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::settle_deposit::settle_deposit_instruction_accounts::__client_accounts_settle_deposit_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::settle_deposit::settle_deposit_instruction_accounts::__cpi_client_accounts_settle_deposit_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::settle_withdrawal::settle_withdrawal_instruction_accounts::__client_accounts_settle_withdrawal_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::settle_withdrawal::settle_withdrawal_instruction_accounts::__cpi_client_accounts_settle_withdrawal_instruction_accounts;

declare_id!("DzYXFRU9PyJiEWLGaTQ8FA35urAtTkLH3G3QvQqMB2tZ");

#[program]
pub mod banking_settlement {
    use super::*;

    pub fn configure_banking_settlement_config(
        ctx: Context<ConfigureBankingSettlementConfigInstructionAccounts>,
    ) -> Result<()> {
        handle_configure_banking_settlement_config_instruction(ctx)
    }

    pub fn settle_deposit(
        ctx: Context<SettleDepositInstructionAccounts>,
        deposit_id: [u8; 16],
        token_amount: u64,
    ) -> Result<()> {
        handle_settle_deposit_instruction(ctx, deposit_id, token_amount)
    }

    pub fn settle_withdrawal(
        ctx: Context<SettleWithdrawalInstructionAccounts>,
        withdrawal_id: [u8; 16],
        token_amount: u64,
    ) -> Result<()> {
        handle_settle_withdrawal_instruction(ctx, withdrawal_id, token_amount)
    }
}

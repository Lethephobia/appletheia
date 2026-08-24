#![allow(clippy::diverging_sub_expression)]

pub mod account;
pub mod instruction_handler;

use anchor_lang::prelude::*;

use instruction_handler::banking_settlement_config_configure::BankingSettlementConfigConfigureInstructionHandler;
use instruction_handler::deposit_settle::DepositSettleInstructionHandler;
use instruction_handler::withdrawal_settle::WithdrawalSettleInstructionHandler;

pub use account::{
    BankingSettlementConfig, DepositSettlementReceipt, PoolAuthority, WithdrawalSettlementReceipt,
};
pub use instruction_handler::{
    BankingSettlementConfigConfigureInstructionAccounts,
    BankingSettlementConfigConfigureInstructionError, DepositSettleInstructionAccounts,
    DepositSettleInstructionError, WithdrawalSettleInstructionAccounts,
    WithdrawalSettleInstructionError,
};

#[doc(hidden)]
pub(crate) use instruction_handler::banking_settlement_config_configure::banking_settlement_config_configure_instruction_accounts::__client_accounts_banking_settlement_config_configure_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::banking_settlement_config_configure::banking_settlement_config_configure_instruction_accounts::__cpi_client_accounts_banking_settlement_config_configure_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::deposit_settle::deposit_settle_instruction_accounts::__client_accounts_deposit_settle_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::deposit_settle::deposit_settle_instruction_accounts::__cpi_client_accounts_deposit_settle_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::withdrawal_settle::withdrawal_settle_instruction_accounts::__client_accounts_withdrawal_settle_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::withdrawal_settle::withdrawal_settle_instruction_accounts::__cpi_client_accounts_withdrawal_settle_instruction_accounts;

declare_id!("DzYXFRU9PyJiEWLGaTQ8FA35urAtTkLH3G3QvQqMB2tZ");

#[program]
pub mod banking_settlement {
    use super::*;

    pub fn configure_banking_settlement_config(
        ctx: Context<BankingSettlementConfigConfigureInstructionAccounts>,
    ) -> Result<()> {
        BankingSettlementConfigConfigureInstructionHandler::handle(ctx)
    }

    pub fn settle_deposit(
        ctx: Context<DepositSettleInstructionAccounts>,
        deposit_id: [u8; 16],
        token_amount: u64,
    ) -> Result<()> {
        DepositSettleInstructionHandler::handle(ctx, deposit_id, token_amount)
    }

    pub fn settle_withdrawal(
        ctx: Context<WithdrawalSettleInstructionAccounts>,
        withdrawal_id: [u8; 16],
        token_amount: u64,
    ) -> Result<()> {
        WithdrawalSettleInstructionHandler::handle(ctx, withdrawal_id, token_amount)
    }
}

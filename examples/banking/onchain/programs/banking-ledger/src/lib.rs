#![allow(clippy::diverging_sub_expression)]

pub mod account;
pub mod instruction_handler;

use anchor_lang::prelude::*;
use banking_anchor::instruction::InstructionHandler;

use instruction_handler::banking_ledger_config_configure::BankingLedgerConfigConfigureInstructionHandler;
use instruction_handler::mint_metadata_update::MintMetadataUpdateInstructionHandler;
use instruction_handler::mint_supply_sync::MintSupplySyncInstructionHandler;
use instruction_handler::mint_upsert::MintUpsertInstructionHandler;
use instruction_handler::pool_token_account_ensure::PoolTokenAccountEnsureInstructionHandler;
use instruction_handler::pool_token_deposit::PoolTokenDepositInstructionHandler;
use instruction_handler::pool_token_transfer::PoolTokenTransferInstructionHandler;

pub use account::{
    BankingLedgerConfig, Mint, MintMetadata, MintState, PoolTokenDepositReceipt,
    PoolTokenTransferMarker, ProgramAuthority,
};
pub use instruction_handler::{
    BankingLedgerConfigConfigureInstructionAccounts, BankingLedgerConfigConfigureInstructionArgs,
    BankingLedgerConfigConfigureInstructionError, MintMetadataUpdateInstructionAccounts,
    MintMetadataUpdateInstructionArgs, MintMetadataUpdateInstructionError,
    MintSupplySyncInstructionAccounts, MintSupplySyncInstructionArgs,
    MintSupplySyncInstructionError, MintUpsertInstructionAccounts, MintUpsertInstructionArgs,
    MintUpsertInstructionError, PoolTokenAccountEnsureInstructionAccounts,
    PoolTokenAccountEnsureInstructionArgs, PoolTokenAccountEnsureInstructionError,
    PoolTokenDepositInstructionAccounts, PoolTokenDepositInstructionArgs,
    PoolTokenDepositInstructionError, PoolTokenTransferInstructionAccounts,
    PoolTokenTransferInstructionArgs, PoolTokenTransferInstructionError,
};

#[doc(hidden)]
pub(crate) use instruction_handler::banking_ledger_config_configure::banking_ledger_config_configure_instruction_accounts::__client_accounts_banking_ledger_config_configure_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::banking_ledger_config_configure::banking_ledger_config_configure_instruction_accounts::__cpi_client_accounts_banking_ledger_config_configure_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::mint_metadata_update::mint_metadata_update_instruction_accounts::__client_accounts_mint_metadata_update_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::mint_metadata_update::mint_metadata_update_instruction_accounts::__cpi_client_accounts_mint_metadata_update_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::mint_supply_sync::mint_supply_sync_instruction_accounts::__client_accounts_mint_supply_sync_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::mint_supply_sync::mint_supply_sync_instruction_accounts::__cpi_client_accounts_mint_supply_sync_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::mint_upsert::mint_upsert_instruction_accounts::__client_accounts_mint_upsert_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::mint_upsert::mint_upsert_instruction_accounts::__cpi_client_accounts_mint_upsert_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::pool_token_account_ensure::pool_token_account_ensure_instruction_accounts::__client_accounts_pool_token_account_ensure_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::pool_token_account_ensure::pool_token_account_ensure_instruction_accounts::__cpi_client_accounts_pool_token_account_ensure_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::pool_token_deposit::pool_token_deposit_instruction_accounts::__client_accounts_pool_token_deposit_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::pool_token_deposit::pool_token_deposit_instruction_accounts::__cpi_client_accounts_pool_token_deposit_instruction_accounts;
#[doc(hidden)]
pub(crate) use instruction_handler::pool_token_transfer::pool_token_transfer_instruction_accounts::__client_accounts_pool_token_transfer_instruction_accounts;
#[cfg(feature = "cpi")]
#[doc(hidden)]
pub(crate) use instruction_handler::pool_token_transfer::pool_token_transfer_instruction_accounts::__cpi_client_accounts_pool_token_transfer_instruction_accounts;

declare_id!("DzYXFRU9PyJiEWLGaTQ8FA35urAtTkLH3G3QvQqMB2tZ");

#[program]
pub mod banking_ledger {
    use super::*;

    pub fn configure_banking_ledger_config(
        ctx: Context<BankingLedgerConfigConfigureInstructionAccounts>,
    ) -> Result<()> {
        let args = BankingLedgerConfigConfigureInstructionArgs;

        BankingLedgerConfigConfigureInstructionHandler::handle(ctx, args)
    }

    pub fn upsert_mint(
        ctx: Context<MintUpsertInstructionAccounts>,
        mint_id: [u8; 16],
        decimals: u8,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let args = MintUpsertInstructionArgs {
            mint_id,
            decimals,
            name,
            symbol,
            uri,
        };

        MintUpsertInstructionHandler::handle(ctx, args)
    }

    pub fn update_mint_metadata(
        ctx: Context<MintMetadataUpdateInstructionAccounts>,
        mint_id: [u8; 16],
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let args = MintMetadataUpdateInstructionArgs {
            mint_id,
            name,
            symbol,
            uri,
        };

        MintMetadataUpdateInstructionHandler::handle(ctx, args)
    }

    pub fn sync_mint_supply(
        ctx: Context<MintSupplySyncInstructionAccounts>,
        mint_id: [u8; 16],
        target_supply: u64,
    ) -> Result<()> {
        let args = MintSupplySyncInstructionArgs {
            mint_id,
            target_supply,
        };

        MintSupplySyncInstructionHandler::handle(ctx, args)
    }

    pub fn ensure_pool_token_account(
        ctx: Context<PoolTokenAccountEnsureInstructionAccounts>,
        mint_id: [u8; 16],
    ) -> Result<()> {
        let args = PoolTokenAccountEnsureInstructionArgs { mint_id };

        PoolTokenAccountEnsureInstructionHandler::handle(ctx, args)
    }

    pub fn transfer_pool_token(
        ctx: Context<PoolTokenTransferInstructionAccounts>,
        idempotency_key: [u8; 16],
        mint_id: [u8; 16],
        amount: u64,
    ) -> Result<()> {
        let args = PoolTokenTransferInstructionArgs {
            idempotency_key,
            mint_id,
            amount,
        };

        PoolTokenTransferInstructionHandler::handle(ctx, args)
    }

    pub fn deposit_pool_token(
        ctx: Context<PoolTokenDepositInstructionAccounts>,
        pool_token_deposit_id: [u8; 16],
        mint_id: [u8; 16],
        amount: u64,
    ) -> Result<()> {
        let args = PoolTokenDepositInstructionArgs {
            pool_token_deposit_id,
            mint_id,
            amount,
        };

        PoolTokenDepositInstructionHandler::handle(ctx, args)
    }
}

use std::str::FromStr;

use anchor_lang::{InstructionData, ToAccountMetas};
use appletheia::domain::AggregateId;
use banking_ledger::{
    BankingLedgerConfig, MintState, PoolTokenTransferMarker, ProgramAuthority,
    accounts::PoolTokenTransferInstructionAccounts, instruction::TransferPoolToken,
};
use banking_ledger_application::mint::{
    PoolTokenTransferExecutor, PoolTokenTransferExecutorError, PoolTokenTransferRequest,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use spl_associated_token_account_interface::address as associated_token_address;

use super::{SolanaPoolTokenTransferExecutorConfig, SolanaPoolTokenTransferExecutorError};

/// Solana implementation of `PoolTokenTransferExecutor`.
pub struct SolanaPoolTokenTransferExecutor {
    rpc_client: RpcClient,
    config: SolanaPoolTokenTransferExecutorConfig,
}

impl SolanaPoolTokenTransferExecutor {
    pub fn new(rpc_client: RpcClient, config: SolanaPoolTokenTransferExecutorConfig) -> Self {
        Self { rpc_client, config }
    }

    fn banking_ledger_config_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[BankingLedgerConfig::SEED], self.config.program_id()).0
    }

    fn mint_state_address(&self, mint_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(&[MintState::SEED, mint_id], self.config.program_id()).0
    }

    fn pool_token_transfer_marker_address(&self, idempotency_key: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(
            &[PoolTokenTransferMarker::SEED, idempotency_key],
            self.config.program_id(),
        )
        .0
    }

    fn program_authority_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[ProgramAuthority::SEED], self.config.program_id()).0
    }

    fn destination_token_account_address(
        destination_owner_address: &Pubkey,
        mint_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Pubkey {
        associated_token_address::get_associated_token_address_with_program_id(
            destination_owner_address,
            mint_address,
            token_program_id,
        )
    }

    async fn send_transaction(
        &self,
        instructions: Vec<Instruction>,
    ) -> Result<(), PoolTokenTransferExecutorError> {
        let blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|error| PoolTokenTransferExecutorError::Backend(Box::new(error)))?;

        let payer = self.config.payer().as_ref();
        let operator = self.config.operator().as_ref();
        let signers = self.unique_signers(&[payer, operator]);

        let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
        transaction.try_sign(&signers, blockhash).map_err(|error| {
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::SignTransaction(error),
            ))
        })?;

        self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .await
            .map_err(|error| PoolTokenTransferExecutorError::Backend(Box::new(error)))?;

        Ok(())
    }

    fn unique_signers<'a>(&self, signers: &[&'a dyn Signer]) -> Vec<&'a dyn Signer> {
        let mut unique: Vec<&dyn Signer> = Vec::new();
        for signer in signers {
            if unique
                .iter()
                .all(|existing| existing.pubkey() != signer.pubkey())
            {
                unique.push(*signer);
            }
        }

        unique
    }
}

impl PoolTokenTransferExecutor for SolanaPoolTokenTransferExecutor {
    async fn execute(
        &self,
        request: PoolTokenTransferRequest,
    ) -> Result<(), PoolTokenTransferExecutorError> {
        let payer = self.config.payer().pubkey();
        let operator = self.config.operator().pubkey();
        let program_id = *self.config.program_id();
        let token_program_id = spl_token_2022_interface::id();
        let associated_token_program_id = spl_associated_token_account_interface::program::id();
        let idempotency_key = *request.withdrawal_id().value().as_bytes();
        let mint_id = *request.currency_id().value().as_bytes();
        let marker_account_address = self.pool_token_transfer_marker_address(&idempotency_key);

        let mint_account_address = Pubkey::from_str(
            request.mint_account().mint_account_address().value(),
        )
        .map_err(|_| {
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::InvalidPubkey {
                    kind: "mint account address",
                    value: request
                        .mint_account()
                        .mint_account_address()
                        .value()
                        .to_owned(),
                },
            ))
        })?;
        let pool_token_account_address = Pubkey::from_str(
            request.mint_account().pool_token_account_address().value(),
        )
        .map_err(|_| {
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::InvalidPubkey {
                    kind: "pool token account address",
                    value: request
                        .mint_account()
                        .pool_token_account_address()
                        .value()
                        .to_owned(),
                },
            ))
        })?;
        let destination_owner_address =
            Pubkey::from_str(request.token_account_owner_address().value()).map_err(|_| {
                PoolTokenTransferExecutorError::Backend(Box::new(
                    SolanaPoolTokenTransferExecutorError::InvalidPubkey {
                        kind: "destination token account owner address",
                        value: request.token_account_owner_address().value().to_owned(),
                    },
                ))
            })?;
        let banking_ledger_config_address = self.banking_ledger_config_address();
        let mint_state_address = self.mint_state_address(&mint_id);
        let program_authority_address = self.program_authority_address();
        let destination_token_account_address = Self::destination_token_account_address(
            &destination_owner_address,
            &mint_account_address,
            &token_program_id,
        );
        let amount = u64::try_from(request.amount().value()).map_err(|_| {
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::AmountOverflow,
            ))
        })?;

        let instructions = vec![Instruction {
            program_id,
            accounts: PoolTokenTransferInstructionAccounts {
                payer,
                banking_ledger_config: banking_ledger_config_address,
                operator,
                mint_state: mint_state_address,
                pool_token_transfer_marker: marker_account_address,
                program_authority: program_authority_address,
                mint: mint_account_address,
                pool_token_account: pool_token_account_address,
                token_account_owner: destination_owner_address,
                destination_token_account: destination_token_account_address,
                system_program: system_program::id(),
                token_program: token_program_id,
                associated_token_program: associated_token_program_id,
            }
            .to_account_metas(None),
            data: TransferPoolToken {
                idempotency_key,
                mint_id,
                amount,
            }
            .data(),
        }];

        self.send_transaction(instructions).await
    }
}

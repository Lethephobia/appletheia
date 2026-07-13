use std::str::FromStr;

use anchor_lang::{InstructionData, ToAccountMetas};
use banking_ledger::{
    BankingLedgerConfig, MintState, PoolTokenDepositReceipt, ProgramAuthority,
    accounts::PoolTokenDepositInstructionAccounts, instruction::DepositPoolToken,
};
use banking_ledger_application::mint::{
    PreparedTokenDepositTransaction, TokenDepositPreparation, TokenDepositPrepareRequest,
    TokenDepositPreparer, TokenDepositPreparerError,
};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use spl_associated_token_account_interface::address as associated_token_address;

use super::{
    BankingLedgerMintId, PoolTokenDepositId, SolanaTokenDepositPreparerConfig,
    SolanaTokenDepositPreparerError,
};

/// Solana implementation of `TokenDepositPreparer`.
pub struct SolanaTokenDepositPreparer {
    rpc_client: RpcClient,
    config: SolanaTokenDepositPreparerConfig,
}

impl SolanaTokenDepositPreparer {
    pub fn new(rpc_client: RpcClient, config: SolanaTokenDepositPreparerConfig) -> Self {
        Self { rpc_client, config }
    }

    fn banking_ledger_config_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[BankingLedgerConfig::SEED], self.config.program_id()).0
    }

    fn mint_state_address(&self, mint_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(&[MintState::SEED, mint_id], self.config.program_id()).0
    }

    fn pool_token_deposit_receipt_address(&self, pool_token_deposit_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(
            &[PoolTokenDepositReceipt::SEED, pool_token_deposit_id],
            self.config.program_id(),
        )
        .0
    }

    fn program_authority_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[ProgramAuthority::SEED], self.config.program_id()).0
    }

    fn source_token_account_address(
        token_account_owner_address: &Pubkey,
        mint_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Pubkey {
        associated_token_address::get_associated_token_address_with_program_id(
            token_account_owner_address,
            mint_address,
            token_program_id,
        )
    }
}

impl TokenDepositPreparer for SolanaTokenDepositPreparer {
    async fn prepare(
        &self,
        request: TokenDepositPrepareRequest,
    ) -> Result<TokenDepositPreparation, TokenDepositPreparerError> {
        let program_id = *self.config.program_id();
        let payer = self.config.payer().pubkey();
        let operator = self.config.operator().pubkey();
        let token_program_id = spl_token_2022_interface::id();
        let associated_token_program_id = spl_associated_token_account_interface::program::id();
        let pool_token_deposit_id = PoolTokenDepositId::from(request.deposit_id()).into_bytes();
        let mint_id = BankingLedgerMintId::from(request.currency_id()).into_bytes();
        let receipt_account_address =
            self.pool_token_deposit_receipt_address(&pool_token_deposit_id);

        let mint_account_address = Pubkey::from_str(
            request.mint_account().mint_account_address().value(),
        )
        .map_err(|_| {
            TokenDepositPreparerError::Backend(Box::new(
                SolanaTokenDepositPreparerError::InvalidPubkey {
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
            TokenDepositPreparerError::Backend(Box::new(
                SolanaTokenDepositPreparerError::InvalidPubkey {
                    kind: "pool token account address",
                    value: request
                        .mint_account()
                        .pool_token_account_address()
                        .value()
                        .to_owned(),
                },
            ))
        })?;
        let token_account_owner_address =
            Pubkey::from_str(request.token_account_owner_address().value()).map_err(|_| {
                TokenDepositPreparerError::Backend(Box::new(
                    SolanaTokenDepositPreparerError::InvalidPubkey {
                        kind: "token account owner address",
                        value: request.token_account_owner_address().value().to_owned(),
                    },
                ))
            })?;
        let amount = u64::try_from(request.amount().value()).map_err(|_| {
            TokenDepositPreparerError::Backend(Box::new(
                SolanaTokenDepositPreparerError::AmountOverflow,
            ))
        })?;
        let source_token_account_address = Self::source_token_account_address(
            &token_account_owner_address,
            &mint_account_address,
            &token_program_id,
        );

        let instruction = Instruction {
            program_id,
            accounts: PoolTokenDepositInstructionAccounts {
                payer,
                banking_ledger_config: self.banking_ledger_config_address(),
                operator,
                mint_state: self.mint_state_address(&mint_id),
                pool_token_deposit_receipt: receipt_account_address,
                program_authority: self.program_authority_address(),
                mint: mint_account_address,
                pool_token_account: pool_token_account_address,
                token_account_owner: token_account_owner_address,
                source_token_account: source_token_account_address,
                system_program: system_program::id(),
                token_program: token_program_id,
                associated_token_program: associated_token_program_id,
            }
            .to_account_metas(None),
            data: DepositPoolToken {
                pool_token_deposit_id,
                mint_id,
                amount,
            }
            .data(),
        };

        let blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|error| {
                TokenDepositPreparerError::Backend(Box::new(SolanaTokenDepositPreparerError::Rpc(
                    error,
                )))
            })?;
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer));
        transaction
            .try_partial_sign(
                &[
                    self.config.payer().as_ref(),
                    self.config.operator().as_ref(),
                ],
                blockhash,
            )
            .map_err(|error| {
                TokenDepositPreparerError::Backend(Box::new(
                    SolanaTokenDepositPreparerError::SignTransaction(error),
                ))
            })?;
        let transaction = bincode::serialize(&transaction).map_err(|error| {
            TokenDepositPreparerError::Backend(Box::new(
                SolanaTokenDepositPreparerError::SerializeTransaction(error),
            ))
        })?;

        Ok(TokenDepositPreparation::new(
            PreparedTokenDepositTransaction::new(BASE64_STANDARD.encode(transaction)),
        ))
    }
}

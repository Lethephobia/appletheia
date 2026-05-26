use std::str::FromStr;

use banking_ledger_application::{
    OnchainTransactionId, PoolTokenTransferExecutor, PoolTokenTransferExecutorError,
    PoolTokenTransferReceipt, PoolTokenTransferRequest,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_system_interface::{instruction as system_instruction, program as system_program};
use solana_transaction::Transaction;
use spl_associated_token_account_interface::address as associated_token_address;
use spl_associated_token_account_interface::instruction as associated_token_instruction;
use spl_token_2022_interface::instruction as token_instruction;

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

    fn pool_token_account_address(
        pool_token_account_owner_address: &Pubkey,
        mint_account_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Pubkey {
        associated_token_address::get_associated_token_address_with_program_id(
            pool_token_account_owner_address,
            mint_account_address,
            token_program_id,
        )
    }

    fn destination_token_account_address(
        destination_owner_address: &Pubkey,
        mint_account_address: &Pubkey,
        token_program_id: &Pubkey,
    ) -> Pubkey {
        associated_token_address::get_associated_token_address_with_program_id(
            destination_owner_address,
            mint_account_address,
            token_program_id,
        )
    }

    fn marker_account_address(
        pool_token_account_owner_address: &Pubkey,
        request: &PoolTokenTransferRequest,
    ) -> Result<Pubkey, PoolTokenTransferExecutorError> {
        Pubkey::create_with_seed(
            pool_token_account_owner_address,
            request.marker_seed().value(),
            &system_program::id(),
        )
        .map_err(|error| {
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::MarkerAccountAddress(error),
            ))
        })
    }

    async fn recover_onchain_transaction_id(
        &self,
        marker_account_address: &Pubkey,
    ) -> Result<Option<OnchainTransactionId>, PoolTokenTransferExecutorError> {
        let Some(_) = self
            .rpc_client
            .get_account_with_commitment(marker_account_address, self.rpc_client.commitment())
            .await
            .map_err(|error| PoolTokenTransferExecutorError::Backend(Box::new(error)))?
            .value
        else {
            return Ok(None);
        };

        let signatures = self
            .rpc_client
            .get_signatures_for_address(marker_account_address)
            .await
            .map_err(|error| PoolTokenTransferExecutorError::Backend(Box::new(error)))?;
        let Some(signature) = signatures.first().map(|status| status.signature.clone()) else {
            return Err(PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::MarkerAccountSignatureMissing {
                    marker_account_address: marker_account_address.to_string(),
                },
            )));
        };

        let onchain_transaction_id = OnchainTransactionId::new(signature.clone()).ok_or(
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::InvalidOnchainTransactionId { signature },
            )),
        )?;

        Ok(Some(onchain_transaction_id))
    }

    async fn send_transaction(
        &self,
        instructions: Vec<Instruction>,
    ) -> Result<solana_sdk::signature::Signature, PoolTokenTransferExecutorError> {
        let blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|error| PoolTokenTransferExecutorError::Backend(Box::new(error)))?;

        let payer = self.config.payer().as_ref();
        let pool_token_account_owner = self.config.pool_token_account_owner().as_ref();
        let mut signers: Vec<&dyn Signer> = vec![payer];
        if payer.pubkey() != pool_token_account_owner.pubkey() {
            signers.push(pool_token_account_owner);
        }

        let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
        transaction.try_sign(&signers, blockhash).map_err(|error| {
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::SignTransaction(error),
            ))
        })?;

        self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .await
            .map_err(|error| PoolTokenTransferExecutorError::Backend(Box::new(error)))
    }
}

impl PoolTokenTransferExecutor for SolanaPoolTokenTransferExecutor {
    async fn execute(
        &self,
        request: PoolTokenTransferRequest,
    ) -> Result<PoolTokenTransferReceipt, PoolTokenTransferExecutorError> {
        let pool_token_account_owner_address = self.config.pool_token_account_owner().pubkey();
        let payer = self.config.payer().pubkey();
        let marker_account_address =
            Self::marker_account_address(&pool_token_account_owner_address, &request)?;
        if let Some(onchain_transaction_id) = self
            .recover_onchain_transaction_id(&marker_account_address)
            .await?
        {
            return Ok(PoolTokenTransferReceipt::new(onchain_transaction_id));
        }

        let mint_account_address = Pubkey::from_str(request.mint_account_address().value())
            .map_err(|_| {
                PoolTokenTransferExecutorError::Backend(Box::new(
                    SolanaPoolTokenTransferExecutorError::InvalidPubkey {
                        kind: "mint account address",
                        value: request.mint_account_address().value().to_owned(),
                    },
                ))
            })?;
        let pool_token_account_address =
            Pubkey::from_str(request.pool_token_account_address().value()).map_err(|_| {
                PoolTokenTransferExecutorError::Backend(Box::new(
                    SolanaPoolTokenTransferExecutorError::InvalidPubkey {
                        kind: "pool token account address",
                        value: request.pool_token_account_address().value().to_owned(),
                    },
                ))
            })?;
        let token_program_id =
            Pubkey::from_str(request.token_program_id().value()).map_err(|_| {
                PoolTokenTransferExecutorError::Backend(Box::new(
                    SolanaPoolTokenTransferExecutorError::InvalidPubkey {
                        kind: "token program ID",
                        value: request.token_program_id().value().to_owned(),
                    },
                ))
            })?;
        let destination_owner_address = Pubkey::from_str(
            request.destination_token_account_owner_address().value(),
        )
        .map_err(|_| {
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::InvalidPubkey {
                    kind: "destination token account owner address",
                    value: request
                        .destination_token_account_owner_address()
                        .value()
                        .to_owned(),
                },
            ))
        })?;
        let expected_pool_token_account_address = Self::pool_token_account_address(
            &pool_token_account_owner_address,
            &mint_account_address,
            &token_program_id,
        );
        if pool_token_account_address != expected_pool_token_account_address {
            return Err(PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::PoolTokenAccountAddressMismatch {
                    expected: expected_pool_token_account_address.to_string(),
                    provided: pool_token_account_address.to_string(),
                },
            )));
        }
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
        let marker_account_lamports = self
            .rpc_client
            .get_minimum_balance_for_rent_exemption(0)
            .await
            .map_err(|error| PoolTokenTransferExecutorError::Backend(Box::new(error)))?;

        let instructions = vec![
            system_instruction::create_account_with_seed(
                &payer,
                &marker_account_address,
                &pool_token_account_owner_address,
                request.marker_seed().value(),
                marker_account_lamports,
                0,
                &system_program::id(),
            ),
            associated_token_instruction::create_associated_token_account_idempotent(
                &payer,
                &destination_owner_address,
                &mint_account_address,
                &token_program_id,
            ),
            token_instruction::transfer_checked(
                &token_program_id,
                &pool_token_account_address,
                &mint_account_address,
                &destination_token_account_address,
                &pool_token_account_owner_address,
                &[],
                amount,
                request.decimals().value(),
            )
            .map_err(|error| {
                PoolTokenTransferExecutorError::Backend(Box::new(
                    SolanaPoolTokenTransferExecutorError::TransferInstruction(error),
                ))
            })?,
        ];

        let signature = match self.send_transaction(instructions).await {
            Ok(signature) => signature.to_string(),
            Err(error) => {
                if let Some(onchain_transaction_id) = self
                    .recover_onchain_transaction_id(&marker_account_address)
                    .await?
                {
                    return Ok(PoolTokenTransferReceipt::new(onchain_transaction_id));
                }
                return Err(error);
            }
        };
        let onchain_transaction_id = OnchainTransactionId::new(signature.clone()).ok_or(
            PoolTokenTransferExecutorError::Backend(Box::new(
                SolanaPoolTokenTransferExecutorError::InvalidOnchainTransactionId { signature },
            )),
        )?;
        Ok(PoolTokenTransferReceipt::new(onchain_transaction_id))
    }
}

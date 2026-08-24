use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use appletheia::domain::AggregateId;
use banking_ledger_application::{
    SolanaWithdrawalSettlementExecution, SolanaWithdrawalSettlementExecutor,
    SolanaWithdrawalSettlementRequest, WithdrawalSettlementExecutorError,
};
use banking_ledger_domain::core::{SolanaTransactionSignature, TokenDecimals};
use banking_settlement::{
    BankingSettlementConfig, PoolAuthority, WithdrawalSettlementReceipt, accounts, instruction,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{instruction::Instruction, signature::Signer};
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use spl_associated_token_account_interface::address as associated_token_address;
use spl_token_2022_interface::{extension::StateWithExtensions, state::Mint};

use super::{
    DefaultSolanaWithdrawalSettlementExecutorConfig, DefaultSolanaWithdrawalSettlementExecutorError,
};

pub struct DefaultSolanaWithdrawalSettlementExecutor {
    rpc_client: RpcClient,
    config: DefaultSolanaWithdrawalSettlementExecutorConfig,
}

impl DefaultSolanaWithdrawalSettlementExecutor {
    pub fn new(
        rpc_client: RpcClient,
        config: DefaultSolanaWithdrawalSettlementExecutorConfig,
    ) -> Self {
        Self { rpc_client, config }
    }
}

impl SolanaWithdrawalSettlementExecutor for DefaultSolanaWithdrawalSettlementExecutor {
    async fn execute(
        &self,
        request: SolanaWithdrawalSettlementRequest,
    ) -> Result<SolanaWithdrawalSettlementExecution, WithdrawalSettlementExecutorError> {
        let mint = Pubkey::new_from_array(*request.token_address().address().as_bytes());
        let owner = Pubkey::new_from_array(*request.token_owner_address().address().as_bytes());
        let mint_account = self
            .rpc_client
            .get_account(&mint)
            .await
            .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
        let token_program = mint_account.owner;
        let withdrawal_id = request.withdrawal_id().value().into_bytes();
        let withdrawal_settlement_receipt = Pubkey::find_program_address(
            &[WithdrawalSettlementReceipt::SEED, &withdrawal_id],
            &self.config.program_id,
        )
        .0;
        let authority =
            Pubkey::find_program_address(&[PoolAuthority::SEED], &self.config.program_id).0;
        let mint_state = StateWithExtensions::<Mint>::unpack(&mint_account.data)
            .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
        let domain_token_amount = request
            .amount()
            .to_token_amount(
                request.currency_decimals(),
                TokenDecimals::new(mint_state.base.decimals),
            )
            .map_err(|_| WithdrawalSettlementExecutorError::InvalidAmount)?;
        let token_amount = u64::try_from(domain_token_amount.value())
            .map_err(|_| WithdrawalSettlementExecutorError::InvalidAmount)?;
        let existing_receipt = self
            .rpc_client
            .get_account_with_commitment(
                &withdrawal_settlement_receipt,
                CommitmentConfig::confirmed(),
            )
            .await
            .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?
            .value;
        if let Some(receipt_account) = existing_receipt {
            if receipt_account.owner != self.config.program_id {
                return Err(WithdrawalSettlementExecutorError::Backend(Box::new(
                    DefaultSolanaWithdrawalSettlementExecutorError::UnexpectedReceiptOwner,
                )));
            }
            let receipt =
                WithdrawalSettlementReceipt::try_deserialize(&mut receipt_account.data.as_slice())
                    .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
            if receipt.version != WithdrawalSettlementReceipt::VERSION
                || receipt.mint != mint
                || receipt.token_account_owner != owner
                || receipt.token_amount != token_amount
            {
                return Err(WithdrawalSettlementExecutorError::Backend(Box::new(
                    DefaultSolanaWithdrawalSettlementExecutorError::ReceiptMismatch,
                )));
            }
            let signature = self
                .rpc_client
                .get_signatures_for_address(&withdrawal_settlement_receipt)
                .await
                .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?
                .into_iter()
                .find(|record| record.err.is_none())
                .ok_or_else(|| {
                    WithdrawalSettlementExecutorError::Backend(Box::new(
                        DefaultSolanaWithdrawalSettlementExecutorError::SuccessfulTransactionNotFound,
                    ))
                })?;
            let transaction_id = SolanaTransactionSignature::new(signature.signature)
                .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
            return Ok(SolanaWithdrawalSettlementExecution { transaction_id });
        }
        let instruction = Instruction {
            program_id: self.config.program_id,
            accounts: accounts::WithdrawalSettleInstructionAccounts {
                payer: self.config.payer.pubkey(),
                banking_settlement_config: Pubkey::find_program_address(
                    &[BankingSettlementConfig::SEED],
                    &self.config.program_id,
                )
                .0,
                operator: self.config.operator.pubkey(),
                withdrawal_settlement_receipt,
                pool_authority: authority,
                mint,
                pool_token_account:
                    associated_token_address::get_associated_token_address_with_program_id(
                        &authority,
                        &mint,
                        &token_program,
                    ),
                token_account_owner: owner,
                destination_token_account:
                    associated_token_address::get_associated_token_address_with_program_id(
                        &owner,
                        &mint,
                        &token_program,
                    ),
                system_program: system_program::id(),
                token_program,
                associated_token_program: spl_associated_token_account_interface::program::id(),
            }
            .to_account_metas(None),
            data: instruction::SettleWithdrawal {
                withdrawal_id,
                token_amount,
            }
            .data(),
        };
        let blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
        let mut transaction =
            Transaction::new_with_payer(&[instruction], Some(&self.config.payer.pubkey()));
        let mut signers: Vec<&dyn Signer> = vec![self.config.payer.as_ref()];
        if self.config.operator.pubkey() != self.config.payer.pubkey() {
            signers.push(self.config.operator.as_ref());
        }
        transaction
            .try_sign(&signers, blockhash)
            .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
        let signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .await
            .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;

        let receipt_account = self
            .rpc_client
            .get_account(&withdrawal_settlement_receipt)
            .await
            .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
        if receipt_account.owner != self.config.program_id {
            return Err(WithdrawalSettlementExecutorError::Backend(Box::new(
                DefaultSolanaWithdrawalSettlementExecutorError::UnexpectedReceiptOwner,
            )));
        }
        let receipt =
            WithdrawalSettlementReceipt::try_deserialize(&mut receipt_account.data.as_slice())
                .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
        if receipt.version != WithdrawalSettlementReceipt::VERSION
            || receipt.mint != mint
            || receipt.token_account_owner != owner
            || receipt.token_amount != token_amount
        {
            return Err(WithdrawalSettlementExecutorError::Backend(Box::new(
                DefaultSolanaWithdrawalSettlementExecutorError::ReceiptMismatch,
            )));
        }
        let transaction_id = SolanaTransactionSignature::new(signature.to_string())
            .map_err(|error| WithdrawalSettlementExecutorError::Backend(Box::new(error)))?;
        Ok(SolanaWithdrawalSettlementExecution { transaction_id })
    }
}

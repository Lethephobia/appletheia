use anchor_lang::{InstructionData, ToAccountMetas};
use appletheia::domain::AggregateId;
use banking_ledger_application::{
    DepositSettlementPreparerError, SolanaDepositSettlementPreparation,
    SolanaDepositSettlementPrepareRequest, SolanaDepositSettlementPreparer,
    SolanaPreparedDepositTransaction,
};
use banking_ledger_domain::core::TokenDecimals;
use banking_settlement::{
    BankingSettlementConfig, DepositSettlementReceipt, PoolAuthority, accounts, instruction,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use spl_associated_token_account_interface::address as associated_token_address;
use spl_token_2022_interface::{extension::StateWithExtensions, state::Mint};

use super::DefaultSolanaDepositSettlementPreparerConfig;

pub struct DefaultSolanaDepositSettlementPreparer {
    rpc_client: RpcClient,
    config: DefaultSolanaDepositSettlementPreparerConfig,
}

impl DefaultSolanaDepositSettlementPreparer {
    pub fn new(
        rpc_client: RpcClient,
        config: DefaultSolanaDepositSettlementPreparerConfig,
    ) -> Self {
        Self { rpc_client, config }
    }

    fn receipt_address(&self, deposit_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(
            &[DepositSettlementReceipt::SEED, deposit_id],
            &self.config.program_id,
        )
        .0
    }

    fn pool_authority_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[PoolAuthority::SEED], &self.config.program_id).0
    }
}

impl SolanaDepositSettlementPreparer for DefaultSolanaDepositSettlementPreparer {
    async fn prepare(
        &self,
        request: SolanaDepositSettlementPrepareRequest,
    ) -> Result<SolanaDepositSettlementPreparation, DepositSettlementPreparerError> {
        let mint = Pubkey::new_from_array(*request.token_address().address().as_bytes());
        let owner = Pubkey::new_from_array(*request.token_owner_address().address().as_bytes());
        let mint_account = self
            .rpc_client
            .get_account(&mint)
            .await
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;
        let token_program = mint_account.owner;
        let deposit_id = request.deposit_id().value().into_bytes();
        let authority = self.pool_authority_address();
        let mint_state = StateWithExtensions::<Mint>::unpack(&mint_account.data)
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;
        let domain_token_amount = request
            .amount()
            .to_token_amount(
                request.currency_decimals(),
                TokenDecimals::new(mint_state.base.decimals),
            )
            .map_err(|_| DepositSettlementPreparerError::InvalidAmount)?;
        let solana_token_amount = u64::try_from(domain_token_amount.value())
            .map_err(|_| DepositSettlementPreparerError::InvalidAmount)?;
        let instruction = Instruction {
            program_id: self.config.program_id,
            accounts: accounts::DepositSettleInstructionAccounts {
                payer: owner,
                banking_settlement_config: Pubkey::find_program_address(
                    &[BankingSettlementConfig::SEED],
                    &self.config.program_id,
                )
                .0,
                operator: self.config.operator.pubkey(),
                deposit_settlement_receipt: self.receipt_address(&deposit_id),
                pool_authority: authority,
                mint,
                pool_token_account:
                    associated_token_address::get_associated_token_address_with_program_id(
                        &authority,
                        &mint,
                        &token_program,
                    ),
                token_account_owner: owner,
                source_token_account:
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
            data: instruction::SettleDeposit {
                deposit_id,
                token_amount: solana_token_amount,
            }
            .data(),
        };
        let blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;
        let mut transaction =
            Transaction::new_unsigned(solana_sdk::message::Message::new_with_blockhash(
                &[instruction],
                Some(&owner),
                &blockhash,
            ));
        transaction
            .try_partial_sign(&[self.config.operator.as_ref()], blockhash)
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;
        let bytes = bincode::serialize(&transaction)
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;

        Ok(SolanaDepositSettlementPreparation {
            transaction: SolanaPreparedDepositTransaction::from_bytes(bytes),
        })
    }
}

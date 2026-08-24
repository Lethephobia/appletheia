use anchor_lang::AccountDeserialize;
use appletheia::domain::AggregateId;
use banking_ledger_application::{
    DepositSettlementVerifierError, SolanaDepositSettlementVerification,
    SolanaDepositSettlementVerifier, SolanaDepositSettlementVerifyRequest,
};
use banking_ledger_domain::core::TokenDecimals;
use banking_settlement::{DepositSettlementReceipt, PoolAuthority};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account_interface::address as associated_token_address;
use spl_token_2022_interface::{extension::StateWithExtensions, state::Mint};

use super::{
    DefaultSolanaDepositSettlementVerifierConfig, DefaultSolanaDepositSettlementVerifierError,
};

pub struct DefaultSolanaDepositSettlementVerifier {
    rpc_client: RpcClient,
    config: DefaultSolanaDepositSettlementVerifierConfig,
}

impl DefaultSolanaDepositSettlementVerifier {
    pub fn new(
        rpc_client: RpcClient,
        config: DefaultSolanaDepositSettlementVerifierConfig,
    ) -> Self {
        Self { rpc_client, config }
    }
}

impl SolanaDepositSettlementVerifier for DefaultSolanaDepositSettlementVerifier {
    async fn verify(
        &self,
        request: SolanaDepositSettlementVerifyRequest,
    ) -> Result<SolanaDepositSettlementVerification, DepositSettlementVerifierError> {
        let transaction_id = request.transaction_id();
        let deposit_id = request.deposit_id().value().into_bytes();
        let receipt_address = Pubkey::find_program_address(
            &[DepositSettlementReceipt::SEED, &deposit_id],
            &self.config.program_id,
        )
        .0;
        let account = self
            .rpc_client
            .get_account(&receipt_address)
            .await
            .map_err(|error| DepositSettlementVerifierError::Backend(Box::new(error)))?;
        let receipt_signatures = self
            .rpc_client
            .get_signatures_for_address(&receipt_address)
            .await
            .map_err(|error| DepositSettlementVerifierError::Backend(Box::new(error)))?;
        let expected_transaction_signature = transaction_id.to_string();
        if !receipt_signatures.iter().any(|record| {
            record.err.is_none() && record.signature == expected_transaction_signature
        }) {
            return Err(DepositSettlementVerifierError::Backend(Box::new(
                DefaultSolanaDepositSettlementVerifierError::ExpectedReceiptNotCreated,
            )));
        }
        if account.owner != self.config.program_id {
            return Err(DepositSettlementVerifierError::Backend(Box::new(
                DefaultSolanaDepositSettlementVerifierError::UnexpectedReceiptOwner,
            )));
        }
        let receipt = DepositSettlementReceipt::try_deserialize(&mut account.data.as_slice())
            .map_err(|error| DepositSettlementVerifierError::Backend(Box::new(error)))?;
        let expected_mint = Pubkey::new_from_array(*request.token_address().address().as_bytes());
        let mint_account = self
            .rpc_client
            .get_account(&expected_mint)
            .await
            .map_err(|error| DepositSettlementVerifierError::Backend(Box::new(error)))?;
        let mint_state = StateWithExtensions::<Mint>::unpack(&mint_account.data)
            .map_err(|error| DepositSettlementVerifierError::Backend(Box::new(error)))?;
        let domain_token_amount = request
            .amount()
            .to_token_amount(
                request.currency_decimals(),
                TokenDecimals::new(mint_state.base.decimals),
            )
            .map_err(|_| DepositSettlementVerifierError::InvalidAmount)?;
        let expected_amount = u64::try_from(domain_token_amount.value())
            .map_err(|_| DepositSettlementVerifierError::InvalidAmount)?;
        let pool_authority =
            Pubkey::find_program_address(&[PoolAuthority::SEED], &self.config.program_id).0;
        let expected_pool_token_account =
            associated_token_address::get_associated_token_address_with_program_id(
                &pool_authority,
                &expected_mint,
                &mint_account.owner,
            );
        if receipt.version != DepositSettlementReceipt::VERSION
            || receipt.mint != expected_mint
            || receipt.pool_token_account != expected_pool_token_account
            || receipt.token_amount != expected_amount
        {
            return Err(DepositSettlementVerifierError::Backend(Box::new(
                DefaultSolanaDepositSettlementVerifierError::ReceiptMismatch,
            )));
        }
        Ok(SolanaDepositSettlementVerification { transaction_id })
    }
}

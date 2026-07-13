use anchor_lang::AccountDeserialize;
use banking_ledger::PoolTokenDepositReceipt;
use banking_ledger_application::mint::{TokenDepositVerifier, TokenDepositVerifierError};
use banking_ledger_domain::deposit::DepositId;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use super::{
    PoolTokenDepositId, SolanaTokenDepositVerifierConfig, SolanaTokenDepositVerifierError,
};

/// Solana implementation of `TokenDepositVerifier`.
pub struct SolanaTokenDepositVerifier {
    rpc_client: RpcClient,
    config: SolanaTokenDepositVerifierConfig,
}

impl SolanaTokenDepositVerifier {
    pub fn new(rpc_client: RpcClient, config: SolanaTokenDepositVerifierConfig) -> Self {
        Self { rpc_client, config }
    }

    fn receipt_address(&self, deposit_id: DepositId) -> Pubkey {
        let pool_token_deposit_id = PoolTokenDepositId::from(deposit_id).into_bytes();
        Pubkey::find_program_address(
            &[PoolTokenDepositReceipt::SEED, &pool_token_deposit_id],
            self.config.program_id(),
        )
        .0
    }
}

impl TokenDepositVerifier for SolanaTokenDepositVerifier {
    async fn verify(&self, deposit_id: DepositId) -> Result<(), TokenDepositVerifierError> {
        let receipt_address = self.receipt_address(deposit_id);
        let data = self
            .rpc_client
            .get_account_data(&receipt_address)
            .await
            .map_err(|error| TokenDepositVerifierError::Backend(Box::new(error)))?;
        let receipt =
            PoolTokenDepositReceipt::try_deserialize(&mut data.as_slice()).map_err(|error| {
                TokenDepositVerifierError::Backend(Box::new(
                    SolanaTokenDepositVerifierError::InvalidReceipt(error),
                ))
            })?;

        if receipt.version != PoolTokenDepositReceipt::VERSION {
            return Err(TokenDepositVerifierError::Backend(Box::new(
                SolanaTokenDepositVerifierError::UnsupportedReceiptVersion(receipt.version),
            )));
        }

        Ok(())
    }
}

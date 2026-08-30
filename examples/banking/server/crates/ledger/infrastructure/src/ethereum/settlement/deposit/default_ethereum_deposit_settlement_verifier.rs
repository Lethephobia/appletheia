use alloy::primitives::{Address, B256, FixedBytes, U256, keccak256};
use alloy::providers::{DynProvider, Provider};
use alloy::sol_types::SolValue;
use appletheia::domain::AggregateId;
use banking_ledger_application::{
    DepositSettlementVerifierError, EthereumDepositSettlementVerification,
    EthereumDepositSettlementVerifier, EthereumDepositSettlementVerifyRequest,
};
use banking_ledger_domain::core::TokenDecimals;

use super::{
    DefaultEthereumDepositSettlementVerifierConfig, DefaultEthereumDepositSettlementVerifierError,
};
use crate::ethereum::contract::{BankingSettlement, IERC20Metadata};

pub struct DefaultEthereumDepositSettlementVerifier {
    provider: DynProvider,
    config: DefaultEthereumDepositSettlementVerifierConfig,
}

impl DefaultEthereumDepositSettlementVerifier {
    pub fn new(
        provider: DynProvider,
        config: DefaultEthereumDepositSettlementVerifierConfig,
    ) -> Self {
        Self { provider, config }
    }
}

impl EthereumDepositSettlementVerifier for DefaultEthereumDepositSettlementVerifier {
    async fn verify(
        &self,
        request: EthereumDepositSettlementVerifyRequest,
    ) -> Result<EthereumDepositSettlementVerification, DepositSettlementVerifierError> {
        let transaction_id = request.transaction_id();
        let transaction_hash = B256::from(*transaction_id.as_bytes());
        let receipt = self
            .provider
            .get_transaction_receipt(transaction_hash)
            .await
            .map_err(|error| {
                DepositSettlementVerifierError::Backend(Box::new(
                    DefaultEthereumDepositSettlementVerifierError::Rpc(error),
                ))
            })?
            .ok_or_else(|| {
                DepositSettlementVerifierError::Backend(Box::new(
                    DefaultEthereumDepositSettlementVerifierError::TransactionNotFound,
                ))
            })?;
        if !receipt.status() {
            return Err(DepositSettlementVerifierError::Backend(Box::new(
                DefaultEthereumDepositSettlementVerifierError::TransactionFailed,
            )));
        }

        let settlement_address = Address::from(*self.config.settlement_contract.as_bytes());
        let token_owner_address =
            Address::from(*request.token_owner_address().address().as_bytes());
        if receipt.from != token_owner_address {
            return Err(DepositSettlementVerifierError::Backend(Box::new(
                DefaultEthereumDepositSettlementVerifierError::UnexpectedSender,
            )));
        }
        if receipt.to != Some(settlement_address) {
            return Err(DepositSettlementVerifierError::Backend(Box::new(
                DefaultEthereumDepositSettlementVerifierError::UnexpectedReceiver,
            )));
        }

        let token_address = Address::from(*request.token_address().address().as_bytes());
        let token_decimals = IERC20Metadata::new(token_address, &self.provider)
            .decimals()
            .call()
            .await
            .map_err(|error| {
                DepositSettlementVerifierError::Backend(Box::new(
                    DefaultEthereumDepositSettlementVerifierError::Contract(error),
                ))
            })?;
        let amount = request
            .amount()
            .to_token_amount(
                request.currency_decimals(),
                TokenDecimals::new(token_decimals),
            )
            .map_err(|_| DepositSettlementVerifierError::InvalidAmount)?;
        let amount = U256::from(amount.value());
        let expected_settlement_hash =
            keccak256((token_address, token_owner_address, amount).abi_encode());
        let deposit_id = FixedBytes::<16>::from(request.deposit_id().value().into_bytes());
        let recorded_settlement_hash = BankingSettlement::new(settlement_address, &self.provider)
            .depositSettlementHash(deposit_id)
            .call()
            .await
            .map_err(|error| {
                DepositSettlementVerifierError::Backend(Box::new(
                    DefaultEthereumDepositSettlementVerifierError::Contract(error),
                ))
            })?;
        if recorded_settlement_hash != expected_settlement_hash {
            return Err(DepositSettlementVerifierError::Backend(Box::new(
                DefaultEthereumDepositSettlementVerifierError::SettlementMismatch,
            )));
        }

        Ok(EthereumDepositSettlementVerification { transaction_id })
    }
}

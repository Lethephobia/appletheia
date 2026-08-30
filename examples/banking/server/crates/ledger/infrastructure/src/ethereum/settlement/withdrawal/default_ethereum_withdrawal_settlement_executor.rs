use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::DynProvider;
use appletheia::domain::AggregateId;
use banking_ledger_application::{
    EthereumWithdrawalSettlementExecution, EthereumWithdrawalSettlementExecutor,
    EthereumWithdrawalSettlementRequest, WithdrawalSettlementExecutorError,
};
use banking_ledger_domain::core::{EvmTransactionHash, TokenDecimals};

use super::{
    DefaultEthereumWithdrawalSettlementExecutorConfig,
    DefaultEthereumWithdrawalSettlementExecutorError,
};
use crate::ethereum::contract::{BankingSettlement, IERC20Metadata};

pub struct DefaultEthereumWithdrawalSettlementExecutor {
    provider: DynProvider,
    config: DefaultEthereumWithdrawalSettlementExecutorConfig,
}

impl DefaultEthereumWithdrawalSettlementExecutor {
    pub fn new(
        provider: DynProvider,
        config: DefaultEthereumWithdrawalSettlementExecutorConfig,
    ) -> Self {
        Self { provider, config }
    }
}

impl EthereumWithdrawalSettlementExecutor for DefaultEthereumWithdrawalSettlementExecutor {
    async fn execute(
        &self,
        request: EthereumWithdrawalSettlementRequest,
    ) -> Result<EthereumWithdrawalSettlementExecution, WithdrawalSettlementExecutorError> {
        let settlement_address = Address::from(*self.config.settlement_contract.as_bytes());
        let token_address = Address::from(*request.token_address().address().as_bytes());
        let token_owner_address =
            Address::from(*request.token_owner_address().address().as_bytes());
        let token_decimals = IERC20Metadata::new(token_address, &self.provider)
            .decimals()
            .call()
            .await
            .map_err(|error| {
                WithdrawalSettlementExecutorError::Backend(Box::new(
                    DefaultEthereumWithdrawalSettlementExecutorError::Contract(error),
                ))
            })?;
        let amount = request
            .amount()
            .to_token_amount(
                request.currency_decimals(),
                TokenDecimals::new(token_decimals),
            )
            .map_err(|_| WithdrawalSettlementExecutorError::InvalidAmount)?;
        let withdrawal_id = FixedBytes::<16>::from(request.withdrawal_id().value().into_bytes());
        let receipt = BankingSettlement::new(settlement_address, &self.provider)
            .settleWithdrawal(BankingSettlement::WithdrawalSettlement {
                withdrawalId: withdrawal_id,
                token: token_address,
                tokenOwner: token_owner_address,
                amount: U256::from(amount.value()),
            })
            .send()
            .await
            .map_err(|error| {
                WithdrawalSettlementExecutorError::Backend(Box::new(
                    DefaultEthereumWithdrawalSettlementExecutorError::Contract(error),
                ))
            })?
            .get_receipt()
            .await
            .map_err(|error| {
                WithdrawalSettlementExecutorError::Backend(Box::new(
                    DefaultEthereumWithdrawalSettlementExecutorError::PendingTransaction(error),
                ))
            })?;
        let transaction_id = EvmTransactionHash::from_bytes(*receipt.transaction_hash.as_ref());

        Ok(EthereumWithdrawalSettlementExecution { transaction_id })
    }
}

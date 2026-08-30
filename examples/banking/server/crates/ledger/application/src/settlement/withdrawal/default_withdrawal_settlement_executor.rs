use banking_ledger_domain::core::{
    ChainNetwork, OnchainTransactionId, TokenAddress, TokenOwnerAddress,
};

use super::{
    EthereumWithdrawalSettlementExecutor, EthereumWithdrawalSettlementRequest,
    SolanaWithdrawalSettlementExecutor, SolanaWithdrawalSettlementRequest,
    WithdrawalSettlementExecution, WithdrawalSettlementExecutor, WithdrawalSettlementExecutorError,
    WithdrawalSettlementRequest,
};

pub struct DefaultWithdrawalSettlementExecutor<S, E>
where
    S: SolanaWithdrawalSettlementExecutor,
    E: EthereumWithdrawalSettlementExecutor,
{
    solana: S,
    ethereum: E,
}

impl<S, E> DefaultWithdrawalSettlementExecutor<S, E>
where
    S: SolanaWithdrawalSettlementExecutor,
    E: EthereumWithdrawalSettlementExecutor,
{
    pub fn new(solana: S, ethereum: E) -> Self {
        Self { solana, ethereum }
    }
}

impl<S, E> WithdrawalSettlementExecutor for DefaultWithdrawalSettlementExecutor<S, E>
where
    S: SolanaWithdrawalSettlementExecutor,
    E: EthereumWithdrawalSettlementExecutor,
{
    async fn execute(
        &self,
        request: WithdrawalSettlementRequest,
    ) -> Result<WithdrawalSettlementExecution, WithdrawalSettlementExecutorError> {
        let transaction_id = match (
            request.chain_network(),
            *request.token_address(),
            *request.token_owner_address(),
        ) {
            (
                ChainNetwork::Solana,
                TokenAddress::Solana(token_address),
                TokenOwnerAddress::Solana(token_owner_address),
            ) => {
                let execution = self
                    .solana
                    .execute(SolanaWithdrawalSettlementRequest::new(
                        request.withdrawal_id(),
                        request.currency_decimals(),
                        token_address,
                        token_owner_address,
                        request.amount(),
                    ))
                    .await?;
                OnchainTransactionId::Solana(execution.transaction_id)
            }
            (
                ChainNetwork::Ethereum,
                TokenAddress::Ethereum(token_address),
                TokenOwnerAddress::Ethereum(token_owner_address),
            ) => {
                let execution = self
                    .ethereum
                    .execute(EthereumWithdrawalSettlementRequest::new(
                        request.withdrawal_id(),
                        request.currency_decimals(),
                        token_address,
                        token_owner_address,
                        request.amount(),
                    ))
                    .await?;
                OnchainTransactionId::Ethereum(execution.transaction_id)
            }
            _ => return Err(WithdrawalSettlementExecutorError::InconsistentChainValues),
        };

        Ok(WithdrawalSettlementExecution { transaction_id })
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::core::{
        ChainNetwork, CurrencyAmount, CurrencyDecimals, EvmAddress, EvmTokenContractAddress,
        EvmTokenOwnerAddress, EvmTransactionHash, OnchainTransactionId, SolanaAccountAddress,
        SolanaMintAccountAddress, SolanaTokenOwnerAddress, SolanaTransactionSignature,
        TokenAddress, TokenOwnerAddress,
    };
    use banking_ledger_domain::withdrawal::WithdrawalId;

    use crate::settlement::{
        EthereumWithdrawalSettlementExecution, SolanaWithdrawalSettlementExecution,
    };

    use super::{
        DefaultWithdrawalSettlementExecutor, EthereumWithdrawalSettlementExecutor,
        EthereumWithdrawalSettlementRequest, SolanaWithdrawalSettlementExecutor,
        SolanaWithdrawalSettlementRequest, WithdrawalSettlementExecutor,
        WithdrawalSettlementExecutorError, WithdrawalSettlementRequest,
    };

    struct StubSolanaWithdrawalSettlementExecutor;

    impl SolanaWithdrawalSettlementExecutor for StubSolanaWithdrawalSettlementExecutor {
        async fn execute(
            &self,
            _request: SolanaWithdrawalSettlementRequest,
        ) -> Result<SolanaWithdrawalSettlementExecution, WithdrawalSettlementExecutorError>
        {
            Ok(SolanaWithdrawalSettlementExecution {
                transaction_id: SolanaTransactionSignature::from_bytes([1; 64]),
            })
        }
    }

    struct StubEthereumWithdrawalSettlementExecutor;

    impl EthereumWithdrawalSettlementExecutor for StubEthereumWithdrawalSettlementExecutor {
        async fn execute(
            &self,
            _request: EthereumWithdrawalSettlementRequest,
        ) -> Result<EthereumWithdrawalSettlementExecution, WithdrawalSettlementExecutorError>
        {
            Ok(EthereumWithdrawalSettlementExecution {
                transaction_id: EvmTransactionHash::from_bytes([2; 32]),
            })
        }
    }

    #[tokio::test]
    async fn routes_withdrawals_to_the_executor_for_the_selected_chain() {
        let executor = DefaultWithdrawalSettlementExecutor::new(
            StubSolanaWithdrawalSettlementExecutor,
            StubEthereumWithdrawalSettlementExecutor,
        );

        let solana_execution = executor
            .execute(solana_request())
            .await
            .expect("Solana withdrawal should be routed");
        assert_eq!(
            solana_execution.transaction_id,
            OnchainTransactionId::Solana(SolanaTransactionSignature::from_bytes([1; 64]))
        );

        let ethereum_execution = executor
            .execute(ethereum_request())
            .await
            .expect("Ethereum withdrawal should be routed");
        assert_eq!(
            ethereum_execution.transaction_id,
            OnchainTransactionId::Ethereum(EvmTransactionHash::from_bytes([2; 32]))
        );
    }

    #[tokio::test]
    async fn rejects_values_from_different_chains_before_dispatch() {
        let executor = DefaultWithdrawalSettlementExecutor::new(
            StubSolanaWithdrawalSettlementExecutor,
            StubEthereumWithdrawalSettlementExecutor,
        );
        let request = WithdrawalSettlementRequest::new(
            WithdrawalId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Solana,
            TokenAddress::Ethereum(EvmTokenContractAddress::new(EvmAddress::from_bytes(
                [3; 20],
            ))),
            TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
                SolanaAccountAddress::from_bytes([4; 32]),
            )),
            CurrencyAmount::new(100),
        );

        assert!(matches!(
            executor.execute(request).await,
            Err(WithdrawalSettlementExecutorError::InconsistentChainValues)
        ));
    }

    fn solana_request() -> WithdrawalSettlementRequest {
        WithdrawalSettlementRequest::new(
            WithdrawalId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Solana,
            TokenAddress::Solana(SolanaMintAccountAddress::new(
                SolanaAccountAddress::from_bytes([3; 32]),
            )),
            TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
                SolanaAccountAddress::from_bytes([4; 32]),
            )),
            CurrencyAmount::new(100),
        )
    }

    fn ethereum_request() -> WithdrawalSettlementRequest {
        WithdrawalSettlementRequest::new(
            WithdrawalId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Ethereum,
            TokenAddress::Ethereum(EvmTokenContractAddress::new(EvmAddress::from_bytes(
                [3; 20],
            ))),
            TokenOwnerAddress::Ethereum(EvmTokenOwnerAddress::new(EvmAddress::from_bytes([4; 20]))),
            CurrencyAmount::new(100),
        )
    }
}

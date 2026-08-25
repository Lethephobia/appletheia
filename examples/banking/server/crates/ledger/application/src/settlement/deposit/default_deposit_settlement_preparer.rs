use banking_ledger_domain::core::{ChainNetwork, TokenAddress, TokenOwnerAddress};

use super::{
    DepositSettlementPreparation, DepositSettlementPrepareRequest, DepositSettlementPreparer,
    DepositSettlementPreparerError, EthereumDepositSettlementPrepareRequest,
    EthereumDepositSettlementPreparer, SolanaDepositSettlementPrepareRequest,
    SolanaDepositSettlementPreparer,
};

pub struct DefaultDepositSettlementPreparer<S, E>
where
    S: SolanaDepositSettlementPreparer,
    E: EthereumDepositSettlementPreparer,
{
    solana: S,
    ethereum: E,
}

impl<S, E> DefaultDepositSettlementPreparer<S, E>
where
    S: SolanaDepositSettlementPreparer,
    E: EthereumDepositSettlementPreparer,
{
    pub fn new(solana: S, ethereum: E) -> Self {
        Self { solana, ethereum }
    }
}

impl<S, E> DepositSettlementPreparer for DefaultDepositSettlementPreparer<S, E>
where
    S: SolanaDepositSettlementPreparer,
    E: EthereumDepositSettlementPreparer,
{
    async fn prepare(
        &self,
        request: DepositSettlementPrepareRequest,
    ) -> Result<DepositSettlementPreparation, DepositSettlementPreparerError> {
        let preparation = match (
            request.chain_network(),
            *request.token_address(),
            *request.token_owner_address(),
        ) {
            (
                ChainNetwork::Solana(network),
                TokenAddress::Solana(token_address),
                TokenOwnerAddress::Solana(token_owner_address),
            ) => {
                let solana_preparation = self
                    .solana
                    .prepare(SolanaDepositSettlementPrepareRequest::new(
                        request.deposit_id(),
                        request.currency_decimals(),
                        network,
                        token_address,
                        token_owner_address,
                        request.amount(),
                    ))
                    .await?;
                DepositSettlementPreparation::Solana(solana_preparation.transaction)
            }
            (
                ChainNetwork::Ethereum(network),
                TokenAddress::Ethereum(token_address),
                TokenOwnerAddress::Ethereum(token_owner_address),
            ) => {
                let ethereum_preparation = self
                    .ethereum
                    .prepare(EthereumDepositSettlementPrepareRequest::new(
                        request.deposit_id(),
                        request.currency_decimals(),
                        network,
                        token_address,
                        token_owner_address,
                        request.amount(),
                    ))
                    .await?;
                DepositSettlementPreparation::Ethereum(ethereum_preparation.transaction_request)
            }
            _ => return Err(DepositSettlementPreparerError::InconsistentChainValues),
        };

        Ok(preparation)
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::core::{
        ChainNetwork, CurrencyAmount, CurrencyDecimals, EthereumNetwork, EvmAddress, EvmChainId,
        EvmTokenContractAddress, EvmTokenOwnerAddress, SolanaAccountAddress,
        SolanaMintAccountAddress, SolanaNetwork, SolanaTokenOwnerAddress, TokenAddress,
        TokenOwnerAddress,
    };
    use banking_ledger_domain::deposit::DepositId;

    use crate::settlement::{
        DepositSettlementPreparation, EthereumDepositSettlementPreparation, EvmCallData,
        EvmTransactionRequest, SolanaDepositSettlementPreparation,
        SolanaPreparedDepositTransaction,
    };

    use super::{
        DefaultDepositSettlementPreparer, DepositSettlementPrepareRequest,
        DepositSettlementPreparer, DepositSettlementPreparerError,
        EthereumDepositSettlementPrepareRequest, EthereumDepositSettlementPreparer,
        SolanaDepositSettlementPrepareRequest, SolanaDepositSettlementPreparer,
    };

    struct StubSolanaDepositSettlementPreparer;

    impl SolanaDepositSettlementPreparer for StubSolanaDepositSettlementPreparer {
        async fn prepare(
            &self,
            _request: SolanaDepositSettlementPrepareRequest,
        ) -> Result<SolanaDepositSettlementPreparation, DepositSettlementPreparerError> {
            Ok(SolanaDepositSettlementPreparation {
                transaction: SolanaPreparedDepositTransaction::from_bytes(b"solana".to_vec()),
            })
        }
    }

    struct StubEthereumDepositSettlementPreparer;

    impl EthereumDepositSettlementPreparer for StubEthereumDepositSettlementPreparer {
        async fn prepare(
            &self,
            _request: EthereumDepositSettlementPrepareRequest,
        ) -> Result<EthereumDepositSettlementPreparation, DepositSettlementPreparerError> {
            Ok(EthereumDepositSettlementPreparation {
                transaction_request: EvmTransactionRequest::new(
                    EvmChainId::new(11_155_111),
                    EvmTokenOwnerAddress::new(EvmAddress::from_bytes([4; 20])),
                    EvmAddress::from_bytes([5; 20]),
                    EvmCallData::from_bytes(b"ethereum".to_vec()),
                ),
            })
        }
    }

    #[tokio::test]
    async fn routes_preparation_to_the_preparer_for_the_selected_chain() {
        let preparer = DefaultDepositSettlementPreparer::new(
            StubSolanaDepositSettlementPreparer,
            StubEthereumDepositSettlementPreparer,
        );

        let solana_preparation = preparer
            .prepare(solana_request())
            .await
            .expect("Solana deposit should be routed");
        assert!(matches!(
            solana_preparation,
            DepositSettlementPreparation::Solana(ref transaction)
                if transaction.as_bytes() == b"solana"
        ));

        let ethereum_preparation = preparer
            .prepare(ethereum_request())
            .await
            .expect("Ethereum deposit should be routed");
        assert!(matches!(
            ethereum_preparation,
            DepositSettlementPreparation::Ethereum(ref transaction_request)
                if transaction_request.call_data().as_bytes() == b"ethereum"
        ));
    }

    #[tokio::test]
    async fn rejects_prepare_values_from_different_chains_before_dispatch() {
        let preparer = DefaultDepositSettlementPreparer::new(
            StubSolanaDepositSettlementPreparer,
            StubEthereumDepositSettlementPreparer,
        );
        let request = DepositSettlementPrepareRequest::new(
            DepositId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Solana(SolanaNetwork::Devnet),
            TokenAddress::Ethereum(EvmTokenContractAddress::new(EvmAddress::from_bytes(
                [3; 20],
            ))),
            TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
                SolanaAccountAddress::from_bytes([4; 32]),
            )),
            CurrencyAmount::new(100),
        );

        assert!(matches!(
            preparer.prepare(request).await,
            Err(DepositSettlementPreparerError::InconsistentChainValues)
        ));
    }

    fn solana_request() -> DepositSettlementPrepareRequest {
        DepositSettlementPrepareRequest::new(
            DepositId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Solana(SolanaNetwork::Devnet),
            TokenAddress::Solana(SolanaMintAccountAddress::new(
                SolanaAccountAddress::from_bytes([3; 32]),
            )),
            TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
                SolanaAccountAddress::from_bytes([4; 32]),
            )),
            CurrencyAmount::new(100),
        )
    }

    fn ethereum_request() -> DepositSettlementPrepareRequest {
        DepositSettlementPrepareRequest::new(
            DepositId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Ethereum(EthereumNetwork::Sepolia),
            TokenAddress::Ethereum(EvmTokenContractAddress::new(EvmAddress::from_bytes(
                [3; 20],
            ))),
            TokenOwnerAddress::Ethereum(EvmTokenOwnerAddress::new(EvmAddress::from_bytes([4; 20]))),
            CurrencyAmount::new(100),
        )
    }
}

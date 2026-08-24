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
        let transaction = match (
            request.chain_network(),
            *request.token_address(),
            *request.token_owner_address(),
        ) {
            (
                ChainNetwork::Solana(network),
                TokenAddress::Solana(token_address),
                TokenOwnerAddress::Solana(token_owner_address),
            ) => {
                let preparation = self
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
                preparation.transaction
            }
            (
                ChainNetwork::Ethereum(network),
                TokenAddress::Ethereum(token_address),
                TokenOwnerAddress::Ethereum(token_owner_address),
            ) => {
                let preparation = self
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
                preparation.transaction
            }
            _ => return Err(DepositSettlementPreparerError::InconsistentChainValues),
        };

        Ok(DepositSettlementPreparation { transaction })
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::core::{
        ChainNetwork, CurrencyAmount, CurrencyDecimals, EthereumNetwork, EvmAddress,
        EvmTokenContractAddress, EvmTokenOwnerAddress, SolanaAccountAddress,
        SolanaMintAccountAddress, SolanaNetwork, SolanaTokenOwnerAddress, TokenAddress,
        TokenOwnerAddress,
    };
    use banking_ledger_domain::deposit::DepositId;

    use crate::settlement::{
        EthereumDepositSettlementPreparation, PreparedDepositTransaction,
        SolanaDepositSettlementPreparation,
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
                transaction: PreparedDepositTransaction::new("solana".to_owned()),
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
                transaction: PreparedDepositTransaction::new("ethereum".to_owned()),
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
        assert_eq!(solana_preparation.transaction.value(), "solana");

        let ethereum_preparation = preparer
            .prepare(ethereum_request())
            .await
            .expect("Ethereum deposit should be routed");
        assert_eq!(ethereum_preparation.transaction.value(), "ethereum");
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

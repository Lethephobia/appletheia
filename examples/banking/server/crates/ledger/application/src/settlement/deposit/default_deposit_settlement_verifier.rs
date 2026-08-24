use banking_ledger_domain::core::{
    ChainNetwork, OnchainTransactionId, TokenAddress, TokenOwnerAddress,
};

use super::{
    DepositSettlementVerification, DepositSettlementVerifier, DepositSettlementVerifierError,
    DepositSettlementVerifyRequest, EthereumDepositSettlementVerifier,
    EthereumDepositSettlementVerifyRequest, SolanaDepositSettlementVerifier,
    SolanaDepositSettlementVerifyRequest,
};

pub struct DefaultDepositSettlementVerifier<S, E>
where
    S: SolanaDepositSettlementVerifier,
    E: EthereumDepositSettlementVerifier,
{
    solana: S,
    ethereum: E,
}

impl<S, E> DefaultDepositSettlementVerifier<S, E>
where
    S: SolanaDepositSettlementVerifier,
    E: EthereumDepositSettlementVerifier,
{
    pub fn new(solana: S, ethereum: E) -> Self {
        Self { solana, ethereum }
    }
}

impl<S, E> DepositSettlementVerifier for DefaultDepositSettlementVerifier<S, E>
where
    S: SolanaDepositSettlementVerifier,
    E: EthereumDepositSettlementVerifier,
{
    async fn verify(
        &self,
        request: DepositSettlementVerifyRequest,
    ) -> Result<DepositSettlementVerification, DepositSettlementVerifierError> {
        let transaction_id = match (
            request.chain_network,
            request.token_address,
            request.token_owner_address,
            request.transaction_id,
        ) {
            (
                ChainNetwork::Solana(network),
                TokenAddress::Solana(token_address),
                TokenOwnerAddress::Solana(token_owner_address),
                OnchainTransactionId::Solana(transaction_id),
            ) => {
                let verification = self
                    .solana
                    .verify(SolanaDepositSettlementVerifyRequest::new(
                        request.deposit_id,
                        request.currency_decimals,
                        network,
                        token_address,
                        token_owner_address,
                        request.amount,
                        transaction_id,
                    ))
                    .await?;
                OnchainTransactionId::Solana(verification.transaction_id)
            }
            (
                ChainNetwork::Ethereum(network),
                TokenAddress::Ethereum(token_address),
                TokenOwnerAddress::Ethereum(token_owner_address),
                OnchainTransactionId::Ethereum(transaction_id),
            ) => {
                let verification = self
                    .ethereum
                    .verify(EthereumDepositSettlementVerifyRequest::new(
                        request.deposit_id,
                        request.currency_decimals,
                        network,
                        token_address,
                        token_owner_address,
                        request.amount,
                        transaction_id,
                    ))
                    .await?;
                OnchainTransactionId::Ethereum(verification.transaction_id)
            }
            _ => return Err(DepositSettlementVerifierError::InconsistentChainValues),
        };

        Ok(DepositSettlementVerification { transaction_id })
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::core::{
        ChainNetwork, CurrencyAmount, CurrencyDecimals, EthereumNetwork, EvmAddress,
        EvmTokenContractAddress, EvmTokenOwnerAddress, EvmTransactionHash, OnchainTransactionId,
        SolanaAccountAddress, SolanaMintAccountAddress, SolanaNetwork, SolanaTokenOwnerAddress,
        SolanaTransactionSignature, TokenAddress, TokenOwnerAddress,
    };
    use banking_ledger_domain::deposit::DepositId;

    use crate::settlement::{
        EthereumDepositSettlementVerification, SolanaDepositSettlementVerification,
    };

    use super::{
        DefaultDepositSettlementVerifier, DepositSettlementVerifier,
        DepositSettlementVerifierError, DepositSettlementVerifyRequest,
        EthereumDepositSettlementVerifier, EthereumDepositSettlementVerifyRequest,
        SolanaDepositSettlementVerifier, SolanaDepositSettlementVerifyRequest,
    };

    struct StubSolanaDepositSettlementVerifier;

    impl SolanaDepositSettlementVerifier for StubSolanaDepositSettlementVerifier {
        async fn verify(
            &self,
            _request: SolanaDepositSettlementVerifyRequest,
        ) -> Result<SolanaDepositSettlementVerification, DepositSettlementVerifierError> {
            Ok(SolanaDepositSettlementVerification {
                transaction_id: SolanaTransactionSignature::from_bytes([1; 64]),
            })
        }
    }

    struct StubEthereumDepositSettlementVerifier;

    impl EthereumDepositSettlementVerifier for StubEthereumDepositSettlementVerifier {
        async fn verify(
            &self,
            _request: EthereumDepositSettlementVerifyRequest,
        ) -> Result<EthereumDepositSettlementVerification, DepositSettlementVerifierError> {
            Ok(EthereumDepositSettlementVerification {
                transaction_id: EvmTransactionHash::from_bytes([2; 32]),
            })
        }
    }

    #[tokio::test]
    async fn routes_verification_to_the_verifier_for_the_selected_chain() {
        let verifier = DefaultDepositSettlementVerifier::new(
            StubSolanaDepositSettlementVerifier,
            StubEthereumDepositSettlementVerifier,
        );

        let solana_verification = verifier
            .verify(solana_request())
            .await
            .expect("Solana deposit should be routed");
        assert_eq!(
            solana_verification.transaction_id,
            OnchainTransactionId::Solana(SolanaTransactionSignature::from_bytes([1; 64]))
        );

        let ethereum_verification = verifier
            .verify(ethereum_request())
            .await
            .expect("Ethereum deposit should be routed");
        assert_eq!(
            ethereum_verification.transaction_id,
            OnchainTransactionId::Ethereum(EvmTransactionHash::from_bytes([2; 32]))
        );
    }

    #[tokio::test]
    async fn rejects_verify_values_from_different_chains_before_dispatch() {
        let verifier = DefaultDepositSettlementVerifier::new(
            StubSolanaDepositSettlementVerifier,
            StubEthereumDepositSettlementVerifier,
        );
        let request = DepositSettlementVerifyRequest {
            deposit_id: DepositId::new(),
            currency_decimals: CurrencyDecimals::new(6),
            chain_network: ChainNetwork::Solana(SolanaNetwork::Devnet),
            token_address: TokenAddress::Ethereum(EvmTokenContractAddress::new(
                EvmAddress::from_bytes([3; 20]),
            )),
            token_owner_address: TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
                SolanaAccountAddress::from_bytes([4; 32]),
            )),
            amount: CurrencyAmount::new(100),
            transaction_id: OnchainTransactionId::Solana(SolanaTransactionSignature::from_bytes(
                [1; 64],
            )),
        };

        assert!(matches!(
            verifier.verify(request).await,
            Err(DepositSettlementVerifierError::InconsistentChainValues)
        ));
    }

    fn solana_request() -> DepositSettlementVerifyRequest {
        DepositSettlementVerifyRequest {
            deposit_id: DepositId::new(),
            currency_decimals: CurrencyDecimals::new(6),
            chain_network: ChainNetwork::Solana(SolanaNetwork::Devnet),
            token_address: TokenAddress::Solana(SolanaMintAccountAddress::new(
                SolanaAccountAddress::from_bytes([3; 32]),
            )),
            token_owner_address: TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
                SolanaAccountAddress::from_bytes([4; 32]),
            )),
            amount: CurrencyAmount::new(100),
            transaction_id: OnchainTransactionId::Solana(SolanaTransactionSignature::from_bytes(
                [1; 64],
            )),
        }
    }

    fn ethereum_request() -> DepositSettlementVerifyRequest {
        DepositSettlementVerifyRequest {
            deposit_id: DepositId::new(),
            currency_decimals: CurrencyDecimals::new(6),
            chain_network: ChainNetwork::Ethereum(EthereumNetwork::Sepolia),
            token_address: TokenAddress::Ethereum(EvmTokenContractAddress::new(
                EvmAddress::from_bytes([3; 20]),
            )),
            token_owner_address: TokenOwnerAddress::Ethereum(EvmTokenOwnerAddress::new(
                EvmAddress::from_bytes([4; 20]),
            )),
            amount: CurrencyAmount::new(100),
            transaction_id: OnchainTransactionId::Ethereum(EvmTransactionHash::from_bytes([2; 32])),
        }
    }
}

use banking_ledger_domain::core::{ChainNetwork, TokenAddress};

use super::{
    EthereumTokenBindingSettlementValidationRequest, EthereumTokenBindingSettlementValidator,
    SolanaTokenBindingSettlementValidationRequest, SolanaTokenBindingSettlementValidator,
    TokenBindingSettlementValidationRequest, TokenBindingSettlementValidator,
    TokenBindingSettlementValidatorError,
};

pub struct DefaultTokenBindingSettlementValidator<S, E>
where
    S: SolanaTokenBindingSettlementValidator,
    E: EthereumTokenBindingSettlementValidator,
{
    solana: S,
    ethereum: E,
}

impl<S, E> DefaultTokenBindingSettlementValidator<S, E>
where
    S: SolanaTokenBindingSettlementValidator,
    E: EthereumTokenBindingSettlementValidator,
{
    pub fn new(solana: S, ethereum: E) -> Self {
        Self { solana, ethereum }
    }
}

impl<S, E> TokenBindingSettlementValidator for DefaultTokenBindingSettlementValidator<S, E>
where
    S: SolanaTokenBindingSettlementValidator,
    E: EthereumTokenBindingSettlementValidator,
{
    async fn validate(
        &self,
        request: TokenBindingSettlementValidationRequest,
    ) -> Result<(), TokenBindingSettlementValidatorError> {
        match (request.chain_network, request.token_address) {
            (ChainNetwork::Solana, TokenAddress::Solana(token_address)) => {
                self.solana
                    .validate(SolanaTokenBindingSettlementValidationRequest::new(
                        request.currency_decimals,
                        token_address,
                    ))
                    .await
            }
            (ChainNetwork::Ethereum, TokenAddress::Ethereum(token_address)) => {
                self.ethereum
                    .validate(EthereumTokenBindingSettlementValidationRequest::new(
                        request.currency_decimals,
                        token_address,
                    ))
                    .await
            }
            _ => Err(TokenBindingSettlementValidatorError::Incompatible),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use banking_ledger_domain::core::{
        ChainNetwork, CurrencyDecimals, EvmAddress, EvmTokenContractAddress, SolanaAccountAddress,
        SolanaMintAccountAddress, TokenAddress,
    };

    use super::{
        DefaultTokenBindingSettlementValidator, EthereumTokenBindingSettlementValidationRequest,
        EthereumTokenBindingSettlementValidator, SolanaTokenBindingSettlementValidationRequest,
        SolanaTokenBindingSettlementValidator, TokenBindingSettlementValidationRequest,
        TokenBindingSettlementValidator, TokenBindingSettlementValidatorError,
    };

    struct StubSolanaTokenBindingSettlementValidator {
        call_count: Arc<AtomicUsize>,
    }

    impl SolanaTokenBindingSettlementValidator for StubSolanaTokenBindingSettlementValidator {
        async fn validate(
            &self,
            _request: SolanaTokenBindingSettlementValidationRequest,
        ) -> Result<(), TokenBindingSettlementValidatorError> {
            self.call_count.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    struct StubEthereumTokenBindingSettlementValidator {
        call_count: Arc<AtomicUsize>,
    }

    impl EthereumTokenBindingSettlementValidator for StubEthereumTokenBindingSettlementValidator {
        async fn validate(
            &self,
            _request: EthereumTokenBindingSettlementValidationRequest,
        ) -> Result<(), TokenBindingSettlementValidatorError> {
            self.call_count.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[tokio::test]
    async fn routes_validation_to_the_validator_for_the_selected_chain() {
        let solana_call_count = Arc::new(AtomicUsize::new(0));
        let ethereum_call_count = Arc::new(AtomicUsize::new(0));
        let validator = DefaultTokenBindingSettlementValidator::new(
            StubSolanaTokenBindingSettlementValidator {
                call_count: Arc::clone(&solana_call_count),
            },
            StubEthereumTokenBindingSettlementValidator {
                call_count: Arc::clone(&ethereum_call_count),
            },
        );

        validator
            .validate(solana_request())
            .await
            .expect("Solana token binding should be routed");
        validator
            .validate(ethereum_request())
            .await
            .expect("Ethereum token binding should be routed");

        assert_eq!(solana_call_count.load(Ordering::Relaxed), 1);
        assert_eq!(ethereum_call_count.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn rejects_values_from_different_chains_before_dispatch() {
        let validator = DefaultTokenBindingSettlementValidator::new(
            StubSolanaTokenBindingSettlementValidator {
                call_count: Arc::new(AtomicUsize::new(0)),
            },
            StubEthereumTokenBindingSettlementValidator {
                call_count: Arc::new(AtomicUsize::new(0)),
            },
        );
        let request = TokenBindingSettlementValidationRequest {
            currency_decimals: CurrencyDecimals::new(6),
            chain_network: ChainNetwork::Solana,
            token_address: TokenAddress::Ethereum(EvmTokenContractAddress::new(
                EvmAddress::from_bytes([3; 20]),
            )),
        };

        assert!(matches!(
            validator.validate(request).await,
            Err(TokenBindingSettlementValidatorError::Incompatible)
        ));
    }

    fn solana_request() -> TokenBindingSettlementValidationRequest {
        TokenBindingSettlementValidationRequest {
            currency_decimals: CurrencyDecimals::new(6),
            chain_network: ChainNetwork::Solana,
            token_address: TokenAddress::Solana(SolanaMintAccountAddress::new(
                SolanaAccountAddress::from_bytes([3; 32]),
            )),
        }
    }

    fn ethereum_request() -> TokenBindingSettlementValidationRequest {
        TokenBindingSettlementValidationRequest {
            currency_decimals: CurrencyDecimals::new(6),
            chain_network: ChainNetwork::Ethereum,
            token_address: TokenAddress::Ethereum(EvmTokenContractAddress::new(
                EvmAddress::from_bytes([3; 20]),
            )),
        }
    }
}

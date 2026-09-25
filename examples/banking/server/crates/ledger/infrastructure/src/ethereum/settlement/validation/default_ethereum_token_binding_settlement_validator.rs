use alloy::contract::Error as ContractError;
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider};
use banking_ledger_application::{
    EthereumTokenBindingSettlementValidationRequest, EthereumTokenBindingSettlementValidator,
    TokenBindingSettlementValidatorError,
};
use banking_ledger_domain::core::TokenDecimals;

use super::DefaultEthereumTokenBindingSettlementValidatorError;
use crate::ethereum::contract::IERC20Metadata;

pub struct DefaultEthereumTokenBindingSettlementValidator {
    provider: DynProvider,
}

impl DefaultEthereumTokenBindingSettlementValidator {
    pub fn new(provider: DynProvider) -> Self {
        Self { provider }
    }

    fn incompatible_contract_call(error: &ContractError) -> bool {
        match error {
            ContractError::ZeroData(_, _) | ContractError::AbiError(_) => true,
            ContractError::TransportError(error) => error
                .as_error_resp()
                .and_then(|response| response.as_revert_data())
                .is_some(),
            _ => false,
        }
    }
}

impl EthereumTokenBindingSettlementValidator for DefaultEthereumTokenBindingSettlementValidator {
    async fn validate(
        &self,
        request: EthereumTokenBindingSettlementValidationRequest,
    ) -> Result<(), TokenBindingSettlementValidatorError> {
        let token_address = Address::from(*request.token_address().address().as_bytes());
        let token_code = self
            .provider
            .get_code_at(token_address)
            .await
            .map_err(|error| {
                TokenBindingSettlementValidatorError::Backend(Box::new(
                    DefaultEthereumTokenBindingSettlementValidatorError::Rpc(error),
                ))
            })?;
        if token_code.is_empty() {
            return Err(TokenBindingSettlementValidatorError::Incompatible);
        }

        let token = IERC20Metadata::new(token_address, &self.provider);
        let decimals = match token.decimals().call().await {
            Ok(decimals) => decimals,
            Err(error) if Self::incompatible_contract_call(&error) => {
                return Err(TokenBindingSettlementValidatorError::Incompatible);
            }
            Err(error) => {
                return Err(TokenBindingSettlementValidatorError::Backend(Box::new(
                    DefaultEthereumTokenBindingSettlementValidatorError::Contract(error),
                )));
            }
        };
        let token_decimals = TokenDecimals::new(decimals);
        if token_decimals.value() < request.currency_decimals().value() {
            return Err(TokenBindingSettlementValidatorError::Incompatible);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{Bytes, U256};
    use alloy::providers::{Provider, ProviderBuilder};
    use alloy::sol_types::SolValue;
    use alloy::transports::mock::Asserter;
    use banking_ledger_application::{
        EthereumTokenBindingSettlementValidationRequest, EthereumTokenBindingSettlementValidator,
        TokenBindingSettlementValidatorError,
    };
    use banking_ledger_domain::core::{CurrencyDecimals, EvmAddress, EvmTokenContractAddress};

    use super::DefaultEthereumTokenBindingSettlementValidator;

    #[tokio::test]
    async fn accepts_a_contract_with_compatible_token_decimals() {
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::from_static(&[1]));
        asserter.push_success(&Bytes::from(U256::from(6).abi_encode()));
        let validator = validator(asserter);

        assert!(validator.validate(request()).await.is_ok());
    }

    #[tokio::test]
    async fn rejects_an_address_without_contract_code() {
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::new());
        let validator = validator(asserter);

        assert!(matches!(
            validator.validate(request()).await,
            Err(TokenBindingSettlementValidatorError::Incompatible)
        ));
    }

    #[tokio::test]
    async fn rejects_token_decimals_smaller_than_currency_decimals() {
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::from_static(&[1]));
        asserter.push_success(&Bytes::from(U256::from(5).abi_encode()));
        let validator = validator(asserter);

        assert!(matches!(
            validator.validate(request()).await,
            Err(TokenBindingSettlementValidatorError::Incompatible)
        ));
    }

    fn validator(asserter: Asserter) -> DefaultEthereumTokenBindingSettlementValidator {
        let provider = ProviderBuilder::new()
            .connect_mocked_client(asserter)
            .erased();
        DefaultEthereumTokenBindingSettlementValidator::new(provider)
    }

    fn request() -> EthereumTokenBindingSettlementValidationRequest {
        EthereumTokenBindingSettlementValidationRequest::new(
            CurrencyDecimals::new(6),
            EvmTokenContractAddress::new(EvmAddress::from_bytes([1; 20])),
        )
    }
}

use banking_ledger_application::{
    EthereumTokenBindingSettlementValidationRequest, EthereumTokenBindingSettlementValidator,
    TokenBindingSettlementValidatorError,
};

use super::EthereumTokenContractInspector;

pub struct DefaultEthereumTokenBindingSettlementValidator<I>
where
    I: EthereumTokenContractInspector,
{
    inspector: I,
}

impl<I> DefaultEthereumTokenBindingSettlementValidator<I>
where
    I: EthereumTokenContractInspector,
{
    pub fn new(inspector: I) -> Self {
        Self { inspector }
    }
}

impl<I> EthereumTokenBindingSettlementValidator
    for DefaultEthereumTokenBindingSettlementValidator<I>
where
    I: EthereumTokenContractInspector,
{
    async fn validate(
        &self,
        request: EthereumTokenBindingSettlementValidationRequest,
    ) -> Result<(), TokenBindingSettlementValidatorError> {
        let inspection = self
            .inspector
            .inspect(request.network(), &request.token_address())
            .await
            .map_err(TokenBindingSettlementValidatorError::Backend)?;
        if !inspection.settlement_usable
            || inspection.decimals.value() < request.currency_decimals().value()
        {
            return Err(TokenBindingSettlementValidatorError::Incompatible);
        }
        Ok(())
    }
}

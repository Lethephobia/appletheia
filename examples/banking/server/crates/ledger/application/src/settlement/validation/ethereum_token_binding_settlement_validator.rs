use super::{
    EthereumTokenBindingSettlementValidationRequest, TokenBindingSettlementValidatorError,
};

#[allow(async_fn_in_trait)]
pub trait EthereumTokenBindingSettlementValidator: Send + Sync {
    async fn validate(
        &self,
        request: EthereumTokenBindingSettlementValidationRequest,
    ) -> Result<(), TokenBindingSettlementValidatorError>;
}

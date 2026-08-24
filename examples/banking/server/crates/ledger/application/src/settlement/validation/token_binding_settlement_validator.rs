use super::{TokenBindingSettlementValidationRequest, TokenBindingSettlementValidatorError};

#[allow(async_fn_in_trait)]
pub trait TokenBindingSettlementValidator: Send + Sync {
    async fn validate(
        &self,
        request: TokenBindingSettlementValidationRequest,
    ) -> Result<(), TokenBindingSettlementValidatorError>;
}

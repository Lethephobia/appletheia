use super::{SolanaTokenBindingSettlementValidationRequest, TokenBindingSettlementValidatorError};

#[allow(async_fn_in_trait)]
pub trait SolanaTokenBindingSettlementValidator: Send + Sync {
    async fn validate(
        &self,
        request: SolanaTokenBindingSettlementValidationRequest,
    ) -> Result<(), TokenBindingSettlementValidatorError>;
}

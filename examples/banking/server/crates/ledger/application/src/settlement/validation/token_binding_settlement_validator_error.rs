use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenBindingSettlementValidatorError {
    #[error("token binding is not usable for settlement")]
    Incompatible,
    #[error("token binding settlement validation backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for TokenBindingSettlementValidatorError {
    fn is_retryable(&self) -> bool {
        matches!(self, Self::Backend(_))
    }
}

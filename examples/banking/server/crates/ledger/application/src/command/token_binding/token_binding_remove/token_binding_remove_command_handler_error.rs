use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenBindingRemoveCommandHandlerError {
    #[error("token binding repository failed")]
    Repository(#[from] RepositoryError<TokenBinding>),
    #[error("token binding aggregate failed")]
    Aggregate(#[from] TokenBindingError),
}

impl Retryability for TokenBindingRemoveCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Repository(error) => error.is_retryable(),
            Self::Aggregate(_) => false,
        }
    }
}

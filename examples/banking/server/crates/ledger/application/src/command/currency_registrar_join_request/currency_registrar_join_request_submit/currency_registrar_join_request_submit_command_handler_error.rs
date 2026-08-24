use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use appletheia::domain::{UniqueValueError, UniqueValuePartError};
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarError, CurrencyRegistrarJoinRequest,
    CurrencyRegistrarJoinRequestError, CurrencyRegistrarMembership,
};
use thiserror::Error;

/// Represents errors returned while submitting an currency registrar join request.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestSubmitCommandHandlerError {
    #[error("currency registrar repository failed")]
    CurrencyRegistrarRepository(#[from] RepositoryError<CurrencyRegistrar>),

    #[error("currency registrar join request repository failed")]
    CurrencyRegistrarJoinRequestRepository(#[from] RepositoryError<CurrencyRegistrarJoinRequest>),

    #[error("currency registrar membership repository failed")]
    CurrencyRegistrarMembershipRepository(#[from] RepositoryError<CurrencyRegistrarMembership>),

    #[error("currency registrar join request aggregate failed")]
    CurrencyRegistrarJoinRequest(#[from] CurrencyRegistrarJoinRequestError),

    #[error("currency registrar aggregate failed")]
    CurrencyRegistrar(#[from] CurrencyRegistrarError),

    #[error("unique value part is invalid")]
    UniqueValuePart(#[from] UniqueValuePartError),

    #[error("unique value is invalid")]
    UniqueValue(#[from] UniqueValueError),
}

impl Retryability for CurrencyRegistrarJoinRequestSubmitCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRegistrarRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarJoinRequestRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarMembershipRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarJoinRequest(_) => false,
            Self::CurrencyRegistrar(_) => false,
            Self::UniqueValuePart(_) => false,
            Self::UniqueValue(_) => false,
        }
    }
}

use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarError, CurrencyRegistrarJoinRequest,
    CurrencyRegistrarJoinRequestError,
};
use thiserror::Error;

/// Represents errors returned while rejecting an currency registrar join request.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestRejectCommandHandlerError {
    #[error("currency registrar repository failed")]
    CurrencyRegistrarRepository(#[from] RepositoryError<CurrencyRegistrar>),

    #[error("currency registrar join request repository failed")]
    CurrencyRegistrarJoinRequestRepository(#[from] RepositoryError<CurrencyRegistrarJoinRequest>),

    #[error("currency registrar join request aggregate failed")]
    CurrencyRegistrarJoinRequest(#[from] CurrencyRegistrarJoinRequestError),

    #[error("currency registrar aggregate failed")]
    CurrencyRegistrar(#[from] CurrencyRegistrarError),
}

impl Retryability for CurrencyRegistrarJoinRequestRejectCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRegistrarRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarJoinRequestRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarJoinRequest(_) => false,
            Self::CurrencyRegistrar(_) => false,
        }
    }
}

use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarError, CurrencyRegistrarInvitation,
    CurrencyRegistrarInvitationError,
};
use thiserror::Error;

/// Represents errors returned while accepting an currency registrar invitation.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarInvitationAcceptCommandHandlerError {
    #[error("currency registrar repository failed")]
    CurrencyRegistrarRepository(#[from] RepositoryError<CurrencyRegistrar>),

    #[error("currency registrar invitation repository failed")]
    CurrencyRegistrarInvitationRepository(#[from] RepositoryError<CurrencyRegistrarInvitation>),

    #[error("currency registrar invitation aggregate failed")]
    CurrencyRegistrarInvitation(#[from] CurrencyRegistrarInvitationError),

    #[error("currency registrar aggregate failed")]
    CurrencyRegistrar(#[from] CurrencyRegistrarError),
}

impl Retryability for CurrencyRegistrarInvitationAcceptCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRegistrarRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarInvitationRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarInvitation(_) => false,
            Self::CurrencyRegistrar(_) => false,
        }
    }
}

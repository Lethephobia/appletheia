use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarError, CurrencyRegistrarInvitation,
    CurrencyRegistrarInvitationError, CurrencyRegistrarMembership,
};
use thiserror::Error;

/// Represents errors returned while issuing an currency registrar invitation.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarInvitationIssueCommandHandlerError {
    #[error("currency registrar repository failed")]
    CurrencyRegistrarRepository(#[from] RepositoryError<CurrencyRegistrar>),

    #[error("currency registrar invitation repository failed")]
    CurrencyRegistrarInvitationRepository(#[from] RepositoryError<CurrencyRegistrarInvitation>),

    #[error("currency registrar membership repository failed")]
    CurrencyRegistrarMembershipRepository(#[from] RepositoryError<CurrencyRegistrarMembership>),

    #[error("currency registrar invitation aggregate failed")]
    CurrencyRegistrarInvitation(#[from] CurrencyRegistrarInvitationError),

    #[error("currency registrar aggregate failed")]
    CurrencyRegistrar(#[from] CurrencyRegistrarError),

    #[error("unique value failed")]
    UniqueValue(#[from] UniqueValueError),
}

impl Retryability for CurrencyRegistrarInvitationIssueCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRegistrarRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarInvitationRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarMembershipRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarInvitation(_) => false,
            Self::CurrencyRegistrar(_) => false,
            Self::UniqueValue(_) => false,
        }
    }
}

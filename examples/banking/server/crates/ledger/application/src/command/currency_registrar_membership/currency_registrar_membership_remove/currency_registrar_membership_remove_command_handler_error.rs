use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency_registrar_membership::{
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarMembershipRemoveCommandHandlerError {
    #[error("currency registrar membership repository failed")]
    Repository(#[from] RepositoryError<CurrencyRegistrarMembership>),
    #[error("currency registrar membership aggregate failed")]
    Aggregate(#[from] CurrencyRegistrarMembershipError),
}

impl Retryability for CurrencyRegistrarMembershipRemoveCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Repository(error) => error.is_retryable(),
            Self::Aggregate(_) => false,
        }
    }
}

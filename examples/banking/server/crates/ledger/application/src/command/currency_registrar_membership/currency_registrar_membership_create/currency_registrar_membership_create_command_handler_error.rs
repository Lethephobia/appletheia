use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_ledger_domain::currency_registrar::CurrencyRegistrar;
use banking_ledger_domain::currency_registrar_membership::{
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarMembershipCreateCommandHandlerError {
    #[error("currency registrar repository failed")]
    CurrencyRegistrarRepository(#[from] RepositoryError<CurrencyRegistrar>),
    #[error("currency registrar membership repository failed")]
    CurrencyRegistrarMembershipRepository(#[from] RepositoryError<CurrencyRegistrarMembership>),
    #[error("currency registrar membership aggregate failed")]
    CurrencyRegistrarMembership(#[from] CurrencyRegistrarMembershipError),
    #[error("currency registrar membership unique value is invalid")]
    UniqueValue(#[from] UniqueValueError),
}

impl Retryability for CurrencyRegistrarMembershipCreateCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRegistrarRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarMembershipRepository(error) => error.is_retryable(),
            Self::CurrencyRegistrarMembership(_) | Self::UniqueValue(_) => false,
        }
    }
}

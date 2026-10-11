use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Organization, OrganizationError, User, UserError};
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountOpenCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("user is inactive")]
    UserInactive,

    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization is inactive")]
    OrganizationInactive,

    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("currency is inactive")]
    CurrencyInactive,
}

impl Retryability for AccountOpenCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::UserRepository(error) => error.is_retryable(),
            Self::User(_) | Self::UserInactive => false,
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::Organization(_) | Self::OrganizationInactive => false,
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) | Self::CurrencyInactive => false,
        }
    }
}

use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingError};
use thiserror::Error;

use crate::settlement::TokenBindingSettlementValidatorError;

#[derive(Debug, Error)]
pub enum TokenBindingDefineCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),
    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),
    #[error("token binding repository failed")]
    TokenBindingRepository(#[from] RepositoryError<TokenBinding>),
    #[error("token binding aggregate failed")]
    TokenBinding(#[from] TokenBindingError),
    #[error("token binding unique value is invalid")]
    UniqueValue(#[from] UniqueValueError),
    #[error("token binding settlement validation failed")]
    SettlementValidation(#[from] TokenBindingSettlementValidatorError),
}

impl Retryability for TokenBindingDefineCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::TokenBindingRepository(error) => error.is_retryable(),
            Self::SettlementValidation(error) => error.is_retryable(),
            Self::Currency(_) | Self::TokenBinding(_) | Self::UniqueValue(_) => false,
        }
    }
}

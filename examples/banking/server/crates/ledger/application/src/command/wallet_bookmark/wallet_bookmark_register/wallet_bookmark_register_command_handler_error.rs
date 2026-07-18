use appletheia::application::Retryability;

use crate::mint::TokenAccountOwnerAddressValidatorError;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkError};
use thiserror::Error;

/// Represents errors returned while registering a wallet bookmark.
#[derive(Debug, Error)]
pub enum WalletBookmarkRegisterCommandHandlerError {
    #[error("wallet bookmark repository failed")]
    WalletBookmarkRepository(#[from] RepositoryError<WalletBookmark>),

    #[error("wallet bookmark aggregate failed")]
    WalletBookmark(#[from] WalletBookmarkError),

    #[error("token account owner address validation failed")]
    TokenAccountOwnerAddressValidator(#[from] TokenAccountOwnerAddressValidatorError),
}

impl Retryability for WalletBookmarkRegisterCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::WalletBookmarkRepository(error) => error.is_retryable(),
            Self::WalletBookmark(_) => false,
            Self::TokenAccountOwnerAddressValidator(error) => error.is_retryable(),
        }
    }
}

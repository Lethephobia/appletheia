use appletheia::application::Retryability;

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
}

impl Retryability for WalletBookmarkRegisterCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::WalletBookmarkRepository(error) => error.is_retryable(),
            Self::WalletBookmark(_) => false,
        }
    }
}

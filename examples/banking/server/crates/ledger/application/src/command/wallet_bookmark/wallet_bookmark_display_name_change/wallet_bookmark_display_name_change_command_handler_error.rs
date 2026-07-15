use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkError};
use thiserror::Error;

/// Represents errors returned while changing a wallet bookmark display name.
#[derive(Debug, Error)]
pub enum WalletBookmarkDisplayNameChangeCommandHandlerError {
    #[error("wallet bookmark repository failed")]
    WalletBookmarkRepository(#[from] RepositoryError<WalletBookmark>),

    #[error("wallet bookmark aggregate failed")]
    WalletBookmark(#[from] WalletBookmarkError),
}

use appletheia::domain::AggregateError;
use thiserror::Error;

use super::WalletBookmarkId;

/// Describes why a `WalletBookmark` aggregate operation failed.
#[derive(Debug, Error)]
pub enum WalletBookmarkError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<WalletBookmarkId>),

    #[error("wallet bookmark has already been registered")]
    AlreadyRegistered,
}

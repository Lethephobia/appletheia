use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{WalletBookmarkId, WalletBookmarkStateError};

/// Describes why a `WalletBookmark` aggregate operation failed.
#[derive(Debug, Error)]
pub enum WalletBookmarkError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<WalletBookmarkId>),

    #[error(transparent)]
    State(#[from] WalletBookmarkStateError),

    #[error("wallet bookmark has already been added")]
    AlreadyAdded,

    #[error("token account owner address is invalid")]
    InvalidTokenAccountOwnerAddress,

    #[error("wallet bookmark has been removed")]
    Removed,
}

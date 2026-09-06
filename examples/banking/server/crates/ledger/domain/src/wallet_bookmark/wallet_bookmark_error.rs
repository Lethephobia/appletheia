use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    WalletBookmarkDescriptionChangeRejectionReason, WalletBookmarkDisplayNameChangeRejectionReason,
    WalletBookmarkId, WalletBookmarkRemoveRejectionReason, WalletBookmarkStateError,
};

/// Describes why a `WalletBookmark` aggregate operation failed.
#[derive(Debug, Error)]
pub enum WalletBookmarkError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<WalletBookmarkId>),

    #[error(transparent)]
    State(#[from] WalletBookmarkStateError),

    #[error("wallet bookmark has already been registered")]
    AlreadyRegistered,
    #[error("wallet bookmark display name change rejected: {0:?}")]
    DisplayNameChangeRejected(WalletBookmarkDisplayNameChangeRejectionReason),
    #[error("wallet bookmark description change rejected: {0:?}")]
    DescriptionChangeRejected(WalletBookmarkDescriptionChangeRejectionReason),
    #[error("wallet bookmark removal rejected: {0:?}")]
    RemoveRejected(WalletBookmarkRemoveRejectionReason),
}

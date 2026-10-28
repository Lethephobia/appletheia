use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::wallet_bookmark::WalletBookmarkError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletBookmarkOwnerDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    WalletBookmark(#[from] WalletBookmarkError),
}

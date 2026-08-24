use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{AccountBalanceError, AccountId, AccountStateError};

/// Describes why an `Account` aggregate operation failed.
#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<AccountId>),

    #[error(transparent)]
    State(#[from] AccountStateError),

    #[error("account is already opened")]
    AlreadyOpened,

    #[error(transparent)]
    AccountBalance(#[from] AccountBalanceError),
}

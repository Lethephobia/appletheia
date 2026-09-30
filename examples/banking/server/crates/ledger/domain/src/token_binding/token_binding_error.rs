use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{TokenBindingId, TokenBindingStateError};

#[derive(Debug, Error)]
pub enum TokenBindingError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TokenBindingId>),
    #[error(transparent)]
    State(#[from] TokenBindingStateError),
    #[error("token binding is already defined")]
    AlreadyDefined,
    #[error("token address does not match the selected chain")]
    ChainMismatch,

    #[error("token binding has been removed")]
    Removed,

    #[error("token is already bound")]
    TokenAlreadyBound,

    #[error("token binding deposit is already enabled")]
    DepositAlreadyEnabled,

    #[error("token binding deposit is already disabled")]
    DepositAlreadyDisabled,

    #[error("token binding withdrawal is already enabled")]
    WithdrawalAlreadyEnabled,

    #[error("token binding withdrawal is already disabled")]
    WithdrawalAlreadyDisabled,
}

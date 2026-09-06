use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    TokenBindingDefineRejectionReason, TokenBindingEnablementChangeRejectionReason, TokenBindingId,
    TokenBindingRemoveRejectionReason, TokenBindingStateError,
};

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
    #[error("token binding definition rejected: {0:?}")]
    DefinitionRejected(TokenBindingDefineRejectionReason),
    #[error("token binding enablement change rejected: {0:?}")]
    EnablementChangeRejected(TokenBindingEnablementChangeRejectionReason),
    #[error("token binding removal rejected: {0:?}")]
    RemovalRejected(TokenBindingRemoveRejectionReason),
}

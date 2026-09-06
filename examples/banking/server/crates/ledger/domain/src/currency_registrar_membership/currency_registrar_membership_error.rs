use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    CurrencyRegistrarMembershipCreateRejectionReason, CurrencyRegistrarMembershipId,
    CurrencyRegistrarMembershipRemoveRejectionReason, CurrencyRegistrarMembershipStateError,
};

/// Describes why a CurrencyRegistrarMembership aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarMembershipError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyRegistrarMembershipId>),
    #[error(transparent)]
    State(#[from] CurrencyRegistrarMembershipStateError),
    #[error("currency registrar membership is already created")]
    AlreadyCreated,
    #[error("currency registrar membership creation rejected: {0:?}")]
    CreateRejected(CurrencyRegistrarMembershipCreateRejectionReason),
    #[error("currency registrar membership removal rejected: {0:?}")]
    RemoveRejected(CurrencyRegistrarMembershipRemoveRejectionReason),
}

use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{CurrencyRegistrarMembershipId, CurrencyRegistrarMembershipStateError};

/// Describes why a CurrencyRegistrarMembership aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarMembershipError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyRegistrarMembershipId>),
    #[error(transparent)]
    State(#[from] CurrencyRegistrarMembershipStateError),
    #[error("currency registrar membership is already created")]
    AlreadyCreated,
}

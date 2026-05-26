use appletheia::domain::AggregateError;
use thiserror::Error;

use super::PayoutDestinationId;

/// Describes why a `PayoutDestination` aggregate operation failed.
#[derive(Debug, Error)]
pub enum PayoutDestinationError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<PayoutDestinationId>),

    #[error("payout destination has already been registered")]
    AlreadyRegistered,
}

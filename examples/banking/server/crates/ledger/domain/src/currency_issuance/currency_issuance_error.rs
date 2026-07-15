use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{CurrencyIssuanceId, CurrencyIssuanceStateError};

/// Describes why a `CurrencyIssuance` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyIssuanceId>),

    #[error(transparent)]
    State(#[from] CurrencyIssuanceStateError),

    #[error("currency issuance was already issued")]
    AlreadyIssued,
}

use appletheia::domain::AggregateError;
use thiserror::Error;

use super::CurrencyIssuanceId;

/// Describes why a `CurrencyIssuance` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyIssuanceId>),

    #[error("currency has already been issued")]
    AlreadyIssued,

    #[error("issuance amount must be greater than zero")]
    ZeroAmount,

    #[error("issuance is already completed")]
    AlreadyCompleted,

    #[error("issuance is already failed")]
    AlreadyFailed,
}

use appletheia::domain::AggregateError;
use thiserror::Error;

use super::CurrencyIssuanceId;

/// Describes why a `CurrencyIssuance` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyIssuanceId>),

    #[error("currency issuance was already issued")]
    AlreadyIssued,
}

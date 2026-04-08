use appletheia::domain::AggregateStateError;
use thiserror::Error;

/// Describes why a currency issuance state value cannot be handled.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),
}

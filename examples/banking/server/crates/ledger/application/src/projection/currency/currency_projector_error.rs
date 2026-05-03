use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::CurrencyProjectionStoreError;

/// Represents errors returned while projecting currency projections.
#[derive(Debug, Error)]
pub enum CurrencyProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] CurrencyProjectionStoreError),
}

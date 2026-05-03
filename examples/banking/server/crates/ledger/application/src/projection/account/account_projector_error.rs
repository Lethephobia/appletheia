use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::AccountProjectionStoreError;

/// Represents errors returned while projecting account projections.
#[derive(Debug, Error)]
pub enum AccountProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] AccountProjectionStoreError),
}

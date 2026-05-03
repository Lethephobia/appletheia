use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::TransferProjectionStoreError;

/// Represents errors returned while projecting transfer projections.
#[derive(Debug, Error)]
pub enum TransferProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] TransferProjectionStoreError),
}

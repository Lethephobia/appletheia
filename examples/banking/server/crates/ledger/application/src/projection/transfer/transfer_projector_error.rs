use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::TransferViewStoreError;

/// Represents errors returned while projecting transfer views.
#[derive(Debug, Error)]
pub enum TransferProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] TransferViewStoreError),
}

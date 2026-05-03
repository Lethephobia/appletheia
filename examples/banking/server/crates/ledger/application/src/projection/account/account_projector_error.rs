use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::AccountViewStoreError;

/// Represents errors returned while projecting account views.
#[derive(Debug, Error)]
pub enum AccountProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] AccountViewStoreError),
}

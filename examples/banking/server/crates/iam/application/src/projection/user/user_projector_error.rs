use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::UserViewStoreError;

/// Represents errors returned while projecting user views.
#[derive(Debug, Error)]
pub enum UserProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] UserViewStoreError),
}

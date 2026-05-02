use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::UserIdentityViewStoreError;

/// Represents errors returned while projecting user identity views.
#[derive(Debug, Error)]
pub enum UserIdentityProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] UserIdentityViewStoreError),
}

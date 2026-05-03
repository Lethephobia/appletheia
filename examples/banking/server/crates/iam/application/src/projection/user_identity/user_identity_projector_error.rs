use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::UserIdentityProjectionStoreError;

/// Represents errors returned while projecting user identity projections.
#[derive(Debug, Error)]
pub enum UserIdentityProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] UserIdentityProjectionStoreError),
}

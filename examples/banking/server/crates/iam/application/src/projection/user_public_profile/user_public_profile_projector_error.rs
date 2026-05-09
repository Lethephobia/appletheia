use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::UserPublicProfileWriterError;

/// Error returned while projecting public user profiles.
#[derive(Debug, Error)]
pub enum UserPublicProfileProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] UserPublicProfileWriterError),
}

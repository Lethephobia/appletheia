use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::UserPrivateInfoWriterError;

/// Error returned while projecting user-private information.
#[derive(Debug, Error)]
pub enum UserPrivateInfoProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] UserPrivateInfoWriterError),
}

use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::PublicUserListWriterError;

/// Error returned while projecting public user lists.
#[derive(Debug, Error)]
pub enum PublicUserListProjectorError {
    #[error("user event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("public user list writer failed")]
    Writer(#[from] PublicUserListWriterError),
}

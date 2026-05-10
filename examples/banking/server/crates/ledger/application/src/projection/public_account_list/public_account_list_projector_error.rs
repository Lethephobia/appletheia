use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::PublicAccountListWriterError;

#[derive(Debug, Error)]
pub enum PublicAccountListProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] PublicAccountListWriterError),
}

use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::PublicAccountListItemWriterError;

#[derive(Debug, Error)]
pub enum PublicAccountListItemProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] PublicAccountListItemWriterError),
}

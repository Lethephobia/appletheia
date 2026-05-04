use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::OwnedAccountListItemWriterError;

/// Error returned while projecting owned account list items.
#[derive(Debug, Error)]
pub enum OwnedAccountListItemProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] OwnedAccountListItemWriterError),
}

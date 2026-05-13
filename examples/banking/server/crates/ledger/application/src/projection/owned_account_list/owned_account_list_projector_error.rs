use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::{OwnedAccountListItemStatusError, OwnedAccountListWriterError};

/// Error returned while projecting owned account lists.
#[derive(Debug, Error)]
pub enum OwnedAccountListProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] OwnedAccountListWriterError),

    #[error(transparent)]
    Status(#[from] OwnedAccountListItemStatusError),
}

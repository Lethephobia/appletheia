use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::{AccountFragmentWriterError, MaterializedAccountStatusError};

/// Error returned while projecting account fragments.
#[derive(Debug, Error)]
pub enum AccountFragmentProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] AccountFragmentWriterError),

    #[error(transparent)]
    Status(#[from] MaterializedAccountStatusError),
}

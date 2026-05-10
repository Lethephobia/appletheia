use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::OwnedAccountTransactionListWriterError;

/// Error returned while projecting owned account transaction lists.
#[derive(Debug, Error)]
pub enum OwnedAccountTransactionListProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] OwnedAccountTransactionListWriterError),
}

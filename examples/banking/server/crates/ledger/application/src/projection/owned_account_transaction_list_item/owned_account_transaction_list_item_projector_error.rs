use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::OwnedAccountTransactionListItemWriterError;

/// Error returned while projecting owned account transaction list items.
#[derive(Debug, Error)]
pub enum OwnedAccountTransactionListItemProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] OwnedAccountTransactionListItemWriterError),
}

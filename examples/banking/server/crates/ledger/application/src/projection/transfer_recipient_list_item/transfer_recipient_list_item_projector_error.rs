use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::TransferRecipientListItemWriterError;

/// Error returned while projecting transfer recipient list items.
#[derive(Debug, Error)]
pub enum TransferRecipientListItemProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] TransferRecipientListItemWriterError),
}

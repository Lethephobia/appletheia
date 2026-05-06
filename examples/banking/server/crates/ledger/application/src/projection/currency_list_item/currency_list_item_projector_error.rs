use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::CurrencyListItemWriterError;

/// Error returned while projecting currency list items.
#[derive(Debug, Error)]
pub enum CurrencyListItemProjectorError {
    #[error("currency event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("currency list item writer failed")]
    Writer(#[from] CurrencyListItemWriterError),
}

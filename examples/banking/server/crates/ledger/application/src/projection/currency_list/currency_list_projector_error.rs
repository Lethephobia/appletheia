use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::{CurrencyListItemStatusError, CurrencyListWriterError};

/// Error returned while projecting currency lists.
#[derive(Debug, Error)]
pub enum CurrencyListProjectorError {
    #[error("currency event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("currency list writer failed")]
    Writer(#[from] CurrencyListWriterError),

    #[error(transparent)]
    Status(#[from] CurrencyListItemStatusError),
}

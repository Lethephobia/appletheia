use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use super::CurrencyFragmentWriterError;

#[derive(Debug, Error)]
pub enum CurrencyFragmentProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),
    #[error(transparent)]
    Writer(#[from] CurrencyFragmentWriterError),
}

use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::CurrencyViewStoreError;

/// Represents errors returned while projecting currency views.
#[derive(Debug, Error)]
pub enum CurrencyProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] CurrencyViewStoreError),
}

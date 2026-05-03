use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::CurrencyIssuanceViewStoreError;

/// Represents errors returned while projecting currency issuance views.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] CurrencyIssuanceViewStoreError),
}

use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::CurrencyIssuanceProjectionStoreError;

/// Represents errors returned while projecting currency issuance projections.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] CurrencyIssuanceProjectionStoreError),
}

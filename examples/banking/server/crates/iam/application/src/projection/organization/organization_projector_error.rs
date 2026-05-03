use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::OrganizationProjectionStoreError;

/// Represents errors returned while projecting organization projections.
#[derive(Debug, Error)]
pub enum OrganizationProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] OrganizationProjectionStoreError),
}

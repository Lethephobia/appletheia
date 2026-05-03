use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::OrganizationJoinRequestProjectionStoreError;

/// Represents errors returned while projecting organization join request projections.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] OrganizationJoinRequestProjectionStoreError),
}

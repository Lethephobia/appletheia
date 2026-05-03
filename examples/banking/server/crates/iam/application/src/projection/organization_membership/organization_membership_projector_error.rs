use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::OrganizationMembershipProjectionStoreError;

/// Represents errors returned while projecting membership projections.
#[derive(Debug, Error)]
pub enum OrganizationMembershipProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] OrganizationMembershipProjectionStoreError),
}

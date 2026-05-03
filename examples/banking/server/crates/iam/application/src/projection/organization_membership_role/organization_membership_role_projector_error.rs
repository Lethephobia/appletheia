use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::OrganizationMembershipRoleProjectionStoreError;

/// Represents errors returned while projecting organization membership role projections.
#[derive(Debug, Error)]
pub enum OrganizationMembershipRoleProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] OrganizationMembershipRoleProjectionStoreError),
}

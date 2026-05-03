use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::OrganizationInvitationProjectionStoreError;

/// Represents errors returned while projecting organization invitation projections.
#[derive(Debug, Error)]
pub enum OrganizationInvitationProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ProjectionStore(#[from] OrganizationInvitationProjectionStoreError),
}

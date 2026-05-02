use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::OrganizationMembershipRoleViewStoreError;

/// Represents errors returned while projecting organization membership role views.
#[derive(Debug, Error)]
pub enum OrganizationMembershipRoleProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] OrganizationMembershipRoleViewStoreError),
}

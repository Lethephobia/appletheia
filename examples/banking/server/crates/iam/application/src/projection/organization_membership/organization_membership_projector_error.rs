use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::OrganizationMembershipViewStoreError;

/// Represents errors returned while projecting membership views.
#[derive(Debug, Error)]
pub enum OrganizationMembershipProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] OrganizationMembershipViewStoreError),
}

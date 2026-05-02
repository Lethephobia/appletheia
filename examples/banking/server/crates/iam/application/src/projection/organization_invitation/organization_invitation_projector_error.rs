use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::OrganizationInvitationViewStoreError;

/// Represents errors returned while projecting organization invitation views.
#[derive(Debug, Error)]
pub enum OrganizationInvitationProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] OrganizationInvitationViewStoreError),
}

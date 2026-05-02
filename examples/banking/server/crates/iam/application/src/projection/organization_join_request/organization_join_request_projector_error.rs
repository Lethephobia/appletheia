use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::OrganizationJoinRequestViewStoreError;

/// Represents errors returned while projecting organization join request views.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] OrganizationJoinRequestViewStoreError),
}

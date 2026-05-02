use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::view::OrganizationViewStoreError;

/// Represents errors returned while projecting organization views.
#[derive(Debug, Error)]
pub enum OrganizationProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    ViewStore(#[from] OrganizationViewStoreError),
}

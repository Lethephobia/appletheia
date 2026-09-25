use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::projection::OrganizationMembershipFragmentWriterError;

/// Error returned while projecting organization membership fragments.
#[derive(Debug, Error)]
pub enum OrganizationMembershipFragmentProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] OrganizationMembershipFragmentWriterError),
}

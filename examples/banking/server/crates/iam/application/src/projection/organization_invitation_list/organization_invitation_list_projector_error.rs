use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::OrganizationInvitationListWriterError;

/// Error returned while projecting organization invitation list read models.
#[derive(Debug, Error)]
pub enum OrganizationInvitationListProjectorError {
    #[error("event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("organization invitation list writer failed")]
    Writer(#[from] OrganizationInvitationListWriterError),
}

use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::UserOrganizationInvitationListWriterError;

/// Error returned while projecting user organization invitation list read models.
#[derive(Debug, Error)]
pub enum UserOrganizationInvitationListProjectorError {
    #[error("event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("user organization invitation list writer failed")]
    Writer(#[from] UserOrganizationInvitationListWriterError),
}

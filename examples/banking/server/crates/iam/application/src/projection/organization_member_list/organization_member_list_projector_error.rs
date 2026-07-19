use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::OrganizationMemberListWriterError;

/// Error returned while projecting organization member lists.
#[derive(Debug, Error)]
pub enum OrganizationMemberListProjectorError {
    #[error("event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("organization member list writer failed")]
    Writer(#[from] OrganizationMemberListWriterError),
}

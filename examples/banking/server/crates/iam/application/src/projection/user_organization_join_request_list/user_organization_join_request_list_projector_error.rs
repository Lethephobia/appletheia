use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

use crate::read_model::UserOrganizationJoinRequestListWriterError;

/// Error returned while projecting user organization join request list read models.
#[derive(Debug, Error)]
pub enum UserOrganizationJoinRequestListProjectorError {
    #[error("event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("user organization join request list writer failed")]
    Writer(#[from] UserOrganizationJoinRequestListWriterError),
}

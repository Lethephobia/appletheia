use std::error::Error as StdError;

use thiserror::Error;

use crate::request_context::MessageId;

#[derive(Debug, Error)]
pub enum IdempotencyServiceError {
    #[error("idempotency key conflict: {message_id}")]
    Conflict { message_id: MessageId },

    #[error("invalid idempotency state transition")]
    InvalidStateTransition,

    #[error("persistence error: {0}")]
    Persistence(#[source] Box<dyn StdError + Send + Sync + 'static>),
}

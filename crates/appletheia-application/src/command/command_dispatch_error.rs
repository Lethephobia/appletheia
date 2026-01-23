use std::error::Error;

use thiserror::Error;

use crate::command::CommandFailureReport;
use crate::command::CommandHasherError;
use crate::idempotency::IdempotencyError;
use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWorkError;

#[derive(Debug, Error)]
pub enum CommandDispatchError<HE>
where
    HE: Error + Send + Sync + 'static,
{
    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("idempotency error: {0}")]
    Idempotency(#[from] IdempotencyError),

    #[error("command handler error: {0}")]
    Handler(#[source] HE),

    #[error("command is still in progress: {message_id}")]
    InProgress { message_id: MessageId },

    #[error("previous command failed: {0}")]
    PreviousFailure(CommandFailureReport),

    #[error("command hasher error: {0}")]
    Hasher(#[from] CommandHasherError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

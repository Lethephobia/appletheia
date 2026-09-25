use thiserror::Error;

use crate::Retryability;
use crate::authorization::AuthorizerError;
use crate::command::{CommandHasherError, IdempotencyServiceError};
use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWorkError;
use crate::unit_of_work::UnitOfWorkFactoryError;

#[derive(Debug, Error)]
pub enum CommandDispatcherError<HE>
where
    HE: Retryability,
{
    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("idempotency error: {0}")]
    Idempotency(#[from] IdempotencyServiceError),

    #[error("command handler error: {0}")]
    Handler(#[source] HE),

    #[error("command is still in progress: {message_id}")]
    InProgress { message_id: MessageId },

    #[error("command hasher error: {0}")]
    Hasher(#[from] CommandHasherError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("authorizer error: {0}")]
    Authorizer(#[from] AuthorizerError),
}

impl<HE> Retryability for CommandDispatcherError<HE>
where
    HE: Retryability,
{
    fn is_retryable(&self) -> bool {
        match self {
            Self::Handler(error) => error.is_retryable(),
            Self::UnitOfWorkFactory(_)
            | Self::UnitOfWork(_)
            | Self::Idempotency(_)
            | Self::InProgress { .. }
            | Self::Hasher(_)
            | Self::Json(_)
            | Self::Authorizer(_) => true,
        }
    }
}

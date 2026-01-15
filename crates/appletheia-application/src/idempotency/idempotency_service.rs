use crate::command::CommandFailureReport;
use crate::command::CommandName;
use crate::idempotency::{IdempotencyBeginResult, IdempotencyError};
use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait IdempotencyService: Send + Sync {
    type Uow: UnitOfWork;

    async fn begin(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        command_name: CommandName,
        request_hash: &str,
    ) -> Result<IdempotencyBeginResult, IdempotencyError>;

    async fn complete_success(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        output: serde_json::Value,
    ) -> Result<(), IdempotencyError>;

    async fn complete_failure(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        error: CommandFailureReport,
    ) -> Result<(), IdempotencyError>;
}

use crate::command::CommandFailureReport;
use crate::command::{CommandHash, CommandName};
use crate::idempotency::IdempotencyOutput;
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
        command_hash: &CommandHash,
    ) -> Result<IdempotencyBeginResult, IdempotencyError>;

    async fn complete_success(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        output: IdempotencyOutput,
    ) -> Result<(), IdempotencyError>;

    async fn complete_failure(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        error: CommandFailureReport,
    ) -> Result<(), IdempotencyError>;
}

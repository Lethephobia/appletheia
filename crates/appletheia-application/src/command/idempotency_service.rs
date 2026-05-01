use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWork;

use super::{
    CommandHash, CommandName, IdempotencyBeginResult, IdempotencyOutput, IdempotencyServiceError,
};

#[allow(async_fn_in_trait)]
pub trait IdempotencyService: Send + Sync {
    type Uow: UnitOfWork;

    async fn begin(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        command_name: CommandName,
        command_hash: &CommandHash,
    ) -> Result<IdempotencyBeginResult, IdempotencyServiceError>;

    async fn complete(
        &self,
        uow: &mut Self::Uow,
        message_id: MessageId,
        output: IdempotencyOutput,
    ) -> Result<(), IdempotencyServiceError>;
}

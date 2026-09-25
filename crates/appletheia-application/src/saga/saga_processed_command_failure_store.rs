use crate::command::CommandFailureId;
use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaInstanceId, SagaProcessedCommandFailureStoreError};

#[allow(async_fn_in_trait)]
pub trait SagaProcessedCommandFailureStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        saga_instance_id: SagaInstanceId,
        command_failure_id: CommandFailureId,
        command_message_id: MessageId,
    ) -> Result<bool, SagaProcessedCommandFailureStoreError>;
}

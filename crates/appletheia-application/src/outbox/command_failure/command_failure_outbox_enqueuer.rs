use crate::command::CommandFailureEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::CommandFailureOutboxEnqueueError;

/// Enqueues terminal command-failure notifications atomically.
#[allow(async_fn_in_trait)]
pub trait CommandFailureOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    async fn enqueue_command_failure(
        &self,
        uow: &mut Self::Uow,
        failure: &CommandFailureEnvelope,
    ) -> Result<(), CommandFailureOutboxEnqueueError>;
}

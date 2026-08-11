use crate::command::CommandEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::CommandOutboxEnqueueError;

#[allow(async_fn_in_trait)]
pub trait CommandOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    /// Enqueues one or more commands atomically.
    async fn enqueue_commands(
        &self,
        uow: &mut Self::Uow,
        commands: &[CommandEnvelope],
    ) -> Result<(), CommandOutboxEnqueueError>;
}

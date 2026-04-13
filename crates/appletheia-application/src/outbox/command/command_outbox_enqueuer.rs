use crate::unit_of_work::UnitOfWork;

use super::{CommandEnvelope, CommandOutboxEnqueueError};

#[allow(async_fn_in_trait)]
pub trait CommandOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    async fn enqueue_command(
        &self,
        uow: &mut Self::Uow,
        command: &CommandEnvelope,
    ) -> Result<(), CommandOutboxEnqueueError> {
        self.enqueue_commands(uow, std::slice::from_ref(command))
            .await
    }

    async fn enqueue_commands(
        &self,
        uow: &mut Self::Uow,
        commands: &[CommandEnvelope],
    ) -> Result<(), CommandOutboxEnqueueError>;
}

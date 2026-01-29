use crate::unit_of_work::UnitOfWork;

use super::{CommandEnvelope, CommandOutboxEnqueueError};

#[allow(async_fn_in_trait)]
pub trait CommandOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    async fn enqueue_commands(
        &self,
        uow: &mut Self::Uow,
        commands: &[CommandEnvelope],
    ) -> Result<(), CommandOutboxEnqueueError>;
}

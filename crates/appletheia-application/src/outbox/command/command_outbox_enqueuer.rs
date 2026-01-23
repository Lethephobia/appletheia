use crate::command::CommandName;
use crate::outbox::OrderingKey;
use crate::unit_of_work::UnitOfWork;

use super::{CommandEnvelope, CommandOutboxEnqueueError};

#[allow(async_fn_in_trait)]
pub trait CommandOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;
    type CommandName: CommandName;

    async fn enqueue_commands(
        &self,
        uow: &mut Self::Uow,
        ordering_key: &OrderingKey,
        commands: &[CommandEnvelope<Self::CommandName>],
    ) -> Result<(), CommandOutboxEnqueueError>;
}

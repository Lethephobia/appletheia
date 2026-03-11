use crate::command::{Command, CommandDispatcherError, CommandHandler, CommandOptions};
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait CommandDispatcher: Send + Sync {
    type Uow: UnitOfWork;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        command: H::Command,
        options: CommandOptions,
    ) -> Result<H::Output, CommandDispatcherError<H::Error>>
    where
        H: CommandHandler<Uow = Self::Uow>,
        H::Command: Command;
}

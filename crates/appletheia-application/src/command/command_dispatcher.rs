use crate::command::{CommandDispatchError, CommandHandler};
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait CommandDispatcher: Send + Sync {
    type Uow: UnitOfWork;

    async fn dispatch<H>(
        &self,
        handler: &H,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: H::Command,
    ) -> Result<H::Output, CommandDispatchError<H::Error>>
    where
        H: CommandHandler<Uow = Self::Uow>;
}

use crate::command::{CommandDispatchError, CommandHandler};
use crate::request_context::RequestContext;

#[allow(async_fn_in_trait)]
pub trait CommandDispatcher {
    async fn dispatch<H: CommandHandler>(
        &self,
        handler: &H,
        uow: &mut H::Uow,
        request_context: &RequestContext,
        command: H::Command,
    ) -> Result<H::Output, CommandDispatchError<H::Error>>;
}

use crate::command::{CommandDispatchError, CommandDispatcher, CommandHandler};
use crate::request_context::RequestContext;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkError};

#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultCommandDispatcher;

impl DefaultCommandDispatcher {
    pub fn new() -> Self {
        Self
    }
}

impl CommandDispatcher for DefaultCommandDispatcher {
    async fn dispatch<H: CommandHandler>(
        &self,
        handler: &H,
        uow: &mut H::Uow,
        request_context: &RequestContext,
        command: H::Command,
    ) -> Result<H::Output, CommandDispatchError<H::Error>> {
        uow.begin().await?;
        let operation_result = handler
            .handle(uow, request_context, command)
            .await
            .map_err(CommandDispatchError::Handler);

        match operation_result {
            Ok(output) => {
                uow.commit().await?;
                Ok(output)
            }
            Err(operation_error) => match uow.rollback().await {
                Ok(()) => Err(operation_error),
                Err(rollback_error) => Err(CommandDispatchError::UnitOfWork(
                    UnitOfWorkError::OperationAndRollbackFailed {
                        operation_error: Box::new(operation_error),
                        rollback_error: Box::new(rollback_error),
                    },
                )),
            },
        }
    }
}

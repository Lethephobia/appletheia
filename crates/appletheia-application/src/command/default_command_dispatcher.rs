use crate::command::{
    Command, CommandDispatchError, CommandDispatcher, CommandFailureReport, CommandHandler,
    CommandHasher,
};
use crate::idempotency::{
    IdempotencyBeginResult, IdempotencyOutput, IdempotencyService, IdempotencyState,
};
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

#[derive(Debug)]
pub struct DefaultCommandDispatcher<CH, IS, U> {
    command_hasher: CH,
    idempotency_service: IS,
    uow_factory: U,
}

impl<CH, IS, U> DefaultCommandDispatcher<CH, IS, U>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
{
    pub fn new(command_hasher: CH, idempotency_service: IS, uow_factory: U) -> Self {
        Self {
            command_hasher,
            idempotency_service,
            uow_factory,
        }
    }

    pub fn command_hasher(&self) -> &CH {
        &self.command_hasher
    }

    pub fn idempotency_service(&self) -> &IS {
        &self.idempotency_service
    }
}

impl<CH, IS, U> CommandDispatcher for DefaultCommandDispatcher<CH, IS, U>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
{
    type Uow = IS::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        command: H::Command,
    ) -> Result<H::Output, CommandDispatchError<H::Error>>
    where
        H: CommandHandler<Uow = Self::Uow>,
        H::Command: Command,
    {
        let command_name = H::Command::NAME;
        let command_hash = self.command_hasher.command_hash(&command)?;
        let message_id = request_context.message_id;

        let mut uow = self.uow_factory.begin().await?;

        let idempotency_begin_result = self
            .idempotency_service
            .begin(&mut uow, message_id, command_name, &command_hash)
            .await;

        let idempotency_begin_result = match idempotency_begin_result {
            Ok(value) => value,
            Err(operation_error) => {
                let operation_error = uow.rollback_with_operation_error(operation_error).await?;
                return Err(operation_error.into());
            }
        };

        match idempotency_begin_result {
            IdempotencyBeginResult::New => {}
            IdempotencyBeginResult::InProgress => match uow.rollback().await {
                Ok(()) => return Err(CommandDispatchError::InProgress { message_id }),
                Err(rollback_error) => return Err(rollback_error.into()),
            },
            IdempotencyBeginResult::Existing { state } => match state {
                IdempotencyState::Succeeded { output } => {
                    let decoded = serde_json::from_value(output.into())?;
                    uow.commit().await?;
                    return Ok(decoded);
                }
                IdempotencyState::Failed { error } => {
                    uow.commit().await?;
                    return Err(CommandDispatchError::PreviousFailure(error));
                }
            },
        }

        let handler_result = handler.handle(&mut uow, request_context, command).await;

        match handler_result {
            Ok(output) => {
                let output_json = serde_json::to_value(&output)?;
                match self
                    .idempotency_service
                    .complete_success(&mut uow, message_id, IdempotencyOutput::from(output_json))
                    .await
                {
                    Ok(()) => {}
                    Err(operation_error) => {
                        let operation_error =
                            uow.rollback_with_operation_error(operation_error).await?;
                        return Err(operation_error.into());
                    }
                }
                uow.commit().await?;
                Ok(output)
            }
            Err(operation_error) => {
                let operation_error = uow
                    .rollback_with_operation_error(operation_error)
                    .await
                    .map_err(CommandDispatchError::UnitOfWork)?;

                let report = CommandFailureReport::from(&operation_error);
                if let Ok(mut uow) = self.uow_factory.begin().await {
                    let idempotency_begin_result = self
                        .idempotency_service
                        .begin(&mut uow, message_id, command_name, &command_hash)
                        .await;
                    match idempotency_begin_result {
                        Ok(IdempotencyBeginResult::New) => {
                            match self
                                .idempotency_service
                                .complete_failure(&mut uow, message_id, report)
                                .await
                            {
                                Ok(()) => {
                                    let _ = uow.commit().await;
                                }
                                Err(_) => {
                                    let _ = uow.rollback().await;
                                }
                            }
                        }
                        Ok(IdempotencyBeginResult::Existing { .. }) => {
                            let _ = uow.commit().await;
                        }
                        Ok(IdempotencyBeginResult::InProgress) => {
                            let _ = uow.rollback().await;
                        }
                        Err(_) => {
                            let _ = uow.rollback().await;
                        }
                    }
                }
                Err(CommandDispatchError::Handler(operation_error))
            }
        }
    }
}

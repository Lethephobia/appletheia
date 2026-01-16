use crate::command::{
    Command, CommandDispatchError, CommandDispatcher, CommandFailureReport, CommandHandler,
    CommandHasher,
};
use crate::idempotency::{
    IdempotencyBeginResult, IdempotencyOutput, IdempotencyService, IdempotencyState,
};
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

#[derive(Debug)]
pub struct DefaultCommandDispatcher<CH, IS> {
    command_hasher: CH,
    idempotency_service: IS,
}

impl<CH, IS> DefaultCommandDispatcher<CH, IS>
where
    CH: CommandHasher,
    IS: IdempotencyService,
{
    pub fn new(command_hasher: CH, idempotency_service: IS) -> Self {
        Self {
            command_hasher,
            idempotency_service,
        }
    }

    pub fn command_hasher(&self) -> &CH {
        &self.command_hasher
    }

    pub fn idempotency_service(&self) -> &IS {
        &self.idempotency_service
    }
}

impl<CH, IS> CommandDispatcher for DefaultCommandDispatcher<CH, IS>
where
    CH: CommandHasher,
    IS: IdempotencyService,
{
    type Uow = IS::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: H::Command,
    ) -> Result<H::Output, CommandDispatchError<H::Error>>
    where
        H: CommandHandler<Uow = Self::Uow>,
    {
        let command_name = H::Command::COMMAND_NAME;
        let command_hash = {
            let command_json = serde_json::to_value(&command)?;
            self.command_hasher.command_hash(command_json)
        };
        let message_id = request_context.message_id;

        uow.begin().await?;

        let idempotency_begin_result = self
            .idempotency_service
            .begin(uow, message_id, command_name, &command_hash)
            .await;

        let idempotency_begin_result = match idempotency_begin_result {
            Ok(value) => value,
            Err(operation_error) => {
                return Err(uow
                    .rollback_with_operation_error(operation_error)
                    .await?
                    .into());
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
                    let decoded: H::Output = serde_json::from_value(output.into())?;
                    uow.commit().await?;
                    return Ok(decoded);
                }
                IdempotencyState::Failed { error } => {
                    uow.commit().await?;
                    return Err(CommandDispatchError::PreviousFailure(error));
                }
            },
        }

        let handler_result = handler.handle(uow, request_context, command).await;

        match handler_result {
            Ok(output) => {
                let output_json = serde_json::to_value(&output)?;
                match self
                    .idempotency_service
                    .complete_success(uow, message_id, IdempotencyOutput::from(output_json))
                    .await
                {
                    Ok(()) => {}
                    Err(operation_error) => {
                        return Err(uow
                            .rollback_with_operation_error(operation_error)
                            .await?
                            .into());
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
                if uow.begin().await.is_ok() {
                    let idempotency_begin_result = self
                        .idempotency_service
                        .begin(uow, message_id, command_name, &command_hash)
                        .await;
                    match idempotency_begin_result {
                        Ok(IdempotencyBeginResult::New) => {
                            match self
                                .idempotency_service
                                .complete_failure(uow, message_id, report)
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
                    };
                }
                Err(CommandDispatchError::Handler(operation_error))
            }
        }
    }
}

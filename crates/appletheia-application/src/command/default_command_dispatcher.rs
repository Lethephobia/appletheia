use crate::authorization::{Authorizer, RelationshipRequirement};
use crate::command::{
    Command, CommandConsistency, CommandDispatcher, CommandDispatcherError, CommandFailureReport,
    CommandHandler, CommandHasher, CommandOptions, IdempotencyBeginResult, IdempotencyOutput,
    IdempotencyService, IdempotencyState,
};
use crate::projection::ReadYourWritesWaiter;
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

#[derive(Debug)]
pub struct DefaultCommandDispatcher<CH, IS, W, U, AZ>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    IS::Uow: UnitOfWork,
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
{
    command_hasher: CH,
    idempotency_service: IS,
    read_your_writes_waiter: W,
    uow_factory: U,
    authorizer: AZ,
}

impl<CH, IS, W, U, AZ> DefaultCommandDispatcher<CH, IS, W, U, AZ>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
{
    pub fn new(
        command_hasher: CH,
        idempotency_service: IS,
        read_your_writes_waiter: W,
        uow_factory: U,
        authorizer: AZ,
    ) -> Self {
        Self {
            command_hasher,
            idempotency_service,
            read_your_writes_waiter,
            uow_factory,
            authorizer,
        }
    }
}

impl<CH, IS, W, U, AZ> CommandDispatcher for DefaultCommandDispatcher<CH, IS, W, U, AZ>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
{
    type Uow = IS::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        command: H::Command,
        options: CommandOptions,
    ) -> Result<H::Output, CommandDispatcherError<H::Error>>
    where
        H: CommandHandler<Uow = Self::Uow>,
        H::Command: Command,
    {
        let command_name = H::Command::NAME;
        let authorization_plan = handler.authorization_plan(&command);
        let requirement = authorization_plan.requirement;
        let authorization_dependencies = authorization_plan.dependencies;

        match options.consistency {
            CommandConsistency::Eventual => {}
            CommandConsistency::ReadYourWrites {
                after,
                timeout,
                poll_interval,
            } => {
                if !matches!(requirement, RelationshipRequirement::None) {
                    let projectors = authorization_dependencies.owned_names();
                    self.read_your_writes_waiter
                        .wait(after, timeout, poll_interval, &projectors)
                        .await?;
                }
            }
        }
        self.authorizer
            .authorize(&request_context.principal, &requirement)
            .await?;

        match options.consistency {
            CommandConsistency::Eventual => {}
            CommandConsistency::ReadYourWrites {
                after,
                timeout,
                poll_interval,
            } => {
                let projectors = H::DEPENDENCIES.owned_names();
                self.read_your_writes_waiter
                    .wait(after, timeout, poll_interval, &projectors)
                    .await?;
            }
        }

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
                Ok(()) => return Err(CommandDispatcherError::InProgress { message_id }),
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
                    return Err(CommandDispatcherError::PreviousFailure(error));
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
                    .map_err(CommandDispatcherError::UnitOfWork)?;

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
                Err(CommandDispatcherError::Handler(operation_error))
            }
        }
    }
}

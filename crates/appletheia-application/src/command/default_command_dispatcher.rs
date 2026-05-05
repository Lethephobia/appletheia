use crate::authorization::Authorizer;
use crate::command::{
    Command, CommandConsistency, CommandDispatchResult, CommandDispatcher, CommandDispatcherError,
    CommandHandler, CommandHasher, CommandOptions, IdempotencyBeginResult, IdempotencyService,
};
use crate::projection::ReadYourWritesWaiter;
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

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
    ) -> Result<CommandDispatchResult<H::Output, H::ReplayOutput>, CommandDispatcherError<H::Error>>
    where
        H: CommandHandler<Uow = Self::Uow>,
        H::Command: Command,
    {
        let command_name = H::Command::NAME;
        let authorization_plan = handler
            .authorization_plan(&command)
            .map_err(CommandDispatcherError::Handler)?;
        self.authorizer
            .authorize(&request_context.principal, &authorization_plan)
            .await?;

        match options.consistency {
            CommandConsistency::Eventual => {}
            CommandConsistency::ReadYourWrites {
                target,
                timeout,
                poll_interval,
            } => {
                self.read_your_writes_waiter
                    .wait(target, timeout, poll_interval, H::PROJECTOR_DEPENDENCIES)
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
            IdempotencyBeginResult::Existing { output } => {
                let decoded = serde_json::from_value(output.into())?;
                uow.commit().await?;
                return Ok(CommandDispatchResult::Replayed(decoded));
            }
        }

        let handler_result = handler.handle(&mut uow, request_context, &command).await;

        match handler_result {
            Ok(handled) => {
                let replay_output = handled.idempotency_output()?;
                let output = handled.into_output();
                match self
                    .idempotency_service
                    .complete(&mut uow, message_id, replay_output)
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
                Ok(CommandDispatchResult::Executed(output))
            }
            Err(operation_error) => {
                let operation_error = uow
                    .rollback_with_operation_error(operation_error)
                    .await
                    .map_err(CommandDispatcherError::UnitOfWork)?;
                Err(CommandDispatcherError::Handler(operation_error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::DefaultCommandDispatcher;
    use crate::authorization::{AuthorizationPlan, Authorizer, AuthorizerError};
    use crate::command::{
        Command, CommandDispatcher, CommandDispatcherError, CommandHandled, CommandHandler,
        CommandHash, CommandHasher, CommandHasherError, CommandName, CommandOptions,
        IdempotencyBeginResult, IdempotencyOutput, IdempotencyService, IdempotencyServiceError,
    };
    use crate::projection::ReadYourWritesTarget;
    use crate::projection::{
        ProjectorDependencies, ReadYourWritesPollInterval, ReadYourWritesTimeout,
        ReadYourWritesWaitError, ReadYourWritesWaiter,
    };
    use crate::request_context::MessageId;
    use crate::request_context::Principal;
    use crate::unit_of_work::{
        UnitOfWork, UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError,
    };

    struct TestWaiter;

    impl ReadYourWritesWaiter for TestWaiter {
        async fn wait(
            &self,
            _target: ReadYourWritesTarget,
            _timeout: ReadYourWritesTimeout,
            _poll_interval: ReadYourWritesPollInterval,
            _projector_dependencies: ProjectorDependencies<'_>,
        ) -> Result<(), ReadYourWritesWaitError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    struct TestUowFactory;

    impl UnitOfWorkFactory for TestUowFactory {
        type Uow = TestUow;

        async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError> {
            Ok(TestUow)
        }
    }

    struct TestAuthorizer;

    impl Authorizer for TestAuthorizer {
        async fn authorize(
            &self,
            _principal: &Principal,
            _authorization_plan: &AuthorizationPlan,
        ) -> Result<(), AuthorizerError> {
            Ok(())
        }
    }

    struct TestCommandHasher;

    impl CommandHasher for TestCommandHasher {
        fn command_hash<C: Command>(
            &self,
            _command: &C,
        ) -> Result<CommandHash, CommandHasherError> {
            CommandHash::new("0".repeat(CommandHash::LENGTH)).map_err(CommandHasherError::from)
        }
    }

    struct TestNewIdempotencyService;

    impl IdempotencyService for TestNewIdempotencyService {
        type Uow = TestUow;

        async fn begin(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _command_name: CommandName,
            _command_hash: &CommandHash,
        ) -> Result<IdempotencyBeginResult, IdempotencyServiceError> {
            Ok(IdempotencyBeginResult::New)
        }

        async fn complete(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _output: IdempotencyOutput,
        ) -> Result<(), IdempotencyServiceError> {
            Ok(())
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestCommand {}

    impl Command for TestCommand {
        const NAME: CommandName = CommandName::new("test");
    }

    #[derive(Debug, thiserror::Error)]
    #[error("business rule failed")]
    struct TestHandlerError;

    struct TestCommandFailureHandler;

    impl CommandHandler for TestCommandFailureHandler {
        type Command = TestCommand;
        type Output = ();
        type ReplayOutput = ();
        type Error = TestHandlerError;
        type Uow = TestUow;

        async fn handle(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &crate::request_context::RequestContext,
            _command: &Self::Command,
        ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
            Err(TestHandlerError)
        }
    }

    #[tokio::test]
    async fn dispatch_rolls_back_command_failure() {
        let dispatcher = DefaultCommandDispatcher::new(
            TestCommandHasher,
            TestNewIdempotencyService,
            TestWaiter,
            TestUowFactory,
            TestAuthorizer,
        );
        let request_context = crate::request_context::RequestContext::new(
            crate::request_context::CorrelationId::from(Uuid::now_v7()),
            crate::request_context::MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid");

        let result = dispatcher
            .dispatch(
                &TestCommandFailureHandler,
                &request_context,
                TestCommand {},
                CommandOptions::default(),
            )
            .await;

        assert!(matches!(result, Err(CommandDispatcherError::Handler(_))));
    }
}

use crate::authorization::Authorizer;
use crate::command::{
    Command, CommandDispatchResult, CommandDispatcher, CommandDispatcherError, CommandHandler,
    CommandHasher, CommandOptions, CommandOutput, IdempotencyBeginResult, IdempotencyOutput,
    IdempotencyService,
};
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

pub struct DefaultCommandDispatcher<CH, IS, U, AZ>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    IS::Uow: UnitOfWork,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
{
    command_hasher: CH,
    idempotency_service: IS,
    uow_factory: U,
    authorizer: AZ,
}

impl<CH, IS, U, AZ> DefaultCommandDispatcher<CH, IS, U, AZ>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
{
    pub fn new(
        command_hasher: CH,
        idempotency_service: IS,
        uow_factory: U,
        authorizer: AZ,
    ) -> Self {
        Self {
            command_hasher,
            idempotency_service,
            uow_factory,
            authorizer,
        }
    }
}

impl<CH, IS, U, AZ> CommandDispatcher for DefaultCommandDispatcher<CH, IS, U, AZ>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
{
    type Uow = IS::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        command: H::Command,
        _options: CommandOptions,
    ) -> Result<
        CommandDispatchResult<H::Output, <H::Output as CommandOutput>::ReplayOutput>,
        CommandDispatcherError<H::Error>,
    >
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

        let command_hash = self.command_hasher.command_hash(&command)?;
        let message_id = request_context.message_id;
        let mut uow = self.uow_factory.begin().await?;

        let idempotency_begin_attempt = self
            .idempotency_service
            .begin(&mut uow, message_id, command_name, &command_hash)
            .await;

        let idempotency_begin_result = match idempotency_begin_attempt {
            Ok(value) => value,
            Err(operation_error) => {
                let rolled_back_error = uow.rollback_with_operation_error(operation_error).await?;
                return Err(rolled_back_error.into());
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
            Ok(output) => {
                let replay_output = serde_json::to_value(output.replay_output())?;
                let idempotency_output = IdempotencyOutput::from(replay_output);
                match self
                    .idempotency_service
                    .complete(&mut uow, message_id, idempotency_output)
                    .await
                {
                    Ok(()) => {}
                    Err(completion_error) => {
                        let rolled_back_error =
                            uow.rollback_with_operation_error(completion_error).await?;
                        return Err(rolled_back_error.into());
                    }
                }
                uow.commit().await?;
                Ok(CommandDispatchResult::Executed(output))
            }
            Err(handler_error) => {
                let rolled_back_handler_error = uow
                    .rollback_with_operation_error(handler_error)
                    .await
                    .map_err(CommandDispatcherError::UnitOfWork)?;
                Err(CommandDispatcherError::Handler(rolled_back_handler_error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::DefaultCommandDispatcher;
    use crate::Retryability;
    use crate::authorization::{AuthorizationPlan, Authorizer, AuthorizerError};
    use crate::command::{
        Command, CommandDispatchResult, CommandDispatcher, CommandDispatcherError, CommandHandler,
        CommandHash, CommandHasher, CommandHasherError, CommandName, CommandOptions, CommandOutput,
        CommandReplayOutput, IdempotencyBeginResult, IdempotencyOutput, IdempotencyService,
        IdempotencyServiceError,
    };
    use crate::request_context::{MessageId, Principal, RequestContext};
    use crate::unit_of_work::{
        UnitOfWork, UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError,
    };

    #[derive(Default)]
    struct TestState {
        begun: AtomicUsize,
        committed: AtomicUsize,
        rolled_back: AtomicUsize,
        handler_calls: AtomicUsize,
    }

    struct TestUow {
        state: Arc<TestState>,
    }

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            self.state.committed.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            self.state.rolled_back.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct TestUowFactory {
        state: Arc<TestState>,
    }

    impl UnitOfWorkFactory for TestUowFactory {
        type Uow = TestUow;

        async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError> {
            self.state.begun.fetch_add(1, Ordering::SeqCst);
            Ok(TestUow {
                state: Arc::clone(&self.state),
            })
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

    struct TestRecordingNewIdempotencyService {
        completed_output: Arc<Mutex<Option<IdempotencyOutput>>>,
    }

    impl IdempotencyService for TestRecordingNewIdempotencyService {
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
            output: IdempotencyOutput,
        ) -> Result<(), IdempotencyServiceError> {
            *self
                .completed_output
                .lock()
                .expect("completed output should be lockable") = Some(output);
            Ok(())
        }
    }

    struct TestExistingIdempotencyService;

    impl IdempotencyService for TestExistingIdempotencyService {
        type Uow = TestUow;

        async fn begin(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _command_name: CommandName,
            _command_hash: &CommandHash,
        ) -> Result<IdempotencyBeginResult, IdempotencyServiceError> {
            Ok(IdempotencyBeginResult::Existing {
                output: IdempotencyOutput::from(serde_json::json!({
                    "value": "replayed",
                })),
            })
        }

        async fn complete(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _output: IdempotencyOutput,
        ) -> Result<(), IdempotencyServiceError> {
            unreachable!("existing idempotency output should not be completed again")
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestCommand {}

    impl Command for TestCommand {
        const NAME: CommandName = CommandName::new("test");
    }

    #[derive(Debug, thiserror::Error)]
    #[error("handler failed")]
    struct TestHandlerError {
        retryable: bool,
    }

    impl Retryability for TestHandlerError {
        fn is_retryable(&self) -> bool {
            self.retryable
        }
    }

    struct TestCommandFailureHandler {
        state: Arc<TestState>,
        retryable: bool,
    }

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct TestOutput {
        value: String,
    }

    impl CommandOutput for TestOutput {
        type ReplayOutput = Self;

        fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
            CommandReplayOutput::Borrowed(self)
        }
    }

    struct TestCommandSuccessHandler {
        state: Arc<TestState>,
    }

    impl CommandHandler for TestCommandSuccessHandler {
        type Command = TestCommand;
        type Output = TestOutput;
        type Error = TestHandlerError;
        type Uow = TestUow;

        async fn handle(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            _command: &Self::Command,
        ) -> Result<Self::Output, Self::Error> {
            self.state.handler_calls.fetch_add(1, Ordering::SeqCst);
            Ok(TestOutput {
                value: "executed".to_owned(),
            })
        }
    }

    impl CommandHandler for TestCommandFailureHandler {
        type Command = TestCommand;
        type Output = TestOutput;
        type Error = TestHandlerError;
        type Uow = TestUow;

        async fn handle(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &crate::request_context::RequestContext,
            _command: &Self::Command,
        ) -> Result<Self::Output, Self::Error> {
            self.state.handler_calls.fetch_add(1, Ordering::SeqCst);
            Err(TestHandlerError {
                retryable: self.retryable,
            })
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            crate::request_context::CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid")
    }

    #[tokio::test]
    async fn dispatch_returns_immediate_output_and_persists_its_replay_json() {
        let state = Arc::new(TestState::default());
        let completed_output = Arc::new(Mutex::new(None));
        let dispatcher = DefaultCommandDispatcher::new(
            TestCommandHasher,
            TestRecordingNewIdempotencyService {
                completed_output: Arc::clone(&completed_output),
            },
            TestUowFactory {
                state: Arc::clone(&state),
            },
            TestAuthorizer,
        );

        let result = dispatcher
            .dispatch(
                &TestCommandSuccessHandler {
                    state: Arc::clone(&state),
                },
                &request_context(),
                TestCommand {},
                CommandOptions::default(),
            )
            .await
            .expect("command should execute");

        assert_eq!(
            result,
            CommandDispatchResult::Executed(TestOutput {
                value: "executed".to_owned(),
            })
        );
        assert_eq!(
            completed_output
                .lock()
                .expect("completed output should be lockable")
                .as_ref()
                .expect("completed output should be recorded")
                .value(),
            &serde_json::json!({ "value": "executed" })
        );
        assert_eq!(state.handler_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn dispatch_returns_typed_replay_output_without_calling_the_handler() {
        let state = Arc::new(TestState::default());
        let dispatcher = DefaultCommandDispatcher::new(
            TestCommandHasher,
            TestExistingIdempotencyService,
            TestUowFactory {
                state: Arc::clone(&state),
            },
            TestAuthorizer,
        );

        let result = dispatcher
            .dispatch(
                &TestCommandSuccessHandler {
                    state: Arc::clone(&state),
                },
                &request_context(),
                TestCommand {},
                CommandOptions::default(),
            )
            .await
            .expect("command should replay");

        assert_eq!(
            result,
            CommandDispatchResult::Replayed(TestOutput {
                value: "replayed".to_owned(),
            })
        );
        assert_eq!(state.handler_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn dispatch_rolls_back_retryable_handler_failure() {
        let state = Arc::new(TestState::default());
        let dispatcher = DefaultCommandDispatcher::new(
            TestCommandHasher,
            TestNewIdempotencyService,
            TestUowFactory {
                state: Arc::clone(&state),
            },
            TestAuthorizer,
        );
        let request_context = request_context();

        let result = dispatcher
            .dispatch(
                &TestCommandFailureHandler {
                    state: Arc::clone(&state),
                    retryable: true,
                },
                &request_context,
                TestCommand {},
                CommandOptions::default(),
            )
            .await;

        let error = result.expect_err("handler failure should be returned");
        assert!(matches!(&error, CommandDispatcherError::Handler(_)));
        assert!(error.is_retryable());
        assert_eq!(state.begun.load(Ordering::SeqCst), 1);
        assert_eq!(state.rolled_back.load(Ordering::SeqCst), 1);
        assert_eq!(state.committed.load(Ordering::SeqCst), 0);
        assert_eq!(state.handler_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn dispatch_reexecutes_non_retryable_failure_for_same_message_id() {
        let state = Arc::new(TestState::default());
        let dispatcher = DefaultCommandDispatcher::new(
            TestCommandHasher,
            TestNewIdempotencyService,
            TestUowFactory {
                state: Arc::clone(&state),
            },
            TestAuthorizer,
        );
        let request_context = request_context();

        let first_result = dispatcher
            .dispatch(
                &TestCommandFailureHandler {
                    state: Arc::clone(&state),
                    retryable: false,
                },
                &request_context,
                TestCommand {},
                CommandOptions::default(),
            )
            .await;
        let second_result = dispatcher
            .dispatch(
                &TestCommandFailureHandler {
                    state: Arc::clone(&state),
                    retryable: false,
                },
                &request_context,
                TestCommand {},
                CommandOptions::default(),
            )
            .await;

        let first_error = first_result.expect_err("first handler failure should be returned");
        let second_error = second_result.expect_err("second handler failure should be returned");
        assert!(matches!(&first_error, CommandDispatcherError::Handler(_)));
        assert!(matches!(&second_error, CommandDispatcherError::Handler(_)));
        assert!(!first_error.is_retryable());
        assert!(!second_error.is_retryable());
        assert_eq!(state.begun.load(Ordering::SeqCst), 2);
        assert_eq!(state.rolled_back.load(Ordering::SeqCst), 2);
        assert_eq!(state.committed.load(Ordering::SeqCst), 0);
        assert_eq!(state.handler_calls.load(Ordering::SeqCst), 2);
    }
}

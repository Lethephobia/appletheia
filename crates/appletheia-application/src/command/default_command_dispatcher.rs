use crate::authorization::{AuthorizationPlan, Authorizer, PrincipalRequirement};
use crate::command::{
    Command, CommandConsistency, CommandDispatchResult, CommandDispatcher, CommandDispatcherError,
    CommandFailureReaction, CommandFailureReport, CommandHandler, CommandHasher, CommandOptions,
    IdempotencyBeginResult, IdempotencyService, IdempotencyState,
};
use crate::outbox::command::CommandOutboxEnqueuer;
use crate::projection::{ProjectorDependencies, ProjectorDescriptor, ReadYourWritesWaiter};
use crate::request_context::{Principal, RequestContext};
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

pub struct DefaultCommandDispatcher<CH, IS, W, U, AZ, Q>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    IS::Uow: UnitOfWork,
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
    Q: CommandOutboxEnqueuer<Uow = IS::Uow>,
{
    command_hasher: CH,
    idempotency_service: IS,
    read_your_writes_waiter: W,
    uow_factory: U,
    authorizer: AZ,
    command_outbox_enqueuer: Q,
}

impl<CH, IS, W, U, AZ, Q> DefaultCommandDispatcher<CH, IS, W, U, AZ, Q>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
    Q: CommandOutboxEnqueuer<Uow = IS::Uow>,
{
    fn authorization_dependencies(
        principal: &Principal,
        authorization_plan: &AuthorizationPlan,
    ) -> Vec<ProjectorDescriptor> {
        if !matches!(principal, Principal::Authenticated { .. }) {
            return Vec::new();
        }

        let AuthorizationPlan::OnlyPrincipals(principal_requirements) = authorization_plan else {
            return Vec::new();
        };

        if principal_requirements.iter().any(|principal_requirement| {
            matches!(principal_requirement, PrincipalRequirement::Authenticated)
        }) {
            return Vec::new();
        }

        principal_requirements
            .iter()
            .filter_map(|principal_requirement| match principal_requirement {
                PrincipalRequirement::AuthenticatedWithRelationship {
                    projector_dependencies,
                    ..
                } => Some(projector_dependencies.to_vec()),
                PrincipalRequirement::System
                | PrincipalRequirement::Anonymous
                | PrincipalRequirement::Authenticated => None,
            })
            .flatten()
            .collect()
    }
}

impl<CH, IS, W, U, AZ, Q> DefaultCommandDispatcher<CH, IS, W, U, AZ, Q>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
    Q: CommandOutboxEnqueuer<Uow = IS::Uow>,
{
    pub fn new(
        command_hasher: CH,
        idempotency_service: IS,
        read_your_writes_waiter: W,
        uow_factory: U,
        authorizer: AZ,
        command_outbox_enqueuer: Q,
    ) -> Self {
        Self {
            command_hasher,
            idempotency_service,
            read_your_writes_waiter,
            uow_factory,
            authorizer,
            command_outbox_enqueuer,
        }
    }
}

impl<CH, IS, W, U, AZ, Q> CommandDispatcher for DefaultCommandDispatcher<CH, IS, W, U, AZ, Q>
where
    CH: CommandHasher,
    IS: IdempotencyService,
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory<Uow = IS::Uow>,
    AZ: Authorizer,
    Q: CommandOutboxEnqueuer<Uow = IS::Uow>,
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
        let authorization_dependencies =
            Self::authorization_dependencies(&request_context.principal, &authorization_plan);

        match options.consistency {
            CommandConsistency::Eventual => {}
            CommandConsistency::ReadYourWrites {
                target,
                timeout,
                poll_interval,
            } => {
                if !authorization_dependencies.is_empty() {
                    let authorization_dependencies =
                        ProjectorDependencies::Some(authorization_dependencies.as_slice());
                    self.read_your_writes_waiter
                        .wait(
                            target,
                            timeout,
                            poll_interval,
                            authorization_dependencies,
                            H::SAGA_DEPENDENCIES,
                        )
                        .await?;
                }
            }
        }
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
                    .wait(
                        target,
                        timeout,
                        poll_interval,
                        H::PROJECTOR_DEPENDENCIES,
                        H::SAGA_DEPENDENCIES,
                    )
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
                    return Ok(CommandDispatchResult::Replayed(decoded));
                }
                IdempotencyState::Failed { error } => {
                    uow.commit().await?;
                    return Err(CommandDispatcherError::PreviousFailure(error));
                }
            },
        }

        let handler_result = handler.handle(&mut uow, request_context, &command).await;

        match handler_result {
            Ok(handled) => {
                let replay_output = handled.idempotency_output()?;
                let output = handled.into_output();
                match self
                    .idempotency_service
                    .complete_success(&mut uow, message_id, replay_output)
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
                let command_failure_reaction = options.failure_reaction.clone();

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
                                Ok(()) => match command_failure_reaction {
                                    CommandFailureReaction::None => {
                                        let _ = uow.commit().await;
                                    }
                                    CommandFailureReaction::FollowUpCommands(_) => {
                                        let commands = command_failure_reaction
                                            .into_command_envelopes(request_context);
                                        match self
                                            .command_outbox_enqueuer
                                            .enqueue_commands(&mut uow, &commands)
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
                                },
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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::DefaultCommandDispatcher;
    use crate::authorization::{
        AggregateRef, AuthorizationPlan, Authorizer, AuthorizerError, PrincipalRequirement,
        RelationName, RelationRefOwned, RelationshipRequirement,
    };
    use crate::command::{
        Command, CommandDispatcher, CommandDispatcherError, CommandFailureReaction,
        CommandFailureReport, CommandHandled, CommandHandler, CommandHash, CommandHasher,
        CommandHasherError, CommandName, CommandOptions, IdempotencyBeginResult, IdempotencyOutput,
        IdempotencyService, IdempotencyServiceError,
    };
    use crate::event::{AggregateIdValue, AggregateTypeOwned};
    use crate::messaging::Subscription;
    use crate::outbox::command::{
        CommandEnvelope, CommandOutboxEnqueueError, CommandOutboxEnqueuer,
    };
    use crate::projection::ReadYourWritesTarget;
    use crate::projection::{
        ProjectorDependencies, ProjectorDescriptor, ProjectorName, ReadYourWritesPollInterval,
        ReadYourWritesTimeout, ReadYourWritesWaitError, ReadYourWritesWaiter,
    };
    use crate::request_context::MessageId;
    use crate::request_context::Principal;
    use crate::saga::SagaDependencies;
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
            _saga_dependencies: SagaDependencies<'_>,
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

    struct TestIdempotencyService;

    impl IdempotencyService for TestIdempotencyService {
        type Uow = TestUow;

        async fn begin(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _command_name: CommandName,
            _command_hash: &CommandHash,
        ) -> Result<IdempotencyBeginResult, IdempotencyServiceError> {
            Ok(IdempotencyBeginResult::InProgress)
        }

        async fn complete_success(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _output: IdempotencyOutput,
        ) -> Result<(), IdempotencyServiceError> {
            Ok(())
        }

        async fn complete_failure(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _error: CommandFailureReport,
        ) -> Result<(), IdempotencyServiceError> {
            Ok(())
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

        async fn complete_success(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _output: IdempotencyOutput,
        ) -> Result<(), IdempotencyServiceError> {
            Ok(())
        }

        async fn complete_failure(
            &self,
            _uow: &mut Self::Uow,
            _message_id: MessageId,
            _error: CommandFailureReport,
        ) -> Result<(), IdempotencyServiceError> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestCommandOutboxEnqueuer {
        commands: Arc<Mutex<Vec<CommandEnvelope>>>,
    }

    impl TestCommandOutboxEnqueuer {
        fn recorded_commands(&self) -> Vec<CommandEnvelope> {
            self.commands.lock().expect("lock").clone()
        }
    }

    impl CommandOutboxEnqueuer for TestCommandOutboxEnqueuer {
        type Uow = TestUow;

        async fn enqueue_commands(
            &self,
            _uow: &mut Self::Uow,
            commands: &[CommandEnvelope],
        ) -> Result<(), CommandOutboxEnqueueError> {
            self.commands
                .lock()
                .expect("lock")
                .extend_from_slice(commands);
            Ok(())
        }
    }

    type TestDispatcher = DefaultCommandDispatcher<
        TestCommandHasher,
        TestIdempotencyService,
        TestWaiter,
        TestUowFactory,
        TestAuthorizer,
        TestCommandOutboxEnqueuer,
    >;

    const PROJECTOR: ProjectorDescriptor =
        ProjectorDescriptor::new(ProjectorName::new("relationship"), Subscription::All);

    fn authenticated_principal() -> Principal {
        Principal::Authenticated {
            subject: AggregateRef {
                aggregate_type: AggregateTypeOwned::try_from("user").expect("valid aggregate type"),
                aggregate_id: AggregateIdValue::from(Uuid::nil()),
            },
        }
    }

    fn relationship_requirement() -> RelationshipRequirement {
        RelationshipRequirement::Check {
            aggregate: AggregateRef {
                aggregate_type: AggregateTypeOwned::try_from("document")
                    .expect("valid aggregate type"),
                aggregate_id: AggregateIdValue::from(Uuid::from_u128(1)),
            },
            relation: RelationRefOwned::new(
                AggregateTypeOwned::try_from("document").expect("valid aggregate type"),
                RelationName::new("viewer").into(),
            ),
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestCommand {}

    impl Command for TestCommand {
        const NAME: CommandName = CommandName::new("test");
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct FollowUpTestCommand {}

    impl Command for FollowUpTestCommand {
        const NAME: CommandName = CommandName::new("follow_up");
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

    #[test]
    fn skips_authorization_dependencies_for_non_authenticated_principals() {
        let authorization_plan = AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: relationship_requirement(),
                projector_dependencies: ProjectorDependencies::Some(&[PROJECTOR]),
            },
        ]);

        assert!(
            TestDispatcher::authorization_dependencies(&Principal::System, &authorization_plan)
                .is_empty()
        );
        assert!(
            TestDispatcher::authorization_dependencies(&Principal::Anonymous, &authorization_plan)
                .is_empty()
        );
        assert!(
            TestDispatcher::authorization_dependencies(
                &Principal::Unavailable,
                &authorization_plan
            )
            .is_empty()
        );
    }

    #[test]
    fn skips_authorization_dependencies_when_authenticated_requirement_is_present() {
        let authorization_plan = AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::Authenticated,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: relationship_requirement(),
                projector_dependencies: ProjectorDependencies::Some(&[PROJECTOR]),
            },
        ]);

        assert!(
            TestDispatcher::authorization_dependencies(
                &authenticated_principal(),
                &authorization_plan
            )
            .is_empty()
        );
    }

    #[test]
    fn collects_relationship_dependencies_for_authenticated_principal() {
        let authorization_plan = AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: relationship_requirement(),
                projector_dependencies: ProjectorDependencies::Some(&[PROJECTOR]),
            },
        ]);

        assert_eq!(
            TestDispatcher::authorization_dependencies(
                &authenticated_principal(),
                &authorization_plan
            ),
            vec![PROJECTOR]
        );
    }

    #[tokio::test]
    async fn dispatch_enqueues_follow_up_commands_for_command_failure() {
        let outbox_enqueuer = TestCommandOutboxEnqueuer::default();
        let dispatcher = DefaultCommandDispatcher::new(
            TestCommandHasher,
            TestNewIdempotencyService,
            TestWaiter,
            TestUowFactory,
            TestAuthorizer,
            outbox_enqueuer.clone(),
        );
        let request_context = crate::request_context::RequestContext::new(
            crate::request_context::CorrelationId::from(Uuid::now_v7()),
            crate::request_context::MessageId::new(),
            crate::request_context::ActorRef::System,
            Principal::System,
        );

        let result = dispatcher
            .dispatch(
                &TestCommandFailureHandler,
                &request_context,
                TestCommand {},
                CommandOptions {
                    failure_reaction: {
                        let mut reaction = CommandFailureReaction::new();
                        reaction
                            .push(&FollowUpTestCommand {}, CommandOptions::default())
                            .expect("reaction should serialize");
                        reaction
                    },
                    ..CommandOptions::default()
                },
            )
            .await;

        assert!(matches!(result, Err(CommandDispatcherError::Handler(_))));

        let recorded = outbox_enqueuer.recorded_commands();
        assert_eq!(recorded.len(), 1);
        assert_eq!(recorded[0].command_name.to_string(), "follow_up");
        assert_eq!(recorded[0].correlation_id, request_context.correlation_id);
        assert_eq!(
            recorded[0].causation_id,
            crate::request_context::CausationId::from(request_context.message_id)
        );
    }
}

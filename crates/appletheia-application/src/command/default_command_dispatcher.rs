use crate::authorization::{AuthorizationPlan, Authorizer, PrincipalRequirement};
use crate::command::{
    Command, CommandConsistency, CommandDispatchResult, CommandDispatcher, CommandDispatcherError,
    CommandFailureReport, CommandHandler, CommandHasher, CommandOptions, IdempotencyBeginResult,
    IdempotencyService, IdempotencyState,
};
use crate::projection::{ProjectorDependencies, ProjectorDescriptor, ReadYourWritesWaiter};
use crate::request_context::{Principal, RequestContext};
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
        let authorization_dependencies =
            Self::authorization_dependencies(&request_context.principal, &authorization_plan);

        match options.consistency {
            CommandConsistency::Eventual => {}
            CommandConsistency::ReadYourWrites {
                after,
                timeout,
                poll_interval,
            } => {
                if !authorization_dependencies.is_empty() {
                    let authorization_dependencies =
                        ProjectorDependencies::Some(authorization_dependencies.as_slice());
                    self.read_your_writes_waiter
                        .wait(after, timeout, poll_interval, authorization_dependencies)
                        .await?;
                }
            }
        }
        self.authorizer
            .authorize(&request_context.principal, &authorization_plan)
            .await?;

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

        let handler_result = handler.handle(&mut uow, request_context, command).await;

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

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::DefaultCommandDispatcher;
    use crate::authorization::{
        AggregateRef, AuthorizationPlan, Authorizer, AuthorizerError, PrincipalRequirement,
        RelationName, RelationshipRequirement,
    };
    use crate::command::{
        Command, CommandFailureReport, CommandHash, CommandHasher, CommandHasherError, CommandName,
        IdempotencyBeginResult, IdempotencyOutput, IdempotencyService, IdempotencyServiceError,
    };
    use crate::event::{AggregateIdValue, AggregateTypeOwned};
    use crate::messaging::Subscription;
    use crate::projection::{
        ProjectorDependencies, ProjectorDescriptor, ProjectorName, ReadYourWritesPollInterval,
        ReadYourWritesTimeout, ReadYourWritesWaitError, ReadYourWritesWaiter,
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
            _after: MessageId,
            _timeout: ReadYourWritesTimeout,
            _poll_interval: ReadYourWritesPollInterval,
            _projector_dependencies: ProjectorDependencies<'_>,
        ) -> Result<(), ReadYourWritesWaitError> {
            Ok(())
        }
    }

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

    type TestDispatcher = DefaultCommandDispatcher<
        TestCommandHasher,
        TestIdempotencyService,
        TestWaiter,
        TestUowFactory,
        TestAuthorizer,
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
            relation: RelationName::new("viewer"),
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
}

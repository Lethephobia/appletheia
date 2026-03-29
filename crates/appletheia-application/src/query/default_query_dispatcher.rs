use crate::authorization::{AuthorizationPlan, Authorizer, PrincipalRequirement};
use crate::projection::{ProjectorDependencies, ProjectorDescriptor, ReadYourWritesWaiter};
use crate::request_context::{Principal, RequestContext};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{QueryConsistency, QueryDispatcher, QueryDispatcherError, QueryHandler, QueryOptions};

pub struct DefaultQueryDispatcher<W, U, AZ>
where
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
{
    read_your_writes_waiter: W,
    uow_factory: U,
    authorizer: AZ,
}

impl<W, U, AZ> DefaultQueryDispatcher<W, U, AZ>
where
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
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

impl<W, U, AZ> DefaultQueryDispatcher<W, U, AZ>
where
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
{
    pub fn new(read_your_writes_waiter: W, uow_factory: U, authorizer: AZ) -> Self {
        Self {
            read_your_writes_waiter,
            uow_factory,
            authorizer,
        }
    }
}

impl<W, U, AZ> QueryDispatcher for DefaultQueryDispatcher<W, U, AZ>
where
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
{
    type Uow = U::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        query: H::Query,
        options: QueryOptions,
    ) -> Result<H::Output, QueryDispatcherError<H::Error>>
    where
        H: QueryHandler<Uow = Self::Uow>,
    {
        let authorization_plan = handler
            .authorization_plan(&query)
            .map_err(QueryDispatcherError::Handler)?;
        let authorization_dependencies =
            Self::authorization_dependencies(&request_context.principal, &authorization_plan);
        match options.consistency {
            QueryConsistency::Eventual => {}
            QueryConsistency::ReadYourWrites {
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

        match options.consistency {
            QueryConsistency::Eventual => {}
            QueryConsistency::ReadYourWrites {
                after,
                timeout,
                poll_interval,
            } => {
                self.read_your_writes_waiter
                    .wait(after, timeout, poll_interval, H::DEPENDENCIES)
                    .await?;
            }
        }

        let mut uow = self.uow_factory.begin().await?;

        let result = handler.handle(&mut uow, request_context, query).await;
        match result {
            Ok(output) => {
                uow.commit().await?;
                Ok(output)
            }
            Err(operation_error) => {
                let operation_error = uow
                    .rollback_with_operation_error(operation_error)
                    .await
                    .map_err(QueryDispatcherError::UnitOfWork)?;
                Err(QueryDispatcherError::Handler(operation_error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::DefaultQueryDispatcher;
    use crate::authorization::{
        AggregateRef, AuthorizationPlan, Authorizer, AuthorizerError, PrincipalRequirement,
        RelationName, RelationshipRequirement,
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

    type TestDispatcher = DefaultQueryDispatcher<TestWaiter, TestUowFactory, TestAuthorizer>;

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

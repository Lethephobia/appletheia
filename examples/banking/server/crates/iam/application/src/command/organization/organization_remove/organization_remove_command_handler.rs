use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use super::{
    OrganizationRemoveCommand, OrganizationRemoveCommandHandlerError, OrganizationRemoveOutput,
};
use crate::authorization::OrganizationRemoverRelation;

pub struct OrganizationRemoveCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationRemoveCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationRemoveCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationRemoveCommand;
    type Output = OrganizationRemoveOutput;
    type Error = OrganizationRemoveCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                command.organization_id,
                OrganizationRemoverRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut organization = self
            .repository
            .read::<Organization>(uow, command.organization_id)
            .await?;

        organization.remove()?;

        self.repository
            .save(uow, request_context, &mut organization)
            .await?;

        Ok(OrganizationRemoveOutput {})
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::aggregate::SerializedAggregate;
    use appletheia::domain::{AggregateId, Event};
    use std::sync::{Arc, Mutex};

    use appletheia::application::aggregate::AggregateRef;
    use appletheia::application::authorization::{
        AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;

    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_domain::{
        Organization, OrganizationHandle, OrganizationId, OrganizationName, OrganizationOwner,
        UserId,
    };
    use uuid::Uuid;

    use super::{
        OrganizationRemoveCommand, OrganizationRemoveCommandHandler, OrganizationRemoveOutput,
    };
    use crate::authorization::OrganizationRemoverRelation;

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

    #[derive(Clone, Default)]
    struct TestRepository {
        organization: Arc<Mutex<Option<Organization>>>,
    }

    impl TestRepository {
        fn copy_aggregate<S: Aggregate, T: Aggregate>(source: &S) -> T {
            let mut restored = SerializedAggregate::try_from_aggregate(source)
                .expect("test aggregate should serialize")
                .try_to_aggregate::<T>()
                .expect("test aggregate types should match");
            for event in source.uncommitted_events() {
                let id = T::Id::try_from_uuid(event.aggregate_id().value()).unwrap();
                let payload =
                    serde_json::from_value(serde_json::to_value(event.payload()).unwrap()).unwrap();
                restored
                    .core_mut()
                    .record_uncommitted_event(Event::from_persisted(
                        event.id(),
                        id,
                        event.aggregate_version(),
                        payload,
                        event.occurred_at(),
                    ));
            }
            restored
        }

        fn new(organization: Organization) -> Self {
            Self {
                organization: Arc::new(Mutex::new(Some(organization))),
            }
        }
    }

    impl Repository for TestRepository {
        type Uow = TestUow;

        async fn read<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            id: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            self.organization
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored))
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: A::TYPE,
                    aggregate_id: id,
                })
        }

        async fn read_at_version<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            id: A::Id,
            _at: appletheia::domain::AggregateVersion,
        ) -> Result<A, RepositoryError<A>> {
            self.organization
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored))
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: A::TYPE,
                    aggregate_id: id,
                })
        }

        async fn find_by_unique_value<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            Ok(None)
        }

        async fn save<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut A,
        ) -> Result<(), RepositoryError<A>> {
            *self.organization.lock().expect("lock") =
                Some(Self::copy_aggregate::<_, Organization>(aggregate));
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        let subject = AggregateRef::new(
            appletheia::application::aggregate::AggregateTypeOwned::try_from("user")
                .expect("aggregate type should be valid"),
            appletheia::application::aggregate::AggregateIdValue::from(Uuid::now_v7()),
        );

        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    fn organization() -> Organization {
        let mut organization = Organization::new();
        organization
            .create(
                OrganizationOwner::User(UserId::new()),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            )
            .expect("organization should create");
        organization
    }

    #[test]
    fn authorization_plan_requires_organization_remover_relationship() {
        let repository = TestRepository::default();
        let handler = OrganizationRemoveCommandHandler::new(repository);
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&OrganizationRemoveCommand { organization_id })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationRemoverRelation::REF
                    )
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_removes_organization_and_returns_output() {
        let organization = organization();
        let organization_id = organization.aggregate_id();
        let repository = TestRepository::new(organization);
        let handler = OrganizationRemoveCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let output = handler
            .handle(
                &mut uow,
                &request_context(),
                &OrganizationRemoveCommand { organization_id },
            )
            .await
            .expect("command should succeed");

        let saved = repository
            .organization
            .lock()
            .expect("lock")
            .clone()
            .expect("organization should be saved");

        assert_eq!(output, OrganizationRemoveOutput {});
        assert!(saved.is_removed().expect("status should exist"));
    }
}

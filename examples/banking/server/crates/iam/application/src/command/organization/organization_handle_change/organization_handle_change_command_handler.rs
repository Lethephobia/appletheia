use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use super::{
    OrganizationHandleChangeCommand, OrganizationHandleChangeCommandHandlerError,
    OrganizationHandleChangeOutput,
};
use crate::authorization::OrganizationHandleChangerRelation;
use crate::projection::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};

/// Handles `OrganizationHandleChangeCommand`.
pub struct OrganizationHandleChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationHandleChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationHandleChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationHandleChangeCommand;
    type Output = OrganizationHandleChangeOutput;
    type ReplayOutput = OrganizationHandleChangeOutput;
    type Error = OrganizationHandleChangeCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Organization>(
                    command.organization_id,
                    OrganizationHandleChangerRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationRoleRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut organization) = self
            .organization_repository
            .find(uow, command.organization_id)
            .await?
        else {
            return Err(OrganizationHandleChangeCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            return Err(OrganizationHandleChangeCommandHandlerError::OrganizationRemoved);
        }

        organization.change_handle(command.handle.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        Ok(CommandHandled::same(OrganizationHandleChangeOutput))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
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
        OrganizationHandleChangeCommand, OrganizationHandleChangeCommandHandler,
        OrganizationHandleChangeOutput,
    };
    use crate::authorization::OrganizationHandleChangerRelation;
    use crate::projection::{
        OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
    };

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
    struct TestOrganizationRepository {
        organization: Arc<Mutex<Option<Organization>>>,
    }

    impl TestOrganizationRepository {
        fn new(organization: Organization) -> Self {
            Self {
                organization: Arc::new(Mutex::new(Some(organization))),
            }
        }
    }

    impl Repository<Organization> for TestOrganizationRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
        ) -> Result<Option<Organization>, RepositoryError<Organization>> {
            Ok(self.organization.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<Organization>, RepositoryError<Organization>> {
            Ok(self.organization.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<Organization>, RepositoryError<Organization>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Organization,
        ) -> Result<(), RepositoryError<Organization>> {
            *self.organization.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        let subject = AggregateRef::new(
            appletheia::application::event::AggregateTypeOwned::try_from("user")
                .expect("aggregate type should be valid"),
            appletheia::application::event::AggregateIdValue::from(Uuid::now_v7()),
        );

        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    fn organization() -> Organization {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationOwner::User(UserId::new()),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
                None,
                None,
                None,
            )
            .expect("organization should create");
        organization
    }

    #[test]
    fn authorization_plan_requires_organization_handle_changer_relationship() {
        let repository = TestOrganizationRepository::default();
        let handler = OrganizationHandleChangeCommandHandler::new(repository);
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&OrganizationHandleChangeCommand {
                organization_id,
                handle: OrganizationHandle::try_from("acme-labs-2")
                    .expect("handle should be valid"),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationHandleChangerRelation::REF
                    ),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                        OrganizationRoleRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_changes_organization_handle_and_returns_output() {
        let organization = organization();
        let organization_id = organization
            .aggregate_id()
            .expect("organization id should exist");
        let repository = TestOrganizationRepository::new(organization);
        let handler = OrganizationHandleChangeCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &OrganizationHandleChangeCommand {
                    organization_id,
                    handle: OrganizationHandle::try_from("acme-labs-2")
                        .expect("handle should be valid"),
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let saved = repository.organization.lock().expect("lock").clone();
        let saved = saved.expect("organization should be saved");

        assert_eq!(output, OrganizationHandleChangeOutput);
        assert_eq!(
            saved.handle().expect("handle should exist"),
            &OrganizationHandle::try_from("acme-labs-2").expect("handle should be valid")
        );
    }
}

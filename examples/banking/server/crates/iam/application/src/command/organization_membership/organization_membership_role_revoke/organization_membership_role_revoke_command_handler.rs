use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationMembership};

use super::{
    OrganizationMembershipRoleRevokeCommand, OrganizationMembershipRoleRevokeCommandHandlerError,
    OrganizationMembershipRoleRevokeOutput,
};
use crate::authorization::OrganizationMembershipRoleRevokerRelation;
use crate::projection::{
    OrganizationMembershipOrganizationRelationshipProjectorSpec,
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};

/// Handles `OrganizationMembershipRoleRevokeCommand`.
pub struct OrganizationMembershipRoleRevokeCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_membership_repository: MR,
}

impl<ORG, MR> OrganizationMembershipRoleRevokeCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    pub fn new(organization_repository: ORG, organization_membership_repository: MR) -> Self {
        Self {
            organization_repository,
            organization_membership_repository,
        }
    }
}

impl<ORG, MR> CommandHandler for OrganizationMembershipRoleRevokeCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    type Command = OrganizationMembershipRoleRevokeCommand;
    type Output = OrganizationMembershipRoleRevokeOutput;
    type ReplayOutput = OrganizationMembershipRoleRevokeOutput;
    type Error = OrganizationMembershipRoleRevokeCommandHandlerError;
    type Uow = ORG::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<OrganizationMembership>(
                    command.organization_membership_id,
                    OrganizationMembershipRoleRevokerRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationMembershipOrganizationRelationshipProjectorSpec::DESCRIPTOR,
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
        let Some(mut organization_membership) = self
            .organization_membership_repository
            .find(uow, command.organization_membership_id)
            .await?
        else {
            return Err(
                OrganizationMembershipRoleRevokeCommandHandlerError::TargetOrganizationMembershipNotFound,
            );
        };

        let Some(organization) = self
            .organization_repository
            .find(uow, *organization_membership.organization_id()?)
            .await?
        else {
            return Err(OrganizationMembershipRoleRevokeCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            return Err(OrganizationMembershipRoleRevokeCommandHandlerError::OrganizationRemoved);
        }

        organization_membership.revoke_role(command.role)?;

        self.organization_membership_repository
            .save(uow, request_context, &mut organization_membership)
            .await?;

        Ok(CommandHandled::same(OrganizationMembershipRoleRevokeOutput))
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
        Organization, OrganizationHandle, OrganizationId, OrganizationMembership,
        OrganizationMembershipId, OrganizationMembershipStatus, OrganizationName,
        OrganizationOwner, OrganizationRole, UserId,
    };
    use uuid::Uuid;

    use super::{
        OrganizationMembershipRoleRevokeCommand, OrganizationMembershipRoleRevokeCommandHandler,
        OrganizationMembershipRoleRevokeOutput,
    };
    use crate::authorization::OrganizationMembershipRoleRevokerRelation;
    use crate::projection::{
        OrganizationMembershipOrganizationRelationshipProjectorSpec,
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

    #[derive(Clone, Default)]
    struct TestOrganizationMembershipRepository {
        organization_membership: Arc<Mutex<Option<OrganizationMembership>>>,
    }

    impl TestOrganizationMembershipRepository {
        fn new(organization_membership: OrganizationMembership) -> Self {
            Self {
                organization_membership: Arc::new(Mutex::new(Some(organization_membership))),
            }
        }
    }

    impl Repository<OrganizationMembership> for TestOrganizationMembershipRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationMembershipId,
        ) -> Result<Option<OrganizationMembership>, RepositoryError<OrganizationMembership>>
        {
            Ok(self.organization_membership.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationMembershipId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<OrganizationMembership>, RepositoryError<OrganizationMembership>>
        {
            Ok(self.organization_membership.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<OrganizationMembership>, RepositoryError<OrganizationMembership>>
        {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut OrganizationMembership,
        ) -> Result<(), RepositoryError<OrganizationMembership>> {
            *self.organization_membership.lock().expect("lock") = Some(aggregate.clone());
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
            )
            .expect("organization should create");
        organization
    }

    fn organization_membership(organization_id: OrganizationId) -> OrganizationMembership {
        let mut organization_membership = OrganizationMembership::default();
        organization_membership
            .create(organization_id, UserId::new())
            .expect("organization membership should create");
        organization_membership
            .grant_role(OrganizationRole::FinanceManager)
            .expect("organization membership should grant role");
        organization_membership
    }

    #[test]
    fn authorization_plan_requires_role_revoker_relationship() {
        let organization_repository = TestOrganizationRepository::default();
        let organization_membership_repository = TestOrganizationMembershipRepository::default();
        let handler = OrganizationMembershipRoleRevokeCommandHandler::new(
            organization_repository,
            organization_membership_repository,
        );
        let organization_membership_id = OrganizationMembershipId::new();

        let plan = handler
            .authorization_plan(&OrganizationMembershipRoleRevokeCommand {
                organization_membership_id,
                role: OrganizationRole::FinanceManager,
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::System,
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::check::<OrganizationMembership>(
                        organization_membership_id,
                        OrganizationMembershipRoleRevokerRelation::REF,
                    ),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        OrganizationMembershipOrganizationRelationshipProjectorSpec::DESCRIPTOR,
                        OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                        OrganizationRoleRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_revokes_role_and_returns_output() {
        let organization = organization();
        let organization_id = organization
            .aggregate_id()
            .expect("organization id should exist");
        let organization_repository = TestOrganizationRepository::new(organization);
        let organization_membership = organization_membership(organization_id);
        let organization_membership_id = organization_membership
            .aggregate_id()
            .expect("organization membership id should exist");
        let organization_membership_repository =
            TestOrganizationMembershipRepository::new(organization_membership);
        let handler = OrganizationMembershipRoleRevokeCommandHandler::new(
            organization_repository,
            organization_membership_repository.clone(),
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &OrganizationMembershipRoleRevokeCommand {
                    organization_membership_id,
                    role: OrganizationRole::FinanceManager,
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let saved = organization_membership_repository
            .organization_membership
            .lock()
            .expect("lock")
            .clone()
            .expect("organization membership should be saved");

        assert_eq!(output, OrganizationMembershipRoleRevokeOutput);
        assert_eq!(
            saved.status().expect("status should exist"),
            OrganizationMembershipStatus::Active
        );
        assert!(
            !saved
                .roles()
                .expect("roles should exist")
                .contains(OrganizationRole::FinanceManager)
        );
    }
}

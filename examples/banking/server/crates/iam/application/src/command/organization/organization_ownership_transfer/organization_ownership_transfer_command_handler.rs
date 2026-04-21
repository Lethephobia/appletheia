use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use super::{
    OrganizationOwnershipTransferCommand, OrganizationOwnershipTransferCommandHandlerError,
    OrganizationOwnershipTransferOutput,
};
use crate::authorization::OrganizationOwnershipTransfererRelation;
use crate::projection::OrganizationOwnerRelationshipProjectorSpec;

/// Handles `OrganizationOwnershipTransferCommand`.
pub struct OrganizationOwnershipTransferCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationOwnershipTransferCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationOwnershipTransferCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationOwnershipTransferCommand;
    type Output = OrganizationOwnershipTransferOutput;
    type ReplayOutput = OrganizationOwnershipTransferOutput;
    type Error = OrganizationOwnershipTransferCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Organization>(
                    command.organization_id,
                    OrganizationOwnershipTransfererRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
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
            return Err(OrganizationOwnershipTransferCommandHandlerError::OrganizationNotFound);
        };

        organization.transfer_ownership(command.owner)?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        Ok(CommandHandled::same(OrganizationOwnershipTransferOutput))
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
        OrganizationProfile, User, UserId,
    };
    use uuid::Uuid;

    use super::{
        OrganizationOwnershipTransferCommand, OrganizationOwnershipTransferCommandHandler,
        OrganizationOwnershipTransferOutput,
    };
    use crate::authorization::OrganizationOwnershipTransfererRelation;
    use crate::projection::OrganizationOwnerRelationshipProjectorSpec;

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
        let subject = AggregateRef::from_id::<User>(UserId::new());

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
                OrganizationProfile::new(
                    OrganizationName::try_from("Acme Labs").expect("name should be valid"),
                    None,
                    None,
                    None,
                ),
            )
            .expect("organization should create");
        organization
    }

    #[test]
    fn authorization_plan_requires_ownership_transferer_relationship() {
        let repository = TestOrganizationRepository::default();
        let handler = OrganizationOwnershipTransferCommandHandler::new(repository);
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&OrganizationOwnershipTransferCommand {
                organization_id,
                owner: OrganizationOwner::User(UserId::new()),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationOwnershipTransfererRelation::REF,
                    ),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_transfers_ownership() {
        let organization = organization();
        let organization_id = organization.aggregate_id().expect("id should exist");
        let repository = TestOrganizationRepository::new(organization);
        let handler = OrganizationOwnershipTransferCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let owner = OrganizationOwner::User(UserId::new());

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &OrganizationOwnershipTransferCommand {
                    organization_id,
                    owner,
                },
            )
            .await
            .expect("command should succeed");

        assert_eq!(handled.into_output(), OrganizationOwnershipTransferOutput);
        assert_eq!(
            repository
                .organization
                .lock()
                .expect("lock")
                .as_ref()
                .expect("organization should exist")
                .owner()
                .expect("owner should exist"),
            owner
        );
    }
}

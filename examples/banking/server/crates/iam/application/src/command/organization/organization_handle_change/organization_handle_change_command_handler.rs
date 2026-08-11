use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_iam_domain::{
    Organization, OrganizationHandle, OrganizationHandleChangeRejectionReason,
    OrganizationHandleChangeResult, OrganizationState,
};

use super::{
    OrganizationHandleChangeCommand, OrganizationHandleChangeCommandHandlerError,
    OrganizationHandleChangeOutput,
};
use crate::authorization::OrganizationHandleChangerRelation;

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

    fn handle_unique_value(
        handle: &OrganizationHandle,
    ) -> Result<UniqueValue, OrganizationHandleChangeCommandHandlerError> {
        Ok(UniqueValue::from_strings([handle.as_ref()])?)
    }
}

impl<OR> CommandHandler for OrganizationHandleChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationHandleChangeCommand;
    type Output = OrganizationHandleChangeOutput;
    type Error = OrganizationHandleChangeCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                command.organization_id,
                OrganizationHandleChangerRelation::REF,
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
            .organization_repository
            .read(uow, command.organization_id)
            .await?;

        let unique_value = Self::handle_unique_value(&command.handle)?;
        if self
            .organization_repository
            .find_by_unique_value(uow, OrganizationState::HANDLE_KEY, &unique_value)
            .await?
            .is_some_and(|existing| existing.aggregate_id() != command.organization_id)
        {
            let reason = OrganizationHandleChangeRejectionReason::AlreadyTaken;
            organization.reject_change_handle(command.handle.clone(), reason)?;

            self.organization_repository
                .save(uow, request_context, &mut organization)
                .await?;

            return Ok(OrganizationHandleChangeOutput::Rejected { reason });
        }

        let result = organization.change_handle(command.handle.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        let output = match result {
            OrganizationHandleChangeResult::Changed => OrganizationHandleChangeOutput::Changed,
            OrganizationHandleChangeResult::Rejected { reason } => {
                OrganizationHandleChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;

    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_domain::{
        Organization, OrganizationCreation, OrganizationHandle, OrganizationId, OrganizationName,
        OrganizationOwner, UserId,
    };
    use uuid::Uuid;

    use super::{
        OrganizationHandleChangeCommand, OrganizationHandleChangeCommandHandler,
        OrganizationHandleChangeOutput,
    };
    use crate::authorization::OrganizationHandleChangerRelation;

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

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
        ) -> Result<Organization, RepositoryError<Organization>> {
            self.organization
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Organization::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
            _at: appletheia::domain::AggregateVersion,
        ) -> Result<Organization, RepositoryError<Organization>> {
            self.organization
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Organization::TYPE,
                    aggregate_id: _id,
                })
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
        let mut organization = Organization::new();
        organization
            .create(OrganizationCreation {
                owner: OrganizationOwner::User(UserId::new()),
                handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                display_name: OrganizationName::try_from("Acme Labs")
                    .expect("name should be valid"),
                description: None,
                website_url: None,
                picture: None,
            })
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
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationHandleChangerRelation::REF
                    )
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_changes_organization_handle_and_returns_output() {
        let organization = organization();
        let organization_id = organization.aggregate_id();
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

        let output = handled;
        let saved = repository.organization.lock().expect("lock").clone();
        let saved = saved.expect("organization should be saved");

        assert_eq!(output, OrganizationHandleChangeOutput::Changed);
        assert_eq!(
            saved.handle().expect("handle should exist"),
            &OrganizationHandle::try_from("acme-labs-2").expect("handle should be valid")
        );
    }
}

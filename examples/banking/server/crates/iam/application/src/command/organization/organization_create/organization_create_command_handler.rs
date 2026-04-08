use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::{Principal, RequestContext};
use appletheia::domain::{Aggregate, AggregateId};
use banking_iam_domain::{Organization, OrganizationOwner, User, UserId};

use super::{
    OrganizationCreateCommand, OrganizationCreateCommandHandlerError, OrganizationCreateOutput,
};

/// Handles `OrganizationCreateCommand`.
pub struct OrganizationCreateCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationCreateCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationCreateCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationCreateCommand;
    type Output = OrganizationCreateOutput;
    type ReplayOutput = OrganizationCreateOutput;
    type Error = OrganizationCreateCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::Authenticated,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let OrganizationCreateCommand { handle, name } = command.clone();
        let mut organization = Organization::default();
        organization.create(handle, name)?;
        if let Principal::Authenticated { subject } = &request_context.principal
            && subject.aggregate_type.value() == User::TYPE.value()
        {
            let owner = UserId::try_from_uuid(subject.aggregate_id.value())
                .map_err(OrganizationCreateCommandHandlerError::InvalidOwnerUserId)?;
            organization.assign_owner(OrganizationOwner::User(owner))?;
        }

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        let organization_id = organization
            .aggregate_id()
            .ok_or(OrganizationCreateCommandHandlerError::MissingOrganizationId)?;
        let output = OrganizationCreateOutput::new(organization_id);

        Ok(CommandHandled::same(output))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, EventPayload};
    use banking_iam_domain::{
        Organization, OrganizationHandle, OrganizationId, OrganizationName, User, UserId,
    };
    use uuid::Uuid;

    use super::{
        OrganizationCreateCommand, OrganizationCreateCommandHandler, OrganizationCreateOutput,
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

    fn request_context() -> (RequestContext, UserId) {
        let user_id = UserId::new();
        let subject = AggregateRef::from_id::<User>(user_id);

        (
            RequestContext::new(
                CorrelationId::from(Uuid::now_v7()),
                MessageId::new(),
                Principal::Authenticated { subject },
            )
            .expect("request context should be valid"),
            user_id,
        )
    }

    fn system_request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid")
    }

    #[test]
    fn authorization_plan_requires_authenticated_or_system_principal() {
        let repository = TestOrganizationRepository::default();
        let handler = OrganizationCreateCommandHandler::new(repository);

        let plan = handler
            .authorization_plan(&OrganizationCreateCommand {
                handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                name: OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::System,
                PrincipalRequirement::Authenticated,
            ])
        );
    }

    #[tokio::test]
    async fn handle_creates_organization_and_returns_id() {
        let repository = TestOrganizationRepository::default();
        let handler = OrganizationCreateCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let (request_context, _) = request_context();

        let handled = handler
            .handle(
                &mut uow,
                &request_context,
                &OrganizationCreateCommand {
                    handle: OrganizationHandle::try_from("acme-labs")
                        .expect("handle should be valid"),
                    name: OrganizationName::try_from("Acme Labs").expect("name should be valid"),
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let saved = repository.organization.lock().expect("lock").clone();
        let saved = saved.expect("organization should be saved");

        assert_eq!(
            output,
            OrganizationCreateOutput::new(saved.aggregate_id().expect("id"))
        );
        assert_eq!(
            saved.name().expect("name should exist"),
            &OrganizationName::try_from("Acme Labs").expect("name should be valid")
        );
        assert_eq!(
            saved.handle().expect("handle should exist"),
            &OrganizationHandle::try_from("acme-labs").expect("handle should be valid")
        );
        assert_eq!(saved.uncommitted_events().len(), 2);
        assert_eq!(
            saved.uncommitted_events()[1].payload().name(),
            banking_iam_domain::OrganizationEventPayload::OWNER_ASSIGNED
        );
    }

    #[tokio::test]
    async fn handle_allows_system_principal() {
        let repository = TestOrganizationRepository::default();
        let handler = OrganizationCreateCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &system_request_context(),
                &OrganizationCreateCommand {
                    handle: OrganizationHandle::try_from("acme-labs")
                        .expect("handle should be valid"),
                    name: OrganizationName::try_from("Acme Labs").expect("name should be valid"),
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let saved = repository.organization.lock().expect("lock").clone();
        let saved = saved.expect("organization should be saved");

        assert_eq!(
            output,
            OrganizationCreateOutput::new(saved.aggregate_id().expect("id"))
        );
        assert_eq!(saved.uncommitted_events().len(), 1);
    }
}

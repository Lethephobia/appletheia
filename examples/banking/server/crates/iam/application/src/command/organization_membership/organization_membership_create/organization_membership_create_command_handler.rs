use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart};
use banking_iam_domain::OrganizationMembershipError;
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationMembership, OrganizationMembershipState, User, UserId,
};

use super::{
    OrganizationMembershipCreateCommand, OrganizationMembershipCreateCommandHandlerError,
    OrganizationMembershipCreateOutput,
};
use crate::authorization::OrganizationMemberAdderRelation;

///
/// The handler reads `Organization` and `User` only to validate their current
/// status; the single aggregate it mutates is `OrganizationMembership`.
pub struct OrganizationMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub(crate) fn organization_user_unique_value(
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<UniqueValue, OrganizationMembershipCreateCommandHandlerError> {
        let organization_value = organization_id.value().to_string();
        let user_value = user_id.value().to_string();
        let organization_part = UniqueValuePart::try_from(organization_value.as_str())?;
        let user_part = UniqueValuePart::try_from(user_value.as_str())?;
        Ok(UniqueValue::new(vec![organization_part, user_part])?)
    }
}

impl<R> CommandHandler for OrganizationMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationMembershipCreateCommand;
    type Output = OrganizationMembershipCreateOutput;
    type Error = OrganizationMembershipCreateCommandHandlerError;
    type Uow = R::Uow;

    /// Accepts the invitation and join-request sagas as `System`, and
    /// organization administrators adding a member directly.
    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                command.organization_id,
                OrganizationMemberAdderRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut membership = OrganizationMembership::new();
        let organization_membership_id = membership.aggregate_id();

        let organization = self
            .repository
            .read::<Organization>(uow, command.organization_id)
            .await?;
        if organization.is_removed()? {
            return Err(OrganizationMembershipError::OrganizationRemoved.into());
        }

        let user = self.repository.read::<User>(uow, command.user_id).await?;
        if user.is_removed()? {
            return Err(OrganizationMembershipError::UserRemoved.into());
        }
        if !user.is_active()? {
            return Err(OrganizationMembershipError::UserInactive.into());
        }

        // The unique constraint on the membership state is the authoritative
        // guard against two effective memberships for the same pair; this
        // lookup only turns the common case into an explicit rejection.
        let unique_value =
            Self::organization_user_unique_value(command.organization_id, command.user_id)?;
        if self
            .repository
            .find_by_unique_value::<OrganizationMembership>(
                uow,
                OrganizationMembershipState::ORGANIZATION_USER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(OrganizationMembershipError::AlreadyMember.into());
        }

        membership.create(
            command.organization_id,
            command.user_id,
            command.roles.clone(),
        )?;

        self.repository
            .save(uow, request_context, &mut membership)
            .await?;

        Ok(OrganizationMembershipCreateOutput {
            organization_membership_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::RequestContext;
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};
    use banking_iam_domain::{Organization, OrganizationId, OrganizationRoles, UserId};

    use super::{OrganizationMembershipCreateCommand, OrganizationMembershipCreateCommandHandler};
    use crate::authorization::OrganizationMemberAdderRelation;

    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    /// Repositories are not exercised: `authorization_plan` reads only the command.
    struct TestRepository;

    impl Repository for TestRepository {
        type Uow = TestUow;

        async fn read<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _id: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            panic!("repository is not exercised by this test")
        }

        async fn read_at_version<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _id: A::Id,
            _at: AggregateVersion,
        ) -> Result<A, RepositoryError<A>> {
            panic!("repository is not exercised by this test")
        }

        async fn find_by_unique_value<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            panic!("repository is not exercised by this test")
        }

        async fn save<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            _aggregate: &mut A,
        ) -> Result<(), RepositoryError<A>> {
            panic!("repository is not exercised by this test")
        }
    }

    #[test]
    fn authorization_plan_accepts_system_and_organization_admins() {
        let handler: OrganizationMembershipCreateCommandHandler<TestRepository> =
            OrganizationMembershipCreateCommandHandler::new(TestRepository);
        let organization_id = OrganizationId::new();
        let command = OrganizationMembershipCreateCommand {
            organization_id,
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
        };

        let plan = handler
            .authorization_plan(&command)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::System,
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationMemberAdderRelation::REF,
                    )
                ),
            ])
        );
    }
}

use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationMembership,
    OrganizationMembershipCreateRejectionReason, OrganizationMembershipCreateResult,
    OrganizationMembershipCreation, OrganizationMembershipState, User, UserId,
};

use super::{
    OrganizationMembershipCreateCommand, OrganizationMembershipCreateCommandHandlerError,
    OrganizationMembershipCreateOutput,
};
use crate::authorization::OrganizationMemberAdderRelation;

/// Handles `OrganizationMembershipCreateCommand`.
///
/// The handler reads `Organization` and `User` only to validate their current
/// status; the single aggregate it mutates is `OrganizationMembership`.
pub struct OrganizationMembershipCreateCommandHandler<OR, MR, UR>
where
    OR: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
    UR: Repository<User, Uow = OR::Uow>,
{
    organization_repository: OR,
    organization_membership_repository: MR,
    user_repository: UR,
}

impl<OR, MR, UR> OrganizationMembershipCreateCommandHandler<OR, MR, UR>
where
    OR: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
    UR: Repository<User, Uow = OR::Uow>,
{
    pub fn new(
        organization_repository: OR,
        organization_membership_repository: MR,
        user_repository: UR,
    ) -> Self {
        Self {
            organization_repository,
            organization_membership_repository,
            user_repository,
        }
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

impl<OR, MR, UR> CommandHandler for OrganizationMembershipCreateCommandHandler<OR, MR, UR>
where
    OR: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
    UR: Repository<User, Uow = OR::Uow>,
{
    type Command = OrganizationMembershipCreateCommand;
    type Output = OrganizationMembershipCreateOutput;
    type Error = OrganizationMembershipCreateCommandHandlerError;
    type Uow = OR::Uow;

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
        let creation = OrganizationMembershipCreation {
            organization_id: command.organization_id,
            user_id: command.user_id,
            roles: command.roles.clone(),
        };

        let organization = self
            .organization_repository
            .read(uow, command.organization_id)
            .await?;
        if organization.is_removed()? {
            let reason = OrganizationMembershipCreateRejectionReason::OrganizationRemoved;
            membership.reject_create(creation, reason)?;

            self.organization_membership_repository
                .save(uow, request_context, &mut membership)
                .await?;

            return Ok(OrganizationMembershipCreateOutput::Rejected {
                organization_membership_id,
                reason,
            });
        }

        let user = self.user_repository.read(uow, command.user_id).await?;
        if user.is_removed()? {
            let reason = OrganizationMembershipCreateRejectionReason::UserRemoved;
            membership.reject_create(creation, reason)?;

            self.organization_membership_repository
                .save(uow, request_context, &mut membership)
                .await?;

            return Ok(OrganizationMembershipCreateOutput::Rejected {
                organization_membership_id,
                reason,
            });
        }
        if !user.is_active()? {
            let reason = OrganizationMembershipCreateRejectionReason::UserInactive;
            membership.reject_create(creation, reason)?;

            self.organization_membership_repository
                .save(uow, request_context, &mut membership)
                .await?;

            return Ok(OrganizationMembershipCreateOutput::Rejected {
                organization_membership_id,
                reason,
            });
        }

        // The unique constraint on the membership state is the authoritative
        // guard against two effective memberships for the same pair; this
        // lookup only turns the common case into an explicit rejection.
        let unique_value =
            Self::organization_user_unique_value(command.organization_id, command.user_id)?;
        if self
            .organization_membership_repository
            .find_by_unique_value(
                uow,
                OrganizationMembershipState::ORGANIZATION_USER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            let reason = OrganizationMembershipCreateRejectionReason::AlreadyMember;
            membership.reject_create(creation, reason)?;

            self.organization_membership_repository
                .save(uow, request_context, &mut membership)
                .await?;

            return Ok(OrganizationMembershipCreateOutput::Rejected {
                organization_membership_id,
                reason,
            });
        }

        let result = membership.create(creation)?;

        self.organization_membership_repository
            .save(uow, request_context, &mut membership)
            .await?;

        let output = match result {
            OrganizationMembershipCreateResult::Created => {
                OrganizationMembershipCreateOutput::Created {
                    organization_membership_id,
                }
            }
            OrganizationMembershipCreateResult::Rejected { reason } => {
                OrganizationMembershipCreateOutput::Rejected {
                    organization_membership_id,
                    reason,
                }
            }
        };

        Ok(output)
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

    impl<A> Repository<A> for TestRepository
    where
        A: Aggregate,
    {
        type Uow = TestUow;

        async fn read(&self, _uow: &mut Self::Uow, _id: A::Id) -> Result<A, RepositoryError<A>> {
            panic!("repository is not exercised by this test")
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: A::Id,
            _at: AggregateVersion,
        ) -> Result<A, RepositoryError<A>> {
            panic!("repository is not exercised by this test")
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            panic!("repository is not exercised by this test")
        }

        async fn save(
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
        let handler: OrganizationMembershipCreateCommandHandler<
            TestRepository,
            TestRepository,
            TestRepository,
        > = OrganizationMembershipCreateCommandHandler::new(
            TestRepository,
            TestRepository,
            TestRepository,
        );
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

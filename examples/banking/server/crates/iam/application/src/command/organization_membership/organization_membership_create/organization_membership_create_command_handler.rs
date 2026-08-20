use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
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

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
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

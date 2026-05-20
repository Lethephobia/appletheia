use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{
    Organization, OrganizationMembership, OrganizationMembershipCreateRejectionReason,
};

use super::{
    OrganizationMembershipCreateCommand, OrganizationMembershipCreateCommandHandlerError,
    OrganizationMembershipCreateOutput,
};

/// Handles `OrganizationMembershipCreateCommand`.
pub struct OrganizationMembershipCreateCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_membership_repository: MR,
}

impl<ORG, MR> OrganizationMembershipCreateCommandHandler<ORG, MR>
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

impl<ORG, MR> CommandHandler for OrganizationMembershipCreateCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    type Command = OrganizationMembershipCreateCommand;
    type Output = OrganizationMembershipCreateOutput;
    type ReplayOutput = OrganizationMembershipCreateOutput;
    type Error = OrganizationMembershipCreateCommandHandlerError;
    type Uow = ORG::Uow;

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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(organization) = self
            .organization_repository
            .find(uow, command.organization_id)
            .await?
        else {
            return Err(OrganizationMembershipCreateCommandHandlerError::OrganizationNotFound);
        };

        let OrganizationMembershipCreateCommand {
            organization_id,
            user_id,
            roles,
        } = command.clone();
        let mut organization_membership = OrganizationMembership::default();
        let result = if organization.is_removed()? {
            let reason = OrganizationMembershipCreateRejectionReason::OrganizationRemoved;
            let organization_membership_id =
                organization_membership.reject_create(organization_id, user_id, roles, reason)?;
            banking_iam_domain::OrganizationMembershipCreateResult::Rejected {
                organization_membership_id,
                reason,
            }
        } else {
            organization_membership.create(organization_id, user_id, roles)?
        };

        self.organization_membership_repository
            .save(uow, request_context, &mut organization_membership)
            .await?;

        let output = match result {
            banking_iam_domain::OrganizationMembershipCreateResult::Created {
                organization_membership_id,
            } => OrganizationMembershipCreateOutput::Created {
                organization_membership_id,
            },
            banking_iam_domain::OrganizationMembershipCreateResult::Rejected {
                organization_membership_id,
                reason,
            } => OrganizationMembershipCreateOutput::Rejected {
                organization_membership_id,
                reason,
            },
        };

        Ok(CommandHandled::same(output))
    }
}

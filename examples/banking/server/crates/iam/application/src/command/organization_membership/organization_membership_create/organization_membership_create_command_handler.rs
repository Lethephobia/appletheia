use appletheia::application::authorization::AuthorizationPlan;
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_domain::OrganizationMembership;

use super::{
    OrganizationMembershipCreateCommand, OrganizationMembershipCreateCommandHandlerError,
    OrganizationMembershipCreateOutput,
};

/// Handles `OrganizationMembershipCreateCommand`.
pub struct OrganizationMembershipCreateCommandHandler<OR>
where
    OR: Repository<OrganizationMembership>,
{
    organization_membership_repository: OR,
}

impl<OR> OrganizationMembershipCreateCommandHandler<OR>
where
    OR: Repository<OrganizationMembership>,
{
    pub fn new(organization_membership_repository: OR) -> Self {
        Self {
            organization_membership_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationMembershipCreateCommandHandler<OR>
where
    OR: Repository<OrganizationMembership>,
{
    type Command = OrganizationMembershipCreateCommand;
    type Output = OrganizationMembershipCreateOutput;
    type ReplayOutput = OrganizationMembershipCreateOutput;
    type Error = OrganizationMembershipCreateCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            appletheia::application::authorization::PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let OrganizationMembershipCreateCommand {
            organization_id,
            user_id,
        } = command.clone();
        let mut organization_membership = OrganizationMembership::default();
        organization_membership.create(organization_id, user_id)?;

        self.organization_membership_repository
            .save(uow, request_context, &mut organization_membership)
            .await?;

        let organization_membership_id = organization_membership.aggregate_id().ok_or(
            OrganizationMembershipCreateCommandHandlerError::MissingOrganizationMembershipId,
        )?;
        let output = OrganizationMembershipCreateOutput::new(organization_membership_id);

        Ok(CommandHandled::same(output))
    }
}

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{
    Organization, OrganizationMembershipGrant, OrganizationMembershipGrantRejectionReason,
    OrganizationMembershipGrantResult, User,
};

use super::{
    UserOrganizationMembershipGrantCommand, UserOrganizationMembershipGrantCommandHandlerError,
    UserOrganizationMembershipGrantOutput,
};

/// Handles `UserOrganizationMembershipGrantCommand`.
pub struct UserOrganizationMembershipGrantCommandHandler<ORG, UR>
where
    ORG: Repository<Organization>,
    UR: Repository<User, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    user_repository: UR,
}

impl<ORG, UR> UserOrganizationMembershipGrantCommandHandler<ORG, UR>
where
    ORG: Repository<Organization>,
    UR: Repository<User, Uow = ORG::Uow>,
{
    pub fn new(organization_repository: ORG, user_repository: UR) -> Self {
        Self {
            organization_repository,
            user_repository,
        }
    }
}

impl<ORG, UR> CommandHandler for UserOrganizationMembershipGrantCommandHandler<ORG, UR>
where
    ORG: Repository<Organization>,
    UR: Repository<User, Uow = ORG::Uow>,
{
    type Command = UserOrganizationMembershipGrantCommand;
    type Output = UserOrganizationMembershipGrantOutput;
    type ReplayOutput = UserOrganizationMembershipGrantOutput;
    type Error = UserOrganizationMembershipGrantCommandHandlerError;
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
            return Err(UserOrganizationMembershipGrantCommandHandlerError::OrganizationNotFound);
        };

        let Some(mut user) = self.user_repository.find(uow, command.user_id).await? else {
            return Err(UserOrganizationMembershipGrantCommandHandlerError::UserNotFound);
        };

        let grant = OrganizationMembershipGrant {
            organization_id: command.organization_id,
            roles: command.roles.clone(),
        };

        if organization.is_removed()? {
            let reason = OrganizationMembershipGrantRejectionReason::OrganizationRemoved;
            user.reject_grant_organization_membership(grant, reason)?;

            self.user_repository
                .save(uow, request_context, &mut user)
                .await?;

            return Ok(CommandHandled::same(
                UserOrganizationMembershipGrantOutput::Rejected { reason },
            ));
        }

        let result = user.grant_organization_membership(grant)?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        let output = match result {
            OrganizationMembershipGrantResult::Granted => {
                UserOrganizationMembershipGrantOutput::Granted
            }
            OrganizationMembershipGrantResult::Rejected { reason } => {
                UserOrganizationMembershipGrantOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_domain::OrganizationMembership;
use banking_iam_domain::team_membership_cleanup::TeamMembershipCleanup;

use super::{
    TeamMembershipCleanupStartCommand, TeamMembershipCleanupStartCommandHandlerError,
    TeamMembershipCleanupStartOutput,
};

pub struct TeamMembershipCleanupStartCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TeamMembershipCleanupStartCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TeamMembershipCleanupStartCommandHandler<R>
where
    R: Repository,
{
    type Command = TeamMembershipCleanupStartCommand;
    type Output = TeamMembershipCleanupStartOutput;
    type Error = TeamMembershipCleanupStartCommandHandlerError;
    type Uow = R::Uow;

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
        let membership = self
            .repository
            .read_shared::<OrganizationMembership>(uow, command.organization_membership_id)
            .await?;
        if !membership.is_removed()? {
            return Err(
                TeamMembershipCleanupStartCommandHandlerError::OrganizationMembershipNotRemoved,
            );
        }
        let mut team_membership_cleanup = TeamMembershipCleanup::new();
        let team_membership_cleanup_id = team_membership_cleanup.aggregate_id();
        team_membership_cleanup.start(command.organization_membership_id)?;

        self.repository
            .save(uow, request_context, &mut team_membership_cleanup)
            .await?;

        Ok(TeamMembershipCleanupStartOutput {
            team_membership_cleanup_id,
        })
    }
}

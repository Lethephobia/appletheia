use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::team_membership_cleanup::TeamMembershipCleanup;

use super::{
    TeamMembershipCleanupMembershipSucceededRecordCommand,
    TeamMembershipCleanupMembershipSucceededRecordCommandHandlerError,
    TeamMembershipCleanupMembershipSucceededRecordOutput,
};

pub struct TeamMembershipCleanupMembershipSucceededRecordCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TeamMembershipCleanupMembershipSucceededRecordCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TeamMembershipCleanupMembershipSucceededRecordCommandHandler<R>
where
    R: Repository,
{
    type Command = TeamMembershipCleanupMembershipSucceededRecordCommand;
    type Output = TeamMembershipCleanupMembershipSucceededRecordOutput;
    type Error = TeamMembershipCleanupMembershipSucceededRecordCommandHandlerError;
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
        let mut team_membership_cleanup = self
            .repository
            .read::<TeamMembershipCleanup>(uow, command.team_membership_cleanup_id)
            .await?;

        team_membership_cleanup.record_membership_succeeded(command.team_membership_id)?;
        if team_membership_cleanup.is_ready_to_complete()? {
            team_membership_cleanup.complete()?;
        }
        self.repository
            .save(uow, request_context, &mut team_membership_cleanup)
            .await?;

        Ok(TeamMembershipCleanupMembershipSucceededRecordOutput {})
    }
}

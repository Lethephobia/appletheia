use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;

use super::{TeamNameChangeCommand, TeamNameChangeCommandHandlerError, TeamNameChangeOutput};
use crate::team::authorization::TeamManagerRelation;
use banking_iam_domain::Team;

pub struct TeamNameChangeCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TeamNameChangeCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TeamNameChangeCommandHandler<R>
where
    R: Repository,
{
    type Command = TeamNameChangeCommand;
    type Output = TeamNameChangeOutput;
    type Error = TeamNameChangeCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Team,
            >(
                command.team_id,
                TeamManagerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut team = self.repository.read::<Team>(uow, command.team_id).await?;
        team.change_name(command.name.clone())?;
        self.repository
            .save(uow, request_context, &mut team)
            .await?;
        Ok(TeamNameChangeOutput {})
    }
}

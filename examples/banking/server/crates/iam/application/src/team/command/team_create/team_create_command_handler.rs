use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;

use super::{TeamCreateCommand, TeamCreateCommandHandlerError, TeamCreateOutput};
use crate::organization::authorization::OrganizationAdminRelation;
use banking_iam_domain::{Organization, Team};

pub struct TeamCreateCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TeamCreateCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TeamCreateCommandHandler<R>
where
    R: Repository,
{
    type Command = TeamCreateCommand;
    type Output = TeamCreateOutput;
    type Error = TeamCreateCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                command.organization_id,
                OrganizationAdminRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let organization = self
            .repository
            .read_shared::<Organization>(uow, command.organization_id)
            .await?;
        if !organization.is_active()? {
            return Err(TeamCreateCommandHandlerError::OrganizationRemoved);
        }
        let mut team = Team::new();
        team.create(command.organization_id, command.name.clone())?;
        if let Some(description) = &command.description {
            team.set_description(Some(description.clone()))?;
        }
        self.repository
            .save(uow, request_context, &mut team)
            .await?;
        Ok(TeamCreateOutput {
            team_id: team.aggregate_id(),
        })
    }
}

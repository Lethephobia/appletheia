use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;

use super::{
    TeamMembershipPromoteToManagerCommand, TeamMembershipPromoteToManagerCommandHandlerError,
    TeamMembershipPromoteToManagerOutput,
};
use crate::authorization::TeamMembershipManagerRelation;
use banking_iam_domain::TeamMembership;

pub struct TeamMembershipPromoteToManagerCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TeamMembershipPromoteToManagerCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TeamMembershipPromoteToManagerCommandHandler<R>
where
    R: Repository,
{
    type Command = TeamMembershipPromoteToManagerCommand;
    type Output = TeamMembershipPromoteToManagerOutput;
    type Error = TeamMembershipPromoteToManagerCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                TeamMembership,
            >(
                command.team_membership_id,
                TeamMembershipManagerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut membership = self
            .repository
            .read::<TeamMembership>(uow, command.team_membership_id)
            .await?;
        membership.promote_to_manager()?;
        self.repository
            .save(uow, request_context, &mut membership)
            .await?;
        Ok(TeamMembershipPromoteToManagerOutput {})
    }
}

use super::{
    TeamRoleAssignmentRevokeCommand, TeamRoleAssignmentRevokeCommandHandlerError,
    TeamRoleAssignmentRevokeOutput,
};
use crate::authorization::TeamRoleAssignmentManagerRelation;
use appletheia::application::{
    authorization::{AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement},
    command::CommandHandler,
    repository::Repository,
    request_context::RequestContext,
};
use banking_iam_domain::TeamRoleAssignment;

pub struct TeamRoleAssignmentRevokeCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TeamRoleAssignmentRevokeCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TeamRoleAssignmentRevokeCommandHandler<R>
where
    R: Repository,
{
    type Command = TeamRoleAssignmentRevokeCommand;
    type Output = TeamRoleAssignmentRevokeOutput;
    type Error = TeamRoleAssignmentRevokeCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                TeamRoleAssignment,
            >(
                command.team_role_assignment_id,
                TeamRoleAssignmentManagerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut assignment = self
            .repository
            .read::<TeamRoleAssignment>(uow, command.team_role_assignment_id)
            .await?;
        assignment.revoke()?;
        self.repository
            .save(uow, request_context, &mut assignment)
            .await?;
        Ok(TeamRoleAssignmentRevokeOutput {})
    }
}

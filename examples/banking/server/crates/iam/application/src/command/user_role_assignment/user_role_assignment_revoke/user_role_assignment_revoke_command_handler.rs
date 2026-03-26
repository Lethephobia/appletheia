use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_domain::{Role, RoleId, RoleName, UserRoleAssignment};

use super::{
    UserRoleAssignmentRevokeCommand, UserRoleAssignmentRevokeCommandHandlerError,
    UserRoleAssignmentRevokeOutput,
};
use crate::authorization::RoleAssigneeRelation;
use crate::projection::RoleAssigneeRelationshipProjectorSpec;

/// Handles `UserRoleAssignmentRevokeCommand`.
pub struct UserRoleAssignmentRevokeCommandHandler<URAR>
where
    URAR: Repository<UserRoleAssignment>,
{
    user_role_assignment_repository: URAR,
}

impl<URAR> UserRoleAssignmentRevokeCommandHandler<URAR>
where
    URAR: Repository<UserRoleAssignment>,
{
    pub fn new(user_role_assignment_repository: URAR) -> Self {
        Self {
            user_role_assignment_repository,
        }
    }

    fn admin_role_requirement()
    -> Result<PrincipalRequirement, UserRoleAssignmentRevokeCommandHandlerError> {
        let role_name = RoleName::try_from("admin")?;
        let role_id = RoleId::from_name(&role_name);
        let aggregate = AggregateRef::from_id::<Role>(role_id);
        let relation = RoleAssigneeRelation::NAME;

        Ok(PrincipalRequirement::AuthenticatedWithRelationship {
            requirement: RelationshipRequirement::Check {
                aggregate,
                relation,
            },
            projector_dependencies: ProjectorDependencies::Some(&[
                RoleAssigneeRelationshipProjectorSpec::NAME,
            ]),
        })
    }
}

impl<URAR> CommandHandler for UserRoleAssignmentRevokeCommandHandler<URAR>
where
    URAR: Repository<UserRoleAssignment>,
{
    type Command = UserRoleAssignmentRevokeCommand;
    type Output = UserRoleAssignmentRevokeOutput;
    type ReplayOutput = UserRoleAssignmentRevokeOutput;
    type Error = UserRoleAssignmentRevokeCommandHandlerError;
    type Uow = URAR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            Self::admin_role_requirement()?,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let UserRoleAssignmentRevokeCommand {
            user_role_assignment_id,
        } = command;
        let Some(mut assignment) = self
            .user_role_assignment_repository
            .find(uow, user_role_assignment_id)
            .await?
        else {
            return Err(UserRoleAssignmentRevokeCommandHandlerError::UserRoleAssignmentNotFound);
        };

        assignment.revoke()?;

        self.user_role_assignment_repository
            .save(uow, request_context, &mut assignment)
            .await?;

        let assignment_id = assignment
            .aggregate_id()
            .ok_or(UserRoleAssignmentRevokeCommandHandlerError::MissingUserRoleAssignmentId)?;
        let output = UserRoleAssignmentRevokeOutput::new(assignment_id);

        Ok(CommandHandled::same(output))
    }
}

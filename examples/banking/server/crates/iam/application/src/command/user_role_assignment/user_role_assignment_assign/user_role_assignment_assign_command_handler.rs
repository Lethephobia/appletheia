use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_domain::{Role, RoleId, RoleName, User, UserRoleAssignment};

use super::{
    UserRoleAssignmentAssignCommand, UserRoleAssignmentAssignCommandHandlerError,
    UserRoleAssignmentAssignOutput,
};
use crate::authorization::RoleAssigneeRelation;
use crate::projection::RoleAssigneeRelationshipProjectorSpec;

/// Handles `UserRoleAssignmentAssignCommand`.
pub struct UserRoleAssignmentAssignCommandHandler<RR, UR, URAR>
where
    RR: Repository<Role>,
    UR: Repository<User, Uow = RR::Uow>,
    URAR: Repository<UserRoleAssignment, Uow = RR::Uow>,
{
    role_repository: RR,
    user_repository: UR,
    user_role_assignment_repository: URAR,
}

impl<RR, UR, URAR> UserRoleAssignmentAssignCommandHandler<RR, UR, URAR>
where
    RR: Repository<Role>,
    UR: Repository<User, Uow = RR::Uow>,
    URAR: Repository<UserRoleAssignment, Uow = RR::Uow>,
{
    pub fn new(
        role_repository: RR,
        user_repository: UR,
        user_role_assignment_repository: URAR,
    ) -> Self {
        Self {
            role_repository,
            user_repository,
            user_role_assignment_repository,
        }
    }

    fn admin_role_requirement()
    -> Result<PrincipalRequirement, UserRoleAssignmentAssignCommandHandlerError> {
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

impl<RR, UR, URAR> CommandHandler for UserRoleAssignmentAssignCommandHandler<RR, UR, URAR>
where
    RR: Repository<Role>,
    UR: Repository<User, Uow = RR::Uow>,
    URAR: Repository<UserRoleAssignment, Uow = RR::Uow>,
{
    type Command = UserRoleAssignmentAssignCommand;
    type Output = UserRoleAssignmentAssignOutput;
    type ReplayOutput = UserRoleAssignmentAssignOutput;
    type Error = UserRoleAssignmentAssignCommandHandlerError;
    type Uow = RR::Uow;

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
        let UserRoleAssignmentAssignCommand { role_id, user_id } = command;

        if self.role_repository.find(uow, role_id).await?.is_none() {
            return Err(UserRoleAssignmentAssignCommandHandlerError::RoleNotFound);
        }

        if self.user_repository.find(uow, user_id).await?.is_none() {
            return Err(UserRoleAssignmentAssignCommandHandlerError::UserNotFound);
        }

        let mut assignment = UserRoleAssignment::default();
        assignment.assign(role_id, user_id)?;

        self.user_role_assignment_repository
            .save(uow, request_context, &mut assignment)
            .await?;

        let assignment_id = assignment
            .aggregate_id()
            .ok_or(UserRoleAssignmentAssignCommandHandlerError::MissingUserRoleAssignmentId)?;
        let output = UserRoleAssignmentAssignOutput::new(assignment_id);

        Ok(CommandHandled::same(output))
    }
}

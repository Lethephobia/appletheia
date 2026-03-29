use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{UserActivateCommand, UserActivateCommandHandlerError, UserActivateOutput};
use crate::authorization::UserActivatorRelation;
use crate::projection::{
    RoleAssigneeRelationshipProjectorSpec, UserStatusManagerRelationshipProjectorSpec,
};

/// Handles `UserActivateCommand`.
pub struct UserActivateCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserActivateCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserActivateCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserActivateCommand;
    type Output = UserActivateOutput;
    type ReplayOutput = UserActivateOutput;
    type Error = UserActivateCommandHandlerError;
    type Uow = UR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<User>(command.user_id),
                    relation: UserActivatorRelation::NAME,
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    RoleAssigneeRelationshipProjectorSpec::DESCRIPTOR,
                    UserStatusManagerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut user) = self.user_repository.find(uow, command.user_id).await? else {
            return Err(UserActivateCommandHandlerError::TargetUserNotFound);
        };

        user.activate()?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(CommandHandled::same(UserActivateOutput))
    }
}

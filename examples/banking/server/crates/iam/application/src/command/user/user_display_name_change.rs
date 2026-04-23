use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::command;
use banking_iam_domain::{User, UserDisplayName, UserError, UserId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::authorization::UserDisplayNameChangerRelation;
use crate::projection::UserOwnerRelationshipProjectorSpec;

/// Changes a user's display name.
#[command(name = "user_display_name_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDisplayNameChangeCommand {
    pub user_id: UserId,
    pub display_name: UserDisplayName,
}

/// Returned after a user display name change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDisplayNameChangeOutput;

/// Represents errors returned while changing a user display name.
#[derive(Debug, Error)]
pub enum UserDisplayNameChangeCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("user was not found")]
    UserNotFound,
}

/// Handles `UserDisplayNameChangeCommand`.
pub struct UserDisplayNameChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserDisplayNameChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserDisplayNameChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserDisplayNameChangeCommand;
    type Output = UserDisplayNameChangeOutput;
    type ReplayOutput = UserDisplayNameChangeOutput;
    type Error = UserDisplayNameChangeCommandHandlerError;
    type Uow = UR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<User>(
                    command.user_id,
                    UserDisplayNameChangerRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut user) = self.user_repository.find(uow, command.user_id).await? else {
            return Err(UserDisplayNameChangeCommandHandlerError::UserNotFound);
        };

        user.change_display_name(command.display_name.clone())?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(CommandHandled::same(UserDisplayNameChangeOutput))
    }
}

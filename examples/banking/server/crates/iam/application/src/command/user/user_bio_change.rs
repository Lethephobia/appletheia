use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::command;
use banking_iam_domain::{User, UserBio, UserError, UserId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::authorization::UserBioChangerRelation;
use crate::projection::UserOwnerRelationshipProjectorSpec;

/// Changes a user's bio.
#[command(name = "user_bio_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBioChangeCommand {
    pub user_id: UserId,
    pub bio: Option<UserBio>,
}

/// Returned after a user bio change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBioChangeOutput;

/// Represents errors returned while changing a user bio.
#[derive(Debug, Error)]
pub enum UserBioChangeCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("user was not found")]
    UserNotFound,
}

/// Handles `UserBioChangeCommand`.
pub struct UserBioChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserBioChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserBioChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserBioChangeCommand;
    type Output = UserBioChangeOutput;
    type ReplayOutput = UserBioChangeOutput;
    type Error = UserBioChangeCommandHandlerError;
    type Uow = UR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<User>(
                    command.user_id,
                    UserBioChangerRelation::REF,
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
            return Err(UserBioChangeCommandHandlerError::UserNotFound);
        };

        user.change_bio(command.bio.clone())?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(CommandHandled::same(UserBioChangeOutput))
    }
}

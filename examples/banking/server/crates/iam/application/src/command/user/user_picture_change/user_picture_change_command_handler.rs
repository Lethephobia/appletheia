use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{
    UserPictureChangeCommand, UserPictureChangeCommandHandlerError, UserPictureChangeOutput,
};
use crate::authorization::UserProfileEditorRelation;
use crate::projection::UserOwnerRelationshipProjectorSpec;

/// Handles `UserPictureChangeCommand`.
pub struct UserPictureChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserPictureChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserPictureChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserPictureChangeCommand;
    type Output = UserPictureChangeOutput;
    type ReplayOutput = UserPictureChangeOutput;
    type Error = UserPictureChangeCommandHandlerError;
    type Uow = UR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<User>(
                    command.user_id,
                    UserProfileEditorRelation::REF,
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
            return Err(UserPictureChangeCommandHandlerError::UserNotFound);
        };

        user.change_picture(command.picture.clone())?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(CommandHandled::same(UserPictureChangeOutput))
    }
}

use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;
use banking_iam_domain::user::UserPictureChangeResult;

use super::{
    UserPictureChangeCommand, UserPictureChangeCommandHandlerError, UserPictureChangeOutput,
};
use crate::authorization::UserProfileEditorRelation;

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
    type Error = UserPictureChangeCommandHandlerError;
    type Uow = UR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                User,
            >(
                command.user_id,
                UserProfileEditorRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut user = self.user_repository.read(uow, command.user_id).await?;

        let result = user.change_picture(command.picture.clone())?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        let output = match result {
            UserPictureChangeResult::Changed => UserPictureChangeOutput::Changed,
            UserPictureChangeResult::Rejected { reason } => {
                UserPictureChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}

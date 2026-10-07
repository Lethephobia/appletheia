use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{UserBioSetCommand, UserBioSetCommandHandlerError, UserBioSetOutput};
use crate::authorization::UserProfileEditorRelation;

pub struct UserBioSetCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> UserBioSetCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for UserBioSetCommandHandler<R>
where
    R: Repository,
{
    type Command = UserBioSetCommand;
    type Output = UserBioSetOutput;
    type Error = UserBioSetCommandHandlerError;
    type Uow = R::Uow;

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
        let mut user = self.repository.read::<User>(uow, command.user_id).await?;

        user.set_bio(command.bio.clone())?;

        self.repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(UserBioSetOutput {})
    }
}

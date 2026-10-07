use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{UserDeactivateCommand, UserDeactivateCommandHandlerError, UserDeactivateOutput};
use crate::authorization::UserDeactivatorRelation;

pub struct UserDeactivateCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> UserDeactivateCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for UserDeactivateCommandHandler<R>
where
    R: Repository,
{
    type Command = UserDeactivateCommand;
    type Output = UserDeactivateOutput;
    type Error = UserDeactivateCommandHandlerError;
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
                UserDeactivatorRelation::REF,
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

        user.deactivate()?;

        self.repository
            .save::<User>(uow, request_context, &mut user)
            .await?;

        Ok(UserDeactivateOutput {})
    }
}

use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{
    UserSetDisplayNameCommand, UserSetDisplayNameCommandHandlerError, UserSetDisplayNameOutput,
};
use crate::authorization::UserOwnerRelation;

pub struct UserSetDisplayNameCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> UserSetDisplayNameCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for UserSetDisplayNameCommandHandler<R>
where
    R: Repository,
{
    type Command = UserSetDisplayNameCommand;
    type Output = UserSetDisplayNameOutput;
    type Error = UserSetDisplayNameCommandHandlerError;
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
                UserOwnerRelation::REF,
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

        user.set_display_name(command.display_name.clone())?;

        self.repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(UserSetDisplayNameOutput {})
    }
}

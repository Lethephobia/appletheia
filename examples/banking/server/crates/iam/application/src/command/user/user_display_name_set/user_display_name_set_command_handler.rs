use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{
    UserDisplayNameSetCommand, UserDisplayNameSetCommandHandlerError, UserDisplayNameSetOutput,
};
use crate::authorization::UserProfileEditorRelation;

pub struct UserDisplayNameSetCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserDisplayNameSetCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserDisplayNameSetCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserDisplayNameSetCommand;
    type Output = UserDisplayNameSetOutput;
    type Error = UserDisplayNameSetCommandHandlerError;
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

        user.set_display_name(command.display_name.clone())?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(UserDisplayNameSetOutput {})
    }
}

use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;
use banking_iam_domain::user::UserDeactivateResult;

use super::{UserDeactivateCommand, UserDeactivateCommandHandlerError, UserDeactivateOutput};
use crate::authorization::UserDeactivatorRelation;

/// Handles `UserDeactivateCommand`.
pub struct UserDeactivateCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserDeactivateCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserDeactivateCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserDeactivateCommand;
    type Output = UserDeactivateOutput;
    type Error = UserDeactivateCommandHandlerError;
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
        let mut user = self.user_repository.read(uow, command.user_id).await?;

        let result = user.deactivate()?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        let output = match result {
            UserDeactivateResult::Deactivated => UserDeactivateOutput::Deactivated,
            UserDeactivateResult::Rejected { reason } => UserDeactivateOutput::Rejected { reason },
        };

        Ok(output)
    }
}

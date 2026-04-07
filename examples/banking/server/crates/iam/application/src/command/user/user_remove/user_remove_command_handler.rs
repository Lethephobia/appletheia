use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationRefOwned,
    RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{UserRemoveCommand, UserRemoveCommandHandlerError, UserRemoveOutput};
use crate::authorization::UserRemoverRelation;
use crate::projection::{
    UserOwnerRelationshipProjectorSpec, UserStatusManagerRelationshipProjectorSpec,
};

/// Handles `UserRemoveCommand`.
pub struct UserRemoveCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserRemoveCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserRemoveCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserRemoveCommand;
    type Output = UserRemoveOutput;
    type ReplayOutput = UserRemoveOutput;
    type Error = UserRemoveCommandHandlerError;
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
                    relation: RelationRefOwned::from(UserRemoverRelation::REF),
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    UserStatusManagerRelationshipProjectorSpec::DESCRIPTOR,
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
            return Err(UserRemoveCommandHandlerError::TargetUserNotFound);
        };

        user.remove()?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(CommandHandled::same(UserRemoveOutput))
    }
}

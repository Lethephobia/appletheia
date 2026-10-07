use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::token_binding::TokenBinding;

use super::{
    TokenBindingRemoveCommand, TokenBindingRemoveCommandHandlerError, TokenBindingRemoveOutput,
};
use crate::authorization::TokenBindingRemoverRelation;

pub struct TokenBindingRemoveCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TokenBindingRemoveCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TokenBindingRemoveCommandHandler<R>
where
    R: Repository,
{
    type Command = TokenBindingRemoveCommand;
    type Output = TokenBindingRemoveOutput;
    type Error = TokenBindingRemoveCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                TokenBinding,
            >(
                command.token_binding_id,
                TokenBindingRemoverRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut token_binding = self
            .repository
            .read::<TokenBinding>(uow, command.token_binding_id)
            .await?;
        token_binding.remove()?;
        self.repository
            .save::<TokenBinding>(uow, request_context, &mut token_binding)
            .await?;
        Ok(TokenBindingRemoveOutput {
            token_binding_id: command.token_binding_id,
        })
    }
}

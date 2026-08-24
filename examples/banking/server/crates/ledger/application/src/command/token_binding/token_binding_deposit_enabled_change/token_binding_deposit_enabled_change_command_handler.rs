use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingEnablementChangeResult};

use super::{
    TokenBindingDepositEnabledChangeCommand, TokenBindingDepositEnabledChangeCommandHandlerError,
    TokenBindingDepositEnabledChangeOutput,
};
use crate::authorization::TokenBindingDepositEnabledChangerRelation;

pub struct TokenBindingDepositEnabledChangeCommandHandler<R>
where
    R: Repository<TokenBinding>,
{
    repository: R,
}

impl<R> TokenBindingDepositEnabledChangeCommandHandler<R>
where
    R: Repository<TokenBinding>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TokenBindingDepositEnabledChangeCommandHandler<R>
where
    R: Repository<TokenBinding>,
{
    type Command = TokenBindingDepositEnabledChangeCommand;
    type Output = TokenBindingDepositEnabledChangeOutput;
    type Error = TokenBindingDepositEnabledChangeCommandHandlerError;
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
                TokenBindingDepositEnabledChangerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut token_binding = self.repository.read(uow, command.token_binding_id).await?;
        let result = token_binding.change_deposit_enabled(command.enabled)?;
        self.repository
            .save(uow, request_context, &mut token_binding)
            .await?;
        Ok(match result {
            TokenBindingEnablementChangeResult::Changed => {
                TokenBindingDepositEnabledChangeOutput::Changed {
                    token_binding_id: command.token_binding_id,
                    enabled: command.enabled,
                }
            }
            TokenBindingEnablementChangeResult::Rejected { reason } => {
                TokenBindingDepositEnabledChangeOutput::Rejected {
                    token_binding_id: command.token_binding_id,
                    enabled: command.enabled,
                    reason,
                }
            }
        })
    }
}

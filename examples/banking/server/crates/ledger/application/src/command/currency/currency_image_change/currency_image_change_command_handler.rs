use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::Currency;

use super::{
    CurrencyImageChangeCommand, CurrencyImageChangeCommandHandlerError, CurrencyImageChangeOutput,
};
use crate::authorization::CurrencyUpdaterRelation;

/// Handles `CurrencyImageChangeCommand`.
pub struct CurrencyImageChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    currency_repository: CR,
}

impl<CR> CurrencyImageChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    pub fn new(currency_repository: CR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CR> CommandHandler for CurrencyImageChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    type Command = CurrencyImageChangeCommand;
    type Output = CurrencyImageChangeOutput;
    type ReplayOutput = CurrencyImageChangeOutput;
    type Error = CurrencyImageChangeCommandHandlerError;
    type Uow = CR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Currency,
            >(
                command.currency_id,
                CurrencyUpdaterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut currency) = self
            .currency_repository
            .find(uow, command.currency_id)
            .await?
        else {
            return Err(CurrencyImageChangeCommandHandlerError::CurrencyNotFound);
        };

        let result = currency.change_image(command.image.clone())?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        Ok(CommandHandled::same(CurrencyImageChangeOutput::from(
            result,
        )))
    }
}

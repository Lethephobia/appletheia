use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyDescriptionChangeResult};

use super::{
    CurrencyDescriptionChangeCommand, CurrencyDescriptionChangeCommandHandlerError,
    CurrencyDescriptionChangeOutput,
};
use crate::authorization::CurrencyUpdaterRelation;

/// Handles `CurrencyDescriptionChangeCommand`.
pub struct CurrencyDescriptionChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    currency_repository: CR,
}

impl<CR> CurrencyDescriptionChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    pub fn new(currency_repository: CR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CR> CommandHandler for CurrencyDescriptionChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    type Command = CurrencyDescriptionChangeCommand;
    type Output = CurrencyDescriptionChangeOutput;
    type ReplayOutput = CurrencyDescriptionChangeOutput;
    type Error = CurrencyDescriptionChangeCommandHandlerError;
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
            return Err(CurrencyDescriptionChangeCommandHandlerError::CurrencyNotFound);
        };

        let result = currency.change_description(command.description.clone())?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyDescriptionChangeResult::Changed => CurrencyDescriptionChangeOutput::Changed,
            CurrencyDescriptionChangeResult::Rejected { reason } => {
                CurrencyDescriptionChangeOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_definition::CurrencyDefinition;

use super::{
    CurrencyDefinitionIncreaseSupplyCommand, CurrencyDefinitionIncreaseSupplyCommandHandlerError,
    CurrencyDefinitionIncreaseSupplyOutput,
};

/// Handles `CurrencyDefinitionIncreaseSupplyCommand`.
pub struct CurrencyDefinitionIncreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    currency_definition_repository: CDR,
}

impl<CDR> CurrencyDefinitionIncreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    pub fn new(currency_definition_repository: CDR) -> Self {
        Self {
            currency_definition_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyDefinitionIncreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    type Command = CurrencyDefinitionIncreaseSupplyCommand;
    type Output = CurrencyDefinitionIncreaseSupplyOutput;
    type ReplayOutput = CurrencyDefinitionIncreaseSupplyOutput;
    type Error = CurrencyDefinitionIncreaseSupplyCommandHandlerError;
    type Uow = CDR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut currency_definition) = self
            .currency_definition_repository
            .find(uow, command.currency_definition_id)
            .await?
        else {
            return Err(
                CurrencyDefinitionIncreaseSupplyCommandHandlerError::CurrencyDefinitionNotFound,
            );
        };

        currency_definition.increase_supply(command.amount)?;
        self.currency_definition_repository
            .save(uow, request_context, &mut currency_definition)
            .await?;

        Ok(CommandHandled::same(CurrencyDefinitionIncreaseSupplyOutput))
    }
}

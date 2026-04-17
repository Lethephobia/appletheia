use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::Currency;

use super::{
    CurrencyIncreaseSupplyCommand, CurrencyIncreaseSupplyCommandHandlerError,
    CurrencyIncreaseSupplyOutput,
};

/// Handles `CurrencyIncreaseSupplyCommand`.
pub struct CurrencyIncreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencyIncreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyIncreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencyIncreaseSupplyCommand;
    type Output = CurrencyIncreaseSupplyOutput;
    type ReplayOutput = CurrencyIncreaseSupplyOutput;
    type Error = CurrencyIncreaseSupplyCommandHandlerError;
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
        let Some(mut currency) = self
            .currency_repository
            .find(uow, command.currency_id)
            .await?
        else {
            return Err(CurrencyIncreaseSupplyCommandHandlerError::CurrencyNotFound);
        };

        currency.increase_supply(command.amount)?;
        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        Ok(CommandHandled::same(CurrencyIncreaseSupplyOutput))
    }
}

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::Currency;

use super::{
    CurrencyDecreaseSupplyCommand, CurrencyDecreaseSupplyCommandHandlerError,
    CurrencyDecreaseSupplyOutput,
};

/// Handles `CurrencyDecreaseSupplyCommand`.
pub struct CurrencyDecreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencyDecreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyDecreaseSupplyCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencyDecreaseSupplyCommand;
    type Output = CurrencyDecreaseSupplyOutput;
    type ReplayOutput = CurrencyDecreaseSupplyOutput;
    type Error = CurrencyDecreaseSupplyCommandHandlerError;
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
            return Err(CurrencyDecreaseSupplyCommandHandlerError::CurrencyNotFound);
        };

        currency.decrease_supply(command.amount)?;
        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        Ok(CommandHandled::same(CurrencyDecreaseSupplyOutput))
    }
}

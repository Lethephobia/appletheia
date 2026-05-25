use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencySupplyReserveResult};

use super::{
    CurrencySupplyReserveCommand, CurrencySupplyReserveCommandHandlerError,
    CurrencySupplyReserveOutput,
};

/// Handles `CurrencySupplyReserveCommand`.
pub struct CurrencySupplyReserveCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencySupplyReserveCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencySupplyReserveCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencySupplyReserveCommand;
    type Output = CurrencySupplyReserveOutput;
    type ReplayOutput = CurrencySupplyReserveOutput;
    type Error = CurrencySupplyReserveCommandHandlerError;
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
            return Err(CurrencySupplyReserveCommandHandlerError::CurrencyNotFound);
        };

        let result = currency.reserve_supply(command.amount)?;
        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencySupplyReserveResult::Reserved => CurrencySupplyReserveOutput::Reserved,
            CurrencySupplyReserveResult::Rejected { reason } => {
                CurrencySupplyReserveOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencySupplyReleaseResult};

use super::{
    CurrencySupplyReleaseCommand, CurrencySupplyReleaseCommandHandlerError,
    CurrencySupplyReleaseOutput,
};

/// Handles `CurrencySupplyReleaseCommand`.
pub struct CurrencySupplyReleaseCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencySupplyReleaseCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencySupplyReleaseCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencySupplyReleaseCommand;
    type Output = CurrencySupplyReleaseOutput;
    type Error = CurrencySupplyReleaseCommandHandlerError;
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
    ) -> Result<Self::Output, Self::Error> {
        let mut currency = self
            .currency_repository
            .read(uow, command.currency_id)
            .await?;

        let result = currency.release_supply(command.amount)?;
        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencySupplyReleaseResult::Released => CurrencySupplyReleaseOutput::Released,
            CurrencySupplyReleaseResult::Rejected { reason } => {
                CurrencySupplyReleaseOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}

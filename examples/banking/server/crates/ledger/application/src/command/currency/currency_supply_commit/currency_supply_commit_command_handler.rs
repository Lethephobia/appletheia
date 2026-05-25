use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencySupplyCommitResult};

use super::{
    CurrencySupplyCommitCommand, CurrencySupplyCommitCommandHandlerError,
    CurrencySupplyCommitOutput,
};

/// Handles `CurrencySupplyCommitCommand`.
pub struct CurrencySupplyCommitCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencySupplyCommitCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencySupplyCommitCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencySupplyCommitCommand;
    type Output = CurrencySupplyCommitOutput;
    type ReplayOutput = CurrencySupplyCommitOutput;
    type Error = CurrencySupplyCommitCommandHandlerError;
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
            return Err(CurrencySupplyCommitCommandHandlerError::CurrencyNotFound);
        };

        let result = currency.commit_supply(command.amount)?;
        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencySupplyCommitResult::Committed => CurrencySupplyCommitOutput::Committed,
            CurrencySupplyCommitResult::Rejected { reason } => {
                CurrencySupplyCommitOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceCompleteResult};

use super::{
    CurrencyIssuanceCompleteCommand, CurrencyIssuanceCompleteCommandHandlerError,
    CurrencyIssuanceCompleteOutput,
};

/// Handles `CurrencyIssuanceCompleteCommand`.
pub struct CurrencyIssuanceCompleteCommandHandler<CIR>
where
    CIR: Repository<CurrencyIssuance>,
{
    currency_issuance_repository: CIR,
}

impl<CIR> CurrencyIssuanceCompleteCommandHandler<CIR>
where
    CIR: Repository<CurrencyIssuance>,
{
    pub fn new(currency_issuance_repository: CIR) -> Self {
        Self {
            currency_issuance_repository,
        }
    }
}

impl<CIR> CommandHandler for CurrencyIssuanceCompleteCommandHandler<CIR>
where
    CIR: Repository<CurrencyIssuance>,
{
    type Command = CurrencyIssuanceCompleteCommand;
    type Output = CurrencyIssuanceCompleteOutput;
    type ReplayOutput = CurrencyIssuanceCompleteOutput;
    type Error = CurrencyIssuanceCompleteCommandHandlerError;
    type Uow = CIR::Uow;

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
        let mut currency_issuance = self
            .currency_issuance_repository
            .read(uow, command.currency_issuance_id)
            .await?;

        let result = currency_issuance.complete()?;
        self.currency_issuance_repository
            .save(uow, request_context, &mut currency_issuance)
            .await?;

        let output = match result {
            CurrencyIssuanceCompleteResult::Completed => CurrencyIssuanceCompleteOutput::Completed,
            CurrencyIssuanceCompleteResult::Rejected { reason } => {
                CurrencyIssuanceCompleteOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}

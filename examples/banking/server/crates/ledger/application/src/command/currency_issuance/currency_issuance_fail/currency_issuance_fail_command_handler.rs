use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceFailResult};

use super::{
    CurrencyIssuanceFailCommand, CurrencyIssuanceFailCommandHandlerError,
    CurrencyIssuanceFailOutput,
};

/// Handles `CurrencyIssuanceFailCommand`.
pub struct CurrencyIssuanceFailCommandHandler<CIR>
where
    CIR: Repository<CurrencyIssuance>,
{
    currency_issuance_repository: CIR,
}

impl<CIR> CurrencyIssuanceFailCommandHandler<CIR>
where
    CIR: Repository<CurrencyIssuance>,
{
    pub fn new(currency_issuance_repository: CIR) -> Self {
        Self {
            currency_issuance_repository,
        }
    }
}

impl<CIR> CommandHandler for CurrencyIssuanceFailCommandHandler<CIR>
where
    CIR: Repository<CurrencyIssuance>,
{
    type Command = CurrencyIssuanceFailCommand;
    type Output = CurrencyIssuanceFailOutput;
    type ReplayOutput = CurrencyIssuanceFailOutput;
    type Error = CurrencyIssuanceFailCommandHandlerError;
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

        let result = currency_issuance.fail(command.reason)?;
        self.currency_issuance_repository
            .save(uow, request_context, &mut currency_issuance)
            .await?;

        let output = match result {
            CurrencyIssuanceFailResult::Failed => CurrencyIssuanceFailOutput::Failed,
            CurrencyIssuanceFailResult::Rejected { reason } => {
                CurrencyIssuanceFailOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}

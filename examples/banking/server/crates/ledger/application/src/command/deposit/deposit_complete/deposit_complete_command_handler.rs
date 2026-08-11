use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::deposit::{Deposit, DepositCompleteResult};

use super::{DepositCompleteCommand, DepositCompleteCommandHandlerError, DepositCompleteOutput};

/// Handles `DepositCompleteCommand`.
pub struct DepositCompleteCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    deposit_repository: DR,
}

impl<DR> DepositCompleteCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    pub fn new(deposit_repository: DR) -> Self {
        Self { deposit_repository }
    }
}

impl<DR> CommandHandler for DepositCompleteCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    type Command = DepositCompleteCommand;
    type Output = DepositCompleteOutput;
    type Error = DepositCompleteCommandHandlerError;
    type Uow = DR::Uow;

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
        let mut deposit = self
            .deposit_repository
            .read(uow, command.deposit_id)
            .await?;

        let result = deposit.complete()?;
        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        let output = match result {
            DepositCompleteResult::Completed => DepositCompleteOutput::Completed,
            DepositCompleteResult::Rejected { reason } => {
                DepositCompleteOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}

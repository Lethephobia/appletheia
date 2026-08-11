use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::deposit::{Deposit, DepositFailResult};

use super::{DepositFailCommand, DepositFailCommandHandlerError, DepositFailOutput};

/// Handles `DepositFailCommand`.
pub struct DepositFailCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    deposit_repository: DR,
}

impl<DR> DepositFailCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    pub fn new(deposit_repository: DR) -> Self {
        Self { deposit_repository }
    }
}

impl<DR> CommandHandler for DepositFailCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    type Command = DepositFailCommand;
    type Output = DepositFailOutput;
    type Error = DepositFailCommandHandlerError;
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

        let result = deposit.fail(command.reason)?;
        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        let output = match result {
            DepositFailResult::Failed => DepositFailOutput::Failed,
            DepositFailResult::Rejected { reason } => DepositFailOutput::Rejected { reason },
        };

        Ok(output)
    }
}

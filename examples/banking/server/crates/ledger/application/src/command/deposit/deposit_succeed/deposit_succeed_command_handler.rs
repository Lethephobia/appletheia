use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::deposit::Deposit;

use super::{DepositSucceedCommand, DepositSucceedCommandHandlerError, DepositSucceedOutput};

pub struct DepositSucceedCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    deposit_repository: DR,
}

impl<DR> DepositSucceedCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    pub fn new(deposit_repository: DR) -> Self {
        Self { deposit_repository }
    }
}

impl<DR> CommandHandler for DepositSucceedCommandHandler<DR>
where
    DR: Repository<Deposit>,
{
    type Command = DepositSucceedCommand;
    type Output = DepositSucceedOutput;
    type Error = DepositSucceedCommandHandlerError;
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

        deposit.succeed()?;
        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        Ok(DepositSucceedOutput {})
    }
}

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::deposit::Deposit;

use super::{DepositSucceedCommand, DepositSucceedCommandHandlerError, DepositSucceedOutput};

pub struct DepositSucceedCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> DepositSucceedCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for DepositSucceedCommandHandler<R>
where
    R: Repository,
{
    type Command = DepositSucceedCommand;
    type Output = DepositSucceedOutput;
    type Error = DepositSucceedCommandHandlerError;
    type Uow = R::Uow;

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
            .repository
            .read::<Deposit>(uow, command.deposit_id)
            .await?;

        deposit.succeed()?;
        self.repository
            .save(uow, request_context, &mut deposit)
            .await?;

        Ok(DepositSucceedOutput {})
    }
}

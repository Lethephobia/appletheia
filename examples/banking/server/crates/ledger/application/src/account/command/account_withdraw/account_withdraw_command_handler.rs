use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{AccountWithdrawCommand, AccountWithdrawCommandHandlerError, AccountWithdrawOutput};

pub struct AccountWithdrawCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountWithdrawCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountWithdrawCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountWithdrawCommand;
    type Output = AccountWithdrawOutput;
    type Error = AccountWithdrawCommandHandlerError;
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
        let mut account = self
            .repository
            .read::<Account>(uow, command.account_id)
            .await?;

        account.withdraw(command.amount)?;
        self.repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(AccountWithdrawOutput {})
    }
}

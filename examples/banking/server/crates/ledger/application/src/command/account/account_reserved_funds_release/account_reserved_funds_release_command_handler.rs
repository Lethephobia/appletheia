use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountReservedFundsReleaseCommand, AccountReservedFundsReleaseCommandHandlerError,
    AccountReservedFundsReleaseOutput,
};

pub struct AccountReservedFundsReleaseCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountReservedFundsReleaseCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountReservedFundsReleaseCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountReservedFundsReleaseCommand;
    type Output = AccountReservedFundsReleaseOutput;
    type Error = AccountReservedFundsReleaseCommandHandlerError;
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

        account.release_reserved_funds(command.amount)?;
        self.repository
            .save::<Account>(uow, request_context, &mut account)
            .await?;

        Ok(AccountReservedFundsReleaseOutput {})
    }
}

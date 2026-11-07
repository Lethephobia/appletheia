use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountReleaseReservedFundsCommand, AccountReleaseReservedFundsCommandHandlerError,
    AccountReleaseReservedFundsOutput,
};

pub struct AccountReleaseReservedFundsCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountReleaseReservedFundsCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountReleaseReservedFundsCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountReleaseReservedFundsCommand;
    type Output = AccountReleaseReservedFundsOutput;
    type Error = AccountReleaseReservedFundsCommandHandlerError;
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
            .save(uow, request_context, &mut account)
            .await?;

        Ok(AccountReleaseReservedFundsOutput {})
    }
}

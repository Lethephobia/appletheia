use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountReleaseReservedFundsCommand, AccountReleaseReservedFundsCommandHandlerError,
    AccountReleaseReservedFundsOutput,
};

/// Handles `AccountReleaseReservedFundsCommand`.
pub struct AccountReleaseReservedFundsCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountReleaseReservedFundsCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountReleaseReservedFundsCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountReleaseReservedFundsCommand;
    type Output = AccountReleaseReservedFundsOutput;
    type ReplayOutput = AccountReleaseReservedFundsOutput;
    type Error = AccountReleaseReservedFundsCommandHandlerError;
    type Uow = AR::Uow;

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
        let Some(mut account) = self
            .account_repository
            .find(uow, command.account_id)
            .await?
        else {
            return Err(AccountReleaseReservedFundsCommandHandlerError::AccountNotFound);
        };

        account.release_reserved_funds(command.amount)?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(CommandHandled::same(AccountReleaseReservedFundsOutput))
    }
}

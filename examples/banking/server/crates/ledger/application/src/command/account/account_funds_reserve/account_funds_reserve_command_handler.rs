use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountFundsReserveCommand, AccountFundsReserveCommandHandlerError, AccountFundsReserveOutput,
};

/// Handles `AccountFundsReserveCommand`.
pub struct AccountFundsReserveCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountFundsReserveCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountFundsReserveCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountFundsReserveCommand;
    type Output = AccountFundsReserveOutput;
    type ReplayOutput = AccountFundsReserveOutput;
    type Error = AccountFundsReserveCommandHandlerError;
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
            return Err(AccountFundsReserveCommandHandlerError::AccountNotFound);
        };

        let result = account.reserve_funds(command.amount)?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(CommandHandled::same(AccountFundsReserveOutput::from(
            result,
        )))
    }
}

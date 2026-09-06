use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountReservedFundsCommitCommand, AccountReservedFundsCommitCommandHandlerError,
    AccountReservedFundsCommitOutput,
};

/// Handles `AccountReservedFundsCommitCommand`.
pub struct AccountReservedFundsCommitCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountReservedFundsCommitCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountReservedFundsCommitCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountReservedFundsCommitCommand;
    type Output = AccountReservedFundsCommitOutput;
    type Error = AccountReservedFundsCommitCommandHandlerError;
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
    ) -> Result<Self::Output, Self::Error> {
        let mut account = self
            .account_repository
            .read(uow, command.account_id)
            .await?;

        account.commit_reserved_funds(command.amount)?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(AccountReservedFundsCommitOutput::Committed)
    }
}

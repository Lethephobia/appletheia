use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::{Account, AccountReservedFundsReleaseResult};

use super::{
    AccountReservedFundsReleaseCommand, AccountReservedFundsReleaseCommandHandlerError,
    AccountReservedFundsReleaseOutput,
};

/// Handles `AccountReservedFundsReleaseCommand`.
pub struct AccountReservedFundsReleaseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountReservedFundsReleaseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountReservedFundsReleaseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountReservedFundsReleaseCommand;
    type Output = AccountReservedFundsReleaseOutput;
    type ReplayOutput = AccountReservedFundsReleaseOutput;
    type Error = AccountReservedFundsReleaseCommandHandlerError;
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
            return Err(AccountReservedFundsReleaseCommandHandlerError::AccountNotFound);
        };

        let result = account.release_reserved_funds(command.amount)?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let output = match result {
            AccountReservedFundsReleaseResult::Released => {
                AccountReservedFundsReleaseOutput::Released
            }
            AccountReservedFundsReleaseResult::Rejected { reason } => {
                AccountReservedFundsReleaseOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}

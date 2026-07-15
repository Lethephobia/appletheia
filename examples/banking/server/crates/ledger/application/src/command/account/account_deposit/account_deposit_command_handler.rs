use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::{Account, AccountDepositResult};

use super::{AccountDepositCommand, AccountDepositCommandHandlerError, AccountDepositOutput};

/// Handles `AccountDepositCommand`.
pub struct AccountDepositCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountDepositCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountDepositCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountDepositCommand;
    type Output = AccountDepositOutput;
    type ReplayOutput = AccountDepositOutput;
    type Error = AccountDepositCommandHandlerError;
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
        let mut account = self
            .account_repository
            .read(uow, command.account_id)
            .await?;

        let result = account.deposit(command.amount)?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let output = match result {
            AccountDepositResult::Deposited => AccountDepositOutput::Deposited,
            AccountDepositResult::Rejected { reason } => AccountDepositOutput::Rejected { reason },
        };

        Ok(CommandHandled::same(output))
    }
}

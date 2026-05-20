use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::{Account, AccountCloseResult};

use super::{AccountCloseCommand, AccountCloseCommandHandlerError, AccountCloseOutput};
use crate::authorization::AccountCloserRelation;

/// Handles `AccountCloseCommand`.
pub struct AccountCloseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountCloseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountCloseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountCloseCommand;
    type Output = AccountCloseOutput;
    type ReplayOutput = AccountCloseOutput;
    type Error = AccountCloseCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.account_id,
                AccountCloserRelation::REF,
            )),
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
            return Err(AccountCloseCommandHandlerError::AccountNotFound);
        };

        let result = account.close()?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let output = match result {
            AccountCloseResult::Closed => AccountCloseOutput::Closed,
            AccountCloseResult::Rejected { reason } => AccountCloseOutput::Rejected { reason },
        };

        Ok(CommandHandled::same(output))
    }
}

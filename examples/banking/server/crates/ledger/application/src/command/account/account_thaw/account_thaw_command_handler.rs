use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::{Account, AccountThawResult};

use super::{AccountThawCommand, AccountThawCommandHandlerError, AccountThawOutput};
use crate::authorization::AccountThawerRelation;

/// Handles `AccountThawCommand`.
pub struct AccountThawCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountThawCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountThawCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountThawCommand;
    type Output = AccountThawOutput;
    type ReplayOutput = AccountThawOutput;
    type Error = AccountThawCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.account_id,
                AccountThawerRelation::REF,
            )),
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

        let result = account.thaw()?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let output = match result {
            AccountThawResult::Thawed => AccountThawOutput::Thawed,
            AccountThawResult::Rejected { reason } => AccountThawOutput::Rejected { reason },
        };

        Ok(CommandHandled::same(output))
    }
}

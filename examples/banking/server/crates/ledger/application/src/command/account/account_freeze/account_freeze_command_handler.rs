use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::{Account, AccountFreezeResult};

use super::{AccountFreezeCommand, AccountFreezeCommandHandlerError, AccountFreezeOutput};
use crate::authorization::AccountFreezerRelation;

/// Handles `AccountFreezeCommand`.
pub struct AccountFreezeCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountFreezeCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountFreezeCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountFreezeCommand;
    type Output = AccountFreezeOutput;
    type Error = AccountFreezeCommandHandlerError;
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
                AccountFreezerRelation::REF,
            )),
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

        let result = account.freeze()?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let output = match result {
            AccountFreezeResult::Frozen => AccountFreezeOutput::Frozen,
            AccountFreezeResult::Rejected { reason } => AccountFreezeOutput::Rejected { reason },
        };

        Ok(output)
    }
}

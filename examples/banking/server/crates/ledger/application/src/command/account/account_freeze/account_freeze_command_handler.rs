use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{AccountFreezeCommand, AccountFreezeCommandHandlerError, AccountFreezeOutput};
use crate::authorization::AccountFreezerRelation;

pub struct AccountFreezeCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountFreezeCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountFreezeCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountFreezeCommand;
    type Output = AccountFreezeOutput;
    type Error = AccountFreezeCommandHandlerError;
    type Uow = R::Uow;

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
            .repository
            .read::<Account>(uow, command.account_id)
            .await?;

        account.freeze()?;
        self.repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(AccountFreezeOutput {})
    }
}

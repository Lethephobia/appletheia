use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{AccountCloseCommand, AccountCloseCommandHandlerError, AccountCloseOutput};
use crate::authorization::AccountCloserRelation;

pub struct AccountCloseCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountCloseCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountCloseCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountCloseCommand;
    type Output = AccountCloseOutput;
    type Error = AccountCloseCommandHandlerError;
    type Uow = R::Uow;

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
    ) -> Result<Self::Output, Self::Error> {
        let mut account = self
            .repository
            .read::<Account>(uow, command.account_id)
            .await?;

        account.close()?;
        self.repository
            .save::<Account>(uow, request_context, &mut account)
            .await?;

        Ok(AccountCloseOutput {})
    }
}

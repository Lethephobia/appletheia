use crate::authorization::AccountManagerRelation;
use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountSetDescriptionCommand, AccountSetDescriptionCommandHandlerError,
    AccountSetDescriptionOutput,
};

pub struct AccountSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountSetDescriptionCommand;
    type Output = AccountSetDescriptionOutput;
    type Error = AccountSetDescriptionCommandHandlerError;
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
                AccountManagerRelation::REF,
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
        account.set_description(command.description.clone())?;
        self.repository
            .save(uow, request_context, &mut account)
            .await?;
        Ok(AccountSetDescriptionOutput {})
    }
}

use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use crate::authorization::AccountDescriptionSetterRelation;

use super::{
    AccountDescriptionSetCommand, AccountDescriptionSetCommandHandlerError,
    AccountDescriptionSetOutput,
};

pub struct AccountDescriptionSetCommandHandler<R>
where
    R: Repository<Account>,
{
    repository: R,
}

impl<R> AccountDescriptionSetCommandHandler<R>
where
    R: Repository<Account>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountDescriptionSetCommandHandler<R>
where
    R: Repository<Account>,
{
    type Command = AccountDescriptionSetCommand;
    type Output = AccountDescriptionSetOutput;
    type Error = AccountDescriptionSetCommandHandlerError;
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
                AccountDescriptionSetterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut account = self.repository.read(uow, command.account_id).await?;
        account.set_description(command.description.clone())?;
        self.repository
            .save(uow, request_context, &mut account)
            .await?;
        Ok(AccountDescriptionSetOutput {})
    }
}

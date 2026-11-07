use crate::authorization::AccountOwnershipManagerRelation;
use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountTransferOwnershipCommand, AccountTransferOwnershipCommandHandlerError,
    AccountTransferOwnershipOutput,
};

pub struct AccountTransferOwnershipCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountTransferOwnershipCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountTransferOwnershipCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountTransferOwnershipCommand;
    type Output = AccountTransferOwnershipOutput;
    type Error = AccountTransferOwnershipCommandHandlerError;
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
                AccountOwnershipManagerRelation::REF,
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

        account.transfer_ownership(command.owner)?;

        self.repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(AccountTransferOwnershipOutput {})
    }
}

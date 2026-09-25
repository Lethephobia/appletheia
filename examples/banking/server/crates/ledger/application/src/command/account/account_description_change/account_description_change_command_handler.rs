use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::{Account, AccountDescriptionChangeResult};

use crate::authorization::AccountDescriptionChangerRelation;

use super::{
    AccountDescriptionChangeCommand, AccountDescriptionChangeCommandHandlerError,
    AccountDescriptionChangeOutput,
};

pub struct AccountDescriptionChangeCommandHandler<R>
where
    R: Repository<Account>,
{
    repository: R,
}

impl<R> AccountDescriptionChangeCommandHandler<R>
where
    R: Repository<Account>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountDescriptionChangeCommandHandler<R>
where
    R: Repository<Account>,
{
    type Command = AccountDescriptionChangeCommand;
    type Output = AccountDescriptionChangeOutput;
    type Error = AccountDescriptionChangeCommandHandlerError;
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
                AccountDescriptionChangerRelation::REF,
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
        let result = account.change_description(command.description.clone())?;
        self.repository
            .save(uow, request_context, &mut account)
            .await?;
        Ok(match result {
            AccountDescriptionChangeResult::Changed => AccountDescriptionChangeOutput::Changed,
            AccountDescriptionChangeResult::Rejected { reason } => {
                AccountDescriptionChangeOutput::Rejected { reason }
            }
        })
    }
}

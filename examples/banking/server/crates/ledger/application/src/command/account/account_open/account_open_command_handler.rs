use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_application::authorization::{
    OrganizationFinanceManagerRelation, UserOwnerRelation,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::account::{Account, AccountOwner};
use banking_ledger_domain::currency::Currency;

use super::{AccountOpenCommand, AccountOpenCommandHandlerError, AccountOpenOutput};
pub struct AccountOpenCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountOpenCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountOpenCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountOpenCommand;
    type Output = AccountOpenOutput;
    type Error = AccountOpenCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        match command.owner {
            AccountOwner::User(user_id) => Ok(AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                ),
            ])),
            AccountOwner::Organization(organization_id) => {
                Ok(AuthorizationPlan::OnlyPrincipals(vec![
                    PrincipalRequirement::AuthenticatedWithRelationship(
                        RelationshipRequirement::check::<Organization>(
                            organization_id,
                            OrganizationFinanceManagerRelation::REF,
                        ),
                    ),
                ]))
            }
        }
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let currency = self
            .repository
            .read::<Currency>(uow, command.currency_id)
            .await?;
        if !currency.is_active()? {
            return Err(AccountOpenCommandHandlerError::CurrencyInactive);
        }

        let mut account = Account::new();
        let account_id = account.aggregate_id();
        account.open(command.owner, command.name.clone(), command.currency_id)?;
        if let Some(description) = &command.description {
            account.set_description(Some(description.clone()))?;
        }

        self.repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(AccountOpenOutput { account_id })
    }
}

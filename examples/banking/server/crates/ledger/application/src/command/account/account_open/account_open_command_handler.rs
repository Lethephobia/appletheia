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
use banking_ledger_domain::account::{Account, AccountOpenResult, AccountOpening, AccountOwner};
use banking_ledger_domain::currency::Currency;

use super::{AccountOpenCommand, AccountOpenCommandHandlerError, AccountOpenOutput};
/// Handles `AccountOpenCommand`.
pub struct AccountOpenCommandHandler<AR, CR>
where
    AR: Repository<Account>,
    CR: Repository<Currency, Uow = AR::Uow>,
{
    account_repository: AR,
    currency_repository: CR,
}

impl<AR, CR> AccountOpenCommandHandler<AR, CR>
where
    AR: Repository<Account>,
    CR: Repository<Currency, Uow = AR::Uow>,
{
    pub fn new(account_repository: AR, currency_repository: CR) -> Self {
        Self {
            account_repository,
            currency_repository,
        }
    }
}

impl<AR, CR> CommandHandler for AccountOpenCommandHandler<AR, CR>
where
    AR: Repository<Account>,
    CR: Repository<Currency, Uow = AR::Uow>,
{
    type Command = AccountOpenCommand;
    type Output = AccountOpenOutput;
    type Error = AccountOpenCommandHandlerError;
    type Uow = AR::Uow;

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
            .currency_repository
            .read(uow, command.currency_id)
            .await?;
        if !currency.is_active()? {
            return Err(AccountOpenCommandHandlerError::CurrencyInactive);
        }

        let mut account = Account::new();
        let account_id = account.aggregate_id();
        let result = account.open(AccountOpening {
            owner: command.owner,
            name: command.name.clone(),
            description: command.description.clone(),
            currency_id: command.currency_id,
        })?;

        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let output = match result {
            AccountOpenResult::Opened => AccountOpenOutput { account_id },
        };

        Ok(output)
    }
}

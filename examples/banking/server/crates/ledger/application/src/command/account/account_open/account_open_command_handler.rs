use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::authorization::{
    OrganizationFinanceManagerRelation, UserOwnerRelation,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::account::{Account, AccountOpenResult, AccountOpening, AccountOwner};

use super::{AccountOpenCommand, AccountOpenCommandHandlerError, AccountOpenOutput};

/// Handles `AccountOpenCommand`.
pub struct AccountOpenCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountOpenCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountOpenCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountOpenCommand;
    type Output = AccountOpenOutput;
    type ReplayOutput = AccountOpenOutput;
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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let mut account = Account::default();
        let result = account.open(AccountOpening {
            owner: command.owner,
            name: command.name.clone(),
            currency_id: command.currency_id,
        })?;

        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let output = match result {
            AccountOpenResult::Opened { account_id } => AccountOpenOutput::new(account_id),
        };

        Ok(CommandHandled::same(output))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;

    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_application::authorization::{
        OrganizationFinanceManagerRelation, UserOwnerRelation,
    };

    use banking_iam_domain::{Organization, OrganizationId, User, UserId};
    use banking_ledger_domain::account::{Account, AccountId, AccountName, AccountOwner};
    use banking_ledger_domain::currency::CurrencyId;
    use uuid::Uuid;

    use super::{AccountOpenCommand, AccountOpenCommandHandler, AccountOpenOutput};

    fn account_name() -> AccountName {
        AccountName::try_from("main").expect("account name should be valid")
    }

    fn account_owner() -> AccountOwner {
        AccountOwner::User(UserId::new())
    }

    fn organization_owner() -> AccountOwner {
        AccountOwner::Organization(OrganizationId::new())
    }

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestAccountRepository {
        account: Arc<Mutex<Option<Account>>>,
    }

    impl Repository<Account> for TestAccountRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: AccountId,
        ) -> Result<Account, RepositoryError<Account>> {
            self.account
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Account::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: AccountId,
            _at: appletheia::domain::AggregateVersion,
        ) -> Result<Account, RepositoryError<Account>> {
            self.account
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Account::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Account,
        ) -> Result<(), RepositoryError<Account>> {
            *self.account.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        let subject = AggregateRef::from_id::<User>(UserId::new());

        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    #[test]
    fn authorization_plan_requires_target_user_owner() {
        let handler = AccountOpenCommandHandler::new(TestAccountRepository::default());
        let owner = account_owner();
        let name = account_name();

        let plan = handler
            .authorization_plan(&AccountOpenCommand {
                owner,
                name,
                currency_id: CurrencyId::new(),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(
                        owner.user_id().copied().expect("user owner expected"),
                        UserOwnerRelation::REF
                    )
                ),
            ])
        );
    }

    #[test]
    fn authorization_plan_requires_organization_finance_manager() {
        let handler = AccountOpenCommandHandler::new(TestAccountRepository::default());
        let owner = organization_owner();
        let name = account_name();

        let plan = handler
            .authorization_plan(&AccountOpenCommand {
                owner,
                name,
                currency_id: CurrencyId::new(),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        owner
                            .organization_id()
                            .copied()
                            .expect("organization owner expected"),
                        OrganizationFinanceManagerRelation::REF
                    )
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_opens_account_for_specified_owner() {
        let repository = TestAccountRepository::default();
        let handler = AccountOpenCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let owner = account_owner();
        let name = account_name();

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &AccountOpenCommand {
                    owner,
                    name: name.clone(),
                    currency_id: CurrencyId::new(),
                },
            )
            .await
            .expect("command should succeed");

        let saved = repository
            .account
            .lock()
            .expect("lock")
            .clone()
            .expect("account should be saved");
        let account_id = saved.aggregate_id().expect("account id should exist");
        assert_eq!(saved.owner().expect("owner should exist"), owner);
        assert_eq!(saved.name().expect("name should exist"), &name);

        assert_eq!(handled.into_output(), AccountOpenOutput::new(account_id));
    }

    #[tokio::test]
    async fn handle_opens_account_for_specified_organization_owner() {
        let repository = TestAccountRepository::default();
        let handler = AccountOpenCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let owner = organization_owner();
        let name = account_name();

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &AccountOpenCommand {
                    owner,
                    name: name.clone(),
                    currency_id: CurrencyId::new(),
                },
            )
            .await
            .expect("command should succeed");

        let saved = repository
            .account
            .lock()
            .expect("lock")
            .clone()
            .expect("account should be saved");
        let account_id = saved.aggregate_id().expect("account id should exist");
        assert_eq!(saved.owner().expect("owner should exist"), owner);
        assert_eq!(saved.name().expect("name should exist"), &name);

        assert_eq!(handled.into_output(), AccountOpenOutput::new(account_id));
    }
}

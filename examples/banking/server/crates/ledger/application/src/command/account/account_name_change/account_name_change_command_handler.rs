use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountNameChangeCommand, AccountNameChangeCommandHandlerError, AccountNameChangeOutput,
};
use crate::authorization::AccountNameChangerRelation;

pub struct AccountNameChangeCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> AccountNameChangeCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for AccountNameChangeCommandHandler<R>
where
    R: Repository,
{
    type Command = AccountNameChangeCommand;
    type Output = AccountNameChangeOutput;
    type Error = AccountNameChangeCommandHandlerError;
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
                AccountNameChangerRelation::REF,
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

        account.change_name(command.name.clone())?;

        self.repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(AccountNameChangeOutput {})
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::aggregate::SerializedAggregate;
    use appletheia::domain::{AggregateId, Event};
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;

    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;

    use banking_ledger_domain::account::{Account, AccountId, AccountName, AccountOwner};
    use banking_ledger_domain::currency::CurrencyId;
    use uuid::Uuid;

    use super::{
        AccountNameChangeCommand, AccountNameChangeCommandHandler, AccountNameChangeOutput,
    };
    use crate::authorization::AccountNameChangerRelation;

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
    struct TestRepository {
        account: Arc<Mutex<Option<Account>>>,
    }

    impl TestRepository {
        fn copy_aggregate<S: Aggregate, T: Aggregate>(source: &S) -> T {
            let mut restored = SerializedAggregate::try_from_aggregate(source)
                .expect("test aggregate should serialize")
                .try_to_aggregate::<T>()
                .expect("test aggregate types should match");
            for event in source.uncommitted_events() {
                let id = T::Id::try_from_uuid(event.aggregate_id().value()).unwrap();
                let payload =
                    serde_json::from_value(serde_json::to_value(event.payload()).unwrap()).unwrap();
                restored
                    .core_mut()
                    .record_uncommitted_event(Event::from_persisted(
                        event.id(),
                        id,
                        event.aggregate_version(),
                        payload,
                        event.occurred_at(),
                    ));
            }
            restored
        }

        fn new(account: Account) -> Self {
            Self {
                account: Arc::new(Mutex::new(Some(account))),
            }
        }
    }

    impl Repository for TestRepository {
        type Uow = TestUow;

        async fn read<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            id: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            self.account
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored))
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: A::TYPE,
                    aggregate_id: id,
                })
        }

        async fn read_at_version<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            id: A::Id,
            _at: appletheia::domain::AggregateVersion,
        ) -> Result<A, RepositoryError<A>> {
            self.account
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored))
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: A::TYPE,
                    aggregate_id: id,
                })
        }

        async fn find_by_unique_value<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            Ok(None)
        }

        async fn save<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut A,
        ) -> Result<(), RepositoryError<A>> {
            *self.account.lock().expect("lock") =
                Some(Self::copy_aggregate::<_, Account>(aggregate));
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid")
    }

    fn account_name(value: &str) -> AccountName {
        AccountName::try_from(value).expect("account name should be valid")
    }

    fn account_owner() -> AccountOwner {
        AccountOwner::User(banking_iam_domain::UserId::new())
    }

    fn opened_account() -> Account {
        let mut account = Account::new();
        account
            .open(account_owner(), account_name("main"), CurrencyId::new())
            .expect("open should succeed");
        account
    }

    #[test]
    fn authorization_plan_requires_account_name_changer() {
        let handler = AccountNameChangeCommandHandler::new(TestRepository::default());
        let account_id = AccountId::new();

        let plan = handler
            .authorization_plan(&AccountNameChangeCommand {
                account_id,
                name: account_name("savings"),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Account>(
                        account_id,
                        AccountNameChangerRelation::REF
                    )
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_changes_account_name() {
        let repository = TestRepository::new(opened_account());
        let handler = AccountNameChangeCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let request_context = request_context();
        let account_id = repository
            .account
            .lock()
            .expect("lock")
            .as_ref()
            .expect("account should exist")
            .aggregate_id();
        let name = account_name("savings");

        let output = handler
            .handle(
                &mut uow,
                &request_context,
                &AccountNameChangeCommand {
                    account_id,
                    name: name.clone(),
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
        assert_eq!(saved.name().expect("name should exist"), &name);

        assert_eq!(output, AccountNameChangeOutput {});
    }
}

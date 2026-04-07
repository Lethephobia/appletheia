use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{AccountRenameCommand, AccountRenameCommandHandlerError, AccountRenameOutput};
use crate::authorization::AccountRenamerRelation;
use crate::projection::AccountOwnerRelationshipProjectorSpec;

/// Handles `AccountRenameCommand`.
pub struct AccountRenameCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountRenameCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountRenameCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountRenameCommand;
    type Output = AccountRenameOutput;
    type ReplayOutput = AccountRenameOutput;
    type Error = AccountRenameCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Account>(
                    command.account_id,
                    AccountRenamerRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    AccountOwnerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut account) = self
            .account_repository
            .find(uow, command.account_id)
            .await?
        else {
            return Err(AccountRenameCommandHandlerError::AccountNotFound);
        };

        account.rename(command.name.clone())?;

        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(CommandHandled::same(AccountRenameOutput))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        ActorRef, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_ledger_domain::account::{Account, AccountId, AccountName, AccountOwner};
    use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
    use uuid::Uuid;

    use super::{AccountRenameCommand, AccountRenameCommandHandler, AccountRenameOutput};
    use crate::authorization::AccountRenamerRelation;
    use crate::projection::AccountOwnerRelationshipProjectorSpec;

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

    impl TestAccountRepository {
        fn new(account: Account) -> Self {
            Self {
                account: Arc::new(Mutex::new(Some(account))),
            }
        }
    }

    impl Repository<Account> for TestAccountRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: AccountId,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(self.account.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: AccountId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(self.account.lock().expect("lock").clone())
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
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            ActorRef::System,
            Principal::System,
        )
    }

    fn account_name(value: &str) -> AccountName {
        AccountName::try_from(value).expect("account name should be valid")
    }

    fn account_owner() -> AccountOwner {
        AccountOwner::User(banking_iam_domain::UserId::new())
    }

    fn opened_account() -> Account {
        let mut account = Account::default();
        account
            .open(
                account_owner(),
                account_name("main"),
                CurrencyDefinitionId::new(),
            )
            .expect("open should succeed");
        account
    }

    #[test]
    fn authorization_plan_allows_system_or_account_owner() {
        let handler = AccountRenameCommandHandler::new(TestAccountRepository::default());
        let account_id = AccountId::new();

        let plan = handler
            .authorization_plan(&AccountRenameCommand {
                account_id,
                name: account_name("savings"),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::System,
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::check::<Account>(
                        account_id,
                        AccountRenamerRelation::REF
                    ),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        AccountOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_renames_account() {
        let repository = TestAccountRepository::new(opened_account());
        let handler = AccountRenameCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let request_context = request_context();
        let account_id = repository
            .account
            .lock()
            .expect("lock")
            .as_ref()
            .expect("account should exist")
            .aggregate_id()
            .expect("account id should exist");
        let name = account_name("savings");

        let handled = handler
            .handle(
                &mut uow,
                &request_context,
                &AccountRenameCommand {
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

        assert_eq!(handled.into_output(), AccountRenameOutput);
    }
}

use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountRequestTransferCommand, AccountRequestTransferCommandHandlerError,
    AccountRequestTransferOutput,
};
use crate::authorization::AccountTransferRequesterRelation;
use crate::projection::AccountOwnerRelationshipProjectorSpec;

/// Handles `AccountRequestTransferCommand`.
pub struct AccountRequestTransferCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountRequestTransferCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountRequestTransferCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountRequestTransferCommand;
    type Output = AccountRequestTransferOutput;
    type ReplayOutput = AccountRequestTransferOutput;
    type Error = AccountRequestTransferCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<Account>(command.account_id),
                    relation: AccountTransferRequesterRelation::NAME,
                },
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
            return Err(AccountRequestTransferCommandHandlerError::SourceAccountNotFound);
        };
        let Some(destination_account) = self
            .account_repository
            .find(uow, command.to_account_id)
            .await?
        else {
            return Err(AccountRequestTransferCommandHandlerError::DestinationAccountNotFound);
        };

        if account.currency_definition_id()? != destination_account.currency_definition_id()? {
            return Err(AccountRequestTransferCommandHandlerError::CurrencyDefinitionMismatch);
        };

        account.request_transfer(command.to_account_id, command.amount)?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(CommandHandled::same(AccountRequestTransferOutput))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::AggregateRef;
    use appletheia::application::command::CommandHandler;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        ActorRef, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{Account, AccountBalance, AccountId};
    use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
    use uuid::Uuid;

    use super::{
        AccountRequestTransferCommand, AccountRequestTransferCommandHandler,
        AccountRequestTransferCommandHandlerError,
    };

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
        accounts: Arc<Mutex<HashMap<AccountId, Account>>>,
    }

    impl TestAccountRepository {
        fn insert(&self, account: Account) {
            let account_id = account.aggregate_id().expect("account id should exist");
            self.accounts
                .lock()
                .expect("lock")
                .insert(account_id, account);
        }
    }

    impl Repository<Account> for TestAccountRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            id: AccountId,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(self.accounts.lock().expect("lock").get(&id).cloned())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            id: AccountId,
            _at: Option<AggregateVersion>,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(self.accounts.lock().expect("lock").get(&id).cloned())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Account,
        ) -> Result<(), RepositoryError<Account>> {
            let account_id = aggregate.aggregate_id().expect("account id should exist");
            self.accounts
                .lock()
                .expect("lock")
                .insert(account_id, aggregate.clone());
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        let subject = AggregateRef::from_id::<User>(UserId::new());

        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            ActorRef::Subject {
                subject: subject.clone(),
            },
            Principal::Authenticated { subject },
        )
    }

    fn opened_account(currency_definition_id: CurrencyDefinitionId) -> Account {
        let mut account = Account::default();
        account
            .open(UserId::new(), currency_definition_id)
            .expect("open should succeed");

        account
    }

    #[tokio::test]
    async fn handle_rejects_destination_account_with_different_currency_definition() {
        let repository = TestAccountRepository::default();
        let source = opened_account(CurrencyDefinitionId::new());
        let destination = opened_account(CurrencyDefinitionId::new());
        let source_account_id = source.aggregate_id().expect("account id should exist");
        let destination_account_id = destination.aggregate_id().expect("account id should exist");
        repository.insert(source);
        repository.insert(destination);

        let handler = AccountRequestTransferCommandHandler::new(repository);
        let mut uow = TestUow;

        let error = handler
            .handle(
                &mut uow,
                &request_context(),
                &AccountRequestTransferCommand {
                    account_id: source_account_id,
                    to_account_id: destination_account_id,
                    amount: AccountBalance::new(10),
                },
            )
            .await
            .expect_err("different currency definitions should fail");

        assert!(matches!(
            error,
            AccountRequestTransferCommandHandlerError::CurrencyDefinitionMismatch
        ));
    }
}

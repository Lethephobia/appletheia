use appletheia::application::ProjectorDependencies;
use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::ProjectorSpec;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::transfer::Transfer;

use crate::authorization::AccountTransferRequesterRelation;
use crate::projection::AccountOwnerRelationshipProjectorSpec;

use super::{TransferRequestCommand, TransferRequestCommandHandlerError, TransferRequestOutput};

/// Handles `TransferRequestCommand`.
pub struct TransferRequestCommandHandler<AR, TR>
where
    AR: Repository<Account, Uow = TR::Uow>,
    TR: Repository<Transfer>,
{
    account_repository: AR,
    transfer_repository: TR,
}

impl<AR, TR> TransferRequestCommandHandler<AR, TR>
where
    AR: Repository<Account, Uow = TR::Uow>,
    TR: Repository<Transfer>,
{
    pub fn new(account_repository: AR, transfer_repository: TR) -> Self {
        Self {
            account_repository,
            transfer_repository,
        }
    }
}

impl<AR, TR> CommandHandler for TransferRequestCommandHandler<AR, TR>
where
    AR: Repository<Account, Uow = TR::Uow>,
    TR: Repository<Transfer>,
{
    type Command = TransferRequestCommand;
    type Output = TransferRequestOutput;
    type ReplayOutput = TransferRequestOutput;
    type Error = TransferRequestCommandHandlerError;
    type Uow = TR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<Account>(command.from_account_id),
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
        let Some(source_account) = self
            .account_repository
            .find(uow, command.from_account_id)
            .await?
        else {
            return Err(TransferRequestCommandHandlerError::SourceAccountNotFound);
        };
        let Some(destination_account) = self
            .account_repository
            .find(uow, command.to_account_id)
            .await?
        else {
            return Err(TransferRequestCommandHandlerError::DestinationAccountNotFound);
        };

        if source_account.currency_definition_id()?
            != destination_account.currency_definition_id()?
        {
            return Err(TransferRequestCommandHandlerError::CurrencyDefinitionMismatch);
        }

        let mut transfer = Transfer::default();
        transfer.request(
            command.from_account_id,
            command.to_account_id,
            command.amount,
        )?;

        self.transfer_repository
            .save(uow, request_context, &mut transfer)
            .await?;

        let transfer_id = transfer
            .aggregate_id()
            .ok_or(TransferRequestCommandHandlerError::MissingTransferId)?;

        Ok(CommandHandled::same(TransferRequestOutput::new(
            transfer_id,
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use appletheia::application::ProjectorDependencies;
    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::projection::ProjectorSpec;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        ActorRef, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{
        Account, AccountBalance, AccountId, AccountName, AccountOwner,
    };
    use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
    use banking_ledger_domain::transfer::{Transfer, TransferId};
    use uuid::Uuid;

    use crate::authorization::AccountTransferRequesterRelation;
    use crate::projection::AccountOwnerRelationshipProjectorSpec;

    use super::{
        TransferRequestCommand, TransferRequestCommandHandler, TransferRequestCommandHandlerError,
    };

    fn account_name() -> AccountName {
        AccountName::try_from("main").expect("account name should be valid")
    }

    fn account_owner() -> AccountOwner {
        AccountOwner::User(UserId::new())
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

    #[derive(Clone, Default)]
    struct TestTransferRepository {
        transfer: Arc<Mutex<Option<Transfer>>>,
    }

    impl Repository<Transfer> for TestTransferRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: TransferId,
        ) -> Result<Option<Transfer>, RepositoryError<Transfer>> {
            Ok(self.transfer.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: TransferId,
            _at: Option<AggregateVersion>,
        ) -> Result<Option<Transfer>, RepositoryError<Transfer>> {
            Ok(self.transfer.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<Transfer>, RepositoryError<Transfer>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Transfer,
        ) -> Result<(), RepositoryError<Transfer>> {
            *self.transfer.lock().expect("lock") = Some(aggregate.clone());
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
            .open(account_owner(), account_name(), currency_definition_id)
            .expect("open should succeed");

        account
    }

    #[test]
    fn authorization_plan_requires_transfer_requester_relationship() {
        let account_repository = TestAccountRepository::default();
        let transfer_repository = TestTransferRepository::default();
        let handler = TransferRequestCommandHandler::new(account_repository, transfer_repository);

        let command = TransferRequestCommand {
            from_account_id: AccountId::new(),
            to_account_id: AccountId::new(),
            amount: AccountBalance::new(10),
        };

        let plan = handler
            .authorization_plan(&command)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::Check {
                        aggregate: AggregateRef::from_id::<Account>(command.from_account_id),
                        relation: AccountTransferRequesterRelation::NAME,
                    },
                    projector_dependencies: ProjectorDependencies::Some(&[
                        AccountOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_rejects_different_currency_definitions() {
        let account_repository = TestAccountRepository::default();
        let source = opened_account(CurrencyDefinitionId::new());
        let destination = opened_account(CurrencyDefinitionId::new());
        let source_account_id = source.aggregate_id().expect("account id should exist");
        let destination_account_id = destination.aggregate_id().expect("account id should exist");
        account_repository.insert(source);
        account_repository.insert(destination);

        let transfer_repository = TestTransferRepository::default();
        let handler = TransferRequestCommandHandler::new(account_repository, transfer_repository);
        let mut uow = TestUow;

        let error = handler
            .handle(
                &mut uow,
                &request_context(),
                &TransferRequestCommand {
                    from_account_id: source_account_id,
                    to_account_id: destination_account_id,
                    amount: AccountBalance::new(10),
                },
            )
            .await
            .expect_err("different currency definitions should fail");

        assert!(matches!(
            error,
            TransferRequestCommandHandlerError::CurrencyDefinitionMismatch
        ));
    }
}

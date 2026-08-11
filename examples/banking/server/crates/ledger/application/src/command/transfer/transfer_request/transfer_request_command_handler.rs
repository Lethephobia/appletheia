use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::transfer::{
    Transfer, TransferRequest, TransferRequestRejectionReason, TransferRequestResult,
};

use crate::authorization::AccountTransferRequesterRelation;

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
    type Error = TransferRequestCommandHandlerError;
    type Uow = TR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.from_account_id,
                AccountTransferRequesterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let source_account = self
            .account_repository
            .read(uow, command.from_account_id)
            .await?;
        let destination_account = self
            .account_repository
            .read(uow, command.to_account_id)
            .await?;

        let mut transfer = Transfer::new();
        let transfer_id = transfer.aggregate_id();
        let request = TransferRequest {
            from_account_id: command.from_account_id,
            to_account_id: command.to_account_id,
            amount: command.amount,
        };
        if source_account.currency_id()? != destination_account.currency_id()? {
            let reason = TransferRequestRejectionReason::CurrencyMismatch;
            transfer.reject_request(request, reason)?;

            self.transfer_repository
                .save(uow, request_context, &mut transfer)
                .await?;

            return Ok(TransferRequestOutput::Rejected {
                transfer_id,
                reason,
            });
        }

        let result = transfer.request(request)?;

        self.transfer_repository
            .save(uow, request_context, &mut transfer)
            .await?;

        let output = match result {
            TransferRequestResult::Requested => TransferRequestOutput::Requested { transfer_id },
            TransferRequestResult::Rejected { reason } => TransferRequestOutput::Rejected {
                transfer_id,
                reason,
            },
        };

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
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
    use appletheia::domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};

    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{
        Account, AccountId, AccountName, AccountOpening, AccountOwner,
    };
    use banking_ledger_domain::core::CurrencyAmount;
    use banking_ledger_domain::currency::CurrencyId;
    use banking_ledger_domain::transfer::{
        Transfer, TransferEventPayload, TransferId, TransferRequestRejectionReason,
    };
    use uuid::Uuid;

    use crate::authorization::AccountTransferRequesterRelation;

    use super::{TransferRequestCommand, TransferRequestCommandHandler, TransferRequestOutput};

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
            let account_id = account.aggregate_id();
            self.accounts
                .lock()
                .expect("lock")
                .insert(account_id, account);
        }
    }

    impl Repository<Account> for TestAccountRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            id: AccountId,
        ) -> Result<Account, RepositoryError<Account>> {
            self.accounts
                .lock()
                .expect("lock")
                .get(&id)
                .cloned()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Account::TYPE,
                    aggregate_id: id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            id: AccountId,
            _at: AggregateVersion,
        ) -> Result<Account, RepositoryError<Account>> {
            self.accounts
                .lock()
                .expect("lock")
                .get(&id)
                .cloned()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Account::TYPE,
                    aggregate_id: id,
                })
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
            let account_id = aggregate.aggregate_id();
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

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: TransferId,
        ) -> Result<Transfer, RepositoryError<Transfer>> {
            self.transfer
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Transfer::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: TransferId,
            _at: AggregateVersion,
        ) -> Result<Transfer, RepositoryError<Transfer>> {
            self.transfer
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Transfer::TYPE,
                    aggregate_id: _id,
                })
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
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    fn opened_account(currency_id: CurrencyId) -> Account {
        let mut account = Account::new();
        account
            .open(AccountOpening {
                owner: account_owner(),
                name: account_name(),
                currency_id,
            })
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
            amount: CurrencyAmount::new(10),
        };

        let plan = handler
            .authorization_plan(&command)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Account>(
                        command.from_account_id,
                        AccountTransferRequesterRelation::REF
                    )
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_rejects_different_currencies() {
        let account_repository = TestAccountRepository::default();
        let source = opened_account(CurrencyId::new());
        let destination = opened_account(CurrencyId::new());
        let source_account_id = source.aggregate_id();
        let destination_account_id = destination.aggregate_id();
        account_repository.insert(source);
        account_repository.insert(destination);

        let transfer_repository = TestTransferRepository::default();
        let handler =
            TransferRequestCommandHandler::new(account_repository, transfer_repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &TransferRequestCommand {
                    from_account_id: source_account_id,
                    to_account_id: destination_account_id,
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect("different currencies should be rejected");

        let saved = transfer_repository
            .transfer
            .lock()
            .expect("lock")
            .clone()
            .expect("rejected transfer should be saved");
        let reason = TransferRequestRejectionReason::CurrencyMismatch;
        assert_eq!(
            handled,
            TransferRequestOutput::Rejected {
                transfer_id: saved.aggregate_id(),
                reason,
            }
        );
        assert!(matches!(
            saved.uncommitted_events()[0].payload(),
            TransferEventPayload::RequestRejected {
                reason: TransferRequestRejectionReason::CurrencyMismatch,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn handle_returns_transfer_id_for_requested_transfer() {
        let currency_id = CurrencyId::new();
        let account_repository = TestAccountRepository::default();
        let source = opened_account(currency_id);
        let destination = opened_account(currency_id);
        let source_account_id = source.aggregate_id();
        let destination_account_id = destination.aggregate_id();
        account_repository.insert(source);
        account_repository.insert(destination);

        let transfer_repository = TestTransferRepository::default();
        let handler = TransferRequestCommandHandler::new(
            account_repository.clone(),
            transfer_repository.clone(),
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &TransferRequestCommand {
                    from_account_id: source_account_id,
                    to_account_id: destination_account_id,
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect("matching currencies should succeed");

        let TransferRequestOutput::Requested { transfer_id } = handled else {
            panic!("expected requested output");
        };

        let saved_transfer = transfer_repository
            .transfer
            .lock()
            .expect("lock")
            .clone()
            .expect("transfer should be saved");
        assert_eq!(saved_transfer.aggregate_id(), transfer_id);
    }
}

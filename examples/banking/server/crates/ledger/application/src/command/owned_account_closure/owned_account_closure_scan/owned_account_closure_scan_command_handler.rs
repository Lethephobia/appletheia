use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{ReferenceIndexLookup, Repository};
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountId, AccountOwner, AccountState};
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosureStatus,
};

use super::{
    OwnedAccountClosureScanCommand, OwnedAccountClosureScanCommandHandlerConfig,
    OwnedAccountClosureScanCommandHandlerError, OwnedAccountClosureScanOutput,
};

pub struct OwnedAccountClosureScanCommandHandler<OACR, RIL>
where
    OACR: Repository<OwnedAccountClosure>,
    RIL: ReferenceIndexLookup<Uow = OACR::Uow>,
{
    owned_account_closure_repository: OACR,
    reference_index_lookup: RIL,
    config: OwnedAccountClosureScanCommandHandlerConfig,
}

impl<OACR, RIL> OwnedAccountClosureScanCommandHandler<OACR, RIL>
where
    OACR: Repository<OwnedAccountClosure>,
    RIL: ReferenceIndexLookup<Uow = OACR::Uow>,
{
    pub fn new(
        owned_account_closure_repository: OACR,
        reference_index_lookup: RIL,
        config: OwnedAccountClosureScanCommandHandlerConfig,
    ) -> Self {
        Self {
            owned_account_closure_repository,
            reference_index_lookup,
            config,
        }
    }
}

impl<OACR, RIL> CommandHandler for OwnedAccountClosureScanCommandHandler<OACR, RIL>
where
    OACR: Repository<OwnedAccountClosure>,
    RIL: ReferenceIndexLookup<Uow = OACR::Uow>,
{
    type Command = OwnedAccountClosureScanCommand;
    type Output = OwnedAccountClosureScanOutput;
    type Error = OwnedAccountClosureScanCommandHandlerError;
    type Uow = OACR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut owned_account_closure = self
            .owned_account_closure_repository
            .read(uow, command.owned_account_closure_id)
            .await?;

        if owned_account_closure.status()? == OwnedAccountClosureStatus::Completed {
            return Err(OwnedAccountClosureScanCommandHandlerError::AlreadyCompleted);
        }
        if owned_account_closure.is_scan_completed()? {
            return Err(OwnedAccountClosureScanCommandHandlerError::ScanAlreadyCompleted);
        }
        let owner = owned_account_closure.owner()?;

        let page = match owner {
            AccountOwner::User(user_id) => {
                self.reference_index_lookup
                    .find_source_ids::<AccountId, _>(
                        uow,
                        Account::TYPE,
                        AccountState::OWNER_USER_REF,
                        user_id,
                        owned_account_closure.next_cursor()?,
                        self.config.page_size,
                    )
                    .await?
            }
            AccountOwner::Organization(organization_id) => {
                self.reference_index_lookup
                    .find_source_ids::<AccountId, _>(
                        uow,
                        Account::TYPE,
                        AccountState::OWNER_ORGANIZATION_REF,
                        organization_id,
                        owned_account_closure.next_cursor()?,
                        self.config.page_size,
                    )
                    .await?
            }
        };

        owned_account_closure.record_scanned(page.source_ids, page.next_cursor)?;
        if owned_account_closure.is_ready_to_complete()? {
            owned_account_closure.complete()?;
        }
        self.owned_account_closure_repository
            .save(uow, request_context, &mut owned_account_closure)
            .await?;

        Ok(OwnedAccountClosureScanOutput {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::{
        OwnedAccountClosureFailedRecordCommand, OwnedAccountClosureFailedRecordCommandHandler,
        OwnedAccountClosureSucceededRecordCommand,
        OwnedAccountClosureSucceededRecordCommandHandler,
    };
    use appletheia::application::repository::ReferenceIndexLookupPageSize;
    use appletheia::application::repository::{
        ReferenceIndexLookupError, ReferenceIndexLookupPage, RepositoryError,
    };
    use appletheia::application::request_context::{CorrelationId, MessageId, Principal};
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{
        AggregateId, AggregateType, AggregateVersion, ReferenceKey, UniqueKey, UniqueValue,
    };
    use banking_iam_domain::UserId;
    use banking_ledger_domain::owned_account_closure::{
        OwnedAccountClosureEventPayload, OwnedAccountClosureId, OwnedAccountClosureStatus,
    };
    use core::num::NonZeroU32;
    use std::sync::{Arc, Mutex};

    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestRepository {
        closure: Arc<Mutex<OwnedAccountClosure>>,
        saved_batches: Arc<Mutex<Vec<Vec<OwnedAccountClosureEventPayload>>>>,
    }

    impl Repository<OwnedAccountClosure> for TestRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _: &mut TestUow,
            _: OwnedAccountClosureId,
        ) -> Result<OwnedAccountClosure, RepositoryError<OwnedAccountClosure>> {
            Ok(self.closure.lock().unwrap().clone())
        }

        async fn read_at_version(
            &self,
            _: &mut TestUow,
            _: OwnedAccountClosureId,
            _: AggregateVersion,
        ) -> Result<OwnedAccountClosure, RepositoryError<OwnedAccountClosure>> {
            unreachable!()
        }

        async fn find_by_unique_value(
            &self,
            _: &mut TestUow,
            _: UniqueKey,
            _: &UniqueValue,
        ) -> Result<Option<OwnedAccountClosure>, RepositoryError<OwnedAccountClosure>> {
            unreachable!()
        }

        async fn save(
            &self,
            _: &mut TestUow,
            _: &RequestContext,
            aggregate: &mut OwnedAccountClosure,
        ) -> Result<(), RepositoryError<OwnedAccountClosure>> {
            self.saved_batches.lock().unwrap().push(
                aggregate
                    .uncommitted_events()
                    .iter()
                    .map(|event| event.payload().clone())
                    .collect(),
            );
            aggregate.core_mut().clear_uncommitted_events();
            *self.closure.lock().unwrap() = aggregate.clone();
            Ok(())
        }
    }

    struct TestLookup {
        expected_cursor: Option<AccountId>,
        account_ids: Vec<AccountId>,
        next_cursor: Option<AccountId>,
    }

    impl ReferenceIndexLookup for TestLookup {
        type Uow = TestUow;

        async fn find_source_ids<I: AggregateId, T: AggregateId>(
            &self,
            _: &mut TestUow,
            _: AggregateType,
            _: ReferenceKey,
            _: T,
            cursor: Option<I>,
            limit: ReferenceIndexLookupPageSize,
        ) -> Result<ReferenceIndexLookupPage<I>, ReferenceIndexLookupError> {
            assert_eq!(
                cursor.map(|id| id.value()),
                self.expected_cursor.map(|id| id.value())
            );
            assert_eq!(limit.as_usize(), 2);
            Ok(ReferenceIndexLookupPage::new(
                self.account_ids
                    .iter()
                    .map(|id| I::try_from_uuid(id.value()).unwrap())
                    .collect(),
                self.next_cursor
                    .map(|id| I::try_from_uuid(id.value()).unwrap()),
            ))
        }
    }

    fn repository() -> TestRepository {
        let mut closure = OwnedAccountClosure::new();
        closure.start(AccountOwner::User(UserId::new())).unwrap();
        closure.core_mut().clear_uncommitted_events();
        TestRepository {
            closure: Arc::new(Mutex::new(closure)),
            saved_batches: Arc::new(Mutex::new(vec![])),
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(MessageId::new().value()),
            MessageId::new(),
            Principal::System,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn scan_saves_one_page_event_without_waiting_for_results() {
        let repository = repository();
        let id = repository.closure.lock().unwrap().aggregate_id();
        let mut account_ids = [AccountId::new(), AccountId::new(), AccountId::new()];
        account_ids.sort();
        let [previous, first, second] = account_ids;
        {
            let mut closure = repository.closure.lock().unwrap();
            closure.record_scanned(vec![], Some(previous)).unwrap();
            closure.core_mut().clear_uncommitted_events();
        }
        let handler = OwnedAccountClosureScanCommandHandler::new(
            repository.clone(),
            TestLookup {
                expected_cursor: Some(previous),
                account_ids: vec![first, second],
                next_cursor: Some(second),
            },
            OwnedAccountClosureScanCommandHandlerConfig {
                page_size: ReferenceIndexLookupPageSize::new(NonZeroU32::new(2).unwrap()),
            },
        );
        handler
            .handle(
                &mut TestUow,
                &request_context(),
                &OwnedAccountClosureScanCommand {
                    owned_account_closure_id: id,
                },
            )
            .await
            .unwrap();
        let saved = repository.saved_batches.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert!(matches!(
            saved[0].as_slice(),
            [OwnedAccountClosureEventPayload::Scanned { account_ids, next_cursor }]
                if account_ids == &vec![first, second] && *next_cursor == Some(second)
        ));
        assert_eq!(
            repository.closure.lock().unwrap().status().unwrap(),
            OwnedAccountClosureStatus::InProgress
        );
    }

    #[tokio::test]
    async fn empty_scan_saves_scan_completion_and_workflow_completion_together() {
        let repository = repository();
        let id = repository.closure.lock().unwrap().aggregate_id();
        let handler = OwnedAccountClosureScanCommandHandler::new(
            repository.clone(),
            TestLookup {
                expected_cursor: None,
                account_ids: vec![],
                next_cursor: None,
            },
            OwnedAccountClosureScanCommandHandlerConfig {
                page_size: ReferenceIndexLookupPageSize::new(NonZeroU32::new(2).unwrap()),
            },
        );
        handler
            .handle(
                &mut TestUow,
                &request_context(),
                &OwnedAccountClosureScanCommand {
                    owned_account_closure_id: id,
                },
            )
            .await
            .unwrap();
        let saved = repository.saved_batches.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert!(matches!(
            saved[0].as_slice(),
            [
                OwnedAccountClosureEventPayload::Scanned { next_cursor: None, .. },
                OwnedAccountClosureEventPayload::Completed {
                    succeeded_count, failed_count
                }
            ] if succeeded_count.value() == 0 && failed_count.value() == 0
        ));
    }

    #[tokio::test]
    async fn final_result_handler_saves_result_and_completion_together() {
        for success in [true, false] {
            let repository = repository();
            let account_id = AccountId::new();
            let id = {
                let mut closure = repository.closure.lock().unwrap();
                closure.record_scanned(vec![account_id], None).unwrap();
                closure.core_mut().clear_uncommitted_events();
                closure.aggregate_id()
            };
            if success {
                OwnedAccountClosureSucceededRecordCommandHandler::new(repository.clone())
                    .handle(
                        &mut TestUow,
                        &request_context(),
                        &OwnedAccountClosureSucceededRecordCommand {
                            owned_account_closure_id: id,
                            account_id,
                        },
                    )
                    .await
                    .unwrap();
            } else {
                OwnedAccountClosureFailedRecordCommandHandler::new(repository.clone())
                    .handle(
                        &mut TestUow,
                        &request_context(),
                        &OwnedAccountClosureFailedRecordCommand {
                            owned_account_closure_id: id,
                            account_id,
                        },
                    )
                    .await
                    .unwrap();
            }
            let saved = repository.saved_batches.lock().unwrap();
            assert_eq!(saved.len(), 1);
            assert_eq!(saved[0].len(), 2);
            assert!(
                matches!(saved[0][0], OwnedAccountClosureEventPayload::Succeeded { .. } if success)
                    || matches!(saved[0][0], OwnedAccountClosureEventPayload::Failed { .. } if !success)
            );
            assert!(matches!(
                saved[0][1],
                OwnedAccountClosureEventPayload::Completed { .. }
            ));
        }
    }

    #[tokio::test]
    async fn scan_rejects_finished_states_without_saving() {
        for case in 1..3 {
            let repository = repository();
            let account_id = AccountId::new();
            let id = {
                let mut closure = repository.closure.lock().unwrap();
                match case {
                    1 => {
                        closure.record_scanned(vec![account_id], None).unwrap();
                    }
                    _ => {
                        closure.record_scanned(vec![], None).unwrap();
                        closure.complete().unwrap();
                    }
                }
                closure.core_mut().clear_uncommitted_events();
                closure.aggregate_id()
            };
            let handler = OwnedAccountClosureScanCommandHandler::new(
                repository.clone(),
                TestLookup {
                    expected_cursor: None,
                    account_ids: vec![],
                    next_cursor: None,
                },
                OwnedAccountClosureScanCommandHandlerConfig {
                    page_size: ReferenceIndexLookupPageSize::new(NonZeroU32::new(2).unwrap()),
                },
            );
            let error = handler
                .handle(
                    &mut TestUow,
                    &request_context(),
                    &OwnedAccountClosureScanCommand {
                        owned_account_closure_id: id,
                    },
                )
                .await
                .unwrap_err();
            match case {
                1 => assert!(matches!(
                    error,
                    OwnedAccountClosureScanCommandHandlerError::ScanAlreadyCompleted
                )),
                _ => assert!(matches!(
                    error,
                    OwnedAccountClosureScanCommandHandlerError::AlreadyCompleted
                )),
            }
            assert!(repository.saved_batches.lock().unwrap().is_empty());
        }
    }
}

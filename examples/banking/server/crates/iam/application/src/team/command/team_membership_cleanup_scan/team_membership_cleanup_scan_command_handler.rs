use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{ReferenceIndexLookup, Repository};
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_domain::team_membership::{TeamMembership, TeamMembershipId, TeamMembershipState};
use banking_iam_domain::team_membership_cleanup::{
    TeamMembershipCleanup, TeamMembershipCleanupStatus,
};

use super::{
    TeamMembershipCleanupScanCommand, TeamMembershipCleanupScanCommandHandlerConfig,
    TeamMembershipCleanupScanCommandHandlerError, TeamMembershipCleanupScanOutput,
};

pub struct TeamMembershipCleanupScanCommandHandler<R, RIL>
where
    R: Repository,
    RIL: ReferenceIndexLookup<Uow = R::Uow>,
{
    repository: R,
    reference_index_lookup: RIL,
    config: TeamMembershipCleanupScanCommandHandlerConfig,
}

impl<R, RIL> TeamMembershipCleanupScanCommandHandler<R, RIL>
where
    R: Repository,
    RIL: ReferenceIndexLookup<Uow = R::Uow>,
{
    pub fn new(
        repository: R,
        reference_index_lookup: RIL,
        config: TeamMembershipCleanupScanCommandHandlerConfig,
    ) -> Self {
        Self {
            repository,
            reference_index_lookup,
            config,
        }
    }
}

impl<R, RIL> CommandHandler for TeamMembershipCleanupScanCommandHandler<R, RIL>
where
    R: Repository,
    RIL: ReferenceIndexLookup<Uow = R::Uow>,
{
    type Command = TeamMembershipCleanupScanCommand;
    type Output = TeamMembershipCleanupScanOutput;
    type Error = TeamMembershipCleanupScanCommandHandlerError;
    type Uow = R::Uow;

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
        let mut team_membership_cleanup = self
            .repository
            .read::<TeamMembershipCleanup>(uow, command.team_membership_cleanup_id)
            .await?;

        if team_membership_cleanup.status()? == TeamMembershipCleanupStatus::Completed {
            return Err(TeamMembershipCleanupScanCommandHandlerError::AlreadyCompleted);
        }
        if team_membership_cleanup.is_scan_completed()? {
            return Err(TeamMembershipCleanupScanCommandHandlerError::ScanAlreadyCompleted);
        }
        let page = self
            .reference_index_lookup
            .find_source_ids::<TeamMembershipId, _>(
                uow,
                TeamMembership::TYPE,
                TeamMembershipState::ORGANIZATION_MEMBERSHIP_REF,
                team_membership_cleanup.organization_membership_id()?,
                team_membership_cleanup.next_cursor()?,
                self.config.page_size,
            )
            .await?;
        let mut team_membership_ids = Vec::new();
        for team_membership_id in page.source_ids {
            let membership = self
                .repository
                .read_shared::<TeamMembership>(uow, team_membership_id)
                .await?;
            if membership.is_active()? {
                team_membership_ids.push(team_membership_id);
            }
        }

        team_membership_cleanup.record_scanned(team_membership_ids, page.next_cursor)?;
        if team_membership_cleanup.is_ready_to_complete()? {
            team_membership_cleanup.complete()?;
        }
        self.repository
            .save(uow, request_context, &mut team_membership_cleanup)
            .await?;

        Ok(TeamMembershipCleanupScanOutput {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::command::{
        TeamMembershipCleanupMembershipFailedRecordCommand,
        TeamMembershipCleanupMembershipFailedRecordCommandHandler,
        TeamMembershipCleanupMembershipSucceededRecordCommand,
        TeamMembershipCleanupMembershipSucceededRecordCommandHandler,
    };
    use appletheia::application::aggregate::SerializedAggregate;
    use appletheia::application::repository::ReferenceIndexLookupPageSize;
    use appletheia::application::repository::{
        ReferenceIndexLookupError, ReferenceIndexLookupPage, RepositoryError,
    };
    use appletheia::application::request_context::{CorrelationId, MessageId, Principal};
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Event;
    use appletheia::domain::{
        AggregateId, AggregateType, AggregateVersion, ReferenceKey, UniqueKey, UniqueValue,
    };
    use banking_iam_domain::team_membership_cleanup::{
        TeamMembershipCleanupEventPayload, TeamMembershipCleanupStatus,
    };
    use banking_iam_domain::{OrganizationMembershipId, TeamId};
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
        cleanup: Arc<Mutex<TeamMembershipCleanup>>,
        deleted_ids: Vec<TeamMembershipId>,
        saved_batches: Arc<Mutex<Vec<Vec<TeamMembershipCleanupEventPayload>>>>,
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
    }

    impl Repository for TestRepository {
        type Uow = TestUow;

        async fn read<A: Aggregate>(
            &self,
            _: &mut TestUow,
            _: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            Ok(Self::copy_aggregate::<_, A>(&*self.cleanup.lock().unwrap()))
        }

        async fn read_shared<A: Aggregate>(
            &self,
            _: &mut TestUow,
            id: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            let mut membership = TeamMembership::new();
            membership
                .create(TeamId::new(), OrganizationMembershipId::new())
                .unwrap();
            if self
                .deleted_ids
                .iter()
                .any(|deleted| deleted.value() == id.value())
            {
                membership.delete().unwrap();
            }
            Ok(Self::copy_aggregate::<_, A>(&membership))
        }

        async fn read_at_version<A: Aggregate>(
            &self,
            _: &mut TestUow,
            _: A::Id,
            _: AggregateVersion,
        ) -> Result<A, RepositoryError<A>> {
            unreachable!()
        }

        async fn find_by_unique_value<A: Aggregate>(
            &self,
            _: &mut TestUow,
            _: UniqueKey,
            _: &UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            unreachable!()
        }

        async fn find_shared_by_unique_value<A: Aggregate>(
            &self,
            _: &mut TestUow,
            _: UniqueKey,
            _: &UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            unreachable!()
        }

        async fn save<A: Aggregate>(
            &self,
            _: &mut TestUow,
            _: &RequestContext,
            aggregate: &mut A,
        ) -> Result<(), RepositoryError<A>> {
            self.saved_batches.lock().unwrap().push(
                aggregate
                    .uncommitted_events()
                    .iter()
                    .map(|event| {
                        serde_json::from_value(serde_json::to_value(event.payload()).unwrap())
                            .unwrap()
                    })
                    .collect(),
            );
            aggregate.core_mut().clear_uncommitted_events();
            let stored = Self::copy_aggregate::<_, TeamMembershipCleanup>(aggregate);
            *self.cleanup.lock().unwrap() = stored;
            Ok(())
        }
    }

    struct TestLookup {
        expected_cursor: Option<TeamMembershipId>,
        team_membership_ids: Vec<TeamMembershipId>,
        next_cursor: Option<TeamMembershipId>,
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
                self.team_membership_ids
                    .iter()
                    .map(|id| I::try_from_uuid(id.value()).unwrap())
                    .collect(),
                self.next_cursor
                    .map(|id| I::try_from_uuid(id.value()).unwrap()),
            ))
        }
    }

    fn repository() -> TestRepository {
        let mut cleanup = TeamMembershipCleanup::new();
        cleanup.start(OrganizationMembershipId::new()).unwrap();
        cleanup.core_mut().clear_uncommitted_events();
        TestRepository {
            cleanup: Arc::new(Mutex::new(cleanup)),
            deleted_ids: vec![],
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
        let id = repository.cleanup.lock().unwrap().aggregate_id();
        let mut team_membership_ids = [
            TeamMembershipId::new(),
            TeamMembershipId::new(),
            TeamMembershipId::new(),
        ];
        team_membership_ids.sort();
        let [previous, first, second] = team_membership_ids;
        {
            let mut cleanup = repository.cleanup.lock().unwrap();
            cleanup.record_scanned(vec![], Some(previous)).unwrap();
            cleanup.core_mut().clear_uncommitted_events();
        }
        let handler = TeamMembershipCleanupScanCommandHandler::new(
            repository.clone(),
            TestLookup {
                expected_cursor: Some(previous),
                team_membership_ids: vec![first, second],
                next_cursor: Some(second),
            },
            TeamMembershipCleanupScanCommandHandlerConfig {
                page_size: ReferenceIndexLookupPageSize::new(NonZeroU32::new(2).unwrap()),
            },
        );
        handler
            .handle(
                &mut TestUow,
                &request_context(),
                &TeamMembershipCleanupScanCommand {
                    team_membership_cleanup_id: id,
                },
            )
            .await
            .unwrap();
        let saved = repository.saved_batches.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert!(matches!(
            saved[0].as_slice(),
            [TeamMembershipCleanupEventPayload::Scanned { team_membership_ids, next_cursor }]
                if team_membership_ids == &vec![first, second] && *next_cursor == Some(second)
        ));
        assert_eq!(
            repository.cleanup.lock().unwrap().status().unwrap(),
            TeamMembershipCleanupStatus::InProgress
        );
    }

    #[tokio::test]
    async fn empty_scan_saves_scan_completion_and_workflow_completion_together() {
        let repository = repository();
        let id = repository.cleanup.lock().unwrap().aggregate_id();
        let handler = TeamMembershipCleanupScanCommandHandler::new(
            repository.clone(),
            TestLookup {
                expected_cursor: None,
                team_membership_ids: vec![],
                next_cursor: None,
            },
            TeamMembershipCleanupScanCommandHandlerConfig {
                page_size: ReferenceIndexLookupPageSize::new(NonZeroU32::new(2).unwrap()),
            },
        );
        handler
            .handle(
                &mut TestUow,
                &request_context(),
                &TeamMembershipCleanupScanCommand {
                    team_membership_cleanup_id: id,
                },
            )
            .await
            .unwrap();
        let saved = repository.saved_batches.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert!(matches!(
            saved[0].as_slice(),
            [
                TeamMembershipCleanupEventPayload::Scanned { next_cursor: None, .. },
                TeamMembershipCleanupEventPayload::Completed {
                    succeeded_count, failed_count
                }
            ] if succeeded_count.value() == 0 && failed_count.value() == 0
        ));
    }

    #[tokio::test]
    async fn final_result_handler_saves_result_and_completion_together() {
        for success in [true, false] {
            let repository = repository();
            let team_membership_id = TeamMembershipId::new();
            let id = {
                let mut cleanup = repository.cleanup.lock().unwrap();
                cleanup
                    .record_scanned(vec![team_membership_id], None)
                    .unwrap();
                cleanup.core_mut().clear_uncommitted_events();
                cleanup.aggregate_id()
            };
            if success {
                TeamMembershipCleanupMembershipSucceededRecordCommandHandler::new(
                    repository.clone(),
                )
                .handle(
                    &mut TestUow,
                    &request_context(),
                    &TeamMembershipCleanupMembershipSucceededRecordCommand {
                        team_membership_cleanup_id: id,
                        team_membership_id,
                    },
                )
                .await
                .unwrap();
            } else {
                TeamMembershipCleanupMembershipFailedRecordCommandHandler::new(repository.clone())
                    .handle(
                        &mut TestUow,
                        &request_context(),
                        &TeamMembershipCleanupMembershipFailedRecordCommand {
                            team_membership_cleanup_id: id,
                            team_membership_id,
                        },
                    )
                    .await
                    .unwrap();
            }
            let saved = repository.saved_batches.lock().unwrap();
            assert_eq!(saved.len(), 1);
            assert_eq!(saved[0].len(), 2);
            assert!(
                matches!(saved[0][0], TeamMembershipCleanupEventPayload::MembershipSucceeded { .. } if success)
                    || matches!(saved[0][0], TeamMembershipCleanupEventPayload::MembershipFailed { .. } if !success)
            );
            assert!(matches!(
                saved[0][1],
                TeamMembershipCleanupEventPayload::Completed { .. }
            ));
        }
    }

    #[tokio::test]
    async fn scan_rejects_finished_states_without_saving() {
        for case in 1..3 {
            let repository = repository();
            let team_membership_id = TeamMembershipId::new();
            let id = {
                let mut cleanup = repository.cleanup.lock().unwrap();
                match case {
                    1 => {
                        cleanup
                            .record_scanned(vec![team_membership_id], None)
                            .unwrap();
                    }
                    _ => {
                        cleanup.record_scanned(vec![], None).unwrap();
                        cleanup.complete().unwrap();
                    }
                }
                cleanup.core_mut().clear_uncommitted_events();
                cleanup.aggregate_id()
            };
            let handler = TeamMembershipCleanupScanCommandHandler::new(
                repository.clone(),
                TestLookup {
                    expected_cursor: None,
                    team_membership_ids: vec![],
                    next_cursor: None,
                },
                TeamMembershipCleanupScanCommandHandlerConfig {
                    page_size: ReferenceIndexLookupPageSize::new(NonZeroU32::new(2).unwrap()),
                },
            );
            let error = handler
                .handle(
                    &mut TestUow,
                    &request_context(),
                    &TeamMembershipCleanupScanCommand {
                        team_membership_cleanup_id: id,
                    },
                )
                .await
                .unwrap_err();
            match case {
                1 => assert!(matches!(
                    error,
                    TeamMembershipCleanupScanCommandHandlerError::ScanAlreadyCompleted
                )),
                _ => assert!(matches!(
                    error,
                    TeamMembershipCleanupScanCommandHandlerError::AlreadyCompleted
                )),
            }
            assert!(repository.saved_batches.lock().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn scan_filters_deleted_memberships_without_losing_page_cursor() {
        for all_deleted in [false, true] {
            let mut repository = repository();
            let id = repository.cleanup.lock().unwrap().aggregate_id();
            let first = TeamMembershipId::new();
            let last = TeamMembershipId::new();
            repository.deleted_ids = if all_deleted {
                vec![first, last]
            } else {
                vec![last]
            };
            let handler = TeamMembershipCleanupScanCommandHandler::new(
                repository.clone(),
                TestLookup {
                    expected_cursor: None,
                    team_membership_ids: vec![first, last],
                    next_cursor: Some(last),
                },
                TeamMembershipCleanupScanCommandHandlerConfig {
                    page_size: ReferenceIndexLookupPageSize::new(NonZeroU32::new(2).unwrap()),
                },
            );
            handler
                .handle(
                    &mut TestUow,
                    &request_context(),
                    &TeamMembershipCleanupScanCommand {
                        team_membership_cleanup_id: id,
                    },
                )
                .await
                .unwrap();
            let saved = repository.saved_batches.lock().unwrap();
            let expected = if all_deleted { vec![] } else { vec![first] };
            assert!(
                matches!(saved[0].as_slice(), [TeamMembershipCleanupEventPayload::Scanned { team_membership_ids, next_cursor }] if team_membership_ids == &expected && *next_cursor == Some(last))
            );
            let cleanup = repository.cleanup.lock().unwrap();
            assert!(!cleanup.is_scan_completed().unwrap());
            assert!(!cleanup.is_ready_to_complete().unwrap());
        }
    }
}

use std::time::Duration as StdDuration;

use tokio::time::Instant;

use appletheia_domain::EventId;

use crate::event::EventLookup;
use crate::request_context::{CausationId, MessageId};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    ProjectorDependencies, ProjectorNameOwned, ProjectorProcessedEventStore,
    ReadYourWritesPollInterval, ReadYourWritesTimeout, ReadYourWritesWaitError,
    ReadYourWritesWaiter,
};

pub struct DefaultReadYourWritesWaiter<U, L, P>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventLookup<Uow = U::Uow>,
    P: ProjectorProcessedEventStore<Uow = U::Uow>,
{
    uow_factory: U,
    lookup: L,
    processed_event_store: P,
}

impl<U, L, P> DefaultReadYourWritesWaiter<U, L, P>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventLookup<Uow = U::Uow>,
    P: ProjectorProcessedEventStore<Uow = U::Uow>,
{
    pub fn new(uow_factory: U, lookup: L, processed_event_store: P) -> Self {
        Self {
            uow_factory,
            lookup,
            processed_event_store,
        }
    }
}

impl<U, L, P> ReadYourWritesWaiter for DefaultReadYourWritesWaiter<U, L, P>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventLookup<Uow = U::Uow>,
    P: ProjectorProcessedEventStore<Uow = U::Uow>,
{
    async fn wait(
        &self,
        after: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
        projector_dependencies: ProjectorDependencies<'_>,
    ) -> Result<(), ReadYourWritesWaitError> {
        if projector_dependencies.as_slice().is_empty() {
            return Ok(());
        }

        let causation_events = {
            let mut uow = self.uow_factory.begin().await?;
            let causation_id = CausationId::from(after);
            let events = self
                .lookup
                .events_by_causation_id(&mut uow, causation_id)
                .await;
            let events = match events {
                Ok(value) => value,
                Err(operation_error) => {
                    let operation_error =
                        uow.rollback_with_operation_error(operation_error).await?;
                    return Err(operation_error.into());
                }
            };
            uow.commit().await?;
            if events.is_empty() {
                return Err(ReadYourWritesWaitError::UnknownMessageId { message_id: after });
            }
            events
        };
        let projector_targets: Vec<(ProjectorNameOwned, Vec<EventId>)> = projector_dependencies
            .as_slice()
            .iter()
            .filter_map(|descriptor| {
                let relevant_event_ids: Vec<EventId> = causation_events
                    .iter()
                    .filter(|event| descriptor.subscription.matches(*event))
                    .map(|event| event.event_id)
                    .collect();
                if relevant_event_ids.is_empty() {
                    return None;
                }

                Some((
                    ProjectorNameOwned::from(descriptor.name),
                    relevant_event_ids,
                ))
            })
            .collect();

        if projector_targets.is_empty() {
            return Ok(());
        }

        let deadline = Instant::now() + timeout.value();
        let poll_duration = poll_interval.value();

        loop {
            let mut pending: Vec<ProjectorNameOwned> = Vec::new();

            for (projector_name, relevant_event_ids) in &projector_targets {
                let all_processed = {
                    let mut uow = self.uow_factory.begin().await?;
                    let all_processed = self
                        .processed_event_store
                        .are_all_processed(&mut uow, projector_name.clone(), relevant_event_ids)
                        .await;
                    let all_processed = match all_processed {
                        Ok(value) => value,
                        Err(operation_error) => {
                            let operation_error =
                                uow.rollback_with_operation_error(operation_error).await?;
                            return Err(operation_error.into());
                        }
                    };

                    uow.commit().await?;
                    all_processed
                };

                if !all_processed {
                    pending.push(projector_name.clone());
                }
            }

            if pending.is_empty() {
                return Ok(());
            }

            let now = Instant::now();
            if now >= deadline {
                return Err(ReadYourWritesWaitError::Timeout {
                    after,
                    pending,
                    timeout,
                });
            }

            let remaining = deadline
                .checked_duration_since(now)
                .unwrap_or(StdDuration::ZERO);
            let sleep_duration = poll_duration.min(remaining);

            if sleep_duration > StdDuration::ZERO {
                tokio::time::sleep(sleep_duration).await;
            } else {
                tokio::task::yield_now().await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use serde_json::json;
    use uuid::Uuid;

    use appletheia_domain::{AggregateType, AggregateVersion, EventId, EventName, EventOccurredAt};

    use super::*;
    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventLookupError, EventNameOwned,
        EventSelector, EventSequence, SerializedEventPayload,
    };
    use crate::messaging::Subscription;
    use crate::projection::{ProjectorDependencies, ProjectorDescriptor, ProjectorName};
    use crate::request_context::{ActorRef, CorrelationId, Principal, RequestContext};
    use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

    const REGISTERED_SELECTOR: EventSelector =
        EventSelector::new(AggregateType::new("user"), EventName::new("registered"));
    const PROFILE_READIED_SELECTOR: EventSelector = EventSelector::new(
        AggregateType::new("user"),
        EventName::new("profile_readied"),
    );
    const REGISTERED_PROJECTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("registered_projector"),
        Subscription::Only(&[REGISTERED_SELECTOR]),
    );
    const PROFILE_PROJECTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("profile_projector"),
        Subscription::Only(&[PROFILE_READIED_SELECTOR]),
    );

    #[derive(Default)]
    struct TestUnitOfWork;

    impl UnitOfWork for TestUnitOfWork {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestUnitOfWorkFactory;

    impl UnitOfWorkFactory for TestUnitOfWorkFactory {
        type Uow = TestUnitOfWork;

        async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError> {
            Ok(TestUnitOfWork)
        }
    }

    #[derive(Clone)]
    struct TestEventLookup {
        events: Arc<Vec<EventEnvelope>>,
    }

    impl TestEventLookup {
        fn new(events: Vec<EventEnvelope>) -> Self {
            Self {
                events: Arc::new(events),
            }
        }
    }

    impl EventLookup for TestEventLookup {
        type Uow = TestUnitOfWork;

        async fn max_event_sequence_by_causation_id(
            &self,
            _uow: &mut Self::Uow,
            _causation_id: CausationId,
        ) -> Result<Option<EventSequence>, EventLookupError> {
            Ok(self.events.last().map(|event| event.event_sequence))
        }

        async fn last_event_id_by_causation_id(
            &self,
            _uow: &mut Self::Uow,
            _causation_id: CausationId,
        ) -> Result<Option<EventId>, EventLookupError> {
            Ok(self.events.last().map(|event| event.event_id))
        }

        async fn events_by_causation_id(
            &self,
            _uow: &mut Self::Uow,
            _causation_id: CausationId,
        ) -> Result<Vec<EventEnvelope>, EventLookupError> {
            Ok(self.events.as_ref().clone())
        }
    }

    #[derive(Clone, Default)]
    struct TestProjectorProcessedEventStore {
        processed: Arc<Mutex<HashSet<(String, EventId)>>>,
    }

    impl TestProjectorProcessedEventStore {
        fn with_processed(
            entries: impl IntoIterator<Item = (ProjectorNameOwned, EventId)>,
        ) -> Self {
            let processed = entries
                .into_iter()
                .map(|(projector_name, event_id)| (projector_name.value().to_string(), event_id))
                .collect();

            Self {
                processed: Arc::new(Mutex::new(processed)),
            }
        }
    }

    impl ProjectorProcessedEventStore for TestProjectorProcessedEventStore {
        type Uow = TestUnitOfWork;

        async fn are_all_processed(
            &self,
            _uow: &mut Self::Uow,
            projector_name: ProjectorNameOwned,
            event_ids: &[EventId],
        ) -> Result<bool, crate::projection::ProjectorProcessedEventStoreError> {
            let processed = self.processed.lock().expect("lock should succeed");
            let projector_name = projector_name.value().to_string();

            Ok(event_ids
                .iter()
                .all(|event_id| processed.contains(&(projector_name.clone(), *event_id))))
        }

        async fn is_processed(
            &self,
            _uow: &mut Self::Uow,
            projector_name: ProjectorNameOwned,
            event_id: EventId,
        ) -> Result<bool, crate::projection::ProjectorProcessedEventStoreError> {
            let processed = self.processed.lock().expect("lock should succeed");

            Ok(processed.contains(&(projector_name.value().to_string(), event_id)))
        }

        async fn mark_processed(
            &self,
            _uow: &mut Self::Uow,
            projector_name: ProjectorNameOwned,
            event_id: EventId,
        ) -> Result<bool, crate::projection::ProjectorProcessedEventStoreError> {
            let mut processed = self.processed.lock().expect("lock should succeed");

            Ok(processed.insert((projector_name.value().to_string(), event_id)))
        }

        async fn reset(
            &self,
            _uow: &mut Self::Uow,
            projector_name: ProjectorNameOwned,
        ) -> Result<(), crate::projection::ProjectorProcessedEventStoreError> {
            let mut processed = self.processed.lock().expect("lock should succeed");
            let projector_name = projector_name.value().to_string();

            processed.retain(|(stored_name, _)| stored_name != &projector_name);
            Ok(())
        }
    }

    fn test_waiter(
        events: Vec<EventEnvelope>,
        processed: impl IntoIterator<Item = (ProjectorNameOwned, EventId)>,
    ) -> DefaultReadYourWritesWaiter<
        TestUnitOfWorkFactory,
        TestEventLookup,
        TestProjectorProcessedEventStore,
    > {
        DefaultReadYourWritesWaiter::new(
            TestUnitOfWorkFactory,
            TestEventLookup::new(events),
            TestProjectorProcessedEventStore::with_processed(processed),
        )
    }

    fn event_envelope(
        event_sequence: i64,
        event_id: EventId,
        event_name: EventName,
        message_id: MessageId,
    ) -> EventEnvelope {
        EventEnvelope {
            event_sequence: EventSequence::try_from(event_sequence).expect("event sequence"),
            event_id,
            aggregate_type: AggregateTypeOwned::from(AggregateType::new("user")),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(event_sequence)
                .expect("aggregate version"),
            event_name: EventNameOwned::from(event_name),
            payload: SerializedEventPayload::try_from(json!({ "event": event_name.value() }))
                .expect("payload"),
            occurred_at: EventOccurredAt::now(),
            correlation_id: CorrelationId::from(Uuid::now_v7()),
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(
                CorrelationId::from(Uuid::now_v7()),
                message_id,
                ActorRef::System,
                Principal::System,
            ),
        }
    }

    #[tokio::test]
    async fn wait_returns_unknown_message_id_when_no_event_exists() {
        let message_id = MessageId::from(Uuid::now_v7());
        let waiter = test_waiter(Vec::new(), Vec::new());

        let result = waiter
            .wait(
                message_id,
                ReadYourWritesTimeout::from(Duration::ZERO),
                ReadYourWritesPollInterval::from(Duration::ZERO),
                ProjectorDependencies::Some(&[REGISTERED_PROJECTOR]),
            )
            .await;

        assert!(matches!(
            result,
            Err(ReadYourWritesWaitError::UnknownMessageId {
                message_id: returned
            }) if returned == message_id
        ));
    }

    #[tokio::test]
    async fn wait_ignores_dependencies_without_relevant_events() {
        let message_id = MessageId::from(Uuid::now_v7());
        let profile_readied_event_id = EventId::try_from(Uuid::now_v7()).expect("event id");
        let waiter = test_waiter(
            vec![event_envelope(
                1,
                profile_readied_event_id,
                EventName::new("profile_readied"),
                message_id,
            )],
            Vec::new(),
        );

        let result = waiter
            .wait(
                message_id,
                ReadYourWritesTimeout::from(Duration::ZERO),
                ReadYourWritesPollInterval::from(Duration::ZERO),
                ProjectorDependencies::Some(&[REGISTERED_PROJECTOR]),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn wait_times_out_when_relevant_events_are_not_processed() {
        let message_id = MessageId::from(Uuid::now_v7());
        let registered_event_id = EventId::try_from(Uuid::now_v7()).expect("event id");
        let waiter = test_waiter(
            vec![event_envelope(
                1,
                registered_event_id,
                EventName::new("registered"),
                message_id,
            )],
            Vec::new(),
        );

        let result = waiter
            .wait(
                message_id,
                ReadYourWritesTimeout::from(Duration::ZERO),
                ReadYourWritesPollInterval::from(Duration::ZERO),
                ProjectorDependencies::Some(&[REGISTERED_PROJECTOR]),
            )
            .await;

        assert!(matches!(
            result,
            Err(ReadYourWritesWaitError::Timeout {
                after,
                pending,
                ..
            }) if after == message_id
                && pending == vec![ProjectorNameOwned::from(REGISTERED_PROJECTOR.name)]
        ));
    }

    #[tokio::test]
    async fn wait_succeeds_when_all_relevant_events_are_processed() {
        let message_id = MessageId::from(Uuid::now_v7());
        let registered_event_id = EventId::try_from(Uuid::now_v7()).expect("event id");
        let profile_readied_event_id = EventId::try_from(Uuid::now_v7()).expect("event id");
        let waiter = test_waiter(
            vec![
                event_envelope(
                    1,
                    registered_event_id,
                    EventName::new("registered"),
                    message_id,
                ),
                event_envelope(
                    2,
                    profile_readied_event_id,
                    EventName::new("profile_readied"),
                    message_id,
                ),
            ],
            vec![
                (
                    ProjectorNameOwned::from(REGISTERED_PROJECTOR.name),
                    registered_event_id,
                ),
                (
                    ProjectorNameOwned::from(PROFILE_PROJECTOR.name),
                    profile_readied_event_id,
                ),
            ],
        );

        let result = waiter
            .wait(
                message_id,
                ReadYourWritesTimeout::from(Duration::from_millis(10)),
                ReadYourWritesPollInterval::from(Duration::ZERO),
                ProjectorDependencies::Some(&[REGISTERED_PROJECTOR, PROFILE_PROJECTOR]),
            )
            .await;

        assert!(result.is_ok());
    }
}

use std::error::Error;

use super::{ProjectorDefinitionError, ProjectorName, ProjectorRoute};
use crate::{
    event::{EventEnvelope, EventSelector},
    unit_of_work::UnitOfWork,
};

/// Holds validated routes; execution and transaction management belong to the runner.
pub struct ProjectorDefinition<'a, U: UnitOfWork, E: Error + Send + Sync + 'static> {
    name: ProjectorName,
    routes: Vec<ProjectorRoute<'a, U, E>>,
}

impl<'a, U: UnitOfWork, E: Error + Send + Sync + 'static> ProjectorDefinition<'a, U, E> {
    pub fn new<R>(name: ProjectorName, routes: R) -> Result<Self, ProjectorDefinitionError>
    where
        R: IntoIterator<Item = ProjectorRoute<'a, U, E>>,
    {
        let collected_routes: Vec<_> = routes.into_iter().collect();
        for (index, route) in collected_routes.iter().enumerate() {
            if collected_routes[..index]
                .iter()
                .any(|other| other.selector == route.selector)
            {
                return Err(ProjectorDefinitionError::DuplicateEventRoute {
                    selector: route.selector,
                });
            }
        }
        Ok(Self {
            name,
            routes: collected_routes,
        })
    }

    pub fn name(&self) -> ProjectorName {
        self.name
    }

    pub fn event_selectors(&self) -> Vec<EventSelector> {
        self.routes.iter().map(|route| route.selector).collect()
    }

    pub fn find_event_route(&self, event: &EventEnvelope) -> Option<&ProjectorRoute<'a, U, E>> {
        self.routes
            .iter()
            .find(|route| route.selector.matches(event))
    }
}

#[cfg(test)]
mod tests {
    mod counter {
        use appletheia_domain::{
            Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
            AggregateStateError, AggregateType, EventName, EventPayload, ReferenceIndexes,
            UniqueConstraints,
        };
        use serde::{Deserialize, Serialize};
        use std::fmt::{self, Display};
        use thiserror::Error;
        use uuid::Uuid;

        #[derive(Debug, Error)]
        pub(super) enum CounterIdError {
            #[error("nil uuid is not allowed")]
            NilUuid,
        }

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub(super) struct CounterId(Uuid);

        impl AggregateId for CounterId {
            type Error = CounterIdError;

            fn new() -> Self {
                Self(Uuid::now_v7())
            }

            fn value(&self) -> Uuid {
                self.0
            }

            fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
                if value.is_nil() {
                    return Err(CounterIdError::NilUuid);
                }

                Ok(Self(value))
            }
        }

        impl Display for CounterId {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterStateError {
            #[error(transparent)]
            AggregateState(#[from] AggregateStateError),
        }

        #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub(super) struct CounterState {
            id: CounterId,
        }

        impl UniqueConstraints<CounterStateError> for CounterState {}

        impl ReferenceIndexes<CounterStateError> for CounterState {}

        impl AggregateState for CounterState {
            type Error = CounterStateError;
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterEventPayloadError {
            #[error(transparent)]
            Serde(#[from] serde_json::Error),
        }

        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(tag = "type", content = "data", rename_all = "snake_case")]
        pub(super) enum CounterEventPayload {
            Opened,
        }

        impl EventPayload for CounterEventPayload {
            type Error = CounterEventPayloadError;

            fn name(&self) -> EventName {
                match self {
                    Self::Opened => EventName::new("opened"),
                }
            }
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterError {
            #[error(transparent)]
            Aggregate(#[from] AggregateError<CounterId>),

            #[error(transparent)]
            State(#[from] CounterStateError),
        }

        #[derive(Clone, Debug, Default)]
        pub(super) struct Counter {
            core: AggregateCore<CounterId, CounterState, CounterEventPayload>,
        }

        impl AggregateApply<CounterEventPayload, CounterError> for Counter {
            fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
                match payload {
                    CounterEventPayload::Opened => {
                        self.set_state(Some(CounterState {
                            id: CounterId::try_from_uuid(Uuid::now_v7())
                                .expect("generated uuid should be valid"),
                        }));
                    }
                }

                Ok(())
            }
        }

        impl Aggregate for Counter {
            type Id = CounterId;
            type State = CounterState;
            type EventPayload = CounterEventPayload;
            type Error = CounterError;

            const TYPE: AggregateType = AggregateType::new("counter");

            fn new() -> Self {
                Self {
                    core: AggregateCore::new(),
                }
            }

            fn from_id(id: Self::Id) -> Self {
                Self {
                    core: AggregateCore::from_id(id),
                }
            }

            fn core(&self) -> &AggregateCore<Self::Id, Self::State, Self::EventPayload> {
                &self.core
            }

            fn core_mut(
                &mut self,
            ) -> &mut AggregateCore<Self::Id, Self::State, Self::EventPayload> {
                &mut self.core
            }
        }
    }

    use super::super::*;
    use super::*;
    use crate::event::{EventFeedBatchSize, EventFeedReader, EventFeedReaderError};
    use crate::messaging::Subscription;
    use crate::messaging::{
        Consumer, ConsumerError, ConsumerGroup, Delivery, DeliveryError, Subscriber,
        SubscriberError,
    };
    use crate::{
        aggregate::{AggregateIdValue, AggregateTypeOwned},
        event::{EventEnvelopeError, EventNameOwned, EventSequence, SerializedEventPayload},
        request_context::{CausationId, CorrelationId, MessageId, Principal, RequestContext},
        unit_of_work::{UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError},
    };
    use appletheia_domain::{
        Aggregate, AggregateVersion, EventId, EventName, EventOccurredAt, EventPayload,
    };
    use counter::{Counter, CounterEventPayload};
    use std::num::NonZeroU32;
    use std::{
        collections::HashSet,
        sync::atomic::{AtomicUsize, Ordering},
        sync::{Arc, Mutex},
    };
    use uuid::Uuid;

    #[derive(Debug, thiserror::Error)]
    enum HandlerError {
        #[error(transparent)]
        EventEnvelope(#[from] EventEnvelopeError),

        #[error("write rejected")]
        Rejected,
    }

    #[derive(Clone, Default)]
    struct Database {
        processed: HashSet<(ProjectorNameOwned, EventId)>,
        writes: usize,
        checkpoint: Option<EventSequence>,
    }

    struct Transaction {
        database: Arc<Mutex<Database>>,
        pending: Database,
    }

    impl UnitOfWork for Transaction {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            *self.database.lock().unwrap() = self.pending;
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct Factory(Arc<Mutex<Database>>);

    impl UnitOfWorkFactory for Factory {
        type Uow = Transaction;

        async fn begin(&self) -> Result<Transaction, UnitOfWorkFactoryError> {
            Ok(Transaction {
                database: self.0.clone(),
                pending: self.0.lock().unwrap().clone(),
            })
        }
    }

    struct ProcessedStore;

    impl ProjectorProcessedEventStore for ProcessedStore {
        type Uow = Transaction;

        async fn are_all_processed(
            &self,
            uow: &mut Transaction,
            name: ProjectorNameOwned,
            ids: &[EventId],
        ) -> Result<bool, ProjectorProcessedEventStoreError> {
            Ok(ids
                .iter()
                .all(|id| uow.pending.processed.contains(&(name.clone(), *id))))
        }

        async fn is_processed(
            &self,
            uow: &mut Transaction,
            name: ProjectorNameOwned,
            id: EventId,
        ) -> Result<bool, ProjectorProcessedEventStoreError> {
            Ok(uow.pending.processed.contains(&(name, id)))
        }

        async fn mark_processed(
            &self,
            uow: &mut Transaction,
            name: ProjectorNameOwned,
            id: EventId,
        ) -> Result<bool, ProjectorProcessedEventStoreError> {
            Ok(uow.pending.processed.insert((name, id)))
        }

        async fn reset(
            &self,
            uow: &mut Transaction,
            name: ProjectorNameOwned,
        ) -> Result<(), ProjectorProcessedEventStoreError> {
            uow.pending.processed.retain(|(stored, _)| stored != &name);
            Ok(())
        }
    }

    struct Store {
        calls: AtomicUsize,
    }

    impl Store {
        async fn write(&self, uow: &mut Transaction) {
            tokio::task::yield_now().await;
            self.calls.fetch_add(1, Ordering::SeqCst);
            uow.pending.writes += 1;
        }
    }

    struct CounterProjector<'a> {
        store: &'a Store,
        builds: AtomicUsize,
        reject: bool,
    }

    impl Projector for CounterProjector<'_> {
        type Uow = Transaction;
        type HandlerError = HandlerError;

        fn definition(
            &self,
        ) -> Result<ProjectorDefinition<'_, Transaction, HandlerError>, ProjectorError> {
            self.builds.fetch_add(1, Ordering::SeqCst);
            Ok(
                ProjectorDefinitionBuilder::new(ProjectorName::new("counter"))
                    .on::<Counter>(EventName::new("opened"))
                    .handle(async |uow, event| {
                        assert!(matches!(event.payload(), CounterEventPayload::Opened));
                        self.store.write(uow).await;
                        if self.reject {
                            return Err(HandlerError::Rejected);
                        }
                        Ok(())
                    })
                    .build()?,
            )
        }
    }

    fn event() -> EventEnvelope {
        let message_id = MessageId::new();
        let correlation_id = CorrelationId::from(message_id.value());
        let payload = CounterEventPayload::Opened;
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).unwrap(),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(Counter::TYPE),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1).unwrap(),
            event_name: EventNameOwned::from(payload.name()),
            payload: SerializedEventPayload::try_from(payload.try_into_json_value().unwrap())
                .unwrap(),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System).unwrap(),
        }
    }

    #[tokio::test]
    async fn borrowed_async_handler_commits_once_and_skips_unmatched_events() {
        let store = Store {
            calls: AtomicUsize::new(0),
        };
        let projector = CounterProjector {
            store: &store,
            builds: AtomicUsize::new(0),
            reject: false,
        };
        let definition = projector.definition().unwrap();
        let input = event();
        assert_eq!(
            definition.event_selectors(),
            vec![EventSelector::new::<Counter>(EventName::new("opened"))]
        );
        assert!(definition.find_event_route(&input).is_some());
        assert_eq!(store.calls.load(Ordering::SeqCst), 0);
        let factory = Factory::default();
        let runner = DefaultProjectorRunner::new(ProcessedStore, factory.clone());
        assert_eq!(
            runner.project(&definition, &input).await.unwrap(),
            ProjectorRunReport::Applied
        );
        assert_eq!(
            runner.project(&definition, &input).await.unwrap(),
            ProjectorRunReport::SkippedAlreadyProcessed
        );
        let mut unmatched = event();
        unmatched.event_name = EventNameOwned::from(EventName::new("other"));
        assert_eq!(
            runner.project(&definition, &unmatched).await.unwrap(),
            ProjectorRunReport::SkippedNotSubscribed
        );
        let db = factory.0.lock().unwrap();
        assert_eq!(db.writes, 1);
        assert_eq!(db.processed.len(), 1);
        assert_eq!(store.calls.load(Ordering::SeqCst), 1);
        assert_eq!(projector.builds.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn handler_and_decode_errors_roll_back_processed_records_and_writes() {
        let store = Store {
            calls: AtomicUsize::new(0),
        };
        let projector = CounterProjector {
            store: &store,
            builds: AtomicUsize::new(0),
            reject: true,
        };
        let definition = projector.definition().unwrap();
        let factory = Factory::default();
        let runner = DefaultProjectorRunner::new(ProcessedStore, factory.clone());
        assert!(matches!(
            runner.project(&definition, &event()).await,
            Err(ProjectorRunnerError::Projection(_))
        ));
        let mut invalid = event();
        invalid.payload =
            SerializedEventPayload::try_from(serde_json::json!({"type": "unknown"})).unwrap();
        let error = runner.project(&definition, &invalid).await.unwrap_err();
        let ProjectorRunnerError::Projection(source) = error else {
            panic!("unexpected error")
        };
        assert!(matches!(
            source.downcast_ref::<HandlerError>(),
            Some(HandlerError::EventEnvelope(_))
        ));
        let db = factory.0.lock().unwrap();
        assert_eq!(db.writes, 0);
        assert!(db.processed.is_empty());
        assert_eq!(store.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn duplicate_selectors_are_rejected_at_build() {
        let result = ProjectorDefinitionBuilder::<Transaction, HandlerError>::new(
            ProjectorName::new("counter"),
        )
        .on::<Counter>(EventName::new("opened"))
        .handle(async |_, _| Ok(()))
        .on::<Counter>(EventName::new("opened"))
        .handle(async |_, _| Ok(()))
        .build();
        assert!(matches!(
            result,
            Err(ProjectorDefinitionBuilderError::Definition(
                ProjectorDefinitionError::DuplicateEventRoute { .. }
            ))
        ));
    }

    struct Feed(Vec<EventEnvelope>);

    impl EventFeedReader for Feed {
        type Uow = Transaction;

        async fn read_after(
            &self,
            _uow: &mut Transaction,
            after: Option<EventSequence>,
            _limit: EventFeedBatchSize,
            subscription: Subscription<'_, EventSelector>,
        ) -> Result<Vec<EventEnvelope>, EventFeedReaderError> {
            Ok(self
                .0
                .iter()
                .filter(|event| {
                    after.is_none_or(|sequence| event.event_sequence > sequence)
                        && subscription.matches(*event)
                })
                .cloned()
                .collect())
        }
    }

    struct Checkpoints;

    impl ProjectionCheckpointStore for Checkpoints {
        type Uow = Transaction;

        async fn load(
            &self,
            uow: &mut Transaction,
            _name: ProjectorNameOwned,
        ) -> Result<Option<EventSequence>, ProjectionCheckpointStoreError> {
            Ok(uow.pending.checkpoint)
        }

        async fn save(
            &self,
            uow: &mut Transaction,
            _name: ProjectorNameOwned,
            sequence: EventSequence,
        ) -> Result<(), ProjectionCheckpointStoreError> {
            uow.pending.checkpoint = Some(sequence);
            Ok(())
        }

        async fn reset(
            &self,
            uow: &mut Transaction,
            _name: ProjectorNameOwned,
        ) -> Result<(), ProjectionCheckpointStoreError> {
            uow.pending.checkpoint = None;
            Ok(())
        }
    }

    #[tokio::test]
    async fn rebuild_constructs_once_and_advances_past_already_processed_events() {
        let store = Store {
            calls: AtomicUsize::new(0),
        };
        let projector = CounterProjector {
            store: &store,
            builds: AtomicUsize::new(0),
            reject: false,
        };
        let first = event();
        let mut second = event();
        second.event_sequence = EventSequence::try_from(2).unwrap();
        let factory = Factory::default();
        factory.0.lock().unwrap().processed.insert((
            ProjectorNameOwned::from(ProjectorName::new("counter")),
            first.event_id,
        ));
        let mut rebuilder = DefaultProjectorRebuilder::new(
            Feed(vec![first, second.clone()]),
            Checkpoints,
            ProcessedStore,
            factory.clone(),
            ProjectorRebuilderConfig {
                batch_size: EventFeedBatchSize::new(NonZeroU32::new(10).unwrap()),
            },
        );
        rebuilder.run_until_idle(&projector).await.unwrap();
        let db = factory.0.lock().unwrap();
        assert_eq!(db.checkpoint, Some(second.event_sequence));
        assert_eq!(db.processed.len(), 2);
        assert_eq!(db.writes, 1);
        assert_eq!(store.calls.load(Ordering::SeqCst), 1);
        assert_eq!(projector.builds.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn rebuild_failure_rolls_back_checkpoint_and_processed_marker() {
        let store = Store {
            calls: AtomicUsize::new(0),
        };
        let projector = CounterProjector {
            store: &store,
            builds: AtomicUsize::new(0),
            reject: true,
        };
        let factory = Factory::default();
        let mut rebuilder = DefaultProjectorRebuilder::new(
            Feed(vec![event()]),
            Checkpoints,
            ProcessedStore,
            factory.clone(),
            ProjectorRebuilderConfig {
                batch_size: EventFeedBatchSize::new(NonZeroU32::new(10).unwrap()),
            },
        );
        assert!(matches!(
            rebuilder.run_until_idle(&projector).await,
            Err(ProjectorRebuilderError::Projection(_))
        ));
        let db = factory.0.lock().unwrap();
        assert_eq!(db.checkpoint, None);
        assert_eq!(db.writes, 0);
        assert!(db.processed.is_empty());
    }

    struct TestSubscriber {
        events: Vec<EventEnvelope>,
        acknowledgements: Arc<AtomicUsize>,
    }

    struct TestConsumer {
        events: std::vec::IntoIter<EventEnvelope>,
        acknowledgements: Arc<AtomicUsize>,
    }

    struct TestDelivery {
        event: EventEnvelope,
        acknowledgements: Arc<AtomicUsize>,
    }

    impl Subscriber<EventEnvelope> for TestSubscriber {
        type Consumer = TestConsumer;
        type Selector = EventSelector;

        async fn subscribe(
            &self,
            group: &ConsumerGroup,
            subscription: Subscription<'_, EventSelector>,
        ) -> Result<TestConsumer, SubscriberError> {
            assert_eq!(group.to_string(), "projector_counter");
            assert_eq!(
                subscription,
                Subscription::AnyOf(&[EventSelector::new::<Counter>(EventName::new("opened"))])
            );
            Ok(TestConsumer {
                events: self.events.clone().into_iter(),
                acknowledgements: self.acknowledgements.clone(),
            })
        }
    }

    impl Consumer<EventEnvelope> for TestConsumer {
        type Delivery = TestDelivery;

        async fn next(&mut self) -> Result<TestDelivery, ConsumerError> {
            let event = self.events.next().ok_or_else(|| {
                ConsumerError::Next(Box::new(std::io::Error::other("end of test stream")))
            })?;
            Ok(TestDelivery {
                event,
                acknowledgements: self.acknowledgements.clone(),
            })
        }
    }

    impl Delivery<EventEnvelope> for TestDelivery {
        fn message(&self) -> &EventEnvelope {
            &self.event
        }

        async fn ack(&mut self) -> Result<(), DeliveryError> {
            self.acknowledgements.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn nack(&mut self) -> Result<(), DeliveryError> {
            panic!("successful delivery should not be nacked")
        }
    }

    #[tokio::test]
    async fn worker_builds_once_and_subscribes_to_collected_selectors() {
        let store = Store {
            calls: AtomicUsize::new(0),
        };
        let projector = CounterProjector {
            store: &store,
            builds: AtomicUsize::new(0),
            reject: false,
        };
        let acknowledgements = Arc::new(AtomicUsize::new(0));
        let input = event();
        let mut unmatched = event();
        unmatched.event_name = EventNameOwned::from(EventName::new("other"));
        let subscriber = TestSubscriber {
            events: vec![input.clone(), input, unmatched],
            acknowledgements: acknowledgements.clone(),
        };
        let factory = Factory::default();
        let worker = DefaultProjectorWorker::new(
            DefaultProjectorRunner::new(ProcessedStore, factory.clone()),
            subscriber,
        );
        assert!(matches!(
            worker.run_forever(&projector).await,
            Err(ProjectorWorkerError::Consumer(_))
        ));
        assert_eq!(acknowledgements.load(Ordering::SeqCst), 3);
        assert_eq!(projector.builds.load(Ordering::SeqCst), 1);
        assert_eq!(store.calls.load(Ordering::SeqCst), 1);
        assert_eq!(factory.0.lock().unwrap().writes, 1);
    }
}

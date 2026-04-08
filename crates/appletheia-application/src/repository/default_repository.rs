use std::marker::PhantomData;
use std::ops::Bound;

use appletheia_domain::{
    Aggregate, AggregateError, AggregateState, AggregateVersion, AggregateVersionRange,
    UniqueConstraints,
};

use crate::event::{EventReader, EventWriter};
use crate::request_context::RequestContext;
use crate::snapshot::{SnapshotPolicy, SnapshotReader, SnapshotWriter};
use crate::unit_of_work::UnitOfWork;

use super::{
    Repository, RepositoryConfig, RepositoryError, UniqueKeyReservationStore,
    UniqueValueOwnerLookup,
};

pub struct DefaultRepository<A, ER, EW, SR, SW, UVOL, UKS, Uow>
where
    A: Aggregate,
    A::State: UniqueConstraints<<A::State as AggregateState>::Error>,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
    UVOL: UniqueValueOwnerLookup<Uow = Uow>,
    UKS: UniqueKeyReservationStore<Uow = Uow>,
{
    config: RepositoryConfig,
    event_reader: ER,
    snapshot_reader: SR,
    event_writer: EW,
    snapshot_writer: SW,
    unique_value_owner_lookup: UVOL,
    unique_key_reservation_store: UKS,
    _marker: PhantomData<fn() -> A>,
}

impl<A, ER, EW, SR, SW, UVOL, UKS, Uow> DefaultRepository<A, ER, EW, SR, SW, UVOL, UKS, Uow>
where
    A: Aggregate,
    A::State: UniqueConstraints<<A::State as AggregateState>::Error>,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
    UVOL: UniqueValueOwnerLookup<Uow = Uow>,
    UKS: UniqueKeyReservationStore<Uow = Uow>,
{
    pub fn new(
        config: RepositoryConfig,
        event_reader: ER,
        snapshot_reader: SR,
        event_writer: EW,
        snapshot_writer: SW,
        unique_value_owner_lookup: UVOL,
        unique_key_reservation_store: UKS,
    ) -> Self {
        Self {
            config,
            event_reader,
            snapshot_reader,
            event_writer,
            snapshot_writer,
            unique_value_owner_lookup,
            unique_key_reservation_store,
            _marker: PhantomData,
        }
    }
}

impl<A, ER, EW, SR, SW, UVOL, UKS, Uow> Repository<A>
    for DefaultRepository<A, ER, EW, SR, SW, UVOL, UKS, Uow>
where
    A: Aggregate,
    A::State: UniqueConstraints<<A::State as AggregateState>::Error>,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
    UVOL: UniqueValueOwnerLookup<Uow = Uow>,
    UKS: UniqueKeyReservationStore<Uow = Uow>,
{
    type Uow = Uow;

    async fn find(&self, uow: &mut Self::Uow, id: A::Id) -> Result<Option<A>, RepositoryError<A>> {
        self.find_at_version(uow, id, None).await
    }

    async fn find_at_version(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
        at: Option<AggregateVersion>,
    ) -> Result<Option<A>, RepositoryError<A>> {
        let snapshot = self
            .snapshot_reader
            .read_latest_snapshot(uow, id, at)
            .await?;
        let events = {
            let start = snapshot
                .as_ref()
                .map(|s| Bound::Excluded(s.aggregate_version()))
                .unwrap_or(Bound::Unbounded);
            let end = at.map(Bound::Included).unwrap_or(Bound::Unbounded);
            let range = AggregateVersionRange::new(start, end);
            self.event_reader.read_events(uow, id, range).await?
        };

        if events.is_empty() && snapshot.is_none() {
            return Ok(None);
        }

        let mut aggregate = A::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::Aggregate)?;

        Ok(Some(aggregate))
    }

    async fn find_by_unique_value(
        &self,
        uow: &mut Self::Uow,
        unique_key: appletheia_domain::UniqueKey,
        unique_value: &appletheia_domain::UniqueValue,
    ) -> Result<Option<A>, RepositoryError<A>> {
        let aggregate_id = self
            .unique_value_owner_lookup
            .find_owner_id(uow, A::TYPE, unique_key, unique_value)
            .await?;
        let Some(aggregate_id) = aggregate_id else {
            return Ok(None);
        };

        self.find(uow, aggregate_id).await
    }

    async fn save(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        aggregate: &mut A,
    ) -> Result<(), RepositoryError<A>> {
        let aggregate_id = aggregate
            .aggregate_id()
            .ok_or_else(|| RepositoryError::Aggregate(AggregateError::<A::Id>::NoState.into()))?;
        let state = aggregate
            .state_required()
            .map_err(RepositoryError::Aggregate)?;
        let unique_entries = state.unique_entries().map_err(RepositoryError::State)?;
        self.unique_key_reservation_store
            .replace(uow, A::TYPE, aggregate_id, &unique_entries)
            .await?;

        let events = aggregate.uncommitted_events();
        self.event_writer
            .write_events_and_outbox(uow, request_context, events)
            .await?;

        match self.config.snapshot_policy {
            SnapshotPolicy::Disabled => {}
            SnapshotPolicy::AtLeast { minimum_interval } => {
                let current_version = aggregate.version().as_u64();
                let latest_snapshot_version = self
                    .snapshot_reader
                    .read_latest_snapshot(uow, aggregate_id, None)
                    .await?
                    .as_ref()
                    .map(|snapshot| snapshot.aggregate_version().as_u64())
                    .unwrap_or(0);

                if current_version.saturating_sub(latest_snapshot_version)
                    >= minimum_interval.as_u64()
                {
                    let snapshot = aggregate
                        .to_snapshot()
                        .map_err(RepositoryError::Aggregate)?;
                    self.snapshot_writer.write_snapshot(uow, &snapshot).await?;
                }
            }
        }

        aggregate.core_mut().clear_uncommitted_events();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultRepository;
    use crate::event::{EventReader, EventReaderError, EventWriter, EventWriterError};
    use crate::repository::{
        Repository, RepositoryConfig, RepositoryError, UniqueKeyReservationStore,
        UniqueKeyReservationStoreError, UniqueValueOwnerLookup, UniqueValueOwnerLookupError,
    };
    use crate::request_context::{CorrelationId, MessageId, Principal, RequestContext};
    use crate::snapshot::{
        SnapshotPolicy, SnapshotReader, SnapshotReaderError, SnapshotWriter, SnapshotWriterError,
    };
    use crate::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia_domain::{
        Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
        AggregateStateError, AggregateType, AggregateVersion, AggregateVersionRange, Event,
        EventName, EventPayload, Snapshot, UniqueConstraints, UniqueEntries, UniqueKey,
        UniqueValue, UniqueValuePart, UniqueValues, UniqueValuesError,
    };
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Display};
    use std::sync::{Arc, Mutex};
    use thiserror::Error;
    use uuid::Uuid;

    #[derive(Debug, Default)]
    struct TestUnitOfWork;

    impl UnitOfWork for TestUnitOfWork {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Debug, Error, Eq, PartialEq)]
    enum CounterIdError {
        #[error("nil uuid is not allowed")]
        NilUuid,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    struct CounterId(Uuid);

    impl Display for CounterId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl AggregateId for CounterId {
        type Error = CounterIdError;

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

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),

        #[error(transparent)]
        UniqueValues(#[from] UniqueValuesError),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        id: CounterId,
        email: Option<String>,
    }

    impl UniqueConstraints<CounterStateError> for CounterState {
        fn unique_entries(&self) -> Result<UniqueEntries, CounterStateError> {
            let mut unique_keys = UniqueEntries::new();
            if let Some(email) = self.email.as_deref() {
                let part = UniqueValuePart::try_from(email).expect("email should be non-empty");
                let value = UniqueValue::new(vec![part]).expect("unique value should be valid");
                let values = UniqueValues::new(vec![value]).expect("unique values should be valid");
                let _ = unique_keys.insert(UniqueKey::new("email"), values);
            }

            Ok(unique_keys)
        }
    }

    impl AggregateState for CounterState {
        type Id = CounterId;
        type Error = CounterStateError;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "snake_case")]
    enum CounterEventPayload {
        Registered {
            id: CounterId,
            email: Option<String>,
        },
    }

    impl CounterEventPayload {
        const REGISTERED: EventName = EventName::new("registered");
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;

        fn name(&self) -> EventName {
            match self {
                Self::Registered { .. } => Self::REGISTERED,
            }
        }
    }

    #[derive(Debug, Error)]
    enum CounterError {
        #[error(transparent)]
        Aggregate(#[from] AggregateError<CounterId>),
    }

    #[derive(Clone, Debug, Default)]
    struct Counter {
        core: AggregateCore<CounterState, CounterEventPayload>,
    }

    impl AggregateApply<CounterEventPayload, CounterError> for Counter {
        fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
            match payload {
                CounterEventPayload::Registered { id, email } => {
                    self.set_state(Some(CounterState {
                        id: *id,
                        email: email.clone(),
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

        fn core(&self) -> &AggregateCore<Self::State, Self::EventPayload> {
            &self.core
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::State, Self::EventPayload> {
            &mut self.core
        }
    }

    #[derive(Debug, Default)]
    struct RecordingEventReader;

    impl EventReader<Counter> for RecordingEventReader {
        type Uow = TestUnitOfWork;

        async fn read_events(
            &self,
            _uow: &mut Self::Uow,
            _aggregate_id: CounterId,
            _range: AggregateVersionRange,
        ) -> Result<Vec<Event<CounterId, CounterEventPayload>>, EventReaderError> {
            Ok(Vec::new())
        }
    }

    #[derive(Debug)]
    struct RecordingEventWriter {
        log: Arc<Mutex<Vec<String>>>,
    }

    impl EventWriter<Counter> for RecordingEventWriter {
        type Uow = TestUnitOfWork;

        async fn write_events_and_outbox(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            events: &[Event<CounterId, CounterEventPayload>],
        ) -> Result<(), EventWriterError> {
            self.log
                .lock()
                .expect("event writer log should be lockable")
                .push(format!("write_events:{}", events.len()));

            Ok(())
        }
    }

    #[derive(Debug, Default)]
    struct RecordingSnapshotReader;

    impl SnapshotReader<Counter> for RecordingSnapshotReader {
        type Uow = TestUnitOfWork;

        async fn read_latest_snapshot(
            &self,
            _uow: &mut Self::Uow,
            _aggregate_id: CounterId,
            _as_of: Option<AggregateVersion>,
        ) -> Result<Option<Snapshot<CounterState>>, SnapshotReaderError> {
            Ok(None)
        }
    }

    #[derive(Debug, Default)]
    struct RecordingSnapshotWriter;

    impl SnapshotWriter<Counter> for RecordingSnapshotWriter {
        type Uow = TestUnitOfWork;

        async fn write_snapshot(
            &self,
            _uow: &mut Self::Uow,
            _snapshot: &Snapshot<CounterState>,
        ) -> Result<(), SnapshotWriterError> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct RecordingUniqueValueOwnerLookup {
        aggregate_id: Option<CounterId>,
        fail: bool,
        log: Arc<Mutex<Vec<String>>>,
    }

    impl UniqueValueOwnerLookup for RecordingUniqueValueOwnerLookup {
        type Uow = TestUnitOfWork;

        async fn find_owner_id<I>(
            &self,
            _uow: &mut Self::Uow,
            aggregate_type: AggregateType,
            unique_key: UniqueKey,
            unique_value: &UniqueValue,
        ) -> Result<Option<I>, UniqueValueOwnerLookupError>
        where
            I: AggregateId,
        {
            self.log
                .lock()
                .expect("lookup log should be lockable")
                .push(format!(
                    "lookup:{}:{}:{}",
                    aggregate_type.value(),
                    unique_key.value(),
                    unique_value.normalized_key()
                ));

            if self.fail {
                return Err(UniqueValueOwnerLookupError::Persistence(Box::new(
                    std::io::Error::other("lookup failed"),
                )));
            }

            self.aggregate_id
                .map(|aggregate_id| {
                    I::try_from_uuid(aggregate_id.value()).map_err(|error| {
                        UniqueValueOwnerLookupError::OwnerAggregateId(Box::new(error))
                    })
                })
                .transpose()
        }
    }

    #[derive(Debug)]
    struct RecordingUniqueKeyReservationStore {
        fail_with_conflict: bool,
        log: Arc<Mutex<Vec<String>>>,
    }

    impl UniqueKeyReservationStore for RecordingUniqueKeyReservationStore {
        type Uow = TestUnitOfWork;

        async fn replace<I>(
            &self,
            _uow: &mut Self::Uow,
            aggregate_type: AggregateType,
            owner_aggregate_id: I,
            unique_entries: &UniqueEntries,
        ) -> Result<(), UniqueKeyReservationStoreError>
        where
            I: AggregateId,
        {
            let entry_count = unique_entries.iter().count();
            let value_count: usize = unique_entries.iter().map(|(_, values)| values.len()).sum();
            self.log
                .lock()
                .expect("unique key log should be lockable")
                .push(format!("replace:{}:{}", entry_count, value_count));

            if self.fail_with_conflict {
                let (unique_key, values) = unique_entries
                    .iter()
                    .next()
                    .expect("conflict tests should provide a unique key");
                let value = values
                    .first()
                    .expect("conflict tests should provide a value");
                return Err(UniqueKeyReservationStoreError::conflict(
                    aggregate_type,
                    *unique_key,
                    value,
                ));
            }

            let _ = owner_aggregate_id.value();
            Ok(())
        }
    }

    fn repository(
        log: Arc<Mutex<Vec<String>>>,
        fail_with_conflict: bool,
    ) -> DefaultRepository<
        Counter,
        RecordingEventReader,
        RecordingEventWriter,
        RecordingSnapshotReader,
        RecordingSnapshotWriter,
        RecordingUniqueValueOwnerLookup,
        RecordingUniqueKeyReservationStore,
        TestUnitOfWork,
    > {
        DefaultRepository::new(
            RepositoryConfig {
                snapshot_policy: SnapshotPolicy::Disabled,
            },
            RecordingEventReader,
            RecordingSnapshotReader,
            RecordingEventWriter {
                log: Arc::clone(&log),
            },
            RecordingSnapshotWriter,
            RecordingUniqueValueOwnerLookup {
                aggregate_id: None,
                fail: false,
                log: Arc::clone(&log),
            },
            RecordingUniqueKeyReservationStore {
                fail_with_conflict,
                log,
            },
        )
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid")
    }

    fn registered_counter(email: Option<&str>) -> Counter {
        let id = CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut aggregate = Counter::default();
        aggregate
            .append_event(CounterEventPayload::Registered {
                id,
                email: email.map(str::to_owned),
            })
            .expect("register event should apply");

        aggregate
    }

    fn repository_with_lookup(
        log: Arc<Mutex<Vec<String>>>,
        aggregate_id: Option<CounterId>,
        fail_lookup: bool,
    ) -> DefaultRepository<
        Counter,
        RecordingEventReader,
        RecordingEventWriter,
        RecordingSnapshotReader,
        RecordingSnapshotWriter,
        RecordingUniqueValueOwnerLookup,
        RecordingUniqueKeyReservationStore,
        TestUnitOfWork,
    > {
        DefaultRepository::new(
            RepositoryConfig {
                snapshot_policy: SnapshotPolicy::Disabled,
            },
            RecordingEventReader,
            RecordingSnapshotReader,
            RecordingEventWriter {
                log: Arc::clone(&log),
            },
            RecordingSnapshotWriter,
            RecordingUniqueValueOwnerLookup {
                aggregate_id,
                fail: fail_lookup,
                log: Arc::clone(&log),
            },
            RecordingUniqueKeyReservationStore {
                fail_with_conflict: false,
                log,
            },
        )
    }

    #[tokio::test]
    async fn save_replaces_unique_keys_before_writing_events() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let repository = repository(Arc::clone(&log), false);
        let request_context = request_context();
        let mut uow = TestUnitOfWork;
        let mut aggregate = registered_counter(Some("foo@example.com"));

        repository
            .save(&mut uow, &request_context, &mut aggregate)
            .await
            .expect("save should succeed");

        assert_eq!(
            *log.lock().expect("log should be lockable"),
            vec!["replace:1:1".to_owned(), "write_events:1".to_owned()]
        );
        assert!(aggregate.uncommitted_events().is_empty());
    }

    #[tokio::test]
    async fn save_replaces_owner_unique_keys_with_empty_set() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let repository = repository(Arc::clone(&log), false);
        let request_context = request_context();
        let mut uow = TestUnitOfWork;
        let mut aggregate = registered_counter(None);

        repository
            .save(&mut uow, &request_context, &mut aggregate)
            .await
            .expect("save should succeed");

        assert_eq!(
            *log.lock().expect("log should be lockable"),
            vec!["replace:0:0".to_owned(), "write_events:1".to_owned()]
        );
    }

    #[tokio::test]
    async fn save_stops_before_writing_events_when_unique_key_conflicts() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let repository = repository(Arc::clone(&log), true);
        let request_context = request_context();
        let mut uow = TestUnitOfWork;
        let mut aggregate = registered_counter(Some("foo@example.com"));

        let error = repository
            .save(&mut uow, &request_context, &mut aggregate)
            .await
            .expect_err("conflict should fail");

        assert!(matches!(
            error,
            RepositoryError::UniqueKeyReservationStore(
                UniqueKeyReservationStoreError::Conflict { .. }
            )
        ));
        assert_eq!(
            *log.lock().expect("log should be lockable"),
            vec!["replace:1:1".to_owned()]
        );
    }

    #[tokio::test]
    async fn find_by_unique_value_returns_none_when_lookup_misses() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let repository = repository_with_lookup(Arc::clone(&log), None, false);
        let mut uow = TestUnitOfWork;
        let unique_value = UniqueValue::new(vec![
            UniqueValuePart::try_from("foo@example.com").expect("unique part should be valid"),
        ])
        .expect("unique value should be valid");

        let aggregate = repository
            .find_by_unique_value(&mut uow, UniqueKey::new("email"), &unique_value)
            .await
            .expect("lookup should succeed");

        assert!(aggregate.is_none());
        assert_eq!(
            *log.lock().expect("log should be lockable"),
            vec!["lookup:counter:email:15:foo@example.com".to_owned()]
        );
    }

    #[tokio::test]
    async fn find_by_unique_value_returns_lookup_errors() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let repository = repository_with_lookup(Arc::clone(&log), None, true);
        let mut uow = TestUnitOfWork;
        let unique_value = UniqueValue::new(vec![
            UniqueValuePart::try_from("foo@example.com").expect("unique part should be valid"),
        ])
        .expect("unique value should be valid");

        let error = repository
            .find_by_unique_value(&mut uow, UniqueKey::new("email"), &unique_value)
            .await
            .expect_err("lookup failure should be returned");

        assert!(matches!(error, RepositoryError::UniqueValueOwnerLookup(_)));
    }
}

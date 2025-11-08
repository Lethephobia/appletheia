use sqlx::PgPool;

use crate::postgresql::event::PgEventRow;
use crate::postgresql::snapshot::PgSnapshotRow;
use appletheia_domain::{
    Aggregate, AggregateId, AggregateState, AggregateVersion, Event, EventId, EventPayload,
    MaterializedAt, OccurredAt, PersistenceErrorKind, Repository, RepositoryError, Snapshot,
    SnapshotId,
};

use std::marker::PhantomData;

pub struct PgRepository<A: Aggregate> {
    pool: PgPool,
    _phantom: PhantomData<A>,
}

impl<A: Aggregate> PgRepository<A> {
    fn map_event(
        &self,
        event: PgEventRow,
    ) -> Result<Event<A::Id, A::EventPayload>, RepositoryError<A>> {
        let id = EventId::try_from(event.id).map_err(RepositoryError::EventId)?;
        let aggregate_id =
            A::Id::try_from_uuid(event.aggregate_id).map_err(RepositoryError::AggregateId)?;
        let aggregate_version = AggregateVersion::try_from(event.aggregate_version)
            .map_err(RepositoryError::AggregateVersion)?;
        let payload = A::EventPayload::try_from_json_value(event.payload)
            .map_err(RepositoryError::EventPayload)?;
        Ok(Event::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            payload,
            OccurredAt::from(event.occurred_at),
        ))
    }

    fn map_snapshot(
        &self,
        snapshot: PgSnapshotRow,
    ) -> Result<Snapshot<A::State>, RepositoryError<A>> {
        let id = SnapshotId::try_from(snapshot.id).map_err(RepositoryError::SnapshotId)?;
        let aggregate_id =
            A::Id::try_from_uuid(snapshot.aggregate_id).map_err(RepositoryError::AggregateId)?;
        let aggregate_version = AggregateVersion::try_from(snapshot.aggregate_version)
            .map_err(RepositoryError::AggregateVersion)?;
        let state = A::State::try_from_json_value(snapshot.state)
            .map_err(RepositoryError::AggregateState)?;
        Ok(Snapshot::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            state,
            MaterializedAt::from(snapshot.materialized_at),
        ))
    }

    fn map_persistence_error(&self, error: sqlx::Error) -> RepositoryError<A> {
        match error {
            sqlx::Error::Database(db) => {
                let code = db.code();
                match code.as_deref() {
                    Some("23505") => RepositoryError::Persistence {
                        kind: PersistenceErrorKind::Conflict,
                        code: Some("23505".into()),
                    },
                    Some("40001") => RepositoryError::Persistence {
                        kind: PersistenceErrorKind::Serialization,
                        code: Some("40001".into()),
                    },
                    Some(_) => RepositoryError::Persistence {
                        kind: PersistenceErrorKind::ConstraintViolation,
                        code: db.code().map(|c| c.into()),
                    },
                    None => RepositoryError::Persistence {
                        kind: PersistenceErrorKind::ConstraintViolation,
                        code: None,
                    },
                }
            }
            sqlx::Error::PoolTimedOut => RepositoryError::Persistence {
                kind: PersistenceErrorKind::Timeout,
                code: None,
            },
            sqlx::Error::Io(_) => RepositoryError::Persistence {
                kind: PersistenceErrorKind::Io,
                code: None,
            },
            _ => RepositoryError::Persistence {
                kind: PersistenceErrorKind::Unknown,
                code: None,
            },
        }
    }
}

impl<A: Aggregate> Repository<A> for PgRepository<A> {
    async fn read_events(
        &self,
        aggregate_id: A::Id,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        RepositoryError<A>,
    > {
        let snapshot_row = sqlx::query_as::<_, PgSnapshotRow>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            FROM snapshots WHERE aggregate_type = $1 AND aggregate_id = $2
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| self.map_persistence_error(e))?;
        let snapshot = snapshot_row.map(|row| self.map_snapshot(row)).transpose()?;

        let version_value_after = snapshot
            .as_ref()
            .map(|s| s.aggregate_version().value())
            .unwrap_or(0);

        let event_rows = sqlx::query_as::<_, PgEventRow>(
            r#"
            SELECT
                event_sequence, id, aggregate_type, aggregate_id, aggregate_version,
                payload, occurred_at, recorded_at, correlation_id, causation_id, context
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version > $3
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .bind(version_value_after)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| self.map_persistence_error(e))?;

        let events = event_rows
            .into_iter()
            .map(|row| self.map_event(row))
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A>>>()?;

        Ok((events, snapshot))
    }

    async fn read_events_at_version(
        &self,
        aggregate_id: A::Id,
        version_at: AggregateVersion,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        RepositoryError<A>,
    > {
        let snapshot_row = sqlx::query_as::<_, PgSnapshotRow>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            FROM snapshots WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version <= $3
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .bind(version_at.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| self.map_persistence_error(e))?;
        let snapshot = snapshot_row.map(|row| self.map_snapshot(row)).transpose()?;

        let version_value_after = snapshot
            .as_ref()
            .map(|s| s.aggregate_version().value())
            .unwrap_or(0);

        let event_rows = sqlx::query_as::<_, PgEventRow>(
            r#"
            SELECT
                event_sequence, id, aggregate_type, aggregate_id, aggregate_version,
                payload, occurred_at, recorded_at, correlation_id, causation_id, context
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2
                AND aggregate_version > $3 AND aggregate_version <= $4
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .bind(version_value_after)
        .bind(version_at.value())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| self.map_persistence_error(e))?;

        let events = event_rows
            .into_iter()
            .map(|row| self.map_event(row))
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A>>>()?;

        Ok((events, snapshot))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fmt, fmt::Display, marker::PhantomData};

    use chrono::{TimeZone, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use sqlx::postgres::PgPoolOptions;
    use thiserror::Error;
    use uuid::{Uuid, Version};

    use appletheia_domain::{
        AggregateError, AggregateState, AggregateVersion, AggregateVersionError, Event,
        EventPayload,
    };

    #[derive(Debug, Error)]
    enum CounterIdError {
        #[error("not a uuidv7: {0}")]
        NotUuidV7(Uuid),
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
    #[serde(try_from = "Uuid", into = "Uuid")]
    struct CounterId(Uuid);

    impl CounterId {
        fn new() -> Self {
            Self(Uuid::now_v7())
        }
    }

    impl AggregateId for CounterId {
        type Error = CounterIdError;

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
            match value.get_version() {
                Some(Version::SortRand) => Ok(Self(value)),
                _ => Err(CounterIdError::NotUuidV7(value)),
            }
        }

        fn value(self) -> Uuid {
            self.0
        }
    }

    impl Display for CounterId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl From<CounterId> for Uuid {
        fn from(value: CounterId) -> Self {
            value.value()
        }
    }

    impl TryFrom<Uuid> for CounterId {
        type Error = CounterIdError;

        fn try_from(value: Uuid) -> Result<Self, Self::Error> {
            Self::try_from_uuid(value)
        }
    }

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error("json error: {0}")]
        Json(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        id: CounterId,
        counter: i32,
    }

    impl CounterState {
        fn new(id: CounterId, counter: i32) -> Self {
            Self { id, counter }
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
        #[error("json error: {0}")]
        Json(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "snake_case")]
    enum CounterEventPayload {
        Imported(),
        Created(),
        Increment(i32),
        Decrement(i32),
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;
    }

    impl Display for CounterEventPayload {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    type CounterEvent = Event<CounterId, CounterEventPayload>;

    #[derive(Debug, Error)]
    enum CounterError {
        #[error("aggregate error: {0}")]
        Aggregate(#[from] AggregateError<CounterId>),

        #[error("invalid event payload: {0}")]
        InvalidEventPayload(CounterEventPayload),

        #[error("state missing")]
        StateMissing,
    }

    impl PartialEq for CounterError {
        fn eq(&self, other: &Self) -> bool {
            matches!(
                (self, other),
                (CounterError::StateMissing, CounterError::StateMissing)
                    | (CounterError::Aggregate(_), CounterError::Aggregate(_))
            )
        }
    }

    impl Eq for CounterError {}

    #[derive(Clone, Debug)]
    struct Counter {
        id: CounterId,
        state: Option<CounterState>,
        version: AggregateVersion,
        uncommitted_events: Vec<CounterEvent>,
    }

    impl Counter {
        pub fn new() -> Self {
            let id = CounterId::new();
            Self {
                id,
                state: None,
                version: AggregateVersion::new(),
                uncommitted_events: Vec::new(),
            }
        }
    }

    impl Default for Counter {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Aggregate for Counter {
        type Id = CounterId;
        type State = CounterState;
        type EventPayload = CounterEventPayload;
        type Error = CounterError;

        const AGGREGATE_TYPE: &'static str = "counter";

        fn state(&self) -> Option<&Self::State> {
            self.state.as_ref()
        }

        fn set_state(&mut self, state: Option<Self::State>) {
            self.state = state;
        }

        fn version(&self) -> AggregateVersion {
            self.version
        }

        fn set_version(&mut self, version: AggregateVersion) {
            self.version = version;
        }

        fn uncommitted_events(&self) -> &[Event<Self::Id, Self::EventPayload>] {
            &self.uncommitted_events
        }

        fn record_uncommitted_event(&mut self, event: Event<Self::Id, Self::EventPayload>) {
            self.uncommitted_events.push(event);
        }

        fn apply(&mut self, payload: &Self::EventPayload) -> Result<(), Self::Error> {
            match self.state.as_mut() {
                None => match payload {
                    CounterEventPayload::Created() => {
                        self.state = Some(CounterState::new(self.id, 0));
                    }
                    _ => {
                        return Err(CounterError::InvalidEventPayload(payload.clone()).into());
                    }
                },
                Some(state) => match payload {
                    CounterEventPayload::Increment(delta) => {
                        state.counter += delta;
                    }
                    CounterEventPayload::Decrement(delta) => {
                        state.counter -= delta;
                    }
                    _ => {
                        return Err(CounterError::StateMissing.into());
                    }
                },
            }
            Ok(())
        }
    }

    fn create_repository() -> PgRepository<Counter> {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://postgres:password@localhost/test")
            .expect("should construct lazy pg pool");

        PgRepository {
            pool,
            _phantom: PhantomData,
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn map_event_returns_domain_event() {
        let repository = create_repository();
        let event_id = Uuid::now_v7();
        let aggregate_uuid = Uuid::now_v7();
        let occurred_at = Utc.with_ymd_and_hms(2024, 5, 20, 12, 0, 0).unwrap();
        let payload = CounterEventPayload::Created();

        let pg_event = PgEventRow {
            event_sequence: 1,
            id: event_id,
            aggregate_type: Counter::AGGREGATE_TYPE.to_string(),
            aggregate_id: aggregate_uuid,
            aggregate_version: 3,
            payload: serde_json::to_value(&payload).expect("payload should serialize"),
            occurred_at,
            recorded_at: Utc::now(),
            correlation_id: Uuid::now_v7(),
            causation_id: Uuid::now_v7(),
            context: serde_json::json!({}),
        };

        let event = repository
            .map_event(pg_event)
            .expect("expected event to map successfully");

        assert_eq!(Uuid::from(event.id()), event_id);
        assert_eq!(Uuid::from(event.aggregate_id()), aggregate_uuid);
        assert_eq!(event.aggregate_version().value(), 3);
        assert_eq!(event.payload(), &payload);
        let mapped_occurred_at: chrono::DateTime<Utc> = event.occurred_at().into();
        assert_eq!(mapped_occurred_at, occurred_at);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn map_event_returns_error_on_negative_version() {
        let repository = create_repository();
        let event_id = Uuid::now_v7();
        let aggregate_uuid = Uuid::now_v7();
        let payload = CounterEventPayload::Created();

        let pg_event = PgEventRow {
            event_sequence: 1,
            id: event_id,
            aggregate_type: Counter::AGGREGATE_TYPE.to_string(),
            aggregate_id: aggregate_uuid,
            aggregate_version: -1,
            payload: serde_json::to_value(&payload).expect("payload should serialize"),
            occurred_at: Utc::now(),
            recorded_at: Utc::now(),
            correlation_id: Uuid::now_v7(),
            causation_id: Uuid::now_v7(),
            context: serde_json::json!({}),
        };

        let err = repository
            .map_event(pg_event)
            .expect_err("expected invalid aggregate version error");

        match err {
            RepositoryError::AggregateVersion(AggregateVersionError::NegativeValue(value)) => {
                assert_eq!(value, -1)
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn map_event_returns_error_on_invalid_aggregate_id() {
        let repository = create_repository();
        let event_id = Uuid::now_v7();
        let invalid_uuid = Uuid::nil();
        let payload = CounterEventPayload::Created();

        let pg_event = PgEventRow {
            event_sequence: 1,
            id: event_id,
            aggregate_type: Counter::AGGREGATE_TYPE.to_string(),
            aggregate_id: invalid_uuid,
            aggregate_version: 1,
            payload: serde_json::to_value(&payload).expect("payload should serialize"),
            occurred_at: Utc::now(),
            recorded_at: Utc::now(),
            correlation_id: Uuid::now_v7(),
            causation_id: Uuid::now_v7(),
            context: serde_json::json!({}),
        };

        let err = repository
            .map_event(pg_event)
            .expect_err("expected aggregate id conversion error");

        match err {
            RepositoryError::AggregateId(CounterIdError::NotUuidV7(value)) => {
                assert_eq!(value, invalid_uuid);
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn map_snapshot_returns_domain_snapshot() {
        let repository = create_repository();
        let snapshot_id = Uuid::now_v7();
        let aggregate_uuid = Uuid::now_v7();
        let materialized_at = Utc.with_ymd_and_hms(2024, 5, 21, 6, 30, 0).unwrap();
        let aggregate_id =
            CounterId::try_from_uuid(aggregate_uuid).expect("uuidv7 should be accepted");
        let state = CounterState::new(aggregate_id, 11);

        let pg_snapshot = PgSnapshotRow {
            id: snapshot_id,
            aggregate_type: Counter::AGGREGATE_TYPE.to_string(),
            aggregate_id: aggregate_uuid,
            aggregate_version: 5,
            state: serde_json::to_value(&state).expect("state should serialize"),
            materialized_at,
        };

        let snapshot = repository
            .map_snapshot(pg_snapshot)
            .expect("expected snapshot to map successfully");

        assert_eq!(Uuid::from(snapshot.id()), snapshot_id);
        assert_eq!(Uuid::from(snapshot.aggregate_id()), aggregate_uuid);
        assert_eq!(snapshot.aggregate_version().value(), 5);
        assert_eq!(snapshot.state(), &state);
        let mapped_materialized_at: chrono::DateTime<Utc> = snapshot.materialized_at().into();
        assert_eq!(mapped_materialized_at, materialized_at);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn map_snapshot_returns_error_on_invalid_state() {
        let repository = create_repository();
        let snapshot_id = Uuid::now_v7();
        let aggregate_uuid = Uuid::now_v7();

        let pg_snapshot = PgSnapshotRow {
            id: snapshot_id,
            aggregate_type: Counter::AGGREGATE_TYPE.to_string(),
            aggregate_id: aggregate_uuid,
            aggregate_version: 2,
            state: json!({ "value": 4 }),
            materialized_at: Utc::now(),
        };

        let err = repository
            .map_snapshot(pg_snapshot)
            .expect_err("expected aggregate state conversion error");

        match err {
            RepositoryError::AggregateState(CounterStateError::Json(_)) => {}
            other => panic!("unexpected error variant: {other:?}"),
        }
    }
}

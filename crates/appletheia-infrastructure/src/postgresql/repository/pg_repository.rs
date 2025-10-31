use sqlx::PgPool;

use crate::postgresql::event::PgEventModel;
use crate::postgresql::snapshot::PgSnapshotModel;
use appletheia_domain::{
    Aggregate, AggregateId, AggregateState, AggregateVersion, CreatedAt, Event, EventId,
    EventPayload, Repository, RepositoryError, Snapshot, SnapshotId,
};

use std::marker::PhantomData;

pub struct PgRepository<A: Aggregate> {
    pool: PgPool,
    _phantom: PhantomData<A>,
}

impl<A: Aggregate> PgRepository<A> {
    fn map_event(
        &self,
        event: PgEventModel,
    ) -> Result<Event<A::Id, A::EventPayload>, RepositoryError<A, sqlx::Error>> {
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
            CreatedAt::from(event.created_at),
        ))
    }

    fn map_snapshot(
        &self,
        snapshot: PgSnapshotModel,
    ) -> Result<Snapshot<A::State>, RepositoryError<A, sqlx::Error>> {
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
            CreatedAt::from(snapshot.created_at),
        ))
    }

    async fn read_events(
        &self,
        aggregate_id: A::Id,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        RepositoryError<A, sqlx::Error>,
    > {
        let snapshot_row = sqlx::query_as::<_, PgSnapshotModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, created_at
            FROM snapshots WHERE aggregate_type = $1 AND aggregate_id = $2
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value().value())
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Persistence)?;
        let snapshot = snapshot_row.map(|row| self.map_snapshot(row)).transpose()?;

        let version_value_after = snapshot
            .as_ref()
            .map(|s| s.aggregate_version().value())
            .unwrap_or(0);

        let event_rows = sqlx::query_as::<_, PgEventModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, payload, created_at
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version > $3
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value().value())
        .bind(version_value_after)
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::Persistence)?;

        let events = event_rows
            .into_iter()
            .map(|row| self.map_event(row))
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A, sqlx::Error>>>(
            )?;

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
        RepositoryError<A, sqlx::Error>,
    > {
        let snapshot_row = sqlx::query_as::<_, PgSnapshotModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, created_at
            FROM snapshots WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version <= $3
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value().value())
        .bind(version_at.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Persistence)?;
        let snapshot = snapshot_row.map(|row| self.map_snapshot(row)).transpose()?;

        let version_value_after = snapshot
            .as_ref()
            .map(|s| s.aggregate_version().value())
            .unwrap_or(0);

        let event_rows = sqlx::query_as::<_, PgEventModel>(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, payload, created_at
            FROM events WHERE aggregate_type = $1 AND aggregate_id = $2
                AND aggregate_version > $3 AND aggregate_version <= $4
            ORDER BY aggregate_version ASC
            "#,
        )
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value().value())
        .bind(version_value_after)
        .bind(version_at.value())
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::Persistence)?;

        let events = event_rows
            .into_iter()
            .map(|row| self.map_event(row))
            .collect::<Result<Vec<Event<A::Id, A::EventPayload>>, RepositoryError<A, sqlx::Error>>>(
            )?;

        Ok((events, snapshot))
    }
}

impl<A: Aggregate> Repository<A, sqlx::Error> for PgRepository<A> {
    async fn find(&self, id: A::Id) -> Result<Option<A>, RepositoryError<A, sqlx::Error>> {
        let (events, snapshot) = self.read_events(id).await?;
        if events.is_empty() && snapshot.is_none() {
            return Ok(None);
        }
        let mut aggregate = A::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::Aggregate)?;
        Ok(Some(aggregate))
    }

    async fn find_at_version(
        &self,
        id: A::Id,
        version_at: AggregateVersion,
    ) -> Result<Option<A>, RepositoryError<A, sqlx::Error>> {
        let (events, snapshot) = self.read_events_at_version(id, version_at).await?;
        if events.is_empty() && snapshot.is_none() {
            return Ok(None);
        }
        let mut aggregate = A::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::Aggregate)?;
        Ok(Some(aggregate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fmt, marker::PhantomData};

    use chrono::{TimeZone, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    use appletheia_domain::{
        AggregateError, AggregateState, AggregateVersion, AggregateVersionError, Event,
        EventPayload, Id, IdError,
    };

    #[derive(Clone, Debug)]
    struct CounterAggregate {
        id: CounterAggregateId,
        state: Option<CounterAggregateState>,
        version: AggregateVersion,
        uncommitted_events: Vec<Event<CounterAggregateId, CounterAggregateEventPayload>>,
    }

    impl CounterAggregate {
        fn new() -> Self {
            let id = CounterAggregateId::from_uuid(Uuid::now_v7())
                .expect("uuidv7 generation should succeed");
            Self {
                id,
                state: None,
                version: AggregateVersion::default(),
                uncommitted_events: Vec::new(),
            }
        }
    }

    impl Default for CounterAggregate {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Aggregate for CounterAggregate {
        type Id = CounterAggregateId;
        type State = CounterAggregateState;
        type EventPayload = CounterAggregateEventPayload;
        type Error = CounterAggregateError;

        const AGGREGATE_TYPE: &'static str = "counter_aggregate";

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
            match payload {
                CounterAggregateEventPayload::Created {
                    aggregate_id,
                    value,
                } => {
                    let id = CounterAggregateId::from_uuid(*aggregate_id)?;
                    self.state = Some(CounterAggregateState { id, value: *value });
                    self.id = id;
                }
                CounterAggregateEventPayload::Incremented { by } => {
                    if let Some(state) = self.state.as_mut() {
                        state.value += *by;
                    } else {
                        return Err(CounterAggregateError::MissingState);
                    }
                }
            }

            Ok(())
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterAggregateState {
        id: CounterAggregateId,
        value: i32,
    }

    impl AggregateState for CounterAggregateState {
        type Id = CounterAggregateId;
        type Error = CounterAggregateStateError;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    enum CounterAggregateEventPayload {
        Created { aggregate_id: Uuid, value: i32 },
        Incremented { by: i32 },
    }

    impl EventPayload for CounterAggregateEventPayload {
        type Error = CounterAggregateEventPayloadError;
    }

    #[derive(Debug)]
    enum CounterAggregateEventPayloadError {
        Json(serde_json::Error),
    }

    impl fmt::Display for CounterAggregateEventPayloadError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Json(err) => write!(f, "json error: {err}"),
            }
        }
    }

    impl std::error::Error for CounterAggregateEventPayloadError {}

    impl From<serde_json::Error> for CounterAggregateEventPayloadError {
        fn from(value: serde_json::Error) -> Self {
            Self::Json(value)
        }
    }

    #[derive(Debug)]
    enum CounterAggregateStateError {
        Json(serde_json::Error),
    }

    impl fmt::Display for CounterAggregateStateError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Json(err) => write!(f, "json error: {err}"),
            }
        }
    }

    impl std::error::Error for CounterAggregateStateError {}

    impl From<serde_json::Error> for CounterAggregateStateError {
        fn from(value: serde_json::Error) -> Self {
            Self::Json(value)
        }
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
    #[serde(try_from = "Uuid", into = "Uuid")]
    struct CounterAggregateId(Id);

    impl CounterAggregateId {
        fn from_uuid(value: Uuid) -> Result<Self, CounterAggregateIdError> {
            Ok(Self(
                Id::try_from(value).map_err(CounterAggregateIdError::from)?,
            ))
        }
    }

    impl AggregateId for CounterAggregateId {
        type Error = CounterAggregateIdError;

        fn value(self) -> Id {
            self.0
        }

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
            Self::from_uuid(value)
        }
    }

    impl From<CounterAggregateId> for Uuid {
        fn from(value: CounterAggregateId) -> Self {
            value.0.value()
        }
    }

    impl TryFrom<Uuid> for CounterAggregateId {
        type Error = CounterAggregateIdError;

        fn try_from(value: Uuid) -> Result<Self, Self::Error> {
            Self::from_uuid(value)
        }
    }

    impl fmt::Display for CounterAggregateId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    enum CounterAggregateIdError {
        Id(IdError),
    }

    impl fmt::Display for CounterAggregateIdError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Id(err) => write!(f, "aggregate id error: {err}"),
            }
        }
    }

    impl std::error::Error for CounterAggregateIdError {}

    impl From<IdError> for CounterAggregateIdError {
        fn from(value: IdError) -> Self {
            Self::Id(value)
        }
    }

    #[derive(Debug)]
    enum CounterAggregateError {
        Aggregate(AggregateError<CounterAggregateId>),
        Id(CounterAggregateIdError),
        MissingState,
    }

    impl fmt::Display for CounterAggregateError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Aggregate(err) => write!(f, "{err}"),
                Self::Id(err) => write!(f, "{err}"),
                Self::MissingState => write!(f, "aggregate state is missing"),
            }
        }
    }

    impl std::error::Error for CounterAggregateError {}

    impl From<AggregateError<CounterAggregateId>> for CounterAggregateError {
        fn from(value: AggregateError<CounterAggregateId>) -> Self {
            Self::Aggregate(value)
        }
    }

    impl From<CounterAggregateIdError> for CounterAggregateError {
        fn from(value: CounterAggregateIdError) -> Self {
            Self::Id(value)
        }
    }

    fn create_repository() -> PgRepository<CounterAggregate> {
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
        let created_at = Utc.with_ymd_and_hms(2024, 5, 20, 12, 0, 0).unwrap();
        let payload = CounterAggregateEventPayload::Created {
            aggregate_id: aggregate_uuid,
            value: 7,
        };

        let pg_event = PgEventModel {
            id: event_id,
            aggregate_type: CounterAggregate::AGGREGATE_TYPE.to_string(),
            aggregate_id: aggregate_uuid,
            aggregate_version: 3,
            payload: serde_json::to_value(&payload).expect("payload should serialize"),
            created_at,
        };

        let event = repository
            .map_event(pg_event)
            .expect("expected event to map successfully");

        assert_eq!(Uuid::from(event.id()), event_id);
        assert_eq!(Uuid::from(event.aggregate_id()), aggregate_uuid);
        assert_eq!(event.aggregate_version().value(), 3);
        assert_eq!(event.payload(), &payload);
        let mapped_created_at: chrono::DateTime<Utc> = event.created_at().into();
        assert_eq!(mapped_created_at, created_at);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn map_event_returns_error_on_negative_version() {
        let repository = create_repository();
        let event_id = Uuid::now_v7();
        let aggregate_uuid = Uuid::now_v7();
        let payload = CounterAggregateEventPayload::Created {
            aggregate_id: aggregate_uuid,
            value: 1,
        };

        let pg_event = PgEventModel {
            id: event_id,
            aggregate_type: CounterAggregate::AGGREGATE_TYPE.to_string(),
            aggregate_id: aggregate_uuid,
            aggregate_version: -1,
            payload: serde_json::to_value(&payload).expect("payload should serialize"),
            created_at: Utc::now(),
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
        let payload = CounterAggregateEventPayload::Created {
            aggregate_id: invalid_uuid,
            value: 1,
        };

        let pg_event = PgEventModel {
            id: event_id,
            aggregate_type: CounterAggregate::AGGREGATE_TYPE.to_string(),
            aggregate_id: invalid_uuid,
            aggregate_version: 1,
            payload: serde_json::to_value(&payload).expect("payload should serialize"),
            created_at: Utc::now(),
        };

        let err = repository
            .map_event(pg_event)
            .expect_err("expected aggregate id conversion error");

        match err {
            RepositoryError::AggregateId(CounterAggregateIdError::Id(IdError::NotUuidV7(
                value,
            ))) => {
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
        let created_at = Utc.with_ymd_and_hms(2024, 5, 21, 6, 30, 0).unwrap();
        let aggregate_id =
            CounterAggregateId::from_uuid(aggregate_uuid).expect("uuidv7 should be accepted");
        let state = CounterAggregateState {
            id: aggregate_id,
            value: 11,
        };

        let pg_snapshot = PgSnapshotModel {
            id: snapshot_id,
            aggregate_type: CounterAggregate::AGGREGATE_TYPE.to_string(),
            aggregate_id: aggregate_uuid,
            aggregate_version: 5,
            state: serde_json::to_value(&state).expect("state should serialize"),
            created_at,
        };

        let snapshot = repository
            .map_snapshot(pg_snapshot)
            .expect("expected snapshot to map successfully");

        assert_eq!(Uuid::from(snapshot.id()), snapshot_id);
        assert_eq!(Uuid::from(snapshot.aggregate_id()), aggregate_uuid);
        assert_eq!(snapshot.aggregate_version().value(), 5);
        assert_eq!(snapshot.state(), &state);
        let mapped_created_at: chrono::DateTime<Utc> = snapshot.created_at().into();
        assert_eq!(mapped_created_at, created_at);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn map_snapshot_returns_error_on_invalid_state() {
        let repository = create_repository();
        let snapshot_id = Uuid::now_v7();
        let aggregate_uuid = Uuid::now_v7();

        let pg_snapshot = PgSnapshotModel {
            id: snapshot_id,
            aggregate_type: CounterAggregate::AGGREGATE_TYPE.to_string(),
            aggregate_id: aggregate_uuid,
            aggregate_version: 2,
            state: json!({ "value": 4 }),
            created_at: Utc::now(),
        };

        let err = repository
            .map_snapshot(pg_snapshot)
            .expect_err("expected aggregate state conversion error");

        match err {
            RepositoryError::AggregateState(CounterAggregateStateError::Json(_)) => {}
            other => panic!("unexpected error variant: {other:?}"),
        }
    }
}

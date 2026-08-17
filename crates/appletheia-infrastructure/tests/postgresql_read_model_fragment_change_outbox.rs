use appletheia_application::event::{
    AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
    SerializedEventPayload,
};
use appletheia_application::outbox::read_model_fragment_change::ReadModelFragmentChangeOutboxEnqueuer;
use appletheia_application::projection::ProjectorName;
use appletheia_application::read_model::{
    ReadModelFragment, ReadModelFragmentChange, ReadModelFragmentChangeEnvelope,
    ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia_application::request_context::{
    CausationId, CorrelationId, MessageId, Principal, RequestContext,
};
use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkFactory};
use appletheia_domain::{AggregateVersion, EventId, EventOccurredAt};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use appletheia_infrastructure::postgresql::PgUnitOfWorkFactory;
use appletheia_infrastructure::postgresql::outbox::read_model_fragment_change::PgReadModelFragmentChangeOutboxEnqueuer;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct TestFragment {
    id: Uuid,
    value: i64,
}

impl ReadModelObservationSource for TestFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        Vec::new()
    }
}

impl ReadModelFragment for TestFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("test_fragment");

    type Key = Uuid;

    fn key(&self) -> Self::Key {
        self.id
    }
}

fn event_envelope(sequence: i64) -> EventEnvelope {
    let correlation_id = CorrelationId::from(Uuid::now_v7());
    let message_id = MessageId::new();
    let context = RequestContext::new(correlation_id, message_id, Principal::System)
        .expect("system request context should be valid");

    EventEnvelope {
        event_sequence: EventSequence::try_from(sequence).expect("sequence should be valid"),
        event_id: EventId::new(),
        aggregate_type: AggregateTypeOwned::try_from("test_item")
            .expect("aggregate type should be valid"),
        aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
        aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
        event_name: EventNameOwned::try_from("item_changed").expect("event name should be valid"),
        payload: SerializedEventPayload::try_from(serde_json::json!({ "value": sequence }))
            .expect("payload should be valid"),
        occurred_at: EventOccurredAt::now(),
        correlation_id,
        causation_id: CausationId::from(message_id),
        context,
    }
}

#[sqlx::test(migrations = "migrations/postgresql")]
#[ignore = "requires a PostgreSQL server with the migration extensions installed"]
async fn enqueue_persists_one_partition_change_group(pool: PgPool) {
    let event = event_envelope(1);
    let fragment = TestFragment {
        id: Uuid::now_v7(),
        value: 42,
    };
    let change =
        ReadModelFragmentChange::try_from_fragment(&fragment).expect("fragment should serialize");
    let envelope = ReadModelFragmentChangeEnvelope::from_changes(
        vec![change],
        &event,
        ProjectorName::new("test_projector"),
    )
    .expect("fragment change should finalize");

    let factory = PgUnitOfWorkFactory::new(pool.clone());
    let mut uow = factory.begin().await.expect("uow should begin");
    PgReadModelFragmentChangeOutboxEnqueuer::new()
        .enqueue_fragment_changes(&mut uow, &[envelope])
        .await
        .expect("fragment changes should enqueue");
    uow.commit().await.expect("uow should commit");

    let row = sqlx::query_as::<_, (serde_json::Value, serde_json::Value)>(
        "SELECT partition, changes FROM read_model_fragment_change_outbox",
    )
    .fetch_one(&pool)
    .await
    .expect("outbox row should exist");

    assert_eq!(
        row.0,
        serde_json::to_value(
            fragment
                .partition()
                .try_into_serialized::<TestFragment>()
                .expect("partition should serialize"),
        )
        .unwrap()
    );
    assert_eq!(row.1.as_array().map(Vec::len), Some(1));
}

#[sqlx::test(migrations = "migrations/postgresql")]
#[ignore = "requires a PostgreSQL server with the migration extensions installed"]
async fn duplicate_projector_event_partition_is_idempotent(pool: PgPool) {
    let event = event_envelope(1);
    let fragment = TestFragment {
        id: Uuid::now_v7(),
        value: 42,
    };
    let make_envelope = || {
        ReadModelFragmentChangeEnvelope::from_changes(
            vec![ReadModelFragmentChange::try_from_fragment(&fragment).unwrap()],
            &event,
            ProjectorName::new("test_projector"),
        )
        .expect("fragment change should finalize")
    };

    let factory = PgUnitOfWorkFactory::new(pool.clone());
    for _ in 0..2 {
        let envelope = make_envelope();
        let mut uow = factory.begin().await.expect("uow should begin");
        PgReadModelFragmentChangeOutboxEnqueuer::new()
            .enqueue_fragment_changes(&mut uow, &[envelope])
            .await
            .expect("duplicate enqueue should be accepted");
        uow.commit().await.expect("uow should commit");
    }

    let count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM read_model_fragment_change_outbox")
            .fetch_one(&pool)
            .await
            .expect("row count should load");

    assert_eq!(count, 1);
}

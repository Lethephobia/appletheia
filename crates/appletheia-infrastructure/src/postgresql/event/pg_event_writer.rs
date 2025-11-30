use std::marker::PhantomData;

use appletheia_application::{
    event::EventWriter,
    outbox::OutboxId,
    request_context::{RequestContext, RequestContextAccess},
    unit_of_work::UnitOfWorkError,
};
use appletheia_domain::{Aggregate, AggregateId, Event};
use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder, Transaction};

use super::PgEventRow;

pub struct PgEventWriter<'c, A: Aggregate> {
    transaction: &'c mut Transaction<'static, Postgres>,
    request_context: RequestContext,
    _aggregate: PhantomData<A>,
}

impl<'c, A: Aggregate> PgEventWriter<'c, A> {
    pub fn new(
        transaction: &'c mut Transaction<'static, Postgres>,
        request_context: RequestContext,
    ) -> Self {
        Self {
            transaction,
            request_context,
            _aggregate: PhantomData,
        }
    }
}

impl<'c, A: Aggregate> RequestContextAccess for PgEventWriter<'c, A> {
    fn request_context(&self) -> &RequestContext {
        &self.request_context
    }
}

impl<'c, A: Aggregate> EventWriter<A> for PgEventWriter<'c, A> {
    type Error = UnitOfWorkError<A>;

    async fn write_events_and_outbox(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), Self::Error> {
        if events.is_empty() {
            return Ok(());
        }

        let correlation_id = self.request_context.correlation_id.0;
        let causation_id = self.request_context.message_id.value();
        let context_json =
            serde_json::to_value(&self.request_context).map_err(UnitOfWorkError::Json)?;

        let mut events_query = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO events (
                id, aggregate_type, aggregate_id, aggregate_version,
                payload, occurred_at, correlation_id, causation_id, context
            ) VALUES
            "#,
        );

        let mut sep = events_query.separated(", ");
        for event in events {
            let id = event.id().value();
            let aggregate_id = event.aggregate_id().value();
            let version = event.aggregate_version().value();
            let payload = serde_json::to_value(event.payload()).map_err(UnitOfWorkError::Json)?;
            let occurred_at: DateTime<Utc> = event.occurred_at().into();

            sep.push("(")
                .push_bind(id)
                .push_bind(A::AGGREGATE_TYPE.value())
                .push_bind(aggregate_id)
                .push_bind(version)
                .push_bind(payload)
                .push_bind(occurred_at)
                .push_bind(correlation_id)
                .push_bind(causation_id)
                .push_bind(&context_json)
                .push(")");
        }
        events_query.push(
            r#"
            RETURNING
                event_sequence,
                id,
                aggregate_type,
                aggregate_id,
                aggregate_version,
                payload,
                occurred_at,
                correlation_id,
                causation_id,
                context
            "#,
        );

        let event_rows = events_query
            .build_query_as::<PgEventRow>()
            .fetch_all(self.transaction.as_mut())
            .await
            .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;

        let mut outbox_query = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO outbox (
                id, event_sequence, event_id, aggregate_type, aggregate_id,
                aggregate_version, payload, occurred_at,
                correlation_id, causation_id
            ) VALUES
            "#,
        );
        let mut sep = outbox_query.separated(", ");
        for event_row in event_rows {
            let outbox_id = OutboxId::new().value();

            sep.push("(")
                .push_bind(outbox_id)
                .push_bind(event_row.event_sequence)
                .push_bind(event_row.id)
                .push_bind(event_row.aggregate_type)
                .push_bind(event_row.aggregate_id)
                .push_bind(event_row.aggregate_version)
                .push_bind(event_row.payload)
                .push_bind(event_row.occurred_at)
                .push_bind(event_row.correlation_id)
                .push_bind(event_row.causation_id)
                .push(")");
        }
        outbox_query
            .build()
            .execute(self.transaction.as_mut())
            .await
            .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;

        Ok(())
    }
}

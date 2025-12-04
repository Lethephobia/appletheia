use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use appletheia_application::outbox::{
    Outbox, OutboxDispatchError, OutboxLifecycle, OutboxWriter, OutboxWriterError,
};

pub struct PgOutboxWriter<'c> {
    transaction: &'c mut Transaction<'static, Postgres>,
}

impl<'c> PgOutboxWriter<'c> {
    pub fn new(transaction: &'c mut Transaction<'static, Postgres>) -> Self {
        Self { transaction }
    }

    fn serialize_last_error(
        outbox: &Outbox,
    ) -> Result<Option<serde_json::Value>, OutboxWriterError> {
        match &outbox.last_error {
            Some(error) => {
                let json = serde_json::to_value(error as &OutboxDispatchError)
                    .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
                Ok(Some(json))
            }
            None => Ok(None),
        }
    }

    async fn update_outbox_row(&mut self, outbox: &Outbox) -> Result<(), OutboxWriterError> {
        let outbox_id: Uuid = outbox.id.value();

        let published_at_value = outbox.state.published_at().map(DateTime::<Utc>::from);
        let attempt_count_value = outbox.state.attempt_count().value();
        let next_attempt_after_value = outbox.state.next_attempt_after().map(DateTime::<Utc>::from);
        let lease_owner_value = outbox.state.lease_owner().map(ToString::to_string);
        let lease_until_value = outbox.state.lease_until().map(DateTime::<Utc>::from);

        let last_error_value = Self::serialize_last_error(outbox)?;

        sqlx::query(
            r#"
            UPDATE outbox
               SET published_at = $2,
                   attempt_count = $3,
                   next_attempt_after = COALESCE($4, next_attempt_after),
                   lease_owner = $5,
                   lease_until = $6,
                   last_error = $7
             WHERE id = $1
            "#,
        )
        .bind(outbox_id)
        .bind(published_at_value)
        .bind(attempt_count_value)
        .bind(next_attempt_after_value)
        .bind(lease_owner_value)
        .bind(lease_until_value)
        .bind(last_error_value)
        .execute(self.transaction.as_mut())
        .await
        .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn insert_dead_letters(
        &mut self,
        dead_lettered_outboxes: &[&Outbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO dead_letters (
                outbox_id,
                event_sequence,
                event_id,
                aggregate_type,
                aggregate_id,
                aggregate_version,
                payload,
                occurred_at,
                correlation_id,
                causation_id,
                context,
                published_at,
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
                last_error,
                dead_lettered_at
            ) VALUES
            "#,
        );

        {
            let mut separated = query_builder.separated(", ");
            for outbox in dead_lettered_outboxes {
                let outbox_id: Uuid = outbox.id.value();

                let event = &outbox.event;
                let event_sequence_value = event.event_sequence.value();
                let event_id_value = event.event_id.value();
                let aggregate_type_value = event.aggregate_type.value().to_owned();
                let aggregate_id_value = event.aggregate_id.value();
                let aggregate_version_value = event.aggregate_version.value();
                let payload_value = event.payload.value().clone();
                let occurred_at_value: DateTime<Utc> = event.occurred_at.into();
                let correlation_id_value: Uuid = event.correlation_id.0;
                let causation_id_value: Uuid = event.causation_id.value();
                let context_value = serde_json::to_value(&event.context)
                    .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

                let published_at_value = outbox.state.published_at().map(DateTime::<Utc>::from);
                let attempt_count_value = outbox.state.attempt_count().value();
                let next_attempt_after_value =
                    outbox.state.next_attempt_after().map(DateTime::<Utc>::from);
                let lease_owner_value = outbox.state.lease_owner().map(ToString::to_string);
                let lease_until_value = outbox.state.lease_until().map(DateTime::<Utc>::from);

                let last_error_value = Self::serialize_last_error(outbox)?;

                let dead_lettered_at_value = match outbox.lifecycle {
                    OutboxLifecycle::DeadLettered { dead_lettered_at } => {
                        DateTime::<Utc>::from(dead_lettered_at)
                    }
                    OutboxLifecycle::Active => {
                        // This should not happen for dead-letter insertion.
                        Utc::now()
                    }
                };

                separated
                    .push("(")
                    .push_bind(outbox_id)
                    .push_bind(event_sequence_value)
                    .push_bind(event_id_value)
                    .push_bind(aggregate_type_value)
                    .push_bind(aggregate_id_value)
                    .push_bind(aggregate_version_value)
                    .push_bind(payload_value)
                    .push_bind(occurred_at_value)
                    .push_bind(correlation_id_value)
                    .push_bind(causation_id_value)
                    .push_bind(context_value)
                    .push_bind(published_at_value)
                    .push_bind(attempt_count_value)
                    .push_bind(next_attempt_after_value)
                    .push_bind(lease_owner_value)
                    .push_bind(lease_until_value)
                    .push_bind(last_error_value)
                    .push_bind(dead_lettered_at_value)
                    .push(")");
            }
        }

        query_builder
            .build()
            .execute(self.transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn delete_outboxes(
        &mut self,
        dead_lettered_outboxes: &[&Outbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder = QueryBuilder::<Postgres>::new("DELETE FROM outbox WHERE id IN (");

        {
            let mut separated = query_builder.separated(", ");
            for outbox in dead_lettered_outboxes {
                let outbox_id: Uuid = outbox.id.value();
                separated.push_bind(outbox_id);
            }
        }

        query_builder.push(")");

        query_builder
            .build()
            .execute(self.transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }
}

impl<'c> OutboxWriter for PgOutboxWriter<'c> {
    async fn write_outbox(&mut self, outboxes: &[Outbox]) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut dead_lettered_outboxes = Vec::new();
        for outbox in outboxes {
            if matches!(outbox.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
                dead_lettered_outboxes.push(outbox);
            }
        }

        for outbox in outboxes {
            self.update_outbox_row(outbox).await?;
        }

        if !dead_lettered_outboxes.is_empty() {
            self.insert_dead_letters(&dead_lettered_outboxes).await?;
            self.delete_outboxes(&dead_lettered_outboxes).await?;
        }

        Ok(())
    }
}

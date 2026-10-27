use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder};

use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::{
    OutboxLifecycle, OutboxWriter, OutboxWriterError, read_model_change::ReadModelChangeOutbox,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgReadModelChangeOutboxWriter;

impl PgReadModelChangeOutboxWriter {
    pub fn new() -> Self {
        Self
    }

    fn serialize_last_error(
        outbox: &ReadModelChangeOutbox,
    ) -> Result<Option<serde_json::Value>, OutboxWriterError> {
        match &outbox.last_error {
            Some(error) => {
                let json = serde_json::to_value(error as &PublishDispatchError)
                    .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
                Ok(Some(json))
            }
            None => Ok(None),
        }
    }

    async fn upsert_outbox_rows(
        uow: &mut PgUnitOfWork,
        outboxes: &[&ReadModelChangeOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO read_model_change_outbox (
                id,
                change_id,
                source_event_id,
                changed_at,
                read_model_type,
                read_model_id,
                published_at,
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
                last_error
            ) VALUES
            "#,
        );

        {
            for (index, outbox) in outboxes.iter().enumerate() {
                if index > 0 {
                    query_builder.push(", ");
                }
                let change = &outbox.change;
                let last_error_value = Self::serialize_last_error(outbox)?;
                let next_attempt_after_value = outbox
                    .state
                    .next_attempt_after()
                    .unwrap_or_default()
                    .value();

                query_builder.push("(");
                let mut separated = query_builder.separated(", ");
                separated
                    .push_bind(outbox.id.value())
                    .push_bind(change.change_id.value())
                    .push_bind(change.source_event_id.value())
                    .push_bind(DateTime::<Utc>::from(change.changed_at))
                    .push_bind(change.change.read_model_ref().read_model_type.value())
                    .push_bind(change.change.read_model_ref().id.value())
                    .push_bind(outbox.state.published_at().map(DateTime::<Utc>::from))
                    .push_bind(outbox.state.attempt_count().value())
                    .push_bind(next_attempt_after_value)
                    .push_bind(outbox.state.lease_owner().map(|owner| owner.value()))
                    .push_bind(outbox.state.lease_until().map(DateTime::<Utc>::from))
                    .push_bind(last_error_value)
                    .push_unseparated(")");
            }
        }

        query_builder.push(
            r#"
            ON CONFLICT (id) DO UPDATE
               SET published_at = EXCLUDED.published_at,
                   attempt_count = EXCLUDED.attempt_count,
                   next_attempt_after = EXCLUDED.next_attempt_after,
                   lease_owner = EXCLUDED.lease_owner,
                   lease_until = EXCLUDED.lease_until,
                   last_error = EXCLUDED.last_error
            "#,
        );

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn insert_dead_letters(
        uow: &mut PgUnitOfWork,
        dead_lettered_outboxes: &[&ReadModelChangeOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO read_model_change_dead_letters (
                read_model_change_outbox_id,
                change_id,
                source_event_id,
                changed_at,
                read_model_type,
                read_model_id,
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
            for (index, outbox) in dead_lettered_outboxes.iter().enumerate() {
                if index > 0 {
                    query_builder.push(", ");
                }
                let outbox_id = outbox.id.value();
                let change = &outbox.change;
                let last_error_value = Self::serialize_last_error(outbox)?;
                let dead_lettered_at_value = match outbox.lifecycle {
                    OutboxLifecycle::DeadLettered { dead_lettered_at } => {
                        DateTime::<Utc>::from(dead_lettered_at)
                    }
                    OutboxLifecycle::Active => Utc::now(),
                };

                query_builder.push("(");
                let mut separated = query_builder.separated(", ");
                separated
                    .push_bind(outbox_id)
                    .push_bind(change.change_id.value())
                    .push_bind(change.source_event_id.value())
                    .push_bind(DateTime::<Utc>::from(change.changed_at))
                    .push_bind(change.change.read_model_ref().read_model_type.value())
                    .push_bind(change.change.read_model_ref().id.value())
                    .push_bind(outbox.state.published_at().map(DateTime::<Utc>::from))
                    .push_bind(outbox.state.attempt_count().value())
                    .push_bind(outbox.state.next_attempt_after().map(DateTime::<Utc>::from))
                    .push_bind(outbox.state.lease_owner().map(|owner| owner.value()))
                    .push_bind(outbox.state.lease_until().map(DateTime::<Utc>::from))
                    .push_bind(last_error_value)
                    .push_bind(dead_lettered_at_value)
                    .push_unseparated(")");
            }
        }

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn delete_outboxes(
        uow: &mut PgUnitOfWork,
        dead_lettered_outboxes: &[&ReadModelChangeOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder =
            QueryBuilder::<Postgres>::new("DELETE FROM read_model_change_outbox WHERE id IN (");

        {
            let mut separated = query_builder.separated(", ");
            for outbox in dead_lettered_outboxes {
                separated.push_bind(outbox.id.value());
            }
        }

        query_builder.push(")");

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn delete_dead_letters(
        uow: &mut PgUnitOfWork,
        active_outboxes: &[&ReadModelChangeOutbox],
    ) -> Result<(), OutboxWriterError> {
        if active_outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            DELETE FROM read_model_change_dead_letters
             WHERE read_model_change_outbox_id IN (
            "#,
        );

        {
            let mut separated = query_builder.separated(", ");
            for outbox in active_outboxes {
                separated.push_bind(outbox.id.value());
            }
        }

        query_builder.push(")");

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }
}

impl Default for PgReadModelChangeOutboxWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboxWriter for PgReadModelChangeOutboxWriter {
    type Uow = PgUnitOfWork;
    type Outbox = ReadModelChangeOutbox;

    async fn write_outbox(
        &self,
        uow: &mut Self::Uow,
        outboxes: &[ReadModelChangeOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut active_outboxes: Vec<&ReadModelChangeOutbox> = Vec::new();
        let mut dead_lettered_outboxes: Vec<&ReadModelChangeOutbox> = Vec::new();
        for outbox in outboxes {
            if matches!(outbox.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
                dead_lettered_outboxes.push(outbox);
            } else {
                active_outboxes.push(outbox);
            }
        }

        Self::upsert_outbox_rows(uow, &active_outboxes).await?;
        Self::delete_dead_letters(uow, &active_outboxes).await?;

        if !dead_lettered_outboxes.is_empty() {
            Self::insert_dead_letters(uow, &dead_lettered_outboxes).await?;
            Self::delete_outboxes(uow, &dead_lettered_outboxes).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postgresql::outbox::read_model_change::{
        PgReadModelChangeOutboxEnqueuer, PgReadModelChangeOutboxFetcher, PgReadModelChangeOutboxRow,
    };
    use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;
    use appletheia_application::outbox::{
        Outbox, OutboxBatchSize, OutboxFetcher, OutboxLeaseDuration, OutboxRelayId,
        OutboxRetryOptions, read_model_change::ReadModelChangeOutboxEnqueuer,
    };
    use appletheia_application::read_model::{
        ReadModelChange, ReadModelChangeEnvelope, ReadModelRef,
    };
    use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkFactory};
    use appletheia_domain::EventId;
    use sqlx::PgPool;

    #[tokio::test]
    #[ignore = "requires PostgreSQL via READ_MODEL_CHANGE_TEST_DATABASE_URL"]
    async fn postgres_read_model_change_outbox_lifecycle() {
        let url = std::env::var("READ_MODEL_CHANGE_TEST_DATABASE_URL").unwrap();
        let pool = PgPool::connect(&url).await.unwrap();
        let factory = PgUnitOfWorkFactory::new(pool);
        let mut uow = factory.begin().await.unwrap();
        sqlx::raw_sql("CREATE SCHEMA read_model_change_outbox_test; SET LOCAL search_path TO read_model_change_outbox_test;")
        .execute(uow.transaction_mut().as_mut())
        .await
        .unwrap();
        sqlx::raw_sql(include_str!(
            "../../../../migrations/postgresql/0001_init.up.sql"
        ))
        .execute(uow.transaction_mut().as_mut())
        .await
        .unwrap();
        let source_event_id = EventId::new();
        let changes: Vec<_> = ["a", "b"]
            .into_iter()
            .map(|id| {
                ReadModelChangeEnvelope::new(
                    source_event_id,
                    ReadModelChange::new(ReadModelRef {
                        read_model_type: "users".parse().unwrap(),
                        id: id.into(),
                    }),
                )
            })
            .collect();
        for change in &changes {
            PgReadModelChangeOutboxEnqueuer
                .enqueue(&mut uow, change)
                .await
                .unwrap();
        }
        let limit = OutboxBatchSize::new(10.try_into().unwrap());
        let mut pending = PgReadModelChangeOutboxFetcher
            .fetch_pending(&mut uow, limit)
            .await
            .unwrap();
        assert_eq!(pending.len(), 2);
        for (outbox, change) in pending.iter().zip(&changes) {
            assert_eq!(&outbox.change, change);
            assert_ne!(outbox.id.value(), change.change_id.value());
        }
        let owner = OutboxRelayId::new();
        for outbox in &mut pending {
            outbox
                .acquire_lease(
                    owner,
                    OutboxLeaseDuration::new(chrono::Duration::seconds(30)),
                )
                .unwrap();
        }
        PgReadModelChangeOutboxWriter
            .write_outbox(&mut uow, &pending)
            .await
            .unwrap();
        assert!(
            PgReadModelChangeOutboxFetcher
                .fetch_pending(&mut uow, limit)
                .await
                .unwrap()
                .is_empty()
        );
        let leased_rows = sqlx::query_as::<_, PgReadModelChangeOutboxRow>(
            "SELECT * FROM read_model_change_outbox",
        )
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .unwrap();
        assert_eq!(leased_rows.len(), 2);
        for row in leased_rows {
            let restored = row.try_into_outbox().unwrap();
            assert_eq!(restored.state.lease_owner(), Some(owner));
        }
        let reason = PublishDispatchError::Permanent {
            code: "invalid".into(),
            message: "rejected".into(),
        };
        let retries = OutboxRetryOptions {
            base_delay: chrono::Duration::zero().into(),
            max_delay: chrono::Duration::seconds(30).into(),
            multiplier: Default::default(),
            max_attempts: Default::default(),
        };
        for outbox in &mut pending {
            outbox.nack(&reason, &retries).unwrap();
        }
        PgReadModelChangeOutboxWriter
            .write_outbox(&mut uow, &pending)
            .await
            .unwrap();
        let mut dead = PgReadModelChangeOutboxFetcher
            .fetch_dead_lettered(&mut uow, limit)
            .await
            .unwrap();
        assert_eq!(dead.len(), 2);
        assert_eq!(dead[0].change, changes[0]);
        assert_eq!(dead[0].last_error, Some(reason));
        for outbox in &mut dead {
            outbox.redrive().unwrap();
        }
        PgReadModelChangeOutboxWriter
            .write_outbox(&mut uow, &dead)
            .await
            .unwrap();
        assert!(
            PgReadModelChangeOutboxFetcher
                .fetch_dead_lettered(&mut uow, limit)
                .await
                .unwrap()
                .is_empty()
        );
        let mut retried = PgReadModelChangeOutboxFetcher
            .fetch_pending(&mut uow, limit)
            .await
            .unwrap();
        assert_eq!(retried.len(), 2);
        assert_eq!(retried[0].id, pending[0].id);
        assert_eq!(retried[0].change, changes[0]);
        for outbox in &mut retried {
            outbox.ack().unwrap();
        }
        PgReadModelChangeOutboxWriter
            .write_outbox(&mut uow, &retried)
            .await
            .unwrap();
        assert!(
            PgReadModelChangeOutboxFetcher
                .fetch_pending(&mut uow, limit)
                .await
                .unwrap()
                .is_empty()
        );
        sqlx::raw_sql(include_str!(
            "../../../../migrations/postgresql/0001_init.down.sql"
        ))
        .execute(uow.transaction_mut().as_mut())
        .await
        .unwrap();
        uow.rollback().await.unwrap();
    }
}

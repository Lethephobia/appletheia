use std::marker::PhantomData;

use appletheia_application::outbox::OutboxId;
use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkConfig, UnitOfWorkError};
use appletheia_domain::{Aggregate, AggregateId, AggregateVersion, Event, Snapshot};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction, query_scalar};

use crate::postgresql::event::PgEventRow;
use crate::postgresql::repository::PgRepository;

#[derive(Debug)]
pub struct PgUnitOfWork<A: Aggregate> {
    pool: PgPool,
    config: UnitOfWorkConfig,
    transaction: Option<Transaction<'static, Postgres>>,
    _aggregate: PhantomData<A>,
}

impl<A: Aggregate> PgUnitOfWork<A> {
    pub fn new(pool: PgPool, config: UnitOfWorkConfig) -> Self {
        Self {
            pool,
            config,
            transaction: None,
            _aggregate: PhantomData,
        }
    }
}

impl<A: Aggregate> UnitOfWork<A> for PgUnitOfWork<A> {
    type Repository<'c>
        = PgRepository<'c, A>
    where
        Self: 'c;

    fn config(&self) -> &UnitOfWorkConfig {
        &self.config
    }

    fn repository(&mut self) -> Result<Self::Repository<'_>, UnitOfWorkError<A>> {
        let transaction = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;
        Ok(PgRepository::new(transaction))
    }

    async fn begin(&mut self) -> Result<(), UnitOfWorkError<A>> {
        if self.is_in_transaction() {
            return Ok(());
        }
        self.transaction = Some(
            self.pool
                .begin()
                .await
                .map_err(|e| UnitOfWorkError::BeginFailed(Box::new(e)))?,
        );
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), UnitOfWorkError<A>> {
        if !self.is_in_transaction() {
            return Err(UnitOfWorkError::NotInTransaction);
        }
        self.transaction
            .take()
            .ok_or(UnitOfWorkError::NotInTransaction)?
            .commit()
            .await
            .map_err(|e| UnitOfWorkError::CommitFailed(Box::new(e)))?;
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), UnitOfWorkError<A>> {
        if !self.is_in_transaction() {
            return Err(UnitOfWorkError::NotInTransaction);
        }
        self.transaction
            .take()
            .ok_or(UnitOfWorkError::NotInTransaction)?
            .rollback()
            .await
            .map_err(|e| UnitOfWorkError::RollbackFailed(Box::new(e)))?;
        Ok(())
    }

    fn is_in_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    async fn write_events_and_outbox(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), UnitOfWorkError<A>> {
        if events.is_empty() {
            return Ok(());
        }

        let correlation_id = self.config.request_context.correlation_id.0;
        let causation_id = self.config.request_context.message_id.value();
        let context_json = serde_json::to_value(self.config.request_context.clone())
            .map_err(UnitOfWorkError::Json)?;

        let tx = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;

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
            .fetch_all(tx.as_mut())
            .await
            .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;

        let mut outbox_query = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO outbox (
                id, event_sequence, event_id, aggregate_type, aggregate_id,
                aggregate_version, payload, occurred_at,
                correlation_id, causation_id, context, ordering_key
            ) VALUES
            "#,
        );
        let mut sep = outbox_query.separated(", ");
        for event_row in event_rows {
            let outbox_id = OutboxId::new().value();
            let ordering_key = format!("{}:{}", event_row.aggregate_type, event_row.aggregate_id);

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
                .push_bind(event_row.context)
                .push_bind(ordering_key)
                .push(")");
        }
        outbox_query
            .build()
            .execute(tx.as_mut())
            .await
            .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn write_snapshot(
        &mut self,
        snapshot: &Snapshot<A::State>,
    ) -> Result<(), UnitOfWorkError<A>> {
        let transaction = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;
        let snapshot_id = snapshot.id().value();
        let state = serde_json::to_value(snapshot.state()).map_err(UnitOfWorkError::Json)?;
        let materialized_at = snapshot.materialized_at().value();
        let aggregate_id = snapshot.aggregate_id().value();
        let aggregate_version = snapshot.aggregate_version().value();
        sqlx::query(
            r#"
            INSERT INTO snapshots (
                id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(snapshot_id)
        .bind(A::AGGREGATE_TYPE.value())
        .bind(aggregate_id)
        .bind(aggregate_version)
        .bind(state)
        .bind(materialized_at)
        .execute(transaction.as_mut())
        .await
        .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn read_latest_snapshot_version(
        &mut self,
        aggregate_id: A::Id,
    ) -> Result<Option<AggregateVersion>, UnitOfWorkError<A>> {
        let transaction = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;
        let latest: Option<i64> = query_scalar(
            r#"
            SELECT aggregate_version
            FROM snapshots
            WHERE aggregate_type = $1 AND aggregate_id = $2
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
        )
        .bind(A::AGGREGATE_TYPE.value())
        .bind(aggregate_id.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;

        latest
            .map(|value| {
                AggregateVersion::try_from(value).map_err(UnitOfWorkError::AggregateVersion)
            })
            .transpose()
    }
}

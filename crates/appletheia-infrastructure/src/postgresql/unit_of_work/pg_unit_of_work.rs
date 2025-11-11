use std::marker::PhantomData;

use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkConfig, UnitOfWorkError};
use appletheia_domain::{Aggregate, AggregateId, AggregateVersion, Event, Snapshot};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction, query_scalar};
use uuid::Uuid;

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

    fn repository<'c>(&'c mut self) -> Result<Self::Repository<'c>, UnitOfWorkError<A>> {
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
            .unwrap()
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
            .unwrap()
            .rollback()
            .await
            .map_err(|e| UnitOfWorkError::RollbackFailed(Box::new(e)))?;
        Ok(())
    }

    fn is_in_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    async fn write_events(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), UnitOfWorkError<A>> {
        if events.is_empty() {
            return Ok(());
        }
        for event in events {
            let event_id = Uuid::from(event.id());
            let aggregate_id = event.aggregate_id().value();
            let aggregate_version = event.aggregate_version().value();
            let payload = serde_json::to_value(event.payload())
                .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;
            let occurred_at: DateTime<Utc> = event.occurred_at().into();
            let correlation_id = self.config.request_context.correlation_id.0;
            let causation_id = self.config.request_context.message_id.value();
            let context = serde_json::to_value(self.config.request_context.clone())
                .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;

            let transaction = self
                .transaction
                .as_mut()
                .ok_or(UnitOfWorkError::NotInTransaction)?;
            sqlx::query(
                r#"
                        INSERT INTO events (
                            id, aggregate_type, aggregate_id, aggregate_version,
                            payload, occurred_at, correlation_id, causation_id, context
                        )
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                        RETURNING event_sequence
                        "#,
            )
            .bind(event_id)
            .bind(A::AGGREGATE_TYPE)
            .bind(aggregate_id)
            .bind(aggregate_version)
            .bind(payload)
            .bind(occurred_at)
            .bind(correlation_id)
            .bind(causation_id)
            .bind(context)
            .execute(transaction.as_mut())
            .await
            .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;
        }
        Ok(())
    }

    async fn write_outbox(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), UnitOfWorkError<A>> {
        if events.is_empty() {
            return Ok(());
        }

        for event in events {
            let payload = serde_json::to_value(event.payload())
                .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;
            let occurred_at: DateTime<Utc> = event.occurred_at().into();
            let aggregate_id = event.aggregate_id().value();
            let aggregate_version = event.aggregate_version().value();
            let event_id = Uuid::from(event.id());
            let outbox_id = Uuid::now_v7();
            let correlation_id = self.config.request_context.correlation_id.0;
            let causation_id = self.config.request_context.message_id.value();
            let context = serde_json::to_value(self.config.request_context.clone())
                .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;
            let ordering_key = self.ordering_key(event.aggregate_id());
            let transaction = self
                .transaction
                .as_mut()
                .ok_or(UnitOfWorkError::NotInTransaction)?;
            sqlx::query(
                r#"
                    INSERT INTO outbox (
                        id, event_sequence, event_id, aggregate_type, aggregate_id,
                        aggregate_version, payload, occurred_at, correlation_id,
                        causation_id, context, ordering_key
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    "#,
            )
            .bind(outbox_id)
            .bind(event_id)
            .bind(A::AGGREGATE_TYPE)
            .bind(aggregate_id)
            .bind(aggregate_version)
            .bind(payload)
            .bind(occurred_at)
            .bind(correlation_id)
            .bind(causation_id)
            .bind(context)
            .bind(ordering_key)
            .execute(transaction.as_mut())
            .await
            .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;
        }
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
        let snapshot_id = Uuid::from(snapshot.id());
        let state = serde_json::to_value(snapshot.state())
            .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;
        let materialized_at: DateTime<Utc> = snapshot.materialized_at().into();
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
        .bind(A::AGGREGATE_TYPE)
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
        .bind(A::AGGREGATE_TYPE)
        .bind(aggregate_id.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;

        latest
            .map(|value| {
                AggregateVersion::try_from(value)
                    .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))
            })
            .transpose()
    }
}

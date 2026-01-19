use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::saga::{
    SagaInstanceRow, SagaInstanceUpdate, SagaName, SagaStore, SagaStoreError,
};
use appletheia_application::unit_of_work::UnitOfWorkError;
use appletheia_domain::EventId;

use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug, Default)]
pub struct PgSagaStore;

impl PgSagaStore {
    pub fn new() -> Self {
        Self
    }

    fn map_uow_error(error: UnitOfWorkError) -> SagaStoreError {
        match error {
            UnitOfWorkError::NotInTransaction => SagaStoreError::NotInTransaction,
            other => SagaStoreError::Persistence(Box::new(other)),
        }
    }
}

#[derive(Debug, FromRow)]
struct PgSagaInstanceRow {
    state: serde_json::Value,
    state_version: i64,
    completed_at: Option<DateTime<Utc>>,
    failed_at: Option<DateTime<Utc>>,
    last_error: Option<serde_json::Value>,
}

impl SagaStore for PgSagaStore {
    type Uow = PgUnitOfWork;

    async fn load_for_update(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: appletheia_application::request_context::CorrelationId,
    ) -> Result<Option<SagaInstanceRow>, SagaStoreError> {
        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let saga_name_value: &str = saga_name.value();
        let correlation_id_value: Uuid = correlation_id.0;

        let row = sqlx::query_as::<_, PgSagaInstanceRow>(
            r#"
            SELECT
              state,
              state_version,
              completed_at,
              failed_at,
              last_error
            FROM saga_instances
            WHERE saga_name = $1
              AND correlation_id = $2
            FOR UPDATE
            "#,
        )
        .bind(saga_name_value)
        .bind(correlation_id_value)
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaStoreError::Persistence(Box::new(source)))?;

        Ok(row.map(|row| SagaInstanceRow {
            state: row.state,
            state_version: row.state_version,
            completed_at: row.completed_at,
            failed_at: row.failed_at,
            last_error: row.last_error,
        }))
    }

    async fn insert_instance_if_absent(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: appletheia_application::request_context::CorrelationId,
        initial_state: serde_json::Value,
    ) -> Result<(), SagaStoreError> {
        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let saga_name_value: &str = saga_name.value();
        let correlation_id_value: Uuid = correlation_id.0;

        sqlx::query(
            r#"
            INSERT INTO saga_instances (
              saga_name,
              correlation_id,
              state
            ) VALUES (
              $1,
              $2,
              $3
            )
            ON CONFLICT (saga_name, correlation_id) DO NOTHING
            "#,
        )
        .bind(saga_name_value)
        .bind(correlation_id_value)
        .bind(initial_state)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| SagaStoreError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn update_instance(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: appletheia_application::request_context::CorrelationId,
        update: SagaInstanceUpdate,
    ) -> Result<(), SagaStoreError> {
        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let saga_name_value: &str = saga_name.value();
        let correlation_id_value: Uuid = correlation_id.0;

        let updated = sqlx::query(
            r#"
            UPDATE saga_instances
               SET state = $3,
                   state_version = state_version + 1,
                   updated_at = now(),
                   completed_at = $4,
                   failed_at = $5,
                   last_error = $6
             WHERE saga_name = $1
               AND correlation_id = $2
            "#,
        )
        .bind(saga_name_value)
        .bind(correlation_id_value)
        .bind(update.state)
        .bind(update.completed_at)
        .bind(update.failed_at)
        .bind(update.last_error)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| SagaStoreError::Persistence(Box::new(source)))?;

        if updated.rows_affected() != 1 {
            return Err(SagaStoreError::Persistence(Box::new(
                std::io::Error::other("failed to update saga instance row"),
            )));
        }

        Ok(())
    }

    async fn mark_event_processed(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: appletheia_application::request_context::CorrelationId,
        event_id: EventId,
    ) -> Result<bool, SagaStoreError> {
        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let saga_name_value: &str = saga_name.value();
        let correlation_id_value: Uuid = correlation_id.0;
        let event_id_value: Uuid = event_id.value();

        let done = sqlx::query(
            r#"
            INSERT INTO saga_processed_events (
              saga_name,
              correlation_id,
              event_id
            ) VALUES (
              $1,
              $2,
              $3
            )
            ON CONFLICT (saga_name, correlation_id, event_id) DO NOTHING
            "#,
        )
        .bind(saga_name_value)
        .bind(correlation_id_value)
        .bind(event_id_value)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| SagaStoreError::Persistence(Box::new(source)))?;

        Ok(done.rows_affected() == 1)
    }
}

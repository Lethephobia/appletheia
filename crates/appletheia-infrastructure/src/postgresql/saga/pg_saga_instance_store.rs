use crate::postgresql::saga::pg_saga_instance_command_row::PgSagaInstanceCommandRow;
use crate::postgresql::saga::pg_saga_instance_row::PgSagaInstanceRow;
use crate::postgresql::unit_of_work::PgUnitOfWork;
use appletheia_application::request_context::{CorrelationId, MessageId};
use appletheia_application::saga::{
    SagaInstance, SagaInstanceStore, SagaInstanceStoreError, SagaNameOwned, SagaState, SagaStatus,
};

#[derive(Debug)]
pub struct PgSagaInstanceStore;

impl PgSagaInstanceStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgSagaInstanceStore {
    fn default() -> Self {
        Self::new()
    }
}

impl PgSagaInstanceStore {
    async fn read_command_message_ids(
        uow: &mut PgUnitOfWork,
        saga_instance_id: uuid::Uuid,
    ) -> Result<Vec<MessageId>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let rows = sqlx::query_as::<_, PgSagaInstanceCommandRow>(
            r#"
            SELECT message_id
            FROM saga_instance_commands
            WHERE saga_instance_id = $1
            ORDER BY created_at ASC, message_id ASC
            "#,
        )
        .bind(saga_instance_id)
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        Ok(rows
            .into_iter()
            .map(PgSagaInstanceCommandRow::into_message_id)
            .collect())
    }
}

impl SagaInstanceStore for PgSagaInstanceStore {
    type Uow = PgUnitOfWork;

    async fn find_by_correlation_id<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<Option<SagaInstance<S>>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let saga_name_value = saga_name.value();
        let correlation_id_value = correlation_id.value();

        let row = sqlx::query_as::<_, PgSagaInstanceRow>(
            r#"
            SELECT
              id,
              correlation_id,
              start_event_id,
              state,
              succeeded_at,
              failed_at
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
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let message_ids = Self::read_command_message_ids(uow, row.id).await?;

        row.try_into_instance::<S>(saga_name, correlation_id, message_ids)
            .map(Some)
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))
    }

    async fn find_by_dispatched_command_message_id<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        dispatched_command_message_id: MessageId,
    ) -> Result<Option<SagaInstance<S>>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let row = sqlx::query_as::<_, PgSagaInstanceRow>(
            r#"
            SELECT
              si.id,
              si.correlation_id,
              si.start_event_id,
              si.state,
              si.succeeded_at,
              si.failed_at
            FROM saga_instances si
            JOIN saga_instance_commands sic
              ON sic.saga_instance_id = si.id
            WHERE si.saga_name = $1
              AND sic.message_id = $2
            FOR UPDATE OF si
            "#,
        )
        .bind(saga_name.value())
        .bind(dispatched_command_message_id.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let message_ids = Self::read_command_message_ids(uow, row.id).await?;
        let correlation_id = CorrelationId::from(row.correlation_id);

        row.try_into_instance::<S>(saga_name, correlation_id, message_ids)
            .map(Some)
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))
    }

    async fn save<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        instance: &SagaInstance<S>,
    ) -> Result<(), SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let saga_instance_id_value = instance.saga_instance_id.value();

        let state_json = match instance.state.as_ref() {
            Some(state) => {
                Some(serde_json::to_value(state).map_err(SagaInstanceStoreError::StateSerialize)?)
            }
            None => None,
        };

        let (completed, failed) = match instance.status {
            SagaStatus::InProgress => (false, false),
            SagaStatus::Succeeded => (true, false),
            SagaStatus::Failed => (false, true),
        };

        let persisted_saga_instance_id = sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
            INSERT INTO saga_instances (
              id,
              saga_name,
              correlation_id,
              start_event_id,
              state,
              succeeded_at,
              failed_at
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5,
              CASE WHEN $6 THEN now() ELSE NULL END,
              CASE WHEN $7 THEN now() ELSE NULL END
            )
            ON CONFLICT (saga_name, correlation_id) DO UPDATE SET
              state = EXCLUDED.state,
              state_version = saga_instances.state_version + 1,
              updated_at = now(),
              succeeded_at = CASE WHEN $6 THEN COALESCE(saga_instances.succeeded_at, now()) ELSE NULL END,
              failed_at = CASE WHEN $7 THEN COALESCE(saga_instances.failed_at, now()) ELSE NULL END
            RETURNING id
            "#,
        )
        .bind(saga_instance_id_value)
        .bind(instance.saga_name.value())
        .bind(instance.correlation_id.value())
        .bind(instance.start_event_id.value())
        .bind(state_json)
        .bind(completed)
        .bind(failed)
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        for message_id in &instance.dispatched_command_message_ids {
            sqlx::query(
                r#"
                INSERT INTO saga_instance_commands (
                  saga_instance_id,
                  message_id
                ) VALUES (
                  $1,
                  $2
                )
                ON CONFLICT DO NOTHING
                "#,
            )
            .bind(persisted_saga_instance_id)
            .bind(message_id.value())
            .execute(transaction.as_mut())
            .await
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;
        }

        Ok(())
    }
}

use appletheia_application::request_context::CorrelationId;
use appletheia_application::saga::{
    SagaInstance, SagaInstanceId, SagaNameOwned, SagaState, SagaStatus, SagaStore, SagaStoreError,
};
use appletheia_application::unit_of_work::UnitOfWorkError;

use super::pg_saga_instance_row_error::PgSagaInstanceRowError;
use crate::postgresql::saga::pg_saga_instance_row::PgSagaInstanceRow;
use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug)]
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

impl Default for PgSagaStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SagaStore for PgSagaStore {
    type Uow = PgUnitOfWork;

    async fn load<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<SagaInstance<S>, SagaStoreError> {
        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let saga_instance_id_value = SagaInstanceId::new().value();
        let saga_name_value = saga_name.value();
        let correlation_id_value = correlation_id.0;

        sqlx::query(
            r#"
            INSERT INTO saga_instances (
              saga_instance_id,
              saga_name,
              correlation_id,
              state
            ) VALUES (
              $1,
              $2,
              $3,
              NULL
            )
            ON CONFLICT (saga_name, correlation_id) DO NOTHING
            "#,
        )
        .bind(saga_instance_id_value)
        .bind(saga_name_value)
        .bind(correlation_id_value)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| SagaStoreError::Persistence(Box::new(source)))?;

        let row = sqlx::query_as::<_, PgSagaInstanceRow>(
            r#"
            SELECT
              saga_instance_id,
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
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| SagaStoreError::Persistence(Box::new(source)))?;

        row.try_into_instance::<S>(saga_name, correlation_id)
            .map_err(|error| match error {
                PgSagaInstanceRowError::SagaInstanceId(_) => {
                    SagaStoreError::InvalidPersistedInstance {
                        message: "saga_instance_id must be a uuidv7",
                    }
                }
                PgSagaInstanceRowError::StateDeserialize(source) => {
                    SagaStoreError::StateDeserialize(source)
                }
                PgSagaInstanceRowError::InvalidPersistedInstance { message } => {
                    SagaStoreError::InvalidPersistedInstance { message }
                }
            })
    }

    async fn save<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        instance: &SagaInstance<S>,
    ) -> Result<(), SagaStoreError> {
        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let saga_instance_id_value = instance.saga_instance_id.value();

        let state_json = match instance.state.as_ref() {
            Some(state) => {
                Some(serde_json::to_value(state).map_err(SagaStoreError::StateSerialize)?)
            }
            None => None,
        };

        let (completed, failed) = match instance.status {
            SagaStatus::InProgress => (false, false),
            SagaStatus::Succeeded => (true, false),
            SagaStatus::Failed => (false, true),
        };

        if matches!(instance.status, SagaStatus::Succeeded | SagaStatus::Failed)
            && state_json.is_none()
        {
            return Err(SagaStoreError::InvalidPersistedInstance {
                message: "terminal saga instance must have non-null state",
            });
        }

        let updated = sqlx::query(
            r#"
            UPDATE saga_instances
               SET state = $2,
                   state_version = state_version + 1,
                   updated_at = now(),
                   succeeded_at = CASE WHEN $3 THEN COALESCE(succeeded_at, now()) ELSE NULL END,
                   failed_at = CASE WHEN $4 THEN COALESCE(failed_at, now()) ELSE NULL END
             WHERE saga_instance_id = $1
            "#,
        )
        .bind(saga_instance_id_value)
        .bind(state_json)
        .bind(completed)
        .bind(failed)
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
}

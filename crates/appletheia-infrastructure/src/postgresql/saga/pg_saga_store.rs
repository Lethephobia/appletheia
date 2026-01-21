use uuid::Uuid;

use appletheia_application::saga::{
    SagaInstance, SagaInstanceId, SagaName, SagaNameOwned, SagaState, SagaStatus, SagaStore,
    SagaStoreError,
};
use appletheia_application::unit_of_work::UnitOfWorkError;

use super::pg_saga_instance_row_error::PgSagaInstanceRowError;
use crate::postgresql::saga::pg_saga_instance_row::PgSagaInstanceRow;
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

impl SagaStore for PgSagaStore {
    type Uow = PgUnitOfWork;

    async fn load<S: SagaState>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: appletheia_application::request_context::CorrelationId,
    ) -> Result<SagaInstance<S>, SagaStoreError> {
        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let saga_instance_id_value: Uuid = SagaInstanceId::new().value();
        let saga_name_value: &str = saga_name.value();
        let correlation_id_value: Uuid = correlation_id.0;

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
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| SagaStoreError::Persistence(Box::new(source)))?;

        let saga_name_owned = SagaNameOwned::from(saga_name);

        row.try_into_instance::<S>(saga_name_owned, correlation_id)
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

        let saga_instance_id_value: Uuid = instance.saga_instance_id.value();

        let (state_json, completed, failed, last_error) = match &instance.status {
            SagaStatus::InProgress { state } => {
                let state_json = match state {
                    Some(state) => {
                        Some(serde_json::to_value(state).map_err(SagaStoreError::StateSerialize)?)
                    }
                    None => None,
                };
                (state_json, false, false, None)
            }
            SagaStatus::Succeeded { state } => {
                let state_json =
                    Some(serde_json::to_value(state).map_err(SagaStoreError::StateSerialize)?);
                (state_json, true, false, None)
            }
            SagaStatus::Failed { state, error } => {
                let state_json =
                    Some(serde_json::to_value(state).map_err(SagaStoreError::StateSerialize)?);
                (state_json, false, true, Some(error.clone()))
            }
        };

        let updated = sqlx::query(
            r#"
            UPDATE saga_instances
               SET state = $2,
                   state_version = state_version + 1,
                   updated_at = now(),
                   succeeded_at = CASE WHEN $3 THEN COALESCE(succeeded_at, now()) ELSE NULL END,
                   failed_at = CASE WHEN $4 THEN COALESCE(failed_at, now()) ELSE NULL END,
                   last_error = CASE WHEN $4 THEN $5 ELSE NULL END
             WHERE saga_instance_id = $1
            "#,
        )
        .bind(saga_instance_id_value)
        .bind(state_json)
        .bind(completed)
        .bind(failed)
        .bind(last_error)
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

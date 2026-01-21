use uuid::Uuid;

use appletheia_application::saga::{
    SagaName, SagaProcessedEventStore, SagaProcessedEventStoreError,
};
use appletheia_application::unit_of_work::UnitOfWorkError;
use appletheia_domain::EventId;

use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug, Default)]
pub struct PgSagaProcessedEventStore;

impl PgSagaProcessedEventStore {
    pub fn new() -> Self {
        Self
    }

    fn map_uow_error(error: UnitOfWorkError) -> SagaProcessedEventStoreError {
        match error {
            UnitOfWorkError::NotInTransaction => SagaProcessedEventStoreError::NotInTransaction,
            other => SagaProcessedEventStoreError::Persistence(Box::new(other)),
        }
    }
}

impl SagaProcessedEventStore for PgSagaProcessedEventStore {
    type Uow = PgUnitOfWork;

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaName,
        correlation_id: appletheia_application::request_context::CorrelationId,
        event_id: EventId,
    ) -> Result<bool, SagaProcessedEventStoreError> {
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
        .map_err(|source| SagaProcessedEventStoreError::Persistence(Box::new(source)))?;

        Ok(done.rows_affected() == 1)
    }
}

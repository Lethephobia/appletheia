use appletheia_application::request_context::CorrelationId;
use appletheia_application::saga::{
    SagaNameOwned, SagaProcessedEventStore, SagaProcessedEventStoreError,
};
use appletheia_domain::EventId;

use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug)]
pub struct PgSagaProcessedEventStore;

impl PgSagaProcessedEventStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgSagaProcessedEventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SagaProcessedEventStore for PgSagaProcessedEventStore {
    type Uow = PgUnitOfWork;

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
        event_id: EventId,
    ) -> Result<bool, SagaProcessedEventStoreError> {
        let transaction = uow.transaction_mut();

        let saga_name_value = saga_name.value();
        let correlation_id_value = correlation_id.0;
        let event_id_value = event_id.value();

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

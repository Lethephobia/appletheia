use std::marker::PhantomData;

use appletheia_application::saga::{SagaProcessedEventStore, SagaProcessedEventStoreError};
use appletheia_application::unit_of_work::UnitOfWorkError;
use appletheia_domain::EventId;

use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug)]
pub struct PgSagaProcessedEventStore<N> {
    _marker: PhantomData<fn() -> N>,
}

impl<N> PgSagaProcessedEventStore<N> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    fn map_uow_error(error: UnitOfWorkError) -> SagaProcessedEventStoreError {
        match error {
            UnitOfWorkError::NotInTransaction => SagaProcessedEventStoreError::NotInTransaction,
            other => SagaProcessedEventStoreError::Persistence(Box::new(other)),
        }
    }
}

impl<N> Default for PgSagaProcessedEventStore<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: appletheia_application::saga::SagaName> SagaProcessedEventStore
    for PgSagaProcessedEventStore<N>
{
    type Uow = PgUnitOfWork;
    type SagaName = N;

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        saga_name: Self::SagaName,
        correlation_id: appletheia_application::request_context::CorrelationId,
        event_id: EventId,
    ) -> Result<bool, SagaProcessedEventStoreError> {
        let transaction = uow.transaction_mut().map_err(Self::map_uow_error)?;

        let saga_name_value = saga_name.to_string();
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
        .bind(&saga_name_value)
        .bind(correlation_id_value)
        .bind(event_id_value)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| SagaProcessedEventStoreError::Persistence(Box::new(source)))?;

        Ok(done.rows_affected() == 1)
    }
}

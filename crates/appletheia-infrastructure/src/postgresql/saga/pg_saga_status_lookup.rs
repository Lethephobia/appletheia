use sqlx::Postgres;

use appletheia_application::request_context::CorrelationId;
use appletheia_application::saga::{
    SagaNameOwned, SagaStatus, SagaStatusLookup, SagaStatusLookupError,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug)]
pub struct PgSagaStatusLookup;

impl PgSagaStatusLookup {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgSagaStatusLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl SagaStatusLookup for PgSagaStatusLookup {
    type Uow = PgUnitOfWork;

    async fn status(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<Option<SagaStatus>, SagaStatusLookupError> {
        let transaction = uow.transaction_mut();

        let row: Option<(bool, bool)> = sqlx::query_as::<Postgres, (bool, bool)>(
            r#"
            SELECT
              succeeded_at IS NOT NULL AS succeeded,
              failed_at IS NOT NULL AS failed
            FROM saga_instances
            WHERE saga_name = $1
              AND correlation_id = $2
            "#,
        )
        .bind(saga_name.value())
        .bind(correlation_id.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaStatusLookupError::Persistence(Box::new(source)))?;

        let Some((succeeded, failed)) = row else {
            return Ok(None);
        };

        match (succeeded, failed) {
            (true, false) => Ok(Some(SagaStatus::Succeeded)),
            (false, true) => Ok(Some(SagaStatus::Failed)),
            (false, false) => Ok(Some(SagaStatus::InProgress)),
            (true, true) => Err(SagaStatusLookupError::InvalidPersistedInstance {
                message: "instance cannot be both succeeded and failed",
            }),
        }
    }
}

use crate::request_context::CorrelationId;
use crate::unit_of_work::UnitOfWork;

use super::{SagaNameOwned, SagaStatus, SagaStatusLookupError};

/// Looks up the current status of a saga instance.
#[allow(async_fn_in_trait)]
pub trait SagaStatusLookup: Send + Sync {
    type Uow: UnitOfWork;

    async fn status(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<Option<SagaStatus>, SagaStatusLookupError>;
}

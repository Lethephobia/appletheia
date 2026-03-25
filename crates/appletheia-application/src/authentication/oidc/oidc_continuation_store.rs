use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::unit_of_work::UnitOfWork;

use super::{OidcContinuation, OidcContinuationStoreError, OidcState};

/// Persists and consumes application-defined OIDC continuations.
#[allow(async_fn_in_trait)]
pub trait OidcContinuationStore<P>: Send + Sync
where
    P: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// The unit of work type used by the store.
    type Uow: UnitOfWork;

    /// Saves a newly created continuation.
    async fn save(
        &self,
        uow: &mut Self::Uow,
        continuation: &OidcContinuation<P>,
    ) -> Result<(), OidcContinuationStoreError>;

    /// Atomically consumes the continuation associated with `state`.
    async fn consume_by_state(
        &self,
        uow: &mut Self::Uow,
        state: &OidcState,
    ) -> Result<OidcContinuation<P>, OidcContinuationStoreError>;
}

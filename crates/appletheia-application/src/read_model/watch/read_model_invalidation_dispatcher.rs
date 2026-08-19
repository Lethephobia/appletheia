use crate::read_model::ReadModelInvalidationEnvelope;

use super::ReadModelWatchRegistryError;

/// Dispatches one internal invalidation to affected server-side subscriptions.
#[allow(async_fn_in_trait)]
pub trait ReadModelInvalidationDispatcher: Send + Sync {
    async fn dispatch(
        &self,
        envelope: &ReadModelInvalidationEnvelope,
    ) -> Result<(), ReadModelWatchRegistryError>;
}

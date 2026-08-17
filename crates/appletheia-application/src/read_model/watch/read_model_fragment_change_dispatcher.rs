use crate::read_model::ReadModelFragmentChangeEnvelope;

use super::ReadModelWatchFragmentDispatcherError;

/// Fans one physical fragment-change envelope out to interested watch sessions.
#[allow(async_fn_in_trait)]
pub trait ReadModelFragmentChangeDispatcher: Send + Sync {
    async fn dispatch(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
    ) -> Result<(), ReadModelWatchFragmentDispatcherError>;
}

use super::{ReadModelWatchRegistrationError, ReadModelWatchSelection, ReadModelWatchSessionId};

/// Registers a materialized snapshot selection for an active client session.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchRegistrar: Send + Sync {
    async fn register(
        &self,
        session_id: &ReadModelWatchSessionId,
        selection: ReadModelWatchSelection,
    ) -> Result<(), ReadModelWatchRegistrationError>;
}

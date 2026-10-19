use crate::read_model::ReadModelRef;

use super::{ReadModelWatchGeneration, ReadModelWatchGenerationRegistrarError};

/// Registers targets for a fixed watch and generation without holding a database transaction
/// across calls. Dropping an incomplete registrar leaves previous targets intact.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchGenerationRegistrar: Send {
    fn generation(&self) -> ReadModelWatchGeneration;

    /// Registers additional targets idempotently. Successful registration is committed
    /// before returning, so the caller can subsequently query under that coverage.
    async fn register_targets(
        &mut self,
        targets: &[ReadModelRef],
    ) -> Result<(), ReadModelWatchGenerationRegistrarError>;

    /// Checks whether all targets belong to this generation, not whether the sets are equal.
    /// An empty target slice is contained as long as the registration remains valid.
    async fn contains_targets(
        &self,
        targets: &[ReadModelRef],
    ) -> Result<bool, ReadModelWatchGenerationRegistrarError>;

    /// Completes this generation and removes older targets, consuming the registrar.
    /// The caller must first verify coverage of the refreshed document.
    async fn complete(self) -> Result<(), ReadModelWatchGenerationRegistrarError>;
}

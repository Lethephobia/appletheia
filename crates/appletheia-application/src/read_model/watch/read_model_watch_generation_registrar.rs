use crate::read_model::ReadModelRef;
use crate::unit_of_work::UnitOfWork;

use super::{ReadModelWatchGeneration, ReadModelWatchGenerationRegistrarError};

/// Registers targets for a fixed watch and generation using caller-owned units of work.
/// The caller manages transaction boundaries; dropping the registrar performs no cleanup.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchGenerationRegistrar: Send {
    type Uow: UnitOfWork;

    fn generation(&self) -> ReadModelWatchGeneration;

    /// Registers additional targets idempotently within the supplied unit of work.
    /// The caller must commit before querying under the new coverage.
    async fn register_targets(
        &mut self,
        uow: &mut Self::Uow,
        targets: &[ReadModelRef],
    ) -> Result<(), ReadModelWatchGenerationRegistrarError>;

    /// Checks whether all targets belong to this generation, not whether the sets are equal.
    /// An empty target slice is contained as long as the registration remains valid.
    async fn contains_targets(
        &self,
        uow: &mut Self::Uow,
        targets: &[ReadModelRef],
    ) -> Result<bool, ReadModelWatchGenerationRegistrarError>;

    /// Completes this generation and removes older targets, consuming the registrar.
    /// The caller must first verify coverage of the refreshed document, then commit
    /// the supplied unit of work to persist removal.
    async fn complete(
        self,
        uow: &mut Self::Uow,
    ) -> Result<(), ReadModelWatchGenerationRegistrarError>;
}

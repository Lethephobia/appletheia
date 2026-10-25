use crate::unit_of_work::UnitOfWork;

use super::{
    ReadModelWatchEndpointId, ReadModelWatchGenerationRegistrar,
    ReadModelWatchGenerationRegistrarFactoryError, ReadModelWatchId,
};

#[allow(async_fn_in_trait)]
pub trait ReadModelWatchGenerationRegistrarFactory: Send + Sync {
    type Uow: UnitOfWork;
    type Registrar<'a>: ReadModelWatchGenerationRegistrar<Uow = Self::Uow>
    where
        Self: 'a;

    /// Advances the generation and returns its registrar within the caller's transaction.
    /// The caller must commit before querying under the new coverage. The registrar
    /// borrows the factory, not the unit of work, so it can span multiple transactions.
    async fn begin_generation(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<Self::Registrar<'_>, ReadModelWatchGenerationRegistrarFactoryError>;
}

use crate::read_model::ReadModelRef;

use super::{
    ReadModelWatchEndpointId, ReadModelWatchGeneration, ReadModelWatchGenerationRegistrar,
    ReadModelWatchGenerationRegistrarError, ReadModelWatchId, ReadModelWatchRegistry,
};

pub struct DefaultReadModelWatchGenerationRegistrar<'a, R>
where
    R: ReadModelWatchRegistry,
{
    registry: &'a R,
    endpoint_id: ReadModelWatchEndpointId,
    watch_id: ReadModelWatchId,
    generation: ReadModelWatchGeneration,
}

impl<'a, R> DefaultReadModelWatchGenerationRegistrar<'a, R>
where
    R: ReadModelWatchRegistry,
{
    pub fn new(
        registry: &'a R,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
    ) -> Self {
        Self {
            registry,
            endpoint_id,
            watch_id,
            generation,
        }
    }
}

impl<R> ReadModelWatchGenerationRegistrar for DefaultReadModelWatchGenerationRegistrar<'_, R>
where
    R: ReadModelWatchRegistry,
{
    type Uow = R::Uow;

    fn generation(&self) -> ReadModelWatchGeneration {
        self.generation
    }

    async fn register_targets(
        &mut self,
        uow: &mut Self::Uow,
        targets: &[ReadModelRef],
    ) -> Result<(), ReadModelWatchGenerationRegistrarError> {
        self.registry
            .register_targets(
                uow,
                self.endpoint_id,
                self.watch_id,
                self.generation,
                targets,
            )
            .await?;
        Ok(())
    }

    async fn contains_targets(
        &self,
        uow: &mut Self::Uow,
        targets: &[ReadModelRef],
    ) -> Result<bool, ReadModelWatchGenerationRegistrarError> {
        Ok(self
            .registry
            .contains_targets(
                uow,
                self.endpoint_id,
                self.watch_id,
                self.generation,
                targets,
            )
            .await?)
    }

    async fn complete(
        self,
        uow: &mut Self::Uow,
    ) -> Result<(), ReadModelWatchGenerationRegistrarError> {
        self.registry
            .delete_previous_generation_targets(
                uow,
                self.endpoint_id,
                self.watch_id,
                self.generation,
            )
            .await?;
        Ok(())
    }
}

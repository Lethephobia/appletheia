use super::{
    DefaultReadModelWatchGenerationRegistrar, ReadModelWatchEndpointId,
    ReadModelWatchGenerationRegistrarFactory, ReadModelWatchGenerationRegistrarFactoryError,
    ReadModelWatchId, ReadModelWatchRegistry,
};

pub struct DefaultReadModelWatchGenerationRegistrarFactory<R>
where
    R: ReadModelWatchRegistry,
{
    registry: R,
}

impl<R> DefaultReadModelWatchGenerationRegistrarFactory<R>
where
    R: ReadModelWatchRegistry,
{
    pub fn new(registry: R) -> Self {
        Self { registry }
    }
}

impl<R> ReadModelWatchGenerationRegistrarFactory
    for DefaultReadModelWatchGenerationRegistrarFactory<R>
where
    R: ReadModelWatchRegistry,
{
    type Uow = R::Uow;
    type Registrar<'a>
        = DefaultReadModelWatchGenerationRegistrar<'a, R>
    where
        Self: 'a;

    async fn begin_generation(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<Self::Registrar<'_>, ReadModelWatchGenerationRegistrarFactoryError> {
        let generation = self
            .registry
            .begin_generation(uow, endpoint_id, watch_id)
            .await?;
        Ok(DefaultReadModelWatchGenerationRegistrar::new(
            &self.registry,
            endpoint_id,
            watch_id,
            generation,
        ))
    }
}

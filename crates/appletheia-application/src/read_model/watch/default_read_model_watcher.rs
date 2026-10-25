use crate::query::{QueryDispatcher, QueryHandler};
use crate::request_context::RequestContext;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    DefaultReadModelWatch, DefaultReadModelWatchDependencies, ReadModelWatchEndpoint,
    ReadModelWatchGenerationRegistrarFactory, ReadModelWatchId, ReadModelWatchRegistry,
    ReadModelWatcher, ReadModelWatcherError,
};

pub struct DefaultReadModelWatcher<D, F, R, E, G>
where
    D: QueryDispatcher,
    F: UnitOfWorkFactory<Uow = D::Uow>,
    R: ReadModelWatchRegistry<Uow = D::Uow>,
    E: ReadModelWatchEndpoint,
    G: ReadModelWatchGenerationRegistrarFactory<Uow = D::Uow>,
{
    query_dispatcher: D,
    uow_factory: F,
    registry: R,
    endpoint: E,
    generation_registrar_factory: G,
}

impl<D, F, R, E, G> DefaultReadModelWatcher<D, F, R, E, G>
where
    D: QueryDispatcher,
    F: UnitOfWorkFactory<Uow = D::Uow>,
    R: ReadModelWatchRegistry<Uow = D::Uow>,
    E: ReadModelWatchEndpoint,
    G: ReadModelWatchGenerationRegistrarFactory<Uow = D::Uow>,
{
    /// Exposes the endpoint so its notification loop and graceful stop remain caller-managed.
    pub fn endpoint(&self) -> &E {
        &self.endpoint
    }

    pub fn new(
        query_dispatcher: D,
        uow_factory: F,
        registry: R,
        endpoint: E,
        generation_registrar_factory: G,
    ) -> Self {
        Self {
            query_dispatcher,
            uow_factory,
            registry,
            endpoint,
            generation_registrar_factory,
        }
    }
}

impl<D, F, R, E, G> ReadModelWatcher for DefaultReadModelWatcher<D, F, R, E, G>
where
    D: QueryDispatcher,
    F: UnitOfWorkFactory<Uow = D::Uow>,
    R: ReadModelWatchRegistry<Uow = D::Uow>,
    E: ReadModelWatchEndpoint,
    G: ReadModelWatchGenerationRegistrarFactory<Uow = D::Uow>,
{
    type Uow = D::Uow;
    type Watch<'a, H>
        = DefaultReadModelWatch<'a, D, F, R, E, G, H>
    where
        Self: 'a,
        H: QueryHandler<Uow = Self::Uow> + 'a;

    async fn watch<'a, H>(
        &'a self,
        handler: &'a H,
        request_context: &'a RequestContext,
        query: H::Query,
    ) -> Result<Self::Watch<'a, H>, ReadModelWatcherError>
    where
        H: QueryHandler<Uow = Self::Uow> + 'a,
    {
        let watch_id = ReadModelWatchId::new();
        let mut uow = self.uow_factory.begin().await?;
        if let Err(source) = self
            .registry
            .register_watch(&mut uow, self.endpoint.endpoint_id(), watch_id)
            .await
        {
            return Err(uow
                .rollback_with_operation_error(ReadModelWatcherError::Registry(source))
                .await?);
        }
        let receiver = match self.endpoint.register_watch(watch_id).await {
            Ok(receiver) => receiver,
            Err(source) => {
                return Err(uow
                    .rollback_with_operation_error(ReadModelWatcherError::Endpoint(source))
                    .await?);
            }
        };
        if let Err(source) = uow.commit().await {
            if let Err(cleanup_error) = self.endpoint.remove_watch(watch_id).await {
                return Err(ReadModelWatcherError::OperationAndCleanupFailed {
                    operation_error: Box::new(ReadModelWatcherError::UnitOfWork(source)),
                    cleanup_error,
                });
            }
            return Err(ReadModelWatcherError::UnitOfWork(source));
        }
        Ok(DefaultReadModelWatch::new(
            DefaultReadModelWatchDependencies {
                query_dispatcher: &self.query_dispatcher,
                uow_factory: &self.uow_factory,
                registry: &self.registry,
                endpoint: &self.endpoint,
                generation_registrar_factory: &self.generation_registrar_factory,
            },
            handler,
            request_context,
            query,
            watch_id,
            receiver,
        ))
    }
}

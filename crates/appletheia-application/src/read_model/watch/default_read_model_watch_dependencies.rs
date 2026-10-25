use crate::query::QueryDispatcher;
use crate::unit_of_work::UnitOfWorkFactory;

use super::{
    ReadModelWatchEndpoint, ReadModelWatchGenerationRegistrarFactory, ReadModelWatchRegistry,
};

pub struct DefaultReadModelWatchDependencies<'a, D, F, R, E, G>
where
    D: QueryDispatcher,
    F: UnitOfWorkFactory<Uow = D::Uow>,
    R: ReadModelWatchRegistry<Uow = D::Uow>,
    E: ReadModelWatchEndpoint,
    G: ReadModelWatchGenerationRegistrarFactory<Uow = D::Uow>,
{
    pub query_dispatcher: &'a D,
    pub uow_factory: &'a F,
    pub registry: &'a R,
    pub endpoint: &'a E,
    pub generation_registrar_factory: &'a G,
}

pub mod repository_config;
pub mod repository_config_access;
pub mod repository_error;
pub mod default_repository;

pub use repository_config::RepositoryConfig;
pub use repository_config_access::RepositoryConfigAccess;
pub use default_repository::DefaultRepository;
pub use repository_error::RepositoryError;

use appletheia_domain::{Aggregate, AggregateVersion};
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate>: Send + Sync {
    type Uow: UnitOfWork;

    async fn find(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
    ) -> Result<Option<A>, RepositoryError<A>>;

    async fn find_at_version(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
        at: Option<AggregateVersion>,
    ) -> Result<Option<A>, RepositoryError<A>>;

    async fn save(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        aggregate: &mut A,
    ) -> Result<(), RepositoryError<A>>;
}

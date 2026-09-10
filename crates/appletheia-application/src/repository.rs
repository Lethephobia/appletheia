pub mod default_repository;
pub mod default_repository_dependencies;
pub mod event_save_hook;
pub mod noop_event_save_hook;
pub mod reference_index_lookup;
pub mod reference_index_lookup_error;
pub mod reference_index_lookup_page;
pub mod reference_index_lookup_page_size;
pub mod reference_index_store;
pub mod reference_index_store_error;
pub mod repository_config;
pub mod repository_error;
pub mod unique_key_reservation_store;
pub mod unique_key_reservation_store_error;
pub mod unique_value_owner_lookup;
pub mod unique_value_owner_lookup_error;

pub use default_repository::*;
pub use default_repository_dependencies::*;
pub use event_save_hook::*;
pub use noop_event_save_hook::*;
pub use reference_index_lookup::*;
pub use reference_index_lookup_error::*;
pub use reference_index_lookup_page::*;
pub use reference_index_lookup_page_size::*;
pub use reference_index_store::*;
pub use reference_index_store_error::*;
pub use repository_config::*;
pub use repository_error::*;
pub use unique_key_reservation_store::*;
pub use unique_key_reservation_store_error::*;
pub use unique_value_owner_lookup::*;
pub use unique_value_owner_lookup_error::*;

use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;
use appletheia_domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate>: Send + Sync {
    type Uow: UnitOfWork;

    async fn read(&self, uow: &mut Self::Uow, id: A::Id) -> Result<A, RepositoryError<A>>;

    async fn read_at_version(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
        at: AggregateVersion,
    ) -> Result<A, RepositoryError<A>>;

    async fn find_by_unique_value(
        &self,
        uow: &mut Self::Uow,
        unique_key: UniqueKey,
        unique_value: &UniqueValue,
    ) -> Result<Option<A>, RepositoryError<A>>;

    async fn save(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        aggregate: &mut A,
    ) -> Result<(), RepositoryError<A>>;
}

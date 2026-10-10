pub mod aggregate_lock_mode;
pub mod aggregate_locker;
pub mod aggregate_locker_error;
pub mod default_repository;
pub mod default_repository_dependencies;
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

pub use aggregate_lock_mode::*;
pub use aggregate_locker::*;
pub use aggregate_locker_error::*;
pub use default_repository::*;
pub use default_repository_dependencies::*;
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

/// Reads and saves multiple aggregate types through one repository and UnitOfWork contract.
///
/// Select the aggregate on each operation, for example `repository.read::<Account>(uow, id)`.
#[allow(async_fn_in_trait)]
pub trait Repository: Send + Sync {
    type Uow: UnitOfWork;

    /// Holds an exclusive aggregate lock until the UnitOfWork ends.
    async fn read<A: Aggregate>(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
    ) -> Result<A, RepositoryError<A>>;

    /// Holds a shared lock; subsequent exclusive reads and saves are rejected.
    async fn read_shared<A: Aggregate>(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
    ) -> Result<A, RepositoryError<A>>;

    /// Reads historical state under a shared lock, retaining an existing exclusive lock.
    /// Acquire the latest state exclusively first when the operation will also save changes.
    async fn read_at_version<A: Aggregate>(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
        at: AggregateVersion,
    ) -> Result<A, RepositoryError<A>>;

    async fn find_by_unique_value<A: Aggregate>(
        &self,
        uow: &mut Self::Uow,
        unique_key: UniqueKey,
        unique_value: &UniqueValue,
    ) -> Result<Option<A>, RepositoryError<A>>;

    async fn find_shared_by_unique_value<A: Aggregate>(
        &self,
        uow: &mut Self::Uow,
        unique_key: UniqueKey,
        unique_value: &UniqueValue,
    ) -> Result<Option<A>, RepositoryError<A>>;

    async fn save<A: Aggregate>(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        aggregate: &mut A,
    ) -> Result<(), RepositoryError<A>>;
}

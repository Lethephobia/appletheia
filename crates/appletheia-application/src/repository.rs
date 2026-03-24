pub mod default_repository;
pub mod repository_config;
pub mod repository_error;
pub mod unique_key_reservation_store;
pub mod unique_key_reservation_store_error;
pub mod unique_value_owner_lookup;
pub mod unique_value_owner_lookup_error;

pub use default_repository::DefaultRepository;
pub use repository_config::RepositoryConfig;
pub use repository_error::RepositoryError;
pub use unique_key_reservation_store::UniqueKeyReservationStore;
pub use unique_key_reservation_store_error::UniqueKeyReservationStoreError;
pub use unique_value_owner_lookup::UniqueValueOwnerLookup;
pub use unique_value_owner_lookup_error::UniqueValueOwnerLookupError;

use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;
use appletheia_domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate>: Send + Sync {
    type Uow: UnitOfWork;

    async fn find(&self, uow: &mut Self::Uow, id: A::Id) -> Result<Option<A>, RepositoryError<A>>;

    async fn find_at_version(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
        at: Option<AggregateVersion>,
    ) -> Result<Option<A>, RepositoryError<A>>;

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

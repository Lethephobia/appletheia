pub mod repository_error;

pub use crate::repository_error::RepositoryError;

use crate::aggregate::{Aggregate, AggregateVersion};

#[allow(async_fn_in_trait)]
pub trait Repository<A: Aggregate> {
    async fn find(&self, id: A::Id) -> Result<Option<A>, RepositoryError<A>>;

    async fn find_at_version(
        &self,
        id: A::Id,
        version: AggregateVersion,
    ) -> Result<Option<A>, RepositoryError<A>>;
}

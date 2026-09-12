use crate::unit_of_work::UnitOfWork;

use super::{RelationshipRequirement, RelationshipResolverError};
use crate::aggregate::AggregateRef;

#[allow(async_fn_in_trait)]
pub trait RelationshipResolver: Send + Sync {
    type Uow: UnitOfWork;

    async fn satisfies(
        &self,
        uow: &mut Self::Uow,
        subject: &AggregateRef,
        requirement: &RelationshipRequirement,
    ) -> Result<bool, RelationshipResolverError>;
}

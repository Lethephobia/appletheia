use std::error::Error;

use crate::request_context::TenantId;
use crate::unit_of_work::UnitOfWork;

use super::{RelationName, RelationshipEdge, ResourceRef};

#[allow(async_fn_in_trait)]
pub trait RelationshipStore<Uow>: Send + Sync
where
    Uow: UnitOfWork,
{
    type Error: Error + Send + Sync + 'static;

    async fn list_subjects(
        &self,
        uow: &mut Uow,
        tenant_id: Option<TenantId>,
        object: &ResourceRef,
        relation: &RelationName,
    ) -> Result<Vec<RelationshipEdge>, Self::Error>;
}

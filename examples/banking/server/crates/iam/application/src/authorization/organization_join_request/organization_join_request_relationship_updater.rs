use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationId, OrganizationJoinRequestId, UserId};

use super::OrganizationJoinRequestRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait OrganizationJoinRequestRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        join_request_id: OrganizationJoinRequestId,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationJoinRequestRelationshipUpdaterError>;

    async fn upsert_requester(
        &self,
        uow: &mut Self::Uow,
        join_request_id: OrganizationJoinRequestId,
        requester_id: UserId,
    ) -> Result<(), OrganizationJoinRequestRelationshipUpdaterError>;
}

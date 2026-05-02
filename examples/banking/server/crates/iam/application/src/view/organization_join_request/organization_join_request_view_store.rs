use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestStatus};

use super::{OrganizationJoinRequestViewStoreError, OrganizationJoinRequestViewUpsert};

/// Persists normalized organization join request views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationJoinRequestViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationJoinRequestViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationJoinRequestViewStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationJoinRequestId,
        status: OrganizationJoinRequestStatus,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationJoinRequestViewStoreError>;
}

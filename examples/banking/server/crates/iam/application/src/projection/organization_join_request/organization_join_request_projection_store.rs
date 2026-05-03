use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestStatus};

use super::{OrganizationJoinRequestProjectionStoreError, OrganizationJoinRequestProjectionUpsert};

/// Persists normalized organization join request projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationJoinRequestProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationJoinRequestProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationJoinRequestProjectionStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationJoinRequestId,
        status: OrganizationJoinRequestStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationJoinRequestProjectionStoreError>;
}

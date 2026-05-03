use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{OrganizationMembershipId, OrganizationMembershipStatus};

use super::{OrganizationMembershipProjectionStoreError, OrganizationMembershipProjectionUpsert};

/// Persists normalized membership projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationMembershipProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationMembershipProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationMembershipProjectionStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationMembershipId,
        status: OrganizationMembershipStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationMembershipProjectionStoreError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationMembershipId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationMembershipProjectionStoreError>;
}

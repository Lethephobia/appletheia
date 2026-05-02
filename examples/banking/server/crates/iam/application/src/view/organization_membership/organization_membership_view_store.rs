use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationMembershipId, OrganizationMembershipStatus};

use super::{OrganizationMembershipViewStoreError, OrganizationMembershipViewUpsert};

/// Persists normalized membership views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationMembershipViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationMembershipViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipViewStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationMembershipId,
        status: OrganizationMembershipStatus,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipViewStoreError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationMembershipId,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipViewStoreError>;
}

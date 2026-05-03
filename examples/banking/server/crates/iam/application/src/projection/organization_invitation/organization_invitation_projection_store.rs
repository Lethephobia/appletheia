use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationInvitationId, OrganizationInvitationStatus};

use super::{OrganizationInvitationProjectionStoreError, OrganizationInvitationProjectionUpsert};

/// Persists normalized organization invitation projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationInvitationProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationInvitationProjectionUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationInvitationProjectionStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationInvitationId,
        status: OrganizationInvitationStatus,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationInvitationProjectionStoreError>;
}

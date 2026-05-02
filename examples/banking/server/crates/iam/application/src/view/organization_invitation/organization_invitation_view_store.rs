use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationInvitationId, OrganizationInvitationStatus};

use super::{OrganizationInvitationViewStoreError, OrganizationInvitationViewUpsert};

/// Persists normalized organization invitation views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationInvitationViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationInvitationViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationInvitationViewStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationInvitationId,
        status: OrganizationInvitationStatus,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationInvitationViewStoreError>;
}

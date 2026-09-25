use appletheia::application::read_model::MaterializationEventContext;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationInvitationId, OrganizationInvitationStatus};

use super::{
    OrganizationInvitationFragment, OrganizationInvitationFragmentUpsert,
    OrganizationInvitationFragmentWriterError,
};

/// Persists organization invitation fragments independently of composed read models.
#[allow(async_fn_in_trait)]
pub trait OrganizationInvitationFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: OrganizationInvitationFragmentUpsert,
    ) -> Result<Option<OrganizationInvitationFragment>, OrganizationInvitationFragmentWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        invitation_id: OrganizationInvitationId,
        status: OrganizationInvitationStatus,
    ) -> Result<Option<OrganizationInvitationFragment>, OrganizationInvitationFragmentWriterError>;
}

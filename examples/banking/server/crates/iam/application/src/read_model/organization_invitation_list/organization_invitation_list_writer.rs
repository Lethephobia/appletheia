use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationInvitationId,
    OrganizationPictureRef, UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{
    OrganizationInvitationListInviteeUpsert, OrganizationInvitationListItemStatus,
    OrganizationInvitationListOrganizationUpsert, OrganizationInvitationListUpsert,
    OrganizationInvitationListWriterError,
};

/// Writes organization invitation lists.
#[allow(async_fn_in_trait)]
pub trait OrganizationInvitationListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_invitation(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationInvitationListUpsert,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        invitation_id: OrganizationInvitationId,
        status: OrganizationInvitationListItemStatus,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationInvitationListOrganizationUpsert,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn upsert_invitee(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationInvitationListInviteeUpsert,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn update_invitee_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn update_invitee_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn update_invitee_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn delete_invitee_and_invitations(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), OrganizationInvitationListWriterError>;

    async fn delete_organization_and_invitations(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationInvitationListWriterError>;
}

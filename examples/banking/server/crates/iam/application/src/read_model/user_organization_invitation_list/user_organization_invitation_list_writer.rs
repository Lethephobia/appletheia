use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationInvitationId,
    OrganizationPictureRef, UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{
    UserOrganizationInvitationListItemStatus, UserOrganizationInvitationListOrganizationUpsert,
    UserOrganizationInvitationListUpsert, UserOrganizationInvitationListUserUpsert,
    UserOrganizationInvitationListWriterError,
};

/// Writes user organization invitation lists.
#[allow(async_fn_in_trait)]
pub trait UserOrganizationInvitationListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_invitation(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationInvitationListUpsert,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        invitation_id: OrganizationInvitationId,
        status: UserOrganizationInvitationListItemStatus,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationInvitationListUserUpsert,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn update_user_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn update_user_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn update_user_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationInvitationListOrganizationUpsert,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn delete_organization_and_invitations(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;

    async fn delete_user_and_invitations(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), UserOrganizationInvitationListWriterError>;
}

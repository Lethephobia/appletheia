use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl, UserDisplayName, UserId, UserPictureRef,
    Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{
    OrganizationManagementInfoOwnerUpsert, OrganizationManagementInfoUpsert,
    OrganizationManagementInfoWriterError,
};

/// Writes organization-management information read models.
#[allow(async_fn_in_trait)]
pub trait OrganizationManagementInfoWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationManagementInfoUpsert,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        owner_user_id: UserId,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationManagementInfoOwnerUpsert,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_owner_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_owner_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn update_owner_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), OrganizationManagementInfoWriterError>;

    async fn delete_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), OrganizationManagementInfoWriterError>;
}

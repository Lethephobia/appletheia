use appletheia::application::read_model::MaterializationEventContext;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl, UserId,
};

use super::{OrganizationFragment, OrganizationFragmentUpsert, OrganizationFragmentWriterError};

/// Writes organization fragment read models.
#[allow(async_fn_in_trait)]
pub trait OrganizationFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: OrganizationFragmentUpsert,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError>;

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        owner_user_id: UserId,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError>;

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError>;

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError>;

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError>;

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
    ) -> Result<bool, OrganizationFragmentWriterError>;
}

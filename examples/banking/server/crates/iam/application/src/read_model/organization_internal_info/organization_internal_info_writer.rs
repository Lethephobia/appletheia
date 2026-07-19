use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{OrganizationInternalInfoUpsert, OrganizationInternalInfoWriterError};

/// Writes organization-internal information read models.
#[allow(async_fn_in_trait)]
pub trait OrganizationInternalInfoWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationInternalInfoUpsert,
    ) -> Result<(), OrganizationInternalInfoWriterError>;

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OrganizationInternalInfoWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationInternalInfoWriterError>;

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
    ) -> Result<(), OrganizationInternalInfoWriterError>;

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
    ) -> Result<(), OrganizationInternalInfoWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationInternalInfoWriterError>;

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), OrganizationInternalInfoWriterError>;
}

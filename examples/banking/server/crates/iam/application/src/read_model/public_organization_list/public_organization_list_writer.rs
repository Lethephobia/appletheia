use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{PublicOrganizationListUpsert, PublicOrganizationListWriterError};

/// Writes public organization list read models.
#[allow(async_fn_in_trait)]
pub trait PublicOrganizationListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicOrganizationListUpsert,
    ) -> Result<(), PublicOrganizationListWriterError>;

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), PublicOrganizationListWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), PublicOrganizationListWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), PublicOrganizationListWriterError>;

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), PublicOrganizationListWriterError>;
}

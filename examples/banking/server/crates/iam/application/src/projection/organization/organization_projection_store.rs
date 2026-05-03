use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationOwner, OrganizationPictureRef, OrganizationWebsiteUrl,
};

use super::{OrganizationProjectionStoreError, OrganizationProjectionUpsert};

/// Persists normalized organization projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationProjectionUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationProjectionStoreError>;

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationProjectionStoreError>;

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        owner: OrganizationOwner,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationProjectionStoreError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationProjectionStoreError>;

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationProjectionStoreError>;

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationProjectionStoreError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationProjectionStoreError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationProjectionStoreError>;
}

use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationJoinRequestId,
    OrganizationPictureRef, UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{
    OrganizationJoinRequestListItemStatus, OrganizationJoinRequestListOrganizationUpsert,
    OrganizationJoinRequestListRequesterUpsert, OrganizationJoinRequestListUpsert,
    OrganizationJoinRequestListWriterError,
};

/// Writes organization join request lists.
#[allow(async_fn_in_trait)]
pub trait OrganizationJoinRequestListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_join_request(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationJoinRequestListUpsert,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        join_request_id: OrganizationJoinRequestId,
        status: OrganizationJoinRequestListItemStatus,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationJoinRequestListOrganizationUpsert,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn upsert_requester(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationJoinRequestListRequesterUpsert,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn update_requester_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn update_requester_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn update_requester_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn delete_requester_and_join_requests(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;

    async fn delete_organization_and_join_requests(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationJoinRequestListWriterError>;
}

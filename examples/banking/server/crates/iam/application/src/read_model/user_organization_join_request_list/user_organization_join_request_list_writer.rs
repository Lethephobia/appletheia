use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationJoinRequestId,
    OrganizationPictureRef, UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{
    UserOrganizationJoinRequestListItemStatus, UserOrganizationJoinRequestListOrganizationUpsert,
    UserOrganizationJoinRequestListUpsert, UserOrganizationJoinRequestListUserUpsert,
    UserOrganizationJoinRequestListWriterError,
};

/// Writes user organization join request lists.
#[allow(async_fn_in_trait)]
pub trait UserOrganizationJoinRequestListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_join_request(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationJoinRequestListUpsert,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        join_request_id: OrganizationJoinRequestId,
        status: UserOrganizationJoinRequestListItemStatus,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationJoinRequestListUserUpsert,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn update_user_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn update_user_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn update_user_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationJoinRequestListOrganizationUpsert,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn delete_organization_and_join_requests(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;

    async fn delete_user_and_join_requests(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError>;
}

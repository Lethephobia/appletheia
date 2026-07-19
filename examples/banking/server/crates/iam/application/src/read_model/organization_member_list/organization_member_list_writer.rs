use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    OrganizationRoles, UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{
    OrganizationMemberListMemberUpsert, OrganizationMemberListMembershipUpsert,
    OrganizationMemberListOrganizationUpsert, OrganizationMemberListWriterError,
};

/// Writes organization member lists.
#[allow(async_fn_in_trait)]
pub trait OrganizationMemberListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationMemberListOrganizationUpsert,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn update_organization_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        owner_user_id: UserId,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn delete_organization_and_memberships(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn upsert_member(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationMemberListMemberUpsert,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn update_member_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn update_member_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn update_member_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn delete_member_and_memberships(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn upsert_membership(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationMemberListMembershipUpsert,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn update_membership_roles(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationRoles,
    ) -> Result<(), OrganizationMemberListWriterError>;

    async fn delete_membership(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMemberListWriterError>;
}

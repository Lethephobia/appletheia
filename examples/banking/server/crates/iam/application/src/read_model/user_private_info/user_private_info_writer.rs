use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::OrganizationRoles;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef, UserBio,
    UserDisplayName, UserId, UserIdentityProvider, UserIdentitySubject, UserPictureRef, Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;
use banking_shared_kernel_domain::contact::Email;

use super::{
    UserPrivateInfoIdentityUpsert, UserPrivateInfoOrganizationMembershipUpsert,
    UserPrivateInfoOrganizationUpsert, UserPrivateInfoStatus, UserPrivateInfoUserUpsert,
    UserPrivateInfoWriterError,
};

#[allow(async_fn_in_trait)]
pub trait UserPrivateInfoWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPrivateInfoUserUpsert,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn upsert_identity(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPrivateInfoIdentityUpsert,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_identity_email(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn upsert_organization_membership(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPrivateInfoOrganizationMembershipUpsert,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_organization_membership_roles(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        organization_id: OrganizationId,
        roles: OrganizationRoles,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn delete_organization_membership(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        organization_id: OrganizationId,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        bio: Option<UserBio>,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        status: UserPrivateInfoStatus,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn delete_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPrivateInfoOrganizationUpsert,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), UserPrivateInfoWriterError>;
}

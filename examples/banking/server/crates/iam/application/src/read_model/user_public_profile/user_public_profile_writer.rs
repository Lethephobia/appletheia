use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{UserPublicProfileStatus, UserPublicProfileUserUpsert, UserPublicProfileWriterError};

#[allow(async_fn_in_trait)]
pub trait UserPublicProfileWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPublicProfileUserUpsert,
    ) -> Result<(), UserPublicProfileWriterError>;

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), UserPublicProfileWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), UserPublicProfileWriterError>;

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        bio: Option<UserBio>,
    ) -> Result<(), UserPublicProfileWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), UserPublicProfileWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        status: UserPublicProfileStatus,
    ) -> Result<(), UserPublicProfileWriterError>;

    async fn delete_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), UserPublicProfileWriterError>;
}

use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{PublicUserListItemStatus, PublicUserListUpsert, PublicUserListWriterError};

/// Writes public user list read models.
#[allow(async_fn_in_trait)]
pub trait PublicUserListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicUserListUpsert,
    ) -> Result<(), PublicUserListWriterError>;

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), PublicUserListWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), PublicUserListWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), PublicUserListWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        status: PublicUserListItemStatus,
    ) -> Result<(), PublicUserListWriterError>;

    async fn delete_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), PublicUserListWriterError>;
}

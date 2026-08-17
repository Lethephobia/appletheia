use appletheia::application::read_model::MaterializationEventContext;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};

use super::{MaterializedUserStatus, UserFragment, UserFragmentUpsert, UserFragmentWriterError};

/// Writes shared public user fragments independently of their consuming read models.
///
/// Each mutation returns the complete live fragment after serialization with concurrent
/// projectors. A writer returns a newer already-materialized fragment when another projector
/// has advanced the same user, and returns `None` only when no live fragment remains.
#[allow(async_fn_in_trait)]
pub trait UserFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: UserFragmentUpsert,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError>;

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: UserId,
        username: Username,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError>;

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: UserId,
        bio: Option<UserBio>,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: UserId,
        status: MaterializedUserStatus,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError>;

    /// Physically removes the fragment and records its terminal event position.
    ///
    /// Returns `true` when the terminal event was accepted or was already observed, and
    /// `false` when a newer event position rejected it.
    async fn delete(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: UserId,
    ) -> Result<bool, UserFragmentWriterError>;
}

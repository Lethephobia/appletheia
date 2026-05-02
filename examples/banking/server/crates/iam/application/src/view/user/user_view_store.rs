use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, UserStatus, Username};

use super::{UserViewStoreError, UserViewUpsert};

/// Persists normalized user views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait UserViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: UserViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError>;

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError>;

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        bio: Option<UserBio>,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        status: UserStatus,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError>;
}

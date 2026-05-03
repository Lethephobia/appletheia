use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, UserStatus, Username};

use super::{UserProjectionStoreError, UserProjectionUpsert};

/// Persists normalized user projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait UserProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: UserProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError>;

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError>;

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        bio: Option<UserBio>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        status: UserStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError>;
}

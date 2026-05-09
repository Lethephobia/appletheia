use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    UserBio, UserDisplayName, UserId, UserIdentityProvider, UserIdentitySubject, UserPictureRef,
    Username, core::Email,
};

use super::{UserPrivateInfoStatus, UserPrivateInfoWriterError};

#[allow(async_fn_in_trait, clippy::too_many_arguments)]
pub trait UserPrivateInfoWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        status: UserPrivateInfoStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn upsert_identity(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_identity_email(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        bio: Option<UserBio>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        status: UserPrivateInfoStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;

    async fn delete_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError>;
}

use appletheia::application::event::EventSequence;
use appletheia::domain::{AggregateId, EventOccurredAt};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserPublicProfileStatus, UserPublicProfileUserUpsert, UserPublicProfileWriter,
    UserPublicProfileWriterError,
};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};

use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;

/// PostgreSQL-backed public user profile writer.
pub struct PgUserPublicProfileWriter;

impl PgUserPublicProfileWriter {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: UserPublicProfileStatus) -> &'static str {
        match status {
            UserPublicProfileStatus::Active => "active",
            UserPublicProfileStatus::Inactive => "inactive",
        }
    }
}

impl Default for PgUserPublicProfileWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl UserPublicProfileWriter for PgUserPublicProfileWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        upsert: UserPublicProfileUserUpsert,
    ) -> Result<(), UserPublicProfileWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO user_public_profiles (
                id, username, display_name, bio, picture_type, picture_object_name,
                picture_external_url, status, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                bio = EXCLUDED.bio,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE user_public_profiles.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.username.as_ref().map(Username::value))
        .bind(upsert.display_name.as_ref().map(UserDisplayName::value))
        .bind(upsert.bio.as_ref().map(UserBio::value))
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(Self::status_name(upsert.status))
        .bind(upsert.occurred_at.value())
        .bind(upsert.occurred_at.value())
        .bind(upsert.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPublicProfileWriterError> {
        sqlx::query(
            r#"
            UPDATE user_public_profiles
               SET username = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(username.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPublicProfileWriterError> {
        sqlx::query(
            r#"
            UPDATE user_public_profiles
               SET display_name = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        bio: Option<UserBio>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPublicProfileWriterError> {
        sqlx::query(
            r#"
            UPDATE user_public_profiles
               SET bio = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(bio.as_ref().map(UserBio::value))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPublicProfileWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE user_public_profiles
               SET picture_type = $2,
                   picture_object_name = $3,
                   picture_external_url = $4,
                   updated_at = $5,
                   updated_event_sequence = $6
             WHERE id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        status: UserPublicProfileStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPublicProfileWriterError> {
        sqlx::query(
            r#"
            UPDATE user_public_profiles
               SET status = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        _occurred_at: EventOccurredAt,
    ) -> Result<(), UserPublicProfileWriterError> {
        sqlx::query(
            r#"
            DELETE FROM user_public_profiles
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }
}

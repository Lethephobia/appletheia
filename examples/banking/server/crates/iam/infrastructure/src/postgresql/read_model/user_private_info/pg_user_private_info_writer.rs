use appletheia::application::event::EventSequence;
use appletheia::domain::{AggregateId, EventOccurredAt};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserPrivateInfoIdentityUpsert, UserPrivateInfoStatus, UserPrivateInfoUserUpsert,
    UserPrivateInfoWriter, UserPrivateInfoWriterError,
};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username, core::Email};

use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;

/// PostgreSQL-backed user-private information writer.
pub struct PgUserPrivateInfoWriter;

impl PgUserPrivateInfoWriter {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: UserPrivateInfoStatus) -> &'static str {
        match status {
            UserPrivateInfoStatus::Active => "active",
            UserPrivateInfoStatus::Inactive => "inactive",
        }
    }
}

impl Default for PgUserPrivateInfoWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl UserPrivateInfoWriter for PgUserPrivateInfoWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        upsert: UserPrivateInfoUserUpsert,
    ) -> Result<(), UserPrivateInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO user_private_infos (
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
            WHERE user_private_infos.updated_event_sequence < EXCLUDED.updated_event_sequence
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn upsert_identity(
        &self,
        uow: &mut Self::Uow,
        upsert: UserPrivateInfoIdentityUpsert,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            INSERT INTO user_private_info_identities (
                user_id, provider, subject, email, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (user_id, provider, subject) DO UPDATE SET
                email = EXCLUDED.email,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE user_private_info_identities.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.user_id.value())
        .bind(upsert.provider.value())
        .bind(upsert.subject.value())
        .bind(upsert.email.as_ref().map(Email::value))
        .bind(upsert.occurred_at.value())
        .bind(upsert.occurred_at.value())
        .bind(upsert.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_identity_email(
        &self,
        uow: &mut Self::Uow,
        update: UserPrivateInfoIdentityUpsert,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_info_identities
               SET email = $4, updated_at = $5,
                   updated_event_sequence = $6
             WHERE user_id = $1
               AND provider = $2
               AND subject = $3
               AND updated_event_sequence < $6
            "#,
        )
        .bind(update.user_id.value())
        .bind(update.provider.value())
        .bind(update.subject.value())
        .bind(update.email.as_ref().map(Email::value))
        .bind(update.occurred_at.value())
        .bind(update.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        bio: Option<UserBio>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        status: UserPrivateInfoStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        _occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            DELETE FROM user_private_info_identities
             WHERE user_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM user_private_infos
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }
}

use appletheia::application::event::EventSequence;
use appletheia::domain::{AggregateId, EventOccurredAt};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserPrivateInfoStatus, UserPrivateInfoWriter, UserPrivateInfoWriterError,
};
use banking_iam_domain::{
    UserBio, UserDisplayName, UserId, UserIdentityProvider, UserIdentitySubject, UserPictureRef,
    Username, core::Email,
};

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
        id: UserId,
        status: UserPrivateInfoStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            INSERT INTO user_private_infos (
                id, status, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE user_private_infos.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn upsert_identity(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
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
        .bind(user_id.value())
        .bind(provider.value())
        .bind(subject.value())
        .bind(email.as_ref().map(Email::value))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_identity_email(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
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
        .bind(user_id.value())
        .bind(provider.value())
        .bind(subject.value())
        .bind(email.as_ref().map(Email::value))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
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
        sqlx::query(
            r#"
            UPDATE user_private_infos
               SET picture = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(picture.map(sqlx::types::Json))
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

use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::domain::EventOccurredAt;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserProjectionStore, UserProjectionStoreError, UserProjectionUpsert,
};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, UserStatus, Username};
use sqlx::types::Json;

/// PostgreSQL-backed user projection store.
pub struct PgUserProjectionStore;

impl PgUserProjectionStore {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: UserStatus) -> &'static str {
        match status {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Removed => "removed",
        }
    }
}

impl Default for PgUserProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl UserProjectionStore for PgUserProjectionStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: UserProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError> {
        sqlx::query(
            r#"
            INSERT INTO users (
                id,
                username,
                display_name,
                bio,
                picture,
                status,
                created_at, updated_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                bio = EXCLUDED.bio,
                picture = EXCLUDED.picture,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE users.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(input.username.as_ref().map(Username::value))
        .bind(input.display_name.as_ref().map(UserDisplayName::value))
        .bind(input.bio.as_ref().map(UserBio::value))
        .bind(input.picture.as_ref().map(Json))
        .bind(Self::status_name(input.status))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET username = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(username.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET display_name = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        bio: Option<UserBio>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET bio = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(bio.as_ref().map(UserBio::value))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET picture = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(picture.as_ref().map(Json))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        status: UserStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET status = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), UserProjectionStoreError> {
        sqlx::query(
            r#"
            DELETE FROM users
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}

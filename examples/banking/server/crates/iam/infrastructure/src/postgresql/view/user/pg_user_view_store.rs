use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{UserViewStore, UserViewStoreError, UserViewUpsert};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, UserStatus, Username};
use sqlx::types::Json;

/// PostgreSQL-backed user view store.
pub struct PgUserViewStore;

impl PgUserViewStore {
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

impl Default for PgUserViewStore {
    fn default() -> Self {
        Self::new()
    }
}

impl UserViewStore for PgUserViewStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: UserViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError> {
        sqlx::query(
            r#"
            INSERT INTO users (
                id,
                username,
                display_name,
                bio,
                picture,
                status,
                updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                bio = EXCLUDED.bio,
                picture = EXCLUDED.picture,
                status = EXCLUDED.status,
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
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET username = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(username.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET display_name = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        bio: Option<UserBio>,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET bio = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(bio.as_ref().map(UserBio::value))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET picture = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(picture.as_ref().map(Json))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        status: UserStatus,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError> {
        sqlx::query(
            r#"
            UPDATE users
               SET status = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
    ) -> Result<(), UserViewStoreError> {
        sqlx::query(
            r#"
            DELETE FROM users
             WHERE id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}

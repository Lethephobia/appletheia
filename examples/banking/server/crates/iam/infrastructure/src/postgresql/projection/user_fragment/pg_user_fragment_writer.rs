use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    MaterializedUserStatus, UserFragment, UserFragmentUpsert, UserFragmentWriter,
    UserFragmentWriterError,
};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};

use super::PgUserFragmentRow;
use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

/// PostgreSQL-backed writer for shared public user fragments.
pub struct PgUserFragmentWriter;

impl PgUserFragmentWriter {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: MaterializedUserStatus) -> &'static str {
        match status {
            MaterializedUserStatus::Active => "active",
            MaterializedUserStatus::Inactive => "inactive",
        }
    }

    fn map_fragment(
        row: Option<PgUserFragmentRow>,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        row.map(UserFragment::try_from)
            .transpose()
            .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))
    }

    async fn lock_fragment(
        uow: &mut PgUnitOfWork,
        user_id: UserId,
    ) -> Result<(), UserFragmentWriterError> {
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
            .bind(user_id.value().to_string())
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn find_current(
        uow: &mut PgUnitOfWork,
        user_id: UserId,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        let row = sqlx::query_as::<_, PgUserFragmentRow>(
            r#"
            SELECT
                id,
                username,
                display_name,
                bio,
                picture_type,
                picture_object_name,
                picture_external_url,
                status,
                created_at,
                source_event_id,
                updated_event_id
            FROM user_fragments
            WHERE id = $1
            "#,
        )
        .bind(user_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_fragment(row)
    }

    async fn map_write_result(
        uow: &mut PgUnitOfWork,
        user_id: UserId,
        written_row: Option<PgUserFragmentRow>,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        if written_row.is_some() {
            return Self::map_fragment(written_row);
        }

        Self::find_current(uow, user_id).await
    }
}

impl Default for PgUserFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl UserFragmentWriter for PgUserFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: UserFragmentUpsert,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        Self::lock_fragment(uow, upsert.id).await?;

        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());

        let written_row = sqlx::query_as::<_, PgUserFragmentRow>(
            r#"
            INSERT INTO user_fragments (
                id, username, display_name, bio, picture_type, picture_object_name,
                picture_external_url, status, updated_at, created_at, source_event_sequence,
                updated_event_sequence, source_event_id, updated_event_id
            )
            SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $9, $10, $10, $11, $11
            WHERE NOT EXISTS (
                SELECT 1
                FROM user_fragment_tombstones
                WHERE user_id = $1 AND event_sequence >= $10
            )
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                bio = EXCLUDED.bio,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE user_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            RETURNING
                id,
                username,
                display_name,
                bio,
                picture_type,
                picture_object_name,
                picture_external_url,
                status,
                created_at,
                source_event_id,
                updated_event_id
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
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_write_result(uow, upsert.id, written_row).await
    }

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        Self::lock_fragment(uow, user_id).await?;

        let written_row = sqlx::query_as::<_, PgUserFragmentRow>(
            r#"
            UPDATE user_fragments
               SET username = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1
               AND updated_event_sequence < $4
               AND NOT EXISTS (
                   SELECT 1 FROM user_fragment_tombstones
                   WHERE user_id = $1 AND event_sequence >= $4
               )
            RETURNING
                id, username, display_name, bio, picture_type,
                picture_object_name, picture_external_url, status, created_at,
                source_event_id, updated_event_id
            "#,
        )
        .bind(user_id.value())
        .bind(username.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_write_result(uow, user_id, written_row).await
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        Self::lock_fragment(uow, user_id).await?;

        let written_row = sqlx::query_as::<_, PgUserFragmentRow>(
            r#"
            UPDATE user_fragments
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1
               AND updated_event_sequence < $4
               AND NOT EXISTS (
                   SELECT 1 FROM user_fragment_tombstones
                   WHERE user_id = $1 AND event_sequence >= $4
               )
            RETURNING
                id, username, display_name, bio, picture_type,
                picture_object_name, picture_external_url, status, created_at,
                source_event_id, updated_event_id
            "#,
        )
        .bind(user_id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_write_result(uow, user_id, written_row).await
    }

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        bio: Option<UserBio>,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        Self::lock_fragment(uow, user_id).await?;

        let written_row = sqlx::query_as::<_, PgUserFragmentRow>(
            r#"
            UPDATE user_fragments
               SET bio = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1
               AND updated_event_sequence < $4
               AND NOT EXISTS (
                   SELECT 1 FROM user_fragment_tombstones
                   WHERE user_id = $1 AND event_sequence >= $4
               )
            RETURNING
                id, username, display_name, bio, picture_type,
                picture_object_name, picture_external_url, status, created_at,
                source_event_id, updated_event_id
            "#,
        )
        .bind(user_id.value())
        .bind(bio.as_ref().map(UserBio::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_write_result(uow, user_id, written_row).await
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        Self::lock_fragment(uow, user_id).await?;

        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());
        let written_row = sqlx::query_as::<_, PgUserFragmentRow>(
            r#"
            UPDATE user_fragments
               SET picture_type = $2, picture_object_name = $3, picture_external_url = $4,
                   updated_at = $5, updated_event_sequence = $6, updated_event_id = $7
             WHERE id = $1
               AND updated_event_sequence < $6
               AND NOT EXISTS (
                   SELECT 1 FROM user_fragment_tombstones
                   WHERE user_id = $1 AND event_sequence >= $6
               )
            RETURNING
                id, username, display_name, bio, picture_type,
                picture_object_name, picture_external_url, status, created_at,
                source_event_id, updated_event_id
            "#,
        )
        .bind(user_id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_write_result(uow, user_id, written_row).await
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        status: MaterializedUserStatus,
    ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
        Self::lock_fragment(uow, user_id).await?;

        let written_row = sqlx::query_as::<_, PgUserFragmentRow>(
            r#"
            UPDATE user_fragments
               SET status = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1
               AND updated_event_sequence < $4
               AND NOT EXISTS (
                   SELECT 1 FROM user_fragment_tombstones
                   WHERE user_id = $1 AND event_sequence >= $4
               )
            RETURNING
                id, username, display_name, bio, picture_type,
                picture_object_name, picture_external_url, status, created_at,
                source_event_id, updated_event_id
            "#,
        )
        .bind(user_id.value())
        .bind(Self::status_name(status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_write_result(uow, user_id, written_row).await
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
    ) -> Result<bool, UserFragmentWriterError> {
        Self::lock_fragment(uow, user_id).await?;

        let accepted_tombstone = sqlx::query_scalar::<_, bool>(
            r#"
            INSERT INTO user_fragment_tombstones (
                user_id, event_sequence, event_id, occurred_at
            )
            SELECT $1, $2, $3, $4
            WHERE NOT EXISTS (
                SELECT 1 FROM user_fragments
                WHERE id = $1 AND updated_event_sequence >= $2
            )
            ON CONFLICT (user_id) DO UPDATE SET
                event_sequence = EXCLUDED.event_sequence,
                event_id = EXCLUDED.event_id,
                occurred_at = EXCLUDED.occurred_at
            WHERE user_fragment_tombstones.event_sequence < EXCLUDED.event_sequence
            RETURNING TRUE
            "#,
        )
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .bind(event_context.occurred_at.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        let accepted = if accepted_tombstone.is_some() {
            true
        } else {
            sqlx::query_scalar::<_, bool>(
                r#"
                SELECT true
                FROM user_fragment_tombstones
                WHERE user_id = $1 AND event_id = $2
                "#,
            )
            .bind(user_id.value())
            .bind(event_context.event_id.value())
            .fetch_optional(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?
            .is_some()
        };

        if !accepted {
            return Ok(false);
        }

        sqlx::query(
            r#"
            DELETE FROM user_fragments
            WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(true)
    }
}

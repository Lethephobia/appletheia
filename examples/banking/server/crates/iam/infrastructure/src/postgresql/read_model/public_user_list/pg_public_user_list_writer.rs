use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    PublicUserListItemStatus, PublicUserListUpsert, PublicUserListWriter, PublicUserListWriterError,
};
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;

/// PostgreSQL-backed public user list writer.
pub struct PgPublicUserListWriter;

impl PgPublicUserListWriter {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: PublicUserListItemStatus) -> &'static str {
        match status {
            PublicUserListItemStatus::Active => "active",
            PublicUserListItemStatus::Inactive => "inactive",
        }
    }
}

impl Default for PgPublicUserListWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicUserListWriter for PgPublicUserListWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicUserListUpsert,
    ) -> Result<(), PublicUserListWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO public_user_list_items (
                id, username, display_name, picture_type, picture_object_name,
                picture_external_url, status, updated_at, created_at, source_event_sequence,
                updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10, $11, $11)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE public_user_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.username.as_ref().map(Username::value))
        .bind(upsert.display_name.as_ref().map(UserDisplayName::value))
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(Self::status_name(upsert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PublicUserListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), PublicUserListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_user_list_items
               SET username = $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(username.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PublicUserListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), PublicUserListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_user_list_items
               SET display_name = $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PublicUserListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), PublicUserListWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE public_user_list_items
               SET picture_type = $2,
                   picture_object_name = $3,
                   picture_external_url = $4,
                   updated_at = $5,
                   updated_event_sequence = $6,
                   updated_event_id = $7
             WHERE id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PublicUserListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        status: PublicUserListItemStatus,
    ) -> Result<(), PublicUserListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_user_list_items
               SET status = $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PublicUserListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn delete_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), PublicUserListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM public_user_list_items
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PublicUserListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }
}

use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationManagementInfoOwnerUpsert, OrganizationManagementInfoUpsert,
    OrganizationManagementInfoWriter, OrganizationManagementInfoWriterError,
};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl, UserDisplayName, UserId, UserPictureRef,
    Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;

/// PostgreSQL-backed organization-management information writer.
pub struct PgOrganizationManagementInfoWriter;

impl PgOrganizationManagementInfoWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgOrganizationManagementInfoWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationManagementInfoWriter for PgOrganizationManagementInfoWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationManagementInfoUpsert,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO organization_management_infos (
                id, owner_user_id, handle, display_name, description, website_url,
                picture_type, picture_object_name, picture_external_url, updated_at, created_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10, $11, $11, $12, $12)
            ON CONFLICT (id) DO UPDATE SET
                owner_user_id = EXCLUDED.owner_user_id,
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                description = EXCLUDED.description,
                website_url = EXCLUDED.website_url,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE organization_management_infos.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.owner_user_id.value())
        .bind(upsert.handle.value())
        .bind(upsert.display_name.value())
        .bind(upsert.description.as_ref().map(OrganizationDescription::value))
        .bind(
            upsert
                .website_url
                .as_ref()
                .map(|website_url| website_url.value().as_str()),
        )
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        owner_user_id: UserId,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_management_infos
               SET owner_user_id = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(owner_user_id.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_management_infos
               SET handle = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(handle.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_management_infos
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4,
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
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_management_infos
               SET description = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(description.as_ref().map(OrganizationDescription::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_management_infos
               SET website_url = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(
            website_url
                .as_ref()
                .map(|website_url| website_url.value().as_str()),
        )
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE organization_management_infos
               SET picture_type = $2, picture_object_name = $3, picture_external_url = $4,
                   updated_at = $5, updated_event_sequence = $6, updated_event_id = $7
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
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            "DELETE FROM organization_management_infos WHERE id = $1 AND updated_event_sequence < $2",
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationManagementInfoOwnerUpsert,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO organization_management_info_owner_users (
                user_id, username, display_name, picture_type, picture_object_name,
                picture_external_url, updated_at, created_at, source_event_sequence,
                updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $7, $8, $8, $9, $9)
            ON CONFLICT (user_id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE organization_management_info_owner_users.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.user_id.value())
        .bind(upsert.username.as_ref().map(Username::value))
        .bind(upsert.display_name.as_ref().map(UserDisplayName::value))
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_owner_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_management_info_owner_users
               SET username = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE user_id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(user_id.value())
        .bind(username.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_owner_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_management_info_owner_users
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE user_id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(user_id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_owner_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE organization_management_info_owner_users
               SET picture_type = $2, picture_object_name = $3, picture_external_url = $4,
                   updated_at = $5, updated_event_sequence = $6, updated_event_id = $7
             WHERE user_id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(user_id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn delete_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), OrganizationManagementInfoWriterError> {
        sqlx::query(
            r#"
            DELETE FROM organization_management_info_owner_users
             WHERE user_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }
}

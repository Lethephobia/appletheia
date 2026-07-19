use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationInternalInfoUpsert, OrganizationInternalInfoWriter,
    OrganizationInternalInfoWriterError,
};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

/// PostgreSQL-backed organization-internal information writer.
pub struct PgOrganizationInternalInfoWriter;

impl PgOrganizationInternalInfoWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgOrganizationInternalInfoWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationInternalInfoWriter for PgOrganizationInternalInfoWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationInternalInfoUpsert,
    ) -> Result<(), OrganizationInternalInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO organization_internal_infos (
                id, handle, display_name, description, website_url, picture_type,
                picture_object_name, picture_external_url, updated_at, created_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9, $10, $10, $11, $11)
            ON CONFLICT (id) DO UPDATE SET
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
            WHERE organization_internal_infos.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
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
        .map_err(|error| OrganizationInternalInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OrganizationInternalInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_internal_infos
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
        .map_err(|error| OrganizationInternalInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationInternalInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_internal_infos
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
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
        .map_err(|error| OrganizationInternalInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
    ) -> Result<(), OrganizationInternalInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_internal_infos
               SET description = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
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
        .map_err(|error| OrganizationInternalInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
    ) -> Result<(), OrganizationInternalInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_internal_infos
               SET website_url = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
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
        .map_err(|error| OrganizationInternalInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationInternalInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE organization_internal_infos
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
        .map_err(|error| OrganizationInternalInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), OrganizationInternalInfoWriterError> {
        sqlx::query(
            "DELETE FROM organization_internal_infos WHERE id = $1 AND updated_event_sequence < $2",
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationInternalInfoWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }
}

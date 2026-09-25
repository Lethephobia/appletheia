use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationFragment, OrganizationFragmentUpsert, OrganizationFragmentWriter,
    OrganizationFragmentWriterError,
};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl, UserId,
};

use super::PgOrganizationFragmentRow;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

/// PostgreSQL-backed writer for shared organization fragments.
pub struct PgOrganizationFragmentWriter;

impl PgOrganizationFragmentWriter {
    pub fn new() -> Self {
        Self
    }

    /// Maps the partition a write returned, or `None` when the guard rejected the event.
    fn map_organization(
        organization_row: Option<PgOrganizationFragmentRow>,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError> {
        let Some(row) = organization_row else {
            return Ok(None);
        };
        let fragment = OrganizationFragment::try_from(row)
            .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(Some(fragment))
    }
}

impl Default for PgOrganizationFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationFragmentWriter for PgOrganizationFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: OrganizationFragmentUpsert,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(upsert.picture.as_ref());

        let upserted_row = sqlx::query_as::<_, PgOrganizationFragmentRow>(
            r#"
            INSERT INTO organization_fragments (
                id, owner_user_id, owner_since, owner_source_event_id, owner_updated_event_id,
                handle, display_name, description, website_url, picture_type, picture_object_name,
                picture_external_url, updated_at, created_at, source_event_sequence,
                updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $10, $12, $12, $3, $4, $5, $6, $7, $8, $9, $10, $10, $11, $11, $12, $12)
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
            WHERE organization_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            RETURNING
                id,
                owner_user_id,
                owner_since,
                owner_source_event_id,
                owner_updated_event_id,
                handle,
                display_name,
                description,
                website_url,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.owner_user_id.value())
        .bind(upsert.handle.value())
        .bind(upsert.display_name.value())
        .bind(
            upsert
                .description
                .as_ref()
                .map(OrganizationDescription::value),
        )
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
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_organization(upserted_row)
    }

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        owner_user_id: UserId,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError> {
        let updated_row = sqlx::query_as::<_, PgOrganizationFragmentRow>(
            r#"
            UPDATE organization_fragments
               SET owner_user_id = $2, owner_since = $3,
                   owner_source_event_id = $5, owner_updated_event_id = $5,
                   updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            RETURNING
                id,
                owner_user_id,
                owner_since,
                owner_source_event_id,
                owner_updated_event_id,
                handle,
                display_name,
                description,
                website_url,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            "#,
        )
        .bind(id.value())
        .bind(owner_user_id.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_organization(updated_row)
    }

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError> {
        let updated_row = sqlx::query_as::<_, PgOrganizationFragmentRow>(
            r#"
            UPDATE organization_fragments
               SET handle = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            RETURNING
                id,
                owner_user_id,
                owner_since,
                owner_source_event_id,
                owner_updated_event_id,
                handle,
                display_name,
                description,
                website_url,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            "#,
        )
        .bind(id.value())
        .bind(handle.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_organization(updated_row)
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError> {
        let updated_row = sqlx::query_as::<_, PgOrganizationFragmentRow>(
            r#"
            UPDATE organization_fragments
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            RETURNING
                id,
                owner_user_id,
                owner_since,
                owner_source_event_id,
                owner_updated_event_id,
                handle,
                display_name,
                description,
                website_url,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_organization(updated_row)
    }

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError> {
        let updated_row = sqlx::query_as::<_, PgOrganizationFragmentRow>(
            r#"
            UPDATE organization_fragments
               SET description = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            RETURNING
                id,
                owner_user_id,
                owner_since,
                owner_source_event_id,
                owner_updated_event_id,
                handle,
                display_name,
                description,
                website_url,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            "#,
        )
        .bind(id.value())
        .bind(description.as_ref().map(OrganizationDescription::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_organization(updated_row)
    }

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError> {
        let updated_row = sqlx::query_as::<_, PgOrganizationFragmentRow>(
            r#"
            UPDATE organization_fragments
               SET website_url = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            RETURNING
                id,
                owner_user_id,
                owner_since,
                owner_source_event_id,
                owner_updated_event_id,
                handle,
                display_name,
                description,
                website_url,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
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
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_organization(updated_row)
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<Option<OrganizationFragment>, OrganizationFragmentWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());

        let updated_row = sqlx::query_as::<_, PgOrganizationFragmentRow>(
            r#"
            UPDATE organization_fragments
               SET picture_type = $2, picture_object_name = $3, picture_external_url = $4,
                   updated_at = $5, updated_event_sequence = $6, updated_event_id = $7
             WHERE id = $1 AND updated_event_sequence < $6
            RETURNING
                id,
                owner_user_id,
                owner_since,
                owner_source_event_id,
                owner_updated_event_id,
                handle,
                display_name,
                description,
                website_url,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            "#,
        )
        .bind(id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_organization(updated_row)
    }

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: OrganizationId,
    ) -> Result<bool, OrganizationFragmentWriterError> {
        let delete_result = sqlx::query(
            "DELETE FROM organization_fragments WHERE id = $1 AND updated_event_sequence < $2",
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(delete_result.rows_affected() > 0)
    }
}

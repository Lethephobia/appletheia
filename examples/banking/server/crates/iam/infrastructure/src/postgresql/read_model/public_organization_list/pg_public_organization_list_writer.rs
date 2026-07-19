use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    PublicOrganizationListUpsert, PublicOrganizationListWriter, PublicOrganizationListWriterError,
};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

/// PostgreSQL-backed public organization list writer.
pub struct PgPublicOrganizationListWriter;

impl PgPublicOrganizationListWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgPublicOrganizationListWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicOrganizationListWriter for PgPublicOrganizationListWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicOrganizationListUpsert,
    ) -> Result<(), PublicOrganizationListWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO public_organization_list_items (
                id, handle, display_name, picture_type, picture_object_name,
                picture_external_url, updated_at, created_at, source_event_sequence,
                updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $7, $8, $8, $9, $9)
            ON CONFLICT (id) DO UPDATE SET
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE public_organization_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.handle.value())
        .bind(upsert.display_name.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PublicOrganizationListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), PublicOrganizationListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_organization_list_items
               SET handle = $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
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
        .map_err(|error| PublicOrganizationListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), PublicOrganizationListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_organization_list_items
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
        .map_err(|error| PublicOrganizationListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), PublicOrganizationListWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE public_organization_list_items
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
        .map_err(|error| PublicOrganizationListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), PublicOrganizationListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM public_organization_list_items
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PublicOrganizationListWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }
}

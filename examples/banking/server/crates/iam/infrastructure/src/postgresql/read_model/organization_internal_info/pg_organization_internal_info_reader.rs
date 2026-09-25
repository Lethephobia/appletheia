use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationInternalInfo, OrganizationInternalInfoReader, OrganizationInternalInfoReaderError,
};
use banking_iam_domain::OrganizationId;

use super::pg_organization_internal_info_row::PgOrganizationInternalInfoRow;

/// PostgreSQL-backed organization-internal information reader.
pub struct PgOrganizationInternalInfoReader;

impl PgOrganizationInternalInfoReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgOrganizationInternalInfoReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationInternalInfoReader for PgOrganizationInternalInfoReader {
    type Uow = PgUnitOfWork;

    async fn find(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
    ) -> Result<Option<OrganizationInternalInfo>, OrganizationInternalInfoReaderError> {
        let row = sqlx::query_as::<_, PgOrganizationInternalInfoRow>(
            r#"
            SELECT
                id AS organization_id,
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
            FROM organization_fragments
            WHERE id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationInternalInfoReaderError::Persistence(Box::new(error)))?;

        row.map(OrganizationInternalInfo::try_from)
            .transpose()
            .map_err(|error| OrganizationInternalInfoReaderError::Persistence(Box::new(error)))
    }
}

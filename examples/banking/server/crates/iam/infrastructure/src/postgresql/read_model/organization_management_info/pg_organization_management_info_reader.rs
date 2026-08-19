use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationManagementInfo, OrganizationManagementInfoReader,
    OrganizationManagementInfoReaderError,
};
use banking_iam_domain::OrganizationId;

use super::pg_organization_management_info_row::PgOrganizationManagementInfoRow;

/// PostgreSQL-backed organization-management information reader.
pub struct PgOrganizationManagementInfoReader;

impl PgOrganizationManagementInfoReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgOrganizationManagementInfoReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationManagementInfoReader for PgOrganizationManagementInfoReader {
    type Uow = PgUnitOfWork;

    async fn find(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
    ) -> Result<Option<OrganizationManagementInfo>, OrganizationManagementInfoReaderError> {
        let row = sqlx::query_as::<_, PgOrganizationManagementInfoRow>(
            r#"
            SELECT
                i.id AS organization_id,
                i.handle,
                i.display_name,
                i.description,
                i.website_url,
                i.picture_type,
                i.picture_object_name,
                i.picture_external_url,
                i.created_at,
                i.source_event_id,
                i.updated_event_id,
                i.owner_user_id,
                o.username AS owner_username,
                o.display_name AS owner_display_name,
                o.picture_type AS owner_picture_type,
                o.picture_object_name AS owner_picture_object_name,
                o.picture_external_url AS owner_picture_external_url,
                o.source_event_id AS owner_source_event_id,
                o.updated_event_id AS owner_updated_event_id
            FROM organization_fragments AS i
            INNER JOIN user_fragments AS o
                    ON o.id = i.owner_user_id
            WHERE i.id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationManagementInfoReaderError::Persistence(Box::new(error)))?;

        row.map(OrganizationManagementInfo::try_from)
            .transpose()
            .map_err(|error| OrganizationManagementInfoReaderError::Persistence(Box::new(error)))
    }
}

use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::OrganizationInternalInfo;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationWebsiteUrl,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_organization_internal_info_row_error::PgOrganizationInternalInfoRowError;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationInternalInfoRow {
    pub organization_id: Uuid,
    pub handle: String,
    pub display_name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgOrganizationInternalInfoRow> for OrganizationInternalInfo {
    type Error = PgOrganizationInternalInfoRowError;

    fn try_from(row: PgOrganizationInternalInfoRow) -> Result<Self, Self::Error> {
        let description = row
            .description
            .map(OrganizationDescription::try_from)
            .transpose()
            .map_err(|error| PgOrganizationInternalInfoRowError::Description(Box::new(error)))?;
        let website_url = row
            .website_url
            .map(OrganizationWebsiteUrl::try_from)
            .transpose()
            .map_err(|error| PgOrganizationInternalInfoRowError::WebsiteUrl(Box::new(error)))?;

        Ok(Self {
            id: OrganizationId::try_from_uuid(row.organization_id).map_err(|error| {
                PgOrganizationInternalInfoRowError::OrganizationId(Box::new(error))
            })?,
            handle: OrganizationHandle::try_from(row.handle)
                .map_err(|error| PgOrganizationInternalInfoRowError::Handle(Box::new(error)))?,
            display_name: OrganizationDisplayName::try_from(row.display_name).map_err(|error| {
                PgOrganizationInternalInfoRowError::DisplayName(Box::new(error))
            })?,
            description,
            website_url,
            picture: PgOrganizationPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| PgOrganizationInternalInfoRowError::Picture(Box::new(error)))?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgOrganizationInternalInfoRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgOrganizationInternalInfoRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}

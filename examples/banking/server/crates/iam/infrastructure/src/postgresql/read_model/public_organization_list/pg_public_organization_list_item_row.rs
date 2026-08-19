use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::PublicOrganizationListItem;
use banking_iam_domain::{OrganizationDisplayName, OrganizationHandle, OrganizationId};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_public_organization_list_item_row_error::PgPublicOrganizationListItemRowError;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgPublicOrganizationListItemRow {
    pub organization_id: Uuid,
    pub handle: String,
    pub display_name: String,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgPublicOrganizationListItemRow> for PublicOrganizationListItem {
    type Error = PgPublicOrganizationListItemRowError;

    fn try_from(row: PgPublicOrganizationListItemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            organization_id: OrganizationId::try_from_uuid(row.organization_id).map_err(
                |error| PgPublicOrganizationListItemRowError::OrganizationId(Box::new(error)),
            )?,
            handle: OrganizationHandle::try_from(row.handle)
                .map_err(|error| PgPublicOrganizationListItemRowError::Handle(Box::new(error)))?,
            display_name: OrganizationDisplayName::try_from(row.display_name).map_err(|error| {
                PgPublicOrganizationListItemRowError::DisplayName(Box::new(error))
            })?,
            picture: PgOrganizationPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| PgPublicOrganizationListItemRowError::Picture(Box::new(error)))?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgPublicOrganizationListItemRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgPublicOrganizationListItemRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}

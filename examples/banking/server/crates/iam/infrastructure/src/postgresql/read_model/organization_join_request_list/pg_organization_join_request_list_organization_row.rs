use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId};
use banking_iam_application::InternalOrganizationSummaryPart;
use banking_iam_domain::{OrganizationDisplayName, OrganizationHandle, OrganizationId};
use uuid::Uuid;

use super::pg_organization_join_request_list_organization_row_error::PgOrganizationJoinRequestListOrganizationRowError;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationJoinRequestListOrganizationRow {
    pub organization_id: Uuid,
    pub handle: String,
    pub display_name: String,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgOrganizationJoinRequestListOrganizationRow> for InternalOrganizationSummaryPart {
    type Error = PgOrganizationJoinRequestListOrganizationRowError;

    fn try_from(row: PgOrganizationJoinRequestListOrganizationRow) -> Result<Self, Self::Error> {
        Ok(Self {
            organization_id: OrganizationId::try_from_uuid(row.organization_id).map_err(
                |error| {
                    PgOrganizationJoinRequestListOrganizationRowError::OrganizationId(Box::new(
                        error,
                    ))
                },
            )?,
            handle: OrganizationHandle::try_from(row.handle).map_err(|error| {
                PgOrganizationJoinRequestListOrganizationRowError::Handle(Box::new(error))
            })?,
            display_name: OrganizationDisplayName::try_from(row.display_name).map_err(|error| {
                PgOrganizationJoinRequestListOrganizationRowError::DisplayName(Box::new(error))
            })?,
            picture: PgOrganizationPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| {
                PgOrganizationJoinRequestListOrganizationRowError::Picture(Box::new(error))
            })?,
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgOrganizationJoinRequestListOrganizationRowError::SourceEventId(Box::new(
                        error,
                    ))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgOrganizationJoinRequestListOrganizationRowError::UpdatedEventId(Box::new(
                        error,
                    ))
                })?,
            ),
        })
    }
}

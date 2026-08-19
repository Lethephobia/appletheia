use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    OrganizationJoinRequestListItem, OrganizationJoinRequestListItemStatus,
    OrganizationJoinRequestListRequester,
};
use banking_iam_domain::{OrganizationJoinRequestId, UserDisplayName, UserId, Username};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_organization_join_request_list_item_row_error::PgOrganizationJoinRequestListItemRowError;
use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationJoinRequestListItemRow {
    pub join_request_id: Uuid,
    pub requester_user_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
    pub requester_username: Option<String>,
    pub requester_display_name: Option<String>,
    pub requester_picture_type: Option<String>,
    pub requester_picture_object_name: Option<String>,
    pub requester_picture_external_url: Option<String>,
    pub requester_source_event_id: Uuid,
    pub requester_updated_event_id: Uuid,
}

impl PgOrganizationJoinRequestListItemRow {
    fn status(
        value: String,
    ) -> Result<OrganizationJoinRequestListItemStatus, PgOrganizationJoinRequestListItemRowError>
    {
        match value.as_str() {
            "pending" => Ok(OrganizationJoinRequestListItemStatus::Pending),
            "approved" => Ok(OrganizationJoinRequestListItemStatus::Approved),
            "rejected" => Ok(OrganizationJoinRequestListItemStatus::Rejected),
            "canceled" => Ok(OrganizationJoinRequestListItemStatus::Canceled),
            _ => Err(PgOrganizationJoinRequestListItemRowError::UnknownStatus(
                value,
            )),
        }
    }
}

impl TryFrom<PgOrganizationJoinRequestListItemRow> for OrganizationJoinRequestListItem {
    type Error = PgOrganizationJoinRequestListItemRowError;

    fn try_from(row: PgOrganizationJoinRequestListItemRow) -> Result<Self, Self::Error> {
        let requester_username = row
            .requester_username
            .map(Username::try_from)
            .transpose()
            .map_err(|error| {
                PgOrganizationJoinRequestListItemRowError::RequesterUsername(Box::new(error))
            })?;
        let requester_display_name = row
            .requester_display_name
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| {
                PgOrganizationJoinRequestListItemRowError::RequesterDisplayName(Box::new(error))
            })?;
        Ok(Self {
            join_request_id: OrganizationJoinRequestId::try_from_uuid(row.join_request_id)
                .map_err(|error| {
                    PgOrganizationJoinRequestListItemRowError::JoinRequestId(Box::new(error))
                })?,
            requester: OrganizationJoinRequestListRequester {
                user_id: UserId::try_from_uuid(row.requester_user_id).map_err(|error| {
                    PgOrganizationJoinRequestListItemRowError::RequesterUserId(Box::new(error))
                })?,
                username: requester_username,
                display_name: requester_display_name,
                picture: PgUserPictureRefColumns {
                    picture_type: row.requester_picture_type,
                    object_name: row.requester_picture_object_name,
                    external_url: row.requester_picture_external_url,
                }
                .into_picture()
                .map_err(|error| {
                    PgOrganizationJoinRequestListItemRowError::RequesterPicture(Box::new(error))
                })?,
                observation: ReadModelObservation::new(
                    EventId::try_from(row.requester_source_event_id).map_err(|error| {
                        PgOrganizationJoinRequestListItemRowError::RequesterSourceEventId(Box::new(
                            error,
                        ))
                    })?,
                    EventId::try_from(row.requester_updated_event_id).map_err(|error| {
                        PgOrganizationJoinRequestListItemRowError::RequesterUpdatedEventId(
                            Box::new(error),
                        )
                    })?,
                ),
            },
            status: PgOrganizationJoinRequestListItemRow::status(row.status)?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgOrganizationJoinRequestListItemRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgOrganizationJoinRequestListItemRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}

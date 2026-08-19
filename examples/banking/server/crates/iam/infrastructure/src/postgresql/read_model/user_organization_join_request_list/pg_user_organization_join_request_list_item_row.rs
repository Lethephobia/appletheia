use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    UserOrganizationJoinRequestListItem, UserOrganizationJoinRequestListItemStatus,
    UserOrganizationJoinRequestListOrganization,
};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationJoinRequestId,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_user_organization_join_request_list_item_row_error::PgUserOrganizationJoinRequestListItemRowError;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserOrganizationJoinRequestListItemRow {
    pub join_request_id: Uuid,
    pub organization_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
    pub organization_handle: String,
    pub organization_display_name: String,
    pub organization_picture_type: Option<String>,
    pub organization_picture_object_name: Option<String>,
    pub organization_picture_external_url: Option<String>,
    pub organization_source_event_id: Uuid,
    pub organization_updated_event_id: Uuid,
}

impl PgUserOrganizationJoinRequestListItemRow {
    fn status(
        value: String,
    ) -> Result<
        UserOrganizationJoinRequestListItemStatus,
        PgUserOrganizationJoinRequestListItemRowError,
    > {
        match value.as_str() {
            "pending" => Ok(UserOrganizationJoinRequestListItemStatus::Pending),
            "approved" => Ok(UserOrganizationJoinRequestListItemStatus::Approved),
            "rejected" => Ok(UserOrganizationJoinRequestListItemStatus::Rejected),
            "canceled" => Ok(UserOrganizationJoinRequestListItemStatus::Canceled),
            _ => Err(PgUserOrganizationJoinRequestListItemRowError::UnknownStatus(value)),
        }
    }
}

impl TryFrom<PgUserOrganizationJoinRequestListItemRow> for UserOrganizationJoinRequestListItem {
    type Error = PgUserOrganizationJoinRequestListItemRowError;

    fn try_from(row: PgUserOrganizationJoinRequestListItemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            join_request_id: OrganizationJoinRequestId::try_from_uuid(row.join_request_id)
                .map_err(|error| {
                    PgUserOrganizationJoinRequestListItemRowError::JoinRequestId(Box::new(error))
                })?,
            organization: UserOrganizationJoinRequestListOrganization {
                organization_id: OrganizationId::try_from_uuid(row.organization_id).map_err(
                    |error| {
                        PgUserOrganizationJoinRequestListItemRowError::OrganizationId(Box::new(
                            error,
                        ))
                    },
                )?,
                handle: OrganizationHandle::try_from(row.organization_handle).map_err(|error| {
                    PgUserOrganizationJoinRequestListItemRowError::OrganizationHandle(Box::new(
                        error,
                    ))
                })?,
                display_name: OrganizationDisplayName::try_from(row.organization_display_name)
                    .map_err(|error| {
                        PgUserOrganizationJoinRequestListItemRowError::OrganizationDisplayName(
                            Box::new(error),
                        )
                    })?,
                picture: PgOrganizationPictureRefColumns {
                    picture_type: row.organization_picture_type,
                    object_name: row.organization_picture_object_name,
                    external_url: row.organization_picture_external_url,
                }
                .into_picture()
                .map_err(|error| {
                    PgUserOrganizationJoinRequestListItemRowError::OrganizationPicture(Box::new(
                        error,
                    ))
                })?,
                observation: ReadModelObservation::new(
                    EventId::try_from(row.organization_source_event_id).map_err(|error| {
                        PgUserOrganizationJoinRequestListItemRowError::OrganizationSourceEventId(
                            Box::new(error),
                        )
                    })?,
                    EventId::try_from(row.organization_updated_event_id).map_err(|error| {
                        PgUserOrganizationJoinRequestListItemRowError::OrganizationUpdatedEventId(
                            Box::new(error),
                        )
                    })?,
                ),
            },
            status: PgUserOrganizationJoinRequestListItemRow::status(row.status)?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgUserOrganizationJoinRequestListItemRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgUserOrganizationJoinRequestListItemRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}

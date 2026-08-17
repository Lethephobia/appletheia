use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId};
use banking_iam_application::InternalUserSummaryPart;
use banking_iam_domain::{UserDisplayName, UserId, Username};
use uuid::Uuid;

use super::pg_user_organization_join_request_list_user_row_error::PgUserOrganizationJoinRequestListUserRowError;
use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserOrganizationJoinRequestListUserRow {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgUserOrganizationJoinRequestListUserRow> for InternalUserSummaryPart {
    type Error = PgUserOrganizationJoinRequestListUserRowError;

    fn try_from(row: PgUserOrganizationJoinRequestListUserRow) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: UserId::try_from_uuid(row.user_id).map_err(|error| {
                PgUserOrganizationJoinRequestListUserRowError::UserId(Box::new(error))
            })?,
            username: row
                .username
                .map(Username::try_from)
                .transpose()
                .map_err(|error| {
                    PgUserOrganizationJoinRequestListUserRowError::Username(Box::new(error))
                })?,
            display_name: row
                .display_name
                .map(UserDisplayName::try_from)
                .transpose()
                .map_err(|error| {
                    PgUserOrganizationJoinRequestListUserRowError::DisplayName(Box::new(error))
                })?,
            picture: PgUserPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| {
                PgUserOrganizationJoinRequestListUserRowError::Picture(Box::new(error))
            })?,
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgUserOrganizationJoinRequestListUserRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgUserOrganizationJoinRequestListUserRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}

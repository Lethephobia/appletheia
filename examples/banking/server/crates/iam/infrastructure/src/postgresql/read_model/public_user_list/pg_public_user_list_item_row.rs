use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::PublicUserListItem;
use banking_iam_domain::{UserDisplayName, UserId, Username};
use banking_shared_kernel_application::read_model::ReadModelObservation;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;
use super::pg_public_user_list_item_row_error::PgPublicUserListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgPublicUserListItemRow {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgPublicUserListItemRow> for PublicUserListItem {
    type Error = PgPublicUserListItemRowError;

    fn try_from(row: PgPublicUserListItemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: UserId::try_from_uuid(row.user_id)
                .map_err(|error| PgPublicUserListItemRowError::UserId(Box::new(error)))?,
            username: row
                .username
                .map(Username::try_from)
                .transpose()
                .map_err(|error| PgPublicUserListItemRowError::Username(Box::new(error)))?,
            display_name: row
                .display_name
                .map(UserDisplayName::try_from)
                .transpose()
                .map_err(|error| PgPublicUserListItemRowError::DisplayName(Box::new(error)))?,
            picture: PgUserPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| PgPublicUserListItemRowError::Picture(Box::new(error)))?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgPublicUserListItemRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgPublicUserListItemRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}

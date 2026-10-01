use appletheia::domain::{AggregateId, EventOccurredAt};
use banking_iam_application::PublicUserListItem;
use banking_iam_domain::{UserDisplayName, UserId, Username};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

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
            .try_into_picture()
            .map_err(|error| PgPublicUserListItemRowError::Picture(Box::new(error)))?,
            created_at: EventOccurredAt::from(row.created_at),
        })
    }
}

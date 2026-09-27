use appletheia::domain::{AggregateId, EventOccurredAt};
use banking_iam_application::{OrganizationMemberListItem, OrganizationMemberListMember};
use banking_iam_domain::{UserDisplayName, UserId, Username};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_organization_member_list_item_row_error::PgOrganizationMemberListItemRowError;
use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationMemberListItemRow {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub roles: String,
    pub is_owner: bool,
    pub joined_at: DateTime<Utc>,
}

impl TryFrom<PgOrganizationMemberListItemRow> for OrganizationMemberListItem {
    type Error = PgOrganizationMemberListItemRowError;

    fn try_from(row: PgOrganizationMemberListItemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            member: OrganizationMemberListMember {
                user_id: UserId::try_from_uuid(row.user_id).map_err(|error| {
                    PgOrganizationMemberListItemRowError::UserId(Box::new(error))
                })?,
                username: row
                    .username
                    .map(Username::try_from)
                    .transpose()
                    .map_err(|error| {
                        PgOrganizationMemberListItemRowError::Username(Box::new(error))
                    })?,
                display_name: row
                    .display_name
                    .map(UserDisplayName::try_from)
                    .transpose()
                    .map_err(|error| {
                        PgOrganizationMemberListItemRowError::DisplayName(Box::new(error))
                    })?,
                picture: PgUserPictureRefColumns {
                    picture_type: row.picture_type,
                    object_name: row.picture_object_name,
                    external_url: row.picture_external_url,
                }
                .try_into_picture()
                .map_err(|error| PgOrganizationMemberListItemRowError::Picture(Box::new(error)))?,
            },
            roles: serde_json::from_str(&row.roles)
                .map_err(|error| PgOrganizationMemberListItemRowError::Roles(Box::new(error)))?,
            is_owner: row.is_owner,
            joined_at: EventOccurredAt::from(row.joined_at),
        })
    }
}

use appletheia::domain::AggregateId;
use banking_iam_application::UserOrganizationMembershipListUser;
use banking_iam_domain::{UserDisplayName, UserId, Username};
use uuid::Uuid;

use super::pg_user_organization_membership_list_user_row_error::PgUserOrganizationMembershipListUserRowError;
use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserOrganizationMembershipListUserRow {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
}

impl TryFrom<PgUserOrganizationMembershipListUserRow> for UserOrganizationMembershipListUser {
    type Error = PgUserOrganizationMembershipListUserRowError;

    fn try_from(row: PgUserOrganizationMembershipListUserRow) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: UserId::try_from_uuid(row.user_id).map_err(|error| {
                PgUserOrganizationMembershipListUserRowError::UserId(Box::new(error))
            })?,
            username: row
                .username
                .map(Username::try_from)
                .transpose()
                .map_err(|error| {
                    PgUserOrganizationMembershipListUserRowError::Username(Box::new(error))
                })?,
            display_name: row
                .display_name
                .map(UserDisplayName::try_from)
                .transpose()
                .map_err(|error| {
                    PgUserOrganizationMembershipListUserRowError::DisplayName(Box::new(error))
                })?,
            picture: PgUserPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .try_into_picture()
            .map_err(|error| {
                PgUserOrganizationMembershipListUserRowError::Picture(Box::new(error))
            })?,
        })
    }
}

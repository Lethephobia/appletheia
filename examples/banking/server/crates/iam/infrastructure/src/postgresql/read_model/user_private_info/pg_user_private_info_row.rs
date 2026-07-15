use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{UserPrivateInfo, UserPrivateInfoStatus};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};
use banking_shared_kernel_application::read_model::ReadModelObservation;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;
use super::pg_user_private_info_identity_row::PgUserPrivateInfoIdentityRow;
use super::pg_user_private_info_organization_membership_row::PgUserPrivateInfoOrganizationMembershipRow;
use super::pg_user_private_info_row_error::PgUserPrivateInfoRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserPrivateInfoRow {
    pub id: Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl PgUserPrivateInfoRow {
    pub fn into_user_private_info(
        self,
        identity_rows: Vec<PgUserPrivateInfoIdentityRow>,
        organization_membership_rows: Vec<PgUserPrivateInfoOrganizationMembershipRow>,
    ) -> Result<UserPrivateInfo, PgUserPrivateInfoRowError> {
        let identities = identity_rows
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;
        let organization_memberships = organization_membership_rows
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(UserPrivateInfo {
            id: UserId::try_from_uuid(self.id)
                .map_err(|error| PgUserPrivateInfoRowError::InvalidUserId(Box::new(error)))?,
            identities,
            organization_memberships,
            username: Self::optional_username(self.username)?,
            display_name: Self::optional_display_name(self.display_name)?,
            bio: Self::optional_bio(self.bio)?,
            picture: PgUserPictureRefColumns {
                picture_type: self.picture_type,
                object_name: self.picture_object_name,
                external_url: self.picture_external_url,
            }
            .into_picture()
            .map_err(|error| PgUserPrivateInfoRowError::InvalidUserPicture(Box::new(error)))?,
            status: Self::status(self.status)?,
            created_at: EventOccurredAt::from(self.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(self.source_event_id).map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidSourceEventId(Box::new(error))
                })?,
                EventId::try_from(self.updated_event_id).map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidUpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }

    fn status(value: String) -> Result<UserPrivateInfoStatus, PgUserPrivateInfoRowError> {
        match value.as_str() {
            "active" => Ok(UserPrivateInfoStatus::Active),
            "inactive" => Ok(UserPrivateInfoStatus::Inactive),
            value => Err(PgUserPrivateInfoRowError::UnknownStatus(value.to_owned())),
        }
    }

    fn optional_username(
        value: Option<String>,
    ) -> Result<Option<Username>, PgUserPrivateInfoRowError> {
        value
            .map(Username::try_from)
            .transpose()
            .map_err(|error| PgUserPrivateInfoRowError::InvalidUsername(Box::new(error)))
    }

    fn optional_display_name(
        value: Option<String>,
    ) -> Result<Option<UserDisplayName>, PgUserPrivateInfoRowError> {
        value
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| PgUserPrivateInfoRowError::InvalidUserDisplayName(Box::new(error)))
    }

    fn optional_bio(value: Option<String>) -> Result<Option<UserBio>, PgUserPrivateInfoRowError> {
        value
            .map(UserBio::try_from)
            .transpose()
            .map_err(|error| PgUserPrivateInfoRowError::InvalidUserBio(Box::new(error)))
    }
}

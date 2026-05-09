use appletheia::domain::{AggregateId, EventOccurredAt};
use banking_iam_application::{UserPrivateInfo, UserPrivateInfoIdentity, UserPrivateInfoStatus};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};
use sqlx::types::chrono::{DateTime, Utc};

use super::pg_user_private_info_row_error::PgUserPrivateInfoRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserPrivateInfoRow {
    pub id: uuid::Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub picture: Option<sqlx::types::Json<UserPictureRef>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl PgUserPrivateInfoRow {
    pub fn into_user_private_info(
        self,
        identities: Vec<UserPrivateInfoIdentity>,
    ) -> Result<UserPrivateInfo, PgUserPrivateInfoRowError> {
        Ok(UserPrivateInfo {
            id: UserId::try_from_uuid(self.id)
                .map_err(|error| PgUserPrivateInfoRowError::InvalidUserId(Box::new(error)))?,
            identities,
            username: Self::optional_username(self.username)?,
            display_name: Self::optional_display_name(self.display_name)?,
            bio: Self::optional_bio(self.bio)?,
            picture: self.picture.map(|value| value.0),
            status: Self::status(self.status)?,
            created_at: EventOccurredAt::from(self.created_at),
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

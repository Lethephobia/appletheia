use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::UserPublicProfile;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};
use banking_shared_kernel_application::read_model::ReadModelObservation;
use sqlx::types::chrono::{DateTime, Utc};

use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;
use super::pg_user_public_profile_row_error::PgUserPublicProfileRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserPublicProfileRow {
    pub id: uuid::Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: uuid::Uuid,
    pub updated_event_id: uuid::Uuid,
}

impl PgUserPublicProfileRow {
    fn optional_username(
        value: Option<String>,
    ) -> Result<Option<Username>, PgUserPublicProfileRowError> {
        value
            .map(Username::try_from)
            .transpose()
            .map_err(|error| PgUserPublicProfileRowError::Username(Box::new(error)))
    }

    fn optional_display_name(
        value: Option<String>,
    ) -> Result<Option<UserDisplayName>, PgUserPublicProfileRowError> {
        value
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| PgUserPublicProfileRowError::UserDisplayName(Box::new(error)))
    }

    fn optional_bio(value: Option<String>) -> Result<Option<UserBio>, PgUserPublicProfileRowError> {
        value
            .map(UserBio::try_from)
            .transpose()
            .map_err(|error| PgUserPublicProfileRowError::UserBio(Box::new(error)))
    }
}

impl TryFrom<PgUserPublicProfileRow> for UserPublicProfile {
    type Error = PgUserPublicProfileRowError;

    fn try_from(row: PgUserPublicProfileRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: UserId::try_from_uuid(row.id)
                .map_err(|error| PgUserPublicProfileRowError::UserId(Box::new(error)))?,
            username: PgUserPublicProfileRow::optional_username(row.username)?,
            display_name: PgUserPublicProfileRow::optional_display_name(row.display_name)?,
            bio: PgUserPublicProfileRow::optional_bio(row.bio)?,
            picture: PgUserPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| PgUserPublicProfileRowError::UserPicture(Box::new(error)))?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id)
                    .map_err(|error| PgUserPublicProfileRowError::SourceEventId(Box::new(error)))?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgUserPublicProfileRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}

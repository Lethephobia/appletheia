use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::UserPublicProfile;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

use super::pg_user_public_profile_row_error::PgUserPublicProfileRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserPublicProfileRow {
    pub id: Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgUserPublicProfileRow> for UserPublicProfile {
    type Error = PgUserPublicProfileRowError;

    fn try_from(row: PgUserPublicProfileRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: UserId::try_from_uuid(row.id)
                .map_err(|error| PgUserPublicProfileRowError::UserId(Box::new(error)))?,
            username: row
                .username
                .map(Username::try_from)
                .transpose()
                .map_err(|error| PgUserPublicProfileRowError::Username(Box::new(error)))?,
            display_name: row
                .display_name
                .map(UserDisplayName::try_from)
                .transpose()
                .map_err(|error| PgUserPublicProfileRowError::DisplayName(Box::new(error)))?,
            bio: row
                .bio
                .map(UserBio::try_from)
                .transpose()
                .map_err(|error| PgUserPublicProfileRowError::Bio(Box::new(error)))?,
            picture: PgUserPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| PgUserPublicProfileRowError::Picture(Box::new(error)))?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id)
                    .map_err(|error| PgUserPublicProfileRowError::SourceEventId(Box::new(error)))?,
                EventId::try_from(row.updated_event_id)
                    .map_err(|error| PgUserPublicProfileRowError::UpdatedEventId(Box::new(error)))?,
            ),
        })
    }
}

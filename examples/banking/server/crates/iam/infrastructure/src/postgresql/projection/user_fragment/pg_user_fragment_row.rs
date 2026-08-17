use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{MaterializedUserStatus, UserFragment};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

use super::pg_user_fragment_row_error::PgUserFragmentRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserFragmentRow {
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

impl TryFrom<PgUserFragmentRow> for UserFragment {
    type Error = PgUserFragmentRowError;

    fn try_from(row: PgUserFragmentRow) -> Result<Self, Self::Error> {
        let username = row
            .username
            .map(Username::try_from)
            .transpose()
            .map_err(|error| PgUserFragmentRowError::Username(Box::new(error)))?;
        let display_name = row
            .display_name
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| PgUserFragmentRowError::DisplayName(Box::new(error)))?;
        let bio = row
            .bio
            .map(UserBio::try_from)
            .transpose()
            .map_err(|error| PgUserFragmentRowError::Bio(Box::new(error)))?;
        let status = match row.status.as_str() {
            "active" => MaterializedUserStatus::Active,
            "inactive" => MaterializedUserStatus::Inactive,
            value => {
                return Err(PgUserFragmentRowError::UnknownStatus(value.to_owned()));
            }
        };

        Ok(Self {
            id: UserId::try_from_uuid(row.id)
                .map_err(|error| PgUserFragmentRowError::UserId(Box::new(error)))?,
            username,
            display_name,
            bio,
            picture: PgUserPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| PgUserFragmentRowError::Picture(Box::new(error)))?,
            status,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id)
                    .map_err(|error| PgUserFragmentRowError::SourceEventId(Box::new(error)))?,
                EventId::try_from(row.updated_event_id)
                    .map_err(|error| PgUserFragmentRowError::UpdatedEventId(Box::new(error)))?,
            ),
        })
    }
}

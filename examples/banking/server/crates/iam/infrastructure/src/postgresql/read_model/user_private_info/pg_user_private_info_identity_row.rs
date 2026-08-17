use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId};
use banking_iam_application::PrivateUserIdentityPart;
use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;
use uuid::Uuid;

use super::pg_user_private_info_row_error::PgUserPrivateInfoRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserPrivateInfoIdentityRow {
    pub user_id: Uuid,
    pub provider: String,
    pub subject: String,
    pub email: Option<String>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl PgUserPrivateInfoIdentityRow {
    fn optional_email(value: Option<String>) -> Result<Option<Email>, PgUserPrivateInfoRowError> {
        value
            .map(Email::try_from)
            .transpose()
            .map_err(|error| PgUserPrivateInfoRowError::InvalidEmail(Box::new(error)))
    }
}

impl TryFrom<PgUserPrivateInfoIdentityRow> for PrivateUserIdentityPart {
    type Error = PgUserPrivateInfoRowError;

    fn try_from(row: PgUserPrivateInfoIdentityRow) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: UserId::try_from_uuid(row.user_id)
                .map_err(|error| PgUserPrivateInfoRowError::InvalidUserId(Box::new(error)))?,
            provider: UserIdentityProvider::try_from(row.provider).map_err(|error| {
                PgUserPrivateInfoRowError::InvalidUserIdentityProvider(Box::new(error))
            })?,
            subject: UserIdentitySubject::try_from(row.subject).map_err(|error| {
                PgUserPrivateInfoRowError::InvalidUserIdentitySubject(Box::new(error))
            })?,
            email: PgUserPrivateInfoIdentityRow::optional_email(row.email)?,
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidSourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidUpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}

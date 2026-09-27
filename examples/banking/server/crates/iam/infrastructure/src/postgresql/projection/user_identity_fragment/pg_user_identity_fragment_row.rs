use appletheia::domain::{AggregateId, EventOccurredAt};
use banking_iam_application::{UserIdentityFragment, UserIdentityFragmentWriterError};
use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserIdentityFragmentRow {
    pub user_id: Uuid,
    pub provider: String,
    pub subject: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgUserIdentityFragmentRow> for UserIdentityFragment {
    type Error = UserIdentityFragmentWriterError;

    fn try_from(row: PgUserIdentityFragmentRow) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: UserId::try_from_uuid(row.user_id).map_err(persistence_error)?,
            provider: UserIdentityProvider::try_from(row.provider).map_err(persistence_error)?,
            subject: UserIdentitySubject::try_from(row.subject).map_err(persistence_error)?,
            email: row
                .email
                .map(Email::try_from)
                .transpose()
                .map_err(persistence_error)?,
            created_at: EventOccurredAt::from(row.created_at),
        })
    }
}

fn persistence_error(
    error: impl std::error::Error + Send + Sync + 'static,
) -> UserIdentityFragmentWriterError {
    UserIdentityFragmentWriterError::Persistence(Box::new(error))
}

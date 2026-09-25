use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserIdentityFragment, UserIdentityFragmentKey, UserIdentityFragmentUpsert,
    UserIdentityFragmentWriter, UserIdentityFragmentWriterError,
};
use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;

mod pg_user_identity_fragment_row;

use pg_user_identity_fragment_row::PgUserIdentityFragmentRow;

/// PostgreSQL-backed user identity fragment writer.
pub struct PgUserIdentityFragmentWriter;

impl PgUserIdentityFragmentWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgUserIdentityFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl UserIdentityFragmentWriter for PgUserIdentityFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: UserIdentityFragmentUpsert,
    ) -> Result<Option<UserIdentityFragment>, UserIdentityFragmentWriterError> {
        let row = sqlx::query_as::<_, PgUserIdentityFragmentRow>(
            r#"
            INSERT INTO user_identity_fragments (
                user_id, provider, subject, email, updated_at, created_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $5, $6, $6, $7, $7)
            ON CONFLICT (user_id, provider, subject) DO UPDATE SET
                email = EXCLUDED.email,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE user_identity_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            RETURNING user_id, provider, subject, email, created_at,
                      source_event_id, updated_event_id
            "#,
        )
        .bind(upsert.user_id.value())
        .bind(upsert.provider.value())
        .bind(upsert.subject.value())
        .bind(upsert.email.as_ref().map(Email::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserIdentityFragmentWriterError::Persistence(Box::new(error)))?;

        row.map(TryInto::try_into).transpose()
    }

    async fn update_email(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<Option<UserIdentityFragment>, UserIdentityFragmentWriterError> {
        let row = sqlx::query_as::<_, PgUserIdentityFragmentRow>(
            r#"
            UPDATE user_identity_fragments
               SET email = $4, updated_at = $5,
                   updated_event_sequence = $6, updated_event_id = $7
             WHERE user_id = $1
               AND provider = $2
               AND subject = $3
               AND updated_event_sequence < $6
            RETURNING user_id, provider, subject, email, created_at,
                      source_event_id, updated_event_id
            "#,
        )
        .bind(user_id.value())
        .bind(provider.value())
        .bind(subject.value())
        .bind(email.as_ref().map(Email::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserIdentityFragmentWriterError::Persistence(Box::new(error)))?;

        row.map(TryInto::try_into).transpose()
    }

    async fn delete_for_user(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
    ) -> Result<Vec<UserIdentityFragmentKey>, UserIdentityFragmentWriterError> {
        let rows = sqlx::query_as::<_, (uuid::Uuid, String, String)>(
            "DELETE FROM user_identity_fragments WHERE user_id = $1 AND updated_event_sequence < $2 RETURNING user_id, provider, subject",
        )
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserIdentityFragmentWriterError::Persistence(Box::new(error)))?;

        rows.into_iter()
            .map(|(removed_user_id, provider, subject)| {
                Ok(UserIdentityFragmentKey {
                    user_id: UserId::try_from_uuid(removed_user_id).map_err(|error| {
                        UserIdentityFragmentWriterError::Persistence(Box::new(error))
                    })?,
                    provider: UserIdentityProvider::try_from(provider).map_err(|error| {
                        UserIdentityFragmentWriterError::Persistence(Box::new(error))
                    })?,
                    subject: UserIdentitySubject::try_from(subject).map_err(|error| {
                        UserIdentityFragmentWriterError::Persistence(Box::new(error))
                    })?,
                })
            })
            .collect()
    }
}

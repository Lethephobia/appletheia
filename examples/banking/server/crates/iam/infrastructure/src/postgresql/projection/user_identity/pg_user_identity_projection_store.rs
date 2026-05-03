use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserIdentityProjectionStore, UserIdentityProjectionStoreError, UserIdentityProjectionUpsert,
};
use banking_iam_domain::{Email, UserId, UserIdentityProvider, UserIdentitySubject};

/// PostgreSQL-backed user identity projection store.
pub struct PgUserIdentityProjectionStore;

impl PgUserIdentityProjectionStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgUserIdentityProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl UserIdentityProjectionStore for PgUserIdentityProjectionStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: UserIdentityProjectionUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityProjectionStoreError> {
        sqlx::query(
            r#"
            INSERT INTO user_identities (
                provider,
                subject,
                user_id,
                email,
                updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (provider, subject) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                email = EXCLUDED.email,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE user_identities.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.provider.value())
        .bind(input.subject.value())
        .bind(input.user_id.value())
        .bind(input.email.as_ref().map(Email::value))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserIdentityProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_email(
        &self,
        uow: &mut Self::Uow,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE user_identities
               SET email = $3,
                   updated_event_sequence = $4
             WHERE provider = $1
               AND subject = $2
               AND updated_event_sequence < $4
            "#,
        )
        .bind(provider.value())
        .bind(subject.value())
        .bind(email.as_ref().map(Email::value))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserIdentityProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete_by_user(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityProjectionStoreError> {
        sqlx::query(
            r#"
            DELETE FROM user_identities
             WHERE user_id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(user_id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserIdentityProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}

use appletheia_application::authentication::oidc::{
    OidcLoginAttempt, OidcLoginAttemptStore, OidcLoginAttemptStoreError, OidcState,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::pg_oidc_login_attempt_row::PgOidcLoginAttemptRow;

#[derive(Debug)]
pub struct PgOidcLoginAttemptStore;

impl PgOidcLoginAttemptStore {
    pub fn new() -> Self {
        Self
    }

    fn parse_uuid(value: &str) -> Result<Uuid, OidcLoginAttemptStoreError> {
        Uuid::parse_str(value)
            .map_err(|source| OidcLoginAttemptStoreError::Backend(Box::new(source)))
    }
}

impl Default for PgOidcLoginAttemptStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OidcLoginAttemptStore for PgOidcLoginAttemptStore {
    type Uow = PgUnitOfWork;

    async fn save(
        &self,
        uow: &mut Self::Uow,
        attempt: &OidcLoginAttempt,
    ) -> Result<(), OidcLoginAttemptStoreError> {
        let transaction = uow.transaction_mut();

        let id_value = attempt.id().value();
        let state_value = Self::parse_uuid(attempt.state().value())?;
        let nonce_value = Self::parse_uuid(attempt.nonce().value())?;
        let pkce_code_verifier_value = attempt
            .pkce_code_verifier()
            .map(|value| value.value().to_string());
        let created_at_value = attempt.created_at();
        let expires_at_value = attempt.expires_at();
        let consumed_at_value = attempt.consumed_at();

        sqlx::query(
            r#"
            INSERT INTO oidc_login_attempts (
              id,
              state,
              nonce,
              pkce_code_verifier,
              created_at,
              expires_at,
              consumed_at
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5,
              $6,
              $7
            )
            "#,
        )
        .bind(id_value)
        .bind(state_value)
        .bind(nonce_value)
        .bind(pkce_code_verifier_value)
        .bind(created_at_value.value())
        .bind(expires_at_value.value())
        .bind(consumed_at_value.map(|value| value.value()))
        .execute(transaction.as_mut())
        .await
        .map_err(|source| OidcLoginAttemptStoreError::Backend(Box::new(source)))?;

        Ok(())
    }

    async fn consume_by_state(
        &self,
        uow: &mut Self::Uow,
        state: &OidcState,
    ) -> Result<OidcLoginAttempt, OidcLoginAttemptStoreError> {
        let transaction = uow.transaction_mut();

        let state_value = Self::parse_uuid(state.value())?;

        let row: Option<PgOidcLoginAttemptRow> = sqlx::query_as(
            r#"
            SELECT
              id,
              state,
              nonce,
              pkce_code_verifier,
              created_at,
              expires_at,
              consumed_at
            FROM oidc_login_attempts
            WHERE state = $1
            FOR UPDATE
            "#,
        )
        .bind(state_value)
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| OidcLoginAttemptStoreError::Backend(Box::new(source)))?;

        let row = row.ok_or(OidcLoginAttemptStoreError::NotFound)?;

        if row.consumed_at.is_some() {
            return Err(OidcLoginAttemptStoreError::AlreadyConsumed);
        }

        let now: DateTime<Utc> = sqlx::query_scalar(
            r#"
            SELECT now()
            "#,
        )
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| OidcLoginAttemptStoreError::Backend(Box::new(source)))?;

        if row.expires_at < now {
            return Err(OidcLoginAttemptStoreError::Expired);
        }

        let consumed_at: chrono::DateTime<chrono::Utc> = sqlx::query_scalar(
            r#"
            UPDATE oidc_login_attempts
               SET consumed_at = now()
             WHERE id = $1
               AND consumed_at IS NULL
            RETURNING consumed_at
            "#,
        )
        .bind(row.id)
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| OidcLoginAttemptStoreError::Backend(Box::new(source)))?;

        row.try_into_oidc_login_attempt_with_consumed_at(Some(consumed_at))
            .map_err(|source| OidcLoginAttemptStoreError::Backend(Box::new(source)))
    }
}

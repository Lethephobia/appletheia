use appletheia_application::authentication::{
    AuthTokenExchangeCodeHash, AuthTokenExchangeCodeRecord, AuthTokenExchangeCodeStore,
    AuthTokenExchangeCodeStoreError,
};
use chrono::{DateTime, Utc};

use crate::postgresql::PgUnitOfWork;

use super::pg_auth_token_exchange_code_row::PgAuthTokenExchangeCodeRow;

/// Persists auth token exchange codes in PostgreSQL.
#[derive(Debug)]
pub struct PgAuthTokenExchangeCodeStore;

impl PgAuthTokenExchangeCodeStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgAuthTokenExchangeCodeStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthTokenExchangeCodeStore for PgAuthTokenExchangeCodeStore {
    type Uow = PgUnitOfWork;

    async fn save(
        &self,
        uow: &mut Self::Uow,
        record: &AuthTokenExchangeCodeRecord,
    ) -> Result<(), AuthTokenExchangeCodeStoreError> {
        let transaction = uow.transaction_mut();

        sqlx::query(
            r#"
            INSERT INTO auth_token_exchange_codes (
                id,
                code_hash,
                code_challenge_method,
                code_challenge,
                encrypted_grant,
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
                $7,
                $8
            )
            "#,
        )
        .bind(record.id().value())
        .bind(record.code_hash().as_str())
        .bind(
            record
                .code_challenge_method()
                .map(|value| value.value().to_owned()),
        )
        .bind(
            record
                .code_challenge()
                .map(|value| value.value().to_owned()),
        )
        .bind(record.encrypted_grant().as_bytes())
        .bind(record.created_at().value())
        .bind(record.expires_at().value())
        .bind(record.consumed_at().map(|value| value.value()))
        .execute(transaction.as_mut())
        .await
        .map_err(|source| AuthTokenExchangeCodeStoreError::Backend(Box::new(source)))?;

        Ok(())
    }

    async fn consume_by_code_hash(
        &self,
        uow: &mut Self::Uow,
        code_hash: &AuthTokenExchangeCodeHash,
    ) -> Result<AuthTokenExchangeCodeRecord, AuthTokenExchangeCodeStoreError> {
        let transaction = uow.transaction_mut();

        let row: Option<PgAuthTokenExchangeCodeRow> = sqlx::query_as(
            r#"
            SELECT
                id,
                code_hash,
                code_challenge_method,
                code_challenge,
                encrypted_grant,
                created_at,
                expires_at,
                consumed_at
            FROM auth_token_exchange_codes
            WHERE code_hash = $1
            FOR UPDATE
            "#,
        )
        .bind(code_hash.as_str())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| AuthTokenExchangeCodeStoreError::Backend(Box::new(source)))?;

        let row = row.ok_or(AuthTokenExchangeCodeStoreError::NotFound)?;

        if row.consumed_at.is_some() {
            return Err(AuthTokenExchangeCodeStoreError::AlreadyConsumed);
        }

        let now: DateTime<Utc> = sqlx::query_scalar(
            r#"
            SELECT now()
            "#,
        )
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| AuthTokenExchangeCodeStoreError::Backend(Box::new(source)))?;

        if row.expires_at < now {
            return Err(AuthTokenExchangeCodeStoreError::Expired);
        }

        let consumed_at: DateTime<Utc> = sqlx::query_scalar(
            r#"
            UPDATE auth_token_exchange_codes
               SET consumed_at = now()
             WHERE id = $1
               AND consumed_at IS NULL
            RETURNING consumed_at
            "#,
        )
        .bind(row.id)
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| AuthTokenExchangeCodeStoreError::Backend(Box::new(source)))?;

        PgAuthTokenExchangeCodeRow {
            consumed_at: Some(consumed_at),
            ..row
        }
        .try_into_record()
    }
}

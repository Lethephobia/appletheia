use std::marker::PhantomData;

use appletheia_application::authentication::oidc::{
    OidcContinuation, OidcContinuationStore, OidcContinuationStoreError, OidcState,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::pg_oidc_continuation_row::PgOidcContinuationRow;
use super::pg_oidc_continuation_row_error::PgOidcContinuationRowError;

#[derive(Debug)]
pub struct PgOidcContinuationStore<P> {
    _marker: PhantomData<fn() -> P>,
}

impl<P> PgOidcContinuationStore<P> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<P> Default for PgOidcContinuationStore<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> OidcContinuationStore<P> for PgOidcContinuationStore<P>
where
    P: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    type Uow = PgUnitOfWork;

    async fn save(
        &self,
        uow: &mut Self::Uow,
        continuation: &OidcContinuation<P>,
    ) -> Result<(), OidcContinuationStoreError> {
        let transaction = uow.transaction_mut();
        let id_value = Uuid::now_v7();
        let state_value = continuation.state().value();
        let payload_value = serde_json::to_value(continuation.payload())
            .map_err(|source| OidcContinuationStoreError::Backend(Box::new(source)))?;
        let expires_at_value = continuation.expires_at();
        let consumed_at_value = continuation.consumed_at();

        sqlx::query(
            r#"
            INSERT INTO oidc_continuations (
              id,
              state,
              payload,
              expires_at,
              consumed_at
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5
            )
            "#,
        )
        .bind(id_value)
        .bind(state_value)
        .bind(payload_value)
        .bind(expires_at_value.value())
        .bind(consumed_at_value.map(|value| value.value()))
        .execute(transaction.as_mut())
        .await
        .map_err(|source| OidcContinuationStoreError::Backend(Box::new(source)))?;

        Ok(())
    }

    async fn consume_by_state(
        &self,
        uow: &mut Self::Uow,
        state: &OidcState,
    ) -> Result<OidcContinuation<P>, OidcContinuationStoreError> {
        let transaction = uow.transaction_mut();

        let row: Option<PgOidcContinuationRow> = sqlx::query_as(
            r#"
            SELECT
              id,
              state,
              payload,
              expires_at,
              consumed_at
            FROM oidc_continuations
            WHERE state = $1
            FOR UPDATE
            "#,
        )
        .bind(state.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| OidcContinuationStoreError::Backend(Box::new(source)))?;

        let row = row.ok_or(OidcContinuationStoreError::NotFound)?;

        if row.consumed_at.is_some() {
            return Err(OidcContinuationStoreError::AlreadyConsumed);
        }

        let now: DateTime<Utc> = sqlx::query_scalar("SELECT now()")
            .fetch_one(transaction.as_mut())
            .await
            .map_err(|source| OidcContinuationStoreError::Backend(Box::new(source)))?;

        if row.expires_at < now {
            return Err(OidcContinuationStoreError::Expired);
        }

        let consumed_row: PgOidcContinuationRow = sqlx::query_as(
            r#"
            UPDATE oidc_continuations
               SET consumed_at = now()
             WHERE id = $1
               AND consumed_at IS NULL
            RETURNING
              id,
              state,
              payload,
              expires_at,
              consumed_at
            "#,
        )
        .bind(row.id)
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| OidcContinuationStoreError::Backend(Box::new(source)))?;

        consumed_row
            .try_into_oidc_continuation()
            .map_err(|source: PgOidcContinuationRowError| {
                OidcContinuationStoreError::Backend(Box::new(source))
            })
    }
}

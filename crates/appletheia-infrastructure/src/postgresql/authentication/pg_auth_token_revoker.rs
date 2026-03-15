use appletheia_application::authentication::{
    AuthTokenExpiresAt, AuthTokenId, AuthTokenIssuedAt, AuthTokenRevocationError, AuthTokenRevoker,
};
use appletheia_application::authorization::AggregateRef;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::postgresql::PgUnitOfWork;

/// Updates auth token revocation state in PostgreSQL.
#[derive(Debug)]
pub struct PgAuthTokenRevoker;

impl PgAuthTokenRevoker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgAuthTokenRevoker {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthTokenRevoker for PgAuthTokenRevoker {
    type Uow = PgUnitOfWork;

    async fn revoke_token(
        &self,
        uow: &mut Self::Uow,
        token_id: AuthTokenId,
        expires_at: AuthTokenExpiresAt,
    ) -> Result<(), AuthTokenRevocationError> {
        let transaction = uow.transaction_mut();
        let token_id = Uuid::from(token_id);
        let expires_at: DateTime<Utc> = expires_at.into();

        sqlx::query(
            r#"
            INSERT INTO auth_token_revocations (
                id,
                token_id,
                expires_at,
                revoked_at
            ) VALUES (
                $1,
                $2,
                $3,
                now()
            )
            ON CONFLICT (token_id) DO UPDATE
               SET expires_at = GREATEST(
                   auth_token_revocations.expires_at,
                   EXCLUDED.expires_at
               )
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(token_id)
        .bind(expires_at)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| AuthTokenRevocationError::Backend(Box::new(source)))?;

        Ok(())
    }

    async fn advance_revocation_cutoff(
        &self,
        uow: &mut Self::Uow,
        subject: &AggregateRef,
        issued_at: AuthTokenIssuedAt,
    ) -> Result<(), AuthTokenRevocationError> {
        let transaction = uow.transaction_mut();
        let revoke_before: DateTime<Utc> = issued_at.into();

        sqlx::query(
            r#"
            INSERT INTO auth_token_revocation_cutoffs (
                id,
                subject_aggregate_type,
                subject_aggregate_id,
                revoke_before,
                updated_at
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                now()
            )
            ON CONFLICT (subject_aggregate_type, subject_aggregate_id) DO UPDATE
               SET revoke_before = GREATEST(
                       auth_token_revocation_cutoffs.revoke_before,
                       EXCLUDED.revoke_before
                   ),
                   updated_at = now()
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(subject.aggregate_type.value())
        .bind(subject.aggregate_id.value())
        .bind(revoke_before)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| AuthTokenRevocationError::Backend(Box::new(source)))?;

        Ok(())
    }
}

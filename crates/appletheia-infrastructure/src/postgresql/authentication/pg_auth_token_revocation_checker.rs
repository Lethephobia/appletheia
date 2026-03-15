use appletheia_application::authentication::{
    AuthTokenClaims, AuthTokenRevocationChecker, AuthTokenRevocationError,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::postgresql::PgUnitOfWork;

/// Checks auth token revocation state in PostgreSQL.
#[derive(Debug)]
pub struct PgAuthTokenRevocationChecker;

impl PgAuthTokenRevocationChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgAuthTokenRevocationChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthTokenRevocationChecker for PgAuthTokenRevocationChecker {
    type Uow = PgUnitOfWork;

    async fn is_token_revoked(
        &self,
        uow: &mut Self::Uow,
        claims: &AuthTokenClaims,
    ) -> Result<bool, AuthTokenRevocationError> {
        let transaction = uow.transaction_mut();

        let token_id = Uuid::from(claims.token_id());
        let subject = claims.subject();
        let issued_at: DateTime<Utc> = claims.issued_at().into();

        let revoked: bool = sqlx::query_scalar(
            r#"
            SELECT
                EXISTS (
                    SELECT 1
                    FROM auth_token_revocations
                    WHERE token_id = $1
                      AND expires_at > now()
                )
                OR EXISTS (
                    SELECT 1
                    FROM auth_token_revocation_cutoffs
                    WHERE subject_aggregate_type = $2
                      AND subject_aggregate_id = $3
                      AND revoke_before >= $4
                )
            "#,
        )
        .bind(token_id)
        .bind(subject.aggregate_type.value())
        .bind(subject.aggregate_id.value())
        .bind(issued_at)
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| AuthTokenRevocationError::Backend(Box::new(source)))?;

        Ok(revoked)
    }
}

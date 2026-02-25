use appletheia_application::{
    AuthToken, AuthTokenClaims, AuthTokenExpiresAt, AuthTokenId, AuthTokenIssueError,
    AuthTokenIssueRequest, AuthTokenIssueResult, AuthTokenIssuedAt, AuthTokenIssuer,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{Header, encode};

use super::jwt_auth_token_claims::JwtAuthTokenClaims;
use super::jwt_auth_token_issuer_config::JwtAuthTokenIssuerConfig;
use super::jwt_auth_token_issuer_error::JwtAuthTokenIssuerError;

#[derive(Clone, Debug)]
pub struct JwtAuthTokenIssuer {
    config: JwtAuthTokenIssuerConfig,
}

impl JwtAuthTokenIssuer {
    pub fn new(config: JwtAuthTokenIssuerConfig) -> Self {
        Self { config }
    }

    fn to_timestamp_seconds(timestamp: DateTime<Utc>) -> Result<u64, JwtAuthTokenIssuerError> {
        let seconds = timestamp.timestamp();
        if seconds < 0 {
            return Err(JwtAuthTokenIssuerError::Timestamp);
        }
        Ok(seconds as u64)
    }
}

impl AuthTokenIssuer for JwtAuthTokenIssuer {
    async fn issue(
        &self,
        request: AuthTokenIssueRequest,
    ) -> Result<AuthTokenIssueResult, AuthTokenIssueError> {
        let encoding_key = self
            .config
            .signing_key()
            .try_into_encoding_key()
            .map_err(|e| {
                AuthTokenIssueError::Backend(Box::new(JwtAuthTokenIssuerError::SigningKey(e)))
            })?;
        let algorithm = self.config.signing_key().algorithm();

        let issued_at = AuthTokenIssuedAt::now();
        let expires_at_datetime = issued_at.value() + self.config.expires_in().value();
        let expires_at = AuthTokenExpiresAt::from(expires_at_datetime);
        let token_id = AuthTokenId::new();

        let claims = AuthTokenClaims::new(
            self.config.issuer_url().clone(),
            self.config.audiences().clone(),
            request.subject().clone(),
            issued_at,
            expires_at,
            token_id,
        );

        let jwt_claims = JwtAuthTokenClaims {
            issuer: self.config.issuer_url().value().to_string(),
            audiences: self
                .config
                .audiences()
                .values()
                .iter()
                .map(|aud| aud.value().to_owned())
                .collect(),
            subject: request.subject().aggregate_id.to_string(),
            subject_type: request.subject().aggregate_type.to_string(),
            issued_at: Self::to_timestamp_seconds(issued_at.value())
                .map_err(|e| AuthTokenIssueError::Backend(Box::new(e)))?,
            expires_at: Self::to_timestamp_seconds(expires_at.value())
                .map_err(|e| AuthTokenIssueError::Backend(Box::new(e)))?,
            token_id: token_id.to_string(),
        };

        let mut header = Header::new(algorithm);
        header.kid = Some(self.config.signing_key().key_id().value().to_owned());

        let token_value = encode(&header, &jwt_claims, &encoding_key).map_err(|source| {
            AuthTokenIssueError::Backend(Box::new(JwtAuthTokenIssuerError::Encode(source)))
        })?;

        let token = AuthToken::new(token_value);
        Ok(AuthTokenIssueResult::new(token, claims))
    }
}

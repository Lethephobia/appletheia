use appletheia_application::{
    AggregateIdValue, AggregateRef, AggregateTypeOwned, AuthToken, AuthTokenAudience,
    AuthTokenAudiences, AuthTokenClaims, AuthTokenExpiresAt, AuthTokenId, AuthTokenIssuedAt,
    AuthTokenIssuerUrl, AuthTokenVerifier, AuthTokenVerifyError,
};
use chrono::{DateTime, TimeZone, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use uuid::Uuid;

use super::jwt_auth_token_claims::JwtAuthTokenClaims;
use super::jwt_auth_token_verifier_config::JwtAuthTokenVerifierConfig;
use super::jwt_auth_token_verifier_error::JwtAuthTokenVerifierError;

#[derive(Clone, Debug)]
pub struct JwtAuthTokenVerifier {
    config: JwtAuthTokenVerifierConfig,
}

impl JwtAuthTokenVerifier {
    pub fn new(config: JwtAuthTokenVerifierConfig) -> Self {
        Self { config }
    }

    fn validation(&self) -> Validation {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.algorithms = vec![Algorithm::RS256, Algorithm::EdDSA];
        validation.leeway = self.config.leeway_seconds().value();
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);

        let issuers: Vec<String> = self
            .config
            .allowed_issuer_urls()
            .iter()
            .map(|url| url.value().to_string())
            .collect();
        validation.set_issuer(&issuers);

        let audiences: Vec<String> = self
            .config
            .allowed_audiences()
            .values()
            .iter()
            .map(|aud| aud.value().to_owned())
            .collect();
        validation.set_audience(&audiences);

        validation
    }

    fn parse_issuer_url(value: &str) -> Result<AuthTokenIssuerUrl, JwtAuthTokenVerifierError> {
        AuthTokenIssuerUrl::parse(value)
            .map_err(|e| JwtAuthTokenVerifierError::InvalidClaimValue(Box::new(e)))
    }

    fn parse_subject(
        aggregate_type: &str,
        aggregate_id: &str,
    ) -> Result<AggregateRef, JwtAuthTokenVerifierError> {
        let aggregate_type = AggregateTypeOwned::try_from(aggregate_type.to_owned())
            .map_err(|e| JwtAuthTokenVerifierError::InvalidClaimValue(Box::new(e)))?;
        let aggregate_id = AggregateIdValue::try_from(aggregate_id.to_owned())
            .map_err(|e| JwtAuthTokenVerifierError::InvalidClaimValue(Box::new(e)))?;
        Ok(AggregateRef {
            aggregate_type,
            aggregate_id,
        })
    }

    fn parse_audiences(
        audiences: &[String],
    ) -> Result<AuthTokenAudiences, JwtAuthTokenVerifierError> {
        let mut iter = audiences.iter();
        let first = iter
            .next()
            .ok_or(JwtAuthTokenVerifierError::MissingRequiredClaim { name: "aud" })?;

        let primary = AuthTokenAudience::new(first.to_owned())
            .map_err(|e| JwtAuthTokenVerifierError::InvalidClaimValue(Box::new(e)))?;

        let additional = iter
            .map(|aud| AuthTokenAudience::new(aud.to_owned()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JwtAuthTokenVerifierError::InvalidClaimValue(Box::new(e)))?;

        AuthTokenAudiences::new(primary, additional)
            .map_err(|e| JwtAuthTokenVerifierError::InvalidClaimValue(Box::new(e)))
    }

    fn timestamp_to_datetime(seconds: u64) -> Result<DateTime<Utc>, JwtAuthTokenVerifierError> {
        let seconds_i64 = i64::try_from(seconds)
            .map_err(|_| JwtAuthTokenVerifierError::InvalidTimestamp { seconds })?;
        Utc.timestamp_opt(seconds_i64, 0)
            .single()
            .ok_or(JwtAuthTokenVerifierError::InvalidTimestamp { seconds })
    }
}

impl AuthTokenVerifier for JwtAuthTokenVerifier {
    async fn verify(&self, token: &AuthToken) -> Result<AuthTokenClaims, AuthTokenVerifyError> {
        let token_value = token.value();

        let header = decode_header(token_value).map_err(|e| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::DecodeHeader(e)))
        })?;

        let key_id = header.kid.ok_or_else(|| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::MissingKeyId))
        })?;

        let jwk = self.config.jwks().find(&key_id).ok_or_else(|| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::UnknownKeyId))
        })?;

        let decoding_key = DecodingKey::from_jwk(jwk).map_err(|e| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::InvalidKey(e)))
        })?;

        let validation = self.validation();
        let token_data = decode::<JwtAuthTokenClaims>(token_value, &decoding_key, &validation)
            .map_err(|e| {
                AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::Decode(e)))
            })?;

        let jwt_claims = token_data.claims;

        if jwt_claims.token_id.is_empty() {
            return Err(AuthTokenVerifyError::Backend(Box::new(
                JwtAuthTokenVerifierError::MissingRequiredClaim { name: "jti" },
            )));
        }

        if jwt_claims.subject_type.is_empty() {
            return Err(AuthTokenVerifyError::Backend(Box::new(
                JwtAuthTokenVerifierError::MissingRequiredClaim { name: "sub_type" },
            )));
        }

        let issuer_url = Self::parse_issuer_url(&jwt_claims.issuer)
            .map_err(|e| AuthTokenVerifyError::Backend(Box::new(e)))?;

        let audiences = Self::parse_audiences(&jwt_claims.audiences)
            .map_err(|e| AuthTokenVerifyError::Backend(Box::new(e)))?;

        let subject = Self::parse_subject(&jwt_claims.subject_type, &jwt_claims.subject)
            .map_err(|e| AuthTokenVerifyError::Backend(Box::new(e)))?;

        let issued_at = Self::timestamp_to_datetime(jwt_claims.issued_at)
            .map(AuthTokenIssuedAt::from)
            .map_err(|e| AuthTokenVerifyError::Backend(Box::new(e)))?;

        let expires_at = Self::timestamp_to_datetime(jwt_claims.expires_at)
            .map(AuthTokenExpiresAt::from)
            .map_err(|e| AuthTokenVerifyError::Backend(Box::new(e)))?;

        let token_uuid = Uuid::try_parse(&jwt_claims.token_id).map_err(|e| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::InvalidTokenId(e)))
        })?;
        let token_id = AuthTokenId::from(token_uuid);

        Ok(AuthTokenClaims::new(
            issuer_url, audiences, subject, issued_at, expires_at, token_id,
        ))
    }
}

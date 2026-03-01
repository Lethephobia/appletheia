use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use appletheia_application::{
    AggregateIdValue, AggregateRef, AggregateTypeOwned, AuthTokenAudience, AuthTokenAudiences,
    AuthTokenClaims, AuthTokenExpiresAt, AuthTokenId, AuthTokenIssuedAt, AuthTokenIssuerUrl,
};
use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

use super::jwt_auth_token_verifier_error::JwtAuthTokenVerifierError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct JwtAuthTokenClaims {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(rename = "aud", deserialize_with = "deserialize_audiences")]
    pub audiences: Vec<String>,

    #[serde(rename = "sub")]
    pub subject: String,

    #[serde(rename = "sub_type")]
    pub subject_type: String,

    #[serde(rename = "iat")]
    pub issued_at: u64,

    #[serde(rename = "exp")]
    pub expires_at: u64,

    #[serde(rename = "jti")]
    pub token_id: String,
}

impl JwtAuthTokenClaims {
    pub fn try_into_auth_token_claims(&self) -> Result<AuthTokenClaims, JwtAuthTokenVerifierError> {
        if self.token_id.trim().is_empty() {
            return Err(JwtAuthTokenVerifierError::MissingRequiredClaim { name: "jti" });
        }

        if self.subject_type.trim().is_empty() {
            return Err(JwtAuthTokenVerifierError::MissingRequiredClaim { name: "sub_type" });
        }

        let issuer_url = AuthTokenIssuerUrl::parse(&self.issuer)
            .map_err(|e| JwtAuthTokenVerifierError::InvalidClaimValue(Box::new(e)))?;

        let audiences = Self::parse_audiences(&self.audiences)?;

        let subject = Self::parse_subject(&self.subject_type, &self.subject)?;

        let issued_at = Self::timestamp_to_datetime(self.issued_at).map(AuthTokenIssuedAt::from)?;

        let expires_at =
            Self::timestamp_to_datetime(self.expires_at).map(AuthTokenExpiresAt::from)?;

        let token_uuid =
            Uuid::try_parse(&self.token_id).map_err(JwtAuthTokenVerifierError::InvalidTokenId)?;
        let token_id = AuthTokenId::from(token_uuid);

        Ok(AuthTokenClaims::new(
            issuer_url, audiences, subject, issued_at, expires_at, token_id,
        ))
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

fn deserialize_audiences<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(vec![s]),
        Value::Array(items) => items
            .into_iter()
            .map(|item| match item {
                Value::String(s) => Ok(s),
                other => Err(serde::de::Error::custom(format!(
                    "audience item must be string but got {other:?}"
                ))),
            })
            .collect(),
        other => Err(serde::de::Error::custom(format!(
            "audience must be string or array but got {other:?}"
        ))),
    }
}

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use appletheia_application::{
    AggregateIdValue, AggregateRef, AggregateTypeOwned, AuthTokenAudience, AuthTokenAudiences,
    AuthTokenClaims, AuthTokenExpiresAt, AuthTokenId, AuthTokenIssuedAt, AuthTokenIssuerUrl,
};
use uuid::Uuid;

use super::jwt_auth_token_claims_error::JwtAuthTokenClaimsError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct JwtAuthTokenClaims {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(
        rename = "aud",
        deserialize_with = "JwtAuthTokenClaims::deserialize_audiences"
    )]
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
    pub fn try_into_auth_token_claims(&self) -> Result<AuthTokenClaims, JwtAuthTokenClaimsError> {
        if self.token_id.trim().is_empty() {
            return Err(JwtAuthTokenClaimsError::MissingRequiredClaim { name: "jti" });
        }

        if self.subject_type.trim().is_empty() {
            return Err(JwtAuthTokenClaimsError::MissingRequiredClaim { name: "sub_type" });
        }

        let issuer_url = self.issuer.parse::<AuthTokenIssuerUrl>().map_err(|e| {
            JwtAuthTokenClaimsError::InvalidClaimValue {
                name: "iss",
                source: Box::new(e),
            }
        })?;

        let audiences = Self::parse_audiences(&self.audiences)?;

        let subject = Self::parse_subject(&self.subject_type, &self.subject)?;

        let issued_at =
            AuthTokenIssuedAt::from_unix_timestamp_seconds(self.issued_at).map_err(|e| {
                JwtAuthTokenClaimsError::InvalidClaimValue {
                    name: "iat",
                    source: Box::new(e),
                }
            })?;

        let expires_at =
            AuthTokenExpiresAt::from_unix_timestamp_seconds(self.expires_at).map_err(|e| {
                JwtAuthTokenClaimsError::InvalidClaimValue {
                    name: "exp",
                    source: Box::new(e),
                }
            })?;

        let token_uuid =
            Uuid::try_parse(&self.token_id).map_err(JwtAuthTokenClaimsError::InvalidTokenId)?;
        let token_id = AuthTokenId::from(token_uuid);

        Ok(AuthTokenClaims::new(
            issuer_url, audiences, subject, issued_at, expires_at, token_id,
        ))
    }

    fn parse_subject(
        aggregate_type: &str,
        aggregate_id: &str,
    ) -> Result<AggregateRef, JwtAuthTokenClaimsError> {
        let aggregate_type =
            AggregateTypeOwned::try_from(aggregate_type.to_owned()).map_err(|e| {
                JwtAuthTokenClaimsError::InvalidClaimValue {
                    name: "sub_type",
                    source: Box::new(e),
                }
            })?;
        let aggregate_id = AggregateIdValue::try_from(aggregate_id.to_owned()).map_err(|e| {
            JwtAuthTokenClaimsError::InvalidClaimValue {
                name: "sub",
                source: Box::new(e),
            }
        })?;
        Ok(AggregateRef {
            aggregate_type,
            aggregate_id,
        })
    }

    fn parse_audiences(
        audiences: &[String],
    ) -> Result<AuthTokenAudiences, JwtAuthTokenClaimsError> {
        let audiences = audiences
            .iter()
            .map(|aud| AuthTokenAudience::new(aud.to_owned()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JwtAuthTokenClaimsError::InvalidClaimValue {
                name: "aud",
                source: Box::new(e),
            })?;

        AuthTokenAudiences::new(audiences).map_err(|e| JwtAuthTokenClaimsError::InvalidClaimValue {
            name: "aud",
            source: Box::new(e),
        })
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
}

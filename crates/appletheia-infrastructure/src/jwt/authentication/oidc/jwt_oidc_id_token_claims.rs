use serde::{Deserialize, Serialize};
use serde_json::Value;

use appletheia_application::authentication::oidc::{
    OidcAccessTokenHash, OidcAudiences, OidcAuthTime, OidcIdTokenClaims, OidcIdTokenExpiresAt,
    OidcIdTokenIssuedAt, OidcIdTokenVerifyError, OidcIssuerUrl, OidcNonce, OidcSubject,
};
use chrono::{DateTime, TimeZone, Utc};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct JwtOidcIdTokenClaims {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(rename = "sub")]
    pub subject: String,

    #[serde(rename = "aud", deserialize_with = "deserialize_audiences")]
    pub audiences: Vec<String>,

    #[serde(rename = "exp")]
    pub expires_at: u64,

    #[serde(rename = "iat")]
    pub issued_at: Option<u64>,

    #[serde(rename = "auth_time")]
    pub auth_time: Option<u64>,

    pub nonce: Option<String>,

    #[serde(rename = "at_hash")]
    pub access_token_hash: Option<String>,
}

impl JwtOidcIdTokenClaims {
    pub fn try_into_id_token_claims(&self) -> Result<OidcIdTokenClaims, OidcIdTokenVerifyError> {
        let issuer_url = self
            .issuer
            .parse::<OidcIssuerUrl>()
            .map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;

        let audiences = OidcAudiences::try_from(self.audiences.clone())
            .map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;

        let expires_at = Self::timestamp_to_datetime(self.expires_at)
            .map(OidcIdTokenExpiresAt::new)
            .map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;

        let issued_at = self
            .issued_at
            .map(Self::timestamp_to_datetime)
            .transpose()
            .map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?
            .map(OidcIdTokenIssuedAt::new);

        let auth_time = self
            .auth_time
            .map(Self::timestamp_to_datetime)
            .transpose()
            .map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?
            .map(OidcAuthTime::new);

        let nonce = self
            .nonce
            .clone()
            .map(OidcNonce::try_from)
            .transpose()
            .map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;

        let access_token_hash = self.access_token_hash.clone().map(OidcAccessTokenHash::new);

        Ok(OidcIdTokenClaims {
            issuer_url,
            subject: OidcSubject::new(self.subject.clone()),
            audiences,
            expires_at,
            issued_at,
            auth_time,
            nonce,
            access_token_hash,
        })
    }

    fn timestamp_to_datetime(seconds: u64) -> Result<DateTime<Utc>, OidcIdTokenVerifyError> {
        let seconds_i64 =
            i64::try_from(seconds).map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;
        Utc.timestamp_opt(seconds_i64, 0)
            .single()
            .ok_or(OidcIdTokenVerifyError::InvalidIdToken)
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

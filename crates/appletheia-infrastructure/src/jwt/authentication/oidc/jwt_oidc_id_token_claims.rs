use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use appletheia_application::authentication::oidc::{
    OidcAccessTokenHash, OidcAddress, OidcAudiences, OidcAuthTime, OidcBirthdate, OidcEmail,
    OidcFamilyName, OidcGender, OidcGivenName, OidcIdTokenClaims, OidcIdTokenExpiresAt,
    OidcIdTokenIssuedAt, OidcIssuerUrl, OidcLocale, OidcMiddleName, OidcName, OidcNickname,
    OidcNonce, OidcPhoneNumber, OidcPictureUrl, OidcPreferredUsername, OidcProfileUrl, OidcSubject,
    OidcUpdatedAt, OidcWebsiteUrl, OidcZoneinfo,
};
use chrono::{DateTime, TimeZone, Utc};

use super::jwt_oidc_id_token_claims_error::JwtOidcIdTokenClaimsError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct JwtOidcIdTokenClaims {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(rename = "sub")]
    pub subject: String,

    #[serde(
        rename = "aud",
        deserialize_with = "JwtOidcIdTokenClaims::deserialize_audiences"
    )]
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

    pub email: Option<String>,

    pub email_verified: Option<bool>,

    pub name: Option<String>,

    pub given_name: Option<String>,

    pub family_name: Option<String>,

    pub middle_name: Option<String>,

    pub nickname: Option<String>,

    pub preferred_username: Option<String>,

    pub profile: Option<String>,

    pub picture: Option<String>,

    pub website: Option<String>,

    pub gender: Option<String>,

    pub birthdate: Option<String>,

    pub zoneinfo: Option<String>,

    pub locale: Option<String>,

    pub phone_number: Option<String>,

    pub phone_number_verified: Option<bool>,

    pub address: Option<OidcAddress>,

    pub updated_at: Option<u64>,
}

impl JwtOidcIdTokenClaims {
    pub fn try_into_id_token_claims(&self) -> Result<OidcIdTokenClaims, JwtOidcIdTokenClaimsError> {
        let issuer_url = self.issuer.parse::<OidcIssuerUrl>().map_err(|e| {
            JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "iss",
                source: Box::new(e),
            }
        })?;

        let audiences = OidcAudiences::try_from(self.audiences.clone()).map_err(|e| {
            JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "aud",
                source: Box::new(e),
            }
        })?;

        let expires_at = Self::timestamp_to_datetime(self.expires_at, "exp")
            .map(OidcIdTokenExpiresAt::new)
            .map_err(|_| JwtOidcIdTokenClaimsError::InvalidTimestampClaim { name: "exp" })?;

        let issued_at = self
            .issued_at
            .map(|value| Self::timestamp_to_datetime(value, "iat"))
            .transpose()
            .map_err(|_| JwtOidcIdTokenClaimsError::InvalidTimestampClaim { name: "iat" })?
            .map(OidcIdTokenIssuedAt::new);

        let auth_time = self
            .auth_time
            .map(|value| Self::timestamp_to_datetime(value, "auth_time"))
            .transpose()
            .map_err(|_| JwtOidcIdTokenClaimsError::InvalidTimestampClaim { name: "auth_time" })?
            .map(OidcAuthTime::new);

        let nonce = self
            .nonce
            .clone()
            .map(OidcNonce::try_from)
            .transpose()
            .map_err(|e| JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "nonce",
                source: Box::new(e),
            })?;

        let picture_url = self
            .picture
            .as_deref()
            .map(Url::parse)
            .transpose()
            .map_err(|e| JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "picture",
                source: Box::new(e),
            })?
            .map(OidcPictureUrl::new);
        let profile_url = self
            .profile
            .as_deref()
            .map(Url::parse)
            .transpose()
            .map_err(|e| JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "profile",
                source: Box::new(e),
            })?
            .map(OidcProfileUrl::new);
        let website_url = self
            .website
            .as_deref()
            .map(Url::parse)
            .transpose()
            .map_err(|e| JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "website",
                source: Box::new(e),
            })?
            .map(OidcWebsiteUrl::new);
        let birthdate = self
            .birthdate
            .clone()
            .map(OidcBirthdate::try_from)
            .transpose()
            .map_err(|e| JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "birthdate",
                source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
            })?;
        let zoneinfo = self
            .zoneinfo
            .clone()
            .map(OidcZoneinfo::try_from)
            .transpose()
            .map_err(|e| JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "zoneinfo",
                source: Box::new(e),
            })?;
        let locale = self
            .locale
            .clone()
            .map(OidcLocale::try_from)
            .transpose()
            .map_err(|e| JwtOidcIdTokenClaimsError::InvalidClaimValue {
                name: "locale",
                source: Box::new(e),
            })?;
        let updated_at = self
            .updated_at
            .map(|value| Self::timestamp_to_datetime(value, "updated_at"))
            .transpose()
            .map_err(|_| JwtOidcIdTokenClaimsError::InvalidTimestampClaim { name: "updated_at" })?
            .map(OidcUpdatedAt::new);

        Ok(OidcIdTokenClaims {
            issuer_url,
            subject: OidcSubject::new(self.subject.clone()),
            audiences,
            expires_at,
            issued_at,
            auth_time,
            nonce,
            access_token_hash: self.access_token_hash.clone().map(OidcAccessTokenHash::new),
            email: self.email.clone().map(OidcEmail::new),
            email_verified: self.email_verified,
            name: self.name.clone().map(OidcName::new),
            given_name: self.given_name.clone().map(OidcGivenName::new),
            family_name: self.family_name.clone().map(OidcFamilyName::new),
            middle_name: self.middle_name.clone().map(OidcMiddleName::new),
            nickname: self.nickname.clone().map(OidcNickname::new),
            preferred_username: self
                .preferred_username
                .clone()
                .map(OidcPreferredUsername::new),
            profile_url,
            picture_url,
            website_url,
            gender: self.gender.clone().map(OidcGender::new),
            birthdate,
            zoneinfo,
            locale,
            phone_number: self.phone_number.clone().map(OidcPhoneNumber::new),
            phone_number_verified: self.phone_number_verified,
            address: self.address.clone(),
            updated_at,
        })
    }

    fn timestamp_to_datetime(
        seconds: u64,
        name: &'static str,
    ) -> Result<DateTime<Utc>, JwtOidcIdTokenClaimsError> {
        let seconds_i64 = i64::try_from(seconds)
            .map_err(|_| JwtOidcIdTokenClaimsError::InvalidTimestampClaim { name })?;
        Utc.timestamp_opt(seconds_i64, 0)
            .single()
            .ok_or(JwtOidcIdTokenClaimsError::InvalidTimestampClaim { name })
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

use appletheia_application::authentication::oidc::{
    OidcAddress, OidcBirthdate, OidcEmail, OidcEmailVerified, OidcFamilyName, OidcGender,
    OidcGivenName, OidcLocale, OidcMiddleName, OidcName, OidcNickname, OidcPhoneNumber,
    OidcPhoneNumberVerified, OidcPictureUrl, OidcPreferredUsername, OidcProfileUrl, OidcSubject,
    OidcUpdatedAt, OidcUserInfo, OidcWebsiteUrl, OidcZoneinfo,
};
use chrono::{TimeZone, Utc};
use serde::Deserialize;
use std::io;
use url::Url;

use super::OidcUserInfoBodyError;

/// Represents the user-info endpoint response body.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct OidcUserInfoBody {
    #[serde(rename = "sub")]
    subject: String,

    email: Option<String>,

    email_verified: Option<bool>,

    name: Option<String>,

    given_name: Option<String>,

    family_name: Option<String>,

    middle_name: Option<String>,

    nickname: Option<String>,

    preferred_username: Option<String>,

    profile: Option<Url>,

    picture: Option<Url>,

    website: Option<Url>,

    gender: Option<String>,

    birthdate: Option<String>,

    zoneinfo: Option<String>,

    locale: Option<String>,

    phone_number: Option<String>,

    phone_number_verified: Option<bool>,

    address: Option<OidcAddress>,

    updated_at: Option<u64>,
}

impl OidcUserInfoBody {
    /// Decodes a user-info response body from JSON bytes.
    pub fn try_from_json_bytes(bytes: &[u8]) -> Result<Self, OidcUserInfoBodyError> {
        serde_json::from_slice(bytes).map_err(OidcUserInfoBodyError::InvalidJson)
    }

    /// Converts the decoded body into application user-info values.
    pub fn try_into_user_info(&self) -> Result<OidcUserInfo, OidcUserInfoBodyError> {
        let birthdate = self
            .birthdate
            .clone()
            .map(OidcBirthdate::try_from)
            .transpose()
            .map_err(|source| OidcUserInfoBodyError::InvalidField {
                field: "birthdate",
                source: Box::new(io::Error::new(io::ErrorKind::InvalidInput, source)),
            })?;
        let zoneinfo = self
            .zoneinfo
            .clone()
            .map(OidcZoneinfo::try_from)
            .transpose()
            .map_err(|source| OidcUserInfoBodyError::InvalidField {
                field: "zoneinfo",
                source: Box::new(source),
            })?;
        let locale = self
            .locale
            .clone()
            .map(OidcLocale::try_from)
            .transpose()
            .map_err(|source| OidcUserInfoBodyError::InvalidField {
                field: "locale",
                source: Box::new(source),
            })?;
        let updated_at = self
            .updated_at
            .map(timestamp_to_datetime)
            .transpose()
            .map_err(|source| OidcUserInfoBodyError::InvalidField {
                field: "updated_at",
                source: Box::new(source),
            })?
            .map(OidcUpdatedAt::new);

        Ok(OidcUserInfo {
            subject: OidcSubject::new(self.subject.clone()),
            email: self.email.clone().map(OidcEmail::new),
            email_verified: self.email_verified.map(OidcEmailVerified::new),
            name: self.name.clone().map(OidcName::new),
            given_name: self.given_name.clone().map(OidcGivenName::new),
            family_name: self.family_name.clone().map(OidcFamilyName::new),
            middle_name: self.middle_name.clone().map(OidcMiddleName::new),
            nickname: self.nickname.clone().map(OidcNickname::new),
            preferred_username: self
                .preferred_username
                .clone()
                .map(OidcPreferredUsername::new),
            profile_url: self.profile.clone().map(OidcProfileUrl::new),
            picture_url: self.picture.clone().map(OidcPictureUrl::new),
            website_url: self.website.clone().map(OidcWebsiteUrl::new),
            gender: self.gender.clone().map(OidcGender::new),
            birthdate,
            zoneinfo,
            locale,
            phone_number: self.phone_number.clone().map(OidcPhoneNumber::new),
            phone_number_verified: self.phone_number_verified.map(OidcPhoneNumberVerified::new),
            address: self.address.clone(),
            updated_at,
        })
    }
}

fn timestamp_to_datetime(seconds: u64) -> Result<chrono::DateTime<Utc>, io::Error> {
    let seconds_i64 = i64::try_from(seconds)
        .map_err(|source| io::Error::new(io::ErrorKind::InvalidInput, source))?;

    Utc.timestamp_opt(seconds_i64, 0)
        .single()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid unix timestamp"))
}

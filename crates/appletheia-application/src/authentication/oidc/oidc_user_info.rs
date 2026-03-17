use serde::{Deserialize, Serialize};

use super::{
    OidcAddress, OidcBirthdate, OidcEmail, OidcEmailVerified, OidcFamilyName, OidcGender,
    OidcGivenName, OidcLocale, OidcMiddleName, OidcName, OidcNickname, OidcPhoneNumber,
    OidcPhoneNumberVerified, OidcPictureUrl, OidcPreferredUsername, OidcProfileUrl, OidcSubject,
    OidcUpdatedAt, OidcWebsiteUrl, OidcZoneinfo,
};

/// Represents the user info returned from the OIDC `userinfo` endpoint.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcUserInfo {
    pub subject: OidcSubject,
    pub email: Option<OidcEmail>,
    pub email_verified: Option<OidcEmailVerified>,
    pub name: Option<OidcName>,
    pub given_name: Option<OidcGivenName>,
    pub family_name: Option<OidcFamilyName>,
    pub middle_name: Option<OidcMiddleName>,
    pub nickname: Option<OidcNickname>,
    pub preferred_username: Option<OidcPreferredUsername>,
    pub profile_url: Option<OidcProfileUrl>,
    pub picture_url: Option<OidcPictureUrl>,
    pub website_url: Option<OidcWebsiteUrl>,
    pub gender: Option<OidcGender>,
    pub birthdate: Option<OidcBirthdate>,
    pub zoneinfo: Option<OidcZoneinfo>,
    pub locale: Option<OidcLocale>,
    pub phone_number: Option<OidcPhoneNumber>,
    pub phone_number_verified: Option<OidcPhoneNumberVerified>,
    pub address: Option<OidcAddress>,
    pub updated_at: Option<OidcUpdatedAt>,
}

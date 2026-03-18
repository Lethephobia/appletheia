use serde::{Deserialize, Serialize};

use super::{
    OidcAccessTokenHash, OidcAddress, OidcAudiences, OidcAuthTime, OidcBirthdate, OidcEmail,
    OidcFamilyName, OidcGender, OidcGivenName, OidcIdTokenExpiresAt, OidcIdTokenIssuedAt,
    OidcIssuerUrl, OidcLocale, OidcMiddleName, OidcName, OidcNickname, OidcNonce, OidcPhoneNumber,
    OidcPictureUrl, OidcPreferredUsername, OidcProfileUrl, OidcSubject, OidcUpdatedAt,
    OidcWebsiteUrl, OidcZoneinfo,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcIdTokenClaims {
    pub issuer_url: OidcIssuerUrl,
    pub subject: OidcSubject,
    pub audiences: OidcAudiences,
    pub expires_at: OidcIdTokenExpiresAt,
    pub issued_at: Option<OidcIdTokenIssuedAt>,
    pub auth_time: Option<OidcAuthTime>,
    pub nonce: Option<OidcNonce>,
    pub access_token_hash: Option<OidcAccessTokenHash>,
    pub email: Option<OidcEmail>,
    pub email_verified: Option<bool>,
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
    pub phone_number_verified: Option<bool>,
    pub address: Option<OidcAddress>,
    pub updated_at: Option<OidcUpdatedAt>,
}

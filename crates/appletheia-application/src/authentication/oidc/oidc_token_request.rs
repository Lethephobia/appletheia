use url::Url;

use super::{OidcClientAuth, OidcClientId, OidcTokenGrant};

#[derive(Clone, Debug)]
pub struct OidcTokenRequest {
    pub token_endpoint_url: Url,
    pub client_id: OidcClientId,
    pub client_auth: OidcClientAuth,
    pub grant: OidcTokenGrant,
}

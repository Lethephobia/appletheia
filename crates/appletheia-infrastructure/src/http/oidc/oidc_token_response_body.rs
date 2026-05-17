/// JSON body returned from an OIDC token endpoint.
#[derive(Debug, serde::Deserialize)]
pub(super) struct OidcTokenResponseBody {
    pub(super) id_token: Option<String>,
    pub(super) access_token: Option<String>,
    pub(super) refresh_token: Option<String>,
    pub(super) expires_in: Option<u64>,
}

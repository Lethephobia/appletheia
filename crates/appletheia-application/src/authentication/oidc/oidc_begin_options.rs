use super::{OidcDisplay, OidcExtraAuthorizeParams, OidcMaxAge, OidcPrompt, OidcScopes};

#[derive(Clone, Debug, Default)]
pub struct OidcBeginOptions {
    pub scopes: OidcScopes,
    pub display: Option<OidcDisplay>,
    pub max_age: Option<OidcMaxAge>,
    pub prompt: Option<OidcPrompt>,
    pub extra_authorize_params: OidcExtraAuthorizeParams,
}

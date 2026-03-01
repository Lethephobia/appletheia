use super::{OidcAuthorizationCode, OidcState};

#[derive(Clone, Debug)]
pub struct OidcCallbackParams {
    pub state: OidcState,
    pub authorization_code: OidcAuthorizationCode,
}

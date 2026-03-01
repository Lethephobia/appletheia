use super::{OidcLoginAttemptExpiresIn, OidcProviderConfig};

#[derive(Clone, Debug)]
pub struct OidcLoginFlowConfig {
    pub provider_config: OidcProviderConfig,
    pub login_attempt_expires_in: OidcLoginAttemptExpiresIn,
}

use crate::jwt::JwtLeewaySeconds;

#[derive(Clone, Debug)]
pub struct JwtOidcIdTokenVerifierConfig {
    leeway_seconds: JwtLeewaySeconds,
}

impl JwtOidcIdTokenVerifierConfig {
    pub fn new() -> Self {
        Self {
            leeway_seconds: JwtLeewaySeconds::default(),
        }
    }

    pub fn with_leeway_seconds(mut self, leeway_seconds: JwtLeewaySeconds) -> Self {
        self.leeway_seconds = leeway_seconds;
        self
    }

    pub fn leeway_seconds(&self) -> JwtLeewaySeconds {
        self.leeway_seconds
    }
}

impl Default for JwtOidcIdTokenVerifierConfig {
    fn default() -> Self {
        Self::new()
    }
}

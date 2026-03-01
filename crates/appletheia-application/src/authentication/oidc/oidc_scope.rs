use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcScope(String);

impl OidcScope {
    pub const OPENID: &'static str = "openid";
    pub const EMAIL: &'static str = "email";
    pub const PROFILE: &'static str = "profile";

    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn openid() -> Self {
        Self(Self::OPENID.to_string())
    }

    pub fn email() -> Self {
        Self(Self::EMAIL.to_string())
    }

    pub fn profile() -> Self {
        Self(Self::PROFILE.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

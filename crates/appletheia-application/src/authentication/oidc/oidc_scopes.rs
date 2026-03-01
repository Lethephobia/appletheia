use serde::{Deserialize, Serialize};

use super::OidcScope;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcScopes(Vec<OidcScope>);

impl OidcScopes {
    pub fn new(mut scopes: Vec<OidcScope>) -> Self {
        if !scopes.iter().any(|s| s.value() == OidcScope::OPENID) {
            scopes.insert(0, OidcScope::openid());
        }
        Self(scopes)
    }

    pub fn values(&self) -> &[OidcScope] {
        &self.0
    }

    pub fn to_scope_string(&self) -> String {
        self.0
            .iter()
            .map(OidcScope::value)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Default for OidcScopes {
    fn default() -> Self {
        Self::new(vec![OidcScope::openid()])
    }
}

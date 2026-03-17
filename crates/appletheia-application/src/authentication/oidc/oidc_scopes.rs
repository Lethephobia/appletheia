use serde::{Deserialize, Serialize};

use super::OidcScope;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcScopes(Vec<OidcScope>);

impl OidcScopes {
    pub fn new(mut scopes: Vec<OidcScope>) -> Self {
        Self::ensure_scope(&mut scopes, OidcScope::openid());
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

    fn ensure_scope(scopes: &mut Vec<OidcScope>, required: OidcScope) {
        if scopes.iter().any(|scope| scope.value() == required.value()) {
            return;
        }

        scopes.push(required);
    }
}

impl Default for OidcScopes {
    fn default() -> Self {
        Self::new(vec![OidcScope::openid()])
    }
}

#[cfg(test)]
mod tests {
    use super::OidcScopes;
    use crate::authentication::oidc::OidcScope;

    #[test]
    fn new_always_includes_openid() {
        let scopes = OidcScopes::new(vec![OidcScope::email(), OidcScope::profile()]);

        assert_eq!(
            scopes.values(),
            &[
                OidcScope::email(),
                OidcScope::profile(),
                OidcScope::openid()
            ]
        );
    }
}

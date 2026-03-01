use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct OidcClientSecret(String);

impl OidcClientSecret {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for OidcClientSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OidcClientSecret([REDACTED])")
    }
}

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct OidcRefreshToken(String);

impl OidcRefreshToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for OidcRefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OidcRefreshToken([REDACTED])")
    }
}

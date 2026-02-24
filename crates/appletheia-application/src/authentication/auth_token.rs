use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthToken([REDACTED])")
    }
}

impl From<String> for AuthToken {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<AuthToken> for String {
    fn from(value: AuthToken) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_returns_inner_str() {
        let token = AuthToken::new("abc".to_owned());
        assert_eq!(token.value(), "abc");
    }

    #[test]
    fn debug_redacts_token_value() {
        let token = AuthToken::new("abc".to_owned());
        assert_eq!(format!("{token:?}"), "AuthToken([REDACTED])");
    }
}

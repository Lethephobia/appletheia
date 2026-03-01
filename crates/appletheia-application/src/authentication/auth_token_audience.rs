use std::fmt;

use serde::{Deserialize, Serialize};

use super::AuthTokenAudienceError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenAudience(String);

impl AuthTokenAudience {
    pub fn new(value: String) -> Result<Self, AuthTokenAudienceError> {
        if value.trim().is_empty() {
            return Err(AuthTokenAudienceError::Empty);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AuthTokenAudience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty() {
        assert!(matches!(
            AuthTokenAudience::new("".to_owned()),
            Err(AuthTokenAudienceError::Empty)
        ));
        assert!(matches!(
            AuthTokenAudience::new("   ".to_owned()),
            Err(AuthTokenAudienceError::Empty)
        ));
    }

    #[test]
    fn new_accepts_non_empty() {
        let audience = AuthTokenAudience::new("audience".to_owned()).unwrap();
        assert_eq!(audience.value(), "audience");
    }
}

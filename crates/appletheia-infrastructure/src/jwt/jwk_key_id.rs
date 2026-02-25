use std::fmt;

use serde::{Deserialize, Serialize};

use crate::jwt::JwkKeyIdError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JwkKeyId(String);

impl JwkKeyId {
    pub fn new(value: String) -> Result<Self, JwkKeyIdError> {
        if value.trim().is_empty() {
            return Err(JwkKeyIdError::Empty);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for JwkKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty() {
        assert!(matches!(
            JwkKeyId::new("".to_owned()),
            Err(JwkKeyIdError::Empty)
        ));
        assert!(matches!(
            JwkKeyId::new("   ".to_owned()),
            Err(JwkKeyIdError::Empty)
        ));
    }

    #[test]
    fn new_accepts_non_empty() {
        let kid = JwkKeyId::new("kid-1".to_owned()).unwrap();
        assert_eq!(kid.value(), "kid-1");
    }
}

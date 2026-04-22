use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{AuthTokenAudience, AuthTokenAudiencesError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenAudiences(Vec<AuthTokenAudience>);

impl AuthTokenAudiences {
    pub fn new(mut audiences: Vec<AuthTokenAudience>) -> Result<Self, AuthTokenAudiencesError> {
        if audiences.is_empty() {
            return Err(AuthTokenAudiencesError::Empty);
        }

        let mut seen = HashSet::new();
        audiences.retain(|audience| seen.insert(audience.clone()));

        Ok(Self(audiences))
    }

    pub fn values(&self) -> &[AuthTokenAudience] {
        &self.0
    }
}

impl TryFrom<Vec<AuthTokenAudience>> for AuthTokenAudiences {
    type Error = AuthTokenAudiencesError;

    fn try_from(value: Vec<AuthTokenAudience>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty() {
        assert!(matches!(
            AuthTokenAudiences::new(vec![]),
            Err(AuthTokenAudiencesError::Empty)
        ));
    }

    #[test]
    fn new_dedupes_audiences() {
        let audiences = vec![
            AuthTokenAudience::new("a".to_owned()).unwrap(),
            AuthTokenAudience::new("a".to_owned()).unwrap(),
            AuthTokenAudience::new("b".to_owned()).unwrap(),
        ];

        let audiences = AuthTokenAudiences::new(audiences).unwrap();
        let values: Vec<&str> = audiences.values().iter().map(|a| a.value()).collect();

        assert_eq!(values, vec!["a", "b"]);
    }
}

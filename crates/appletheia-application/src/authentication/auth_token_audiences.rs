use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{AuthTokenAudience, AuthTokenAudiencesError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenAudiences(Vec<AuthTokenAudience>);

impl AuthTokenAudiences {
    pub fn new(
        primary: AuthTokenAudience,
        additional: impl IntoIterator<Item = AuthTokenAudience>,
    ) -> Result<Self, AuthTokenAudiencesError> {
        let mut audiences = Vec::new();
        audiences.push(primary);
        audiences.extend(additional);

        let mut seen = HashSet::new();
        audiences.retain(|audience| seen.insert(audience.clone()));

        if audiences.is_empty() {
            return Err(AuthTokenAudiencesError::Empty);
        }

        Ok(Self(audiences))
    }

    pub fn values(&self) -> &[AuthTokenAudience] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dedupes_audiences() {
        let primary = AuthTokenAudience::new("a".to_owned()).unwrap();
        let additional = vec![
            AuthTokenAudience::new("a".to_owned()).unwrap(),
            AuthTokenAudience::new("b".to_owned()).unwrap(),
        ];

        let audiences = AuthTokenAudiences::new(primary, additional).unwrap();
        let values: Vec<&str> = audiences.values().iter().map(|a| a.value()).collect();

        assert_eq!(values, vec!["a", "b"]);
    }
}

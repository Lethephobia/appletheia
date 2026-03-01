use serde::{Deserialize, Serialize};

use super::{OidcAudiencesError, OidcClientId};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OidcAudiences(Vec<OidcClientId>);

impl OidcAudiences {
    pub fn new(audiences: Vec<OidcClientId>) -> Result<Self, OidcAudiencesError> {
        if audiences.is_empty() {
            return Err(OidcAudiencesError::Empty);
        }
        Ok(Self(audiences))
    }

    pub fn values(&self) -> &[OidcClientId] {
        &self.0
    }
}

impl TryFrom<Vec<OidcClientId>> for OidcAudiences {
    type Error = OidcAudiencesError;

    fn try_from(value: Vec<OidcClientId>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<Vec<String>> for OidcAudiences {
    type Error = OidcAudiencesError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let audiences = value.into_iter().map(OidcClientId::new).collect();
        Self::new(audiences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty() {
        assert_eq!(OidcAudiences::new(vec![]), Err(OidcAudiencesError::Empty));
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty() {
        assert_eq!(OidcAudiences::new(vec![]), Err(OidcAudiencesError::Empty));
    }
}

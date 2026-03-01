use jsonwebtoken::jwk::JwkSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::jwt::JwksError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Jwks {
    value: JwkSet,
}

impl Jwks {
    pub fn new(jwk_set: JwkSet) -> Self {
        Self { value: jwk_set }
    }

    pub fn value(&self) -> &JwkSet {
        &self.value
    }

    pub fn find(&self, key_id: &str) -> Option<&jsonwebtoken::jwk::Jwk> {
        self.value.find(key_id)
    }
}

impl Serialize for Jwks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Jwks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let jwk_set = JwkSet::deserialize(deserializer)?;
        Ok(Self::new(jwk_set))
    }
}

impl TryFrom<&str> for Jwks {
    type Error = JwksError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let jwk_set: JwkSet = serde_json::from_str(value).map_err(JwksError::InvalidJson)?;
        Ok(Self::new(jwk_set))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str_parses_jwks() {
        let json = r#"{"keys":[]}"#;
        let jwks = Jwks::try_from(json).unwrap();
        assert_eq!(jwks.value().keys.len(), 0);
    }
}

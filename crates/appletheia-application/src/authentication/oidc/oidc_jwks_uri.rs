use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcJwksUri(Url);

impl OidcJwksUri {
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &Url {
        &self.0
    }
}

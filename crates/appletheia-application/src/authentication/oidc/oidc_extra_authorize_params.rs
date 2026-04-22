use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcExtraAuthorizeParams(BTreeMap<String, String>);

impl OidcExtraAuthorizeParams {
    pub fn new(value: BTreeMap<String, String>) -> Self {
        Self(value)
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.0.insert(key, value)
    }

    pub fn as_map(&self) -> &BTreeMap<String, String> {
        &self.0
    }
}

impl From<BTreeMap<String, String>> for OidcExtraAuthorizeParams {
    fn from(value: BTreeMap<String, String>) -> Self {
        Self::new(value)
    }
}

impl IntoIterator for OidcExtraAuthorizeParams {
    type Item = (String, String);
    type IntoIter = std::collections::btree_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

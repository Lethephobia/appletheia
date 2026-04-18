use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ObjectUploadMethod {
    Put,
}

impl ObjectUploadMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Put => "PUT",
        }
    }
}

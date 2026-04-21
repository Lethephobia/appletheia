use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ObjectChecksumAlgorithm {
    Md5,
    Crc32c,
    Sha256,
}

impl ObjectChecksumAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Md5 => "md5",
            Self::Crc32c => "crc32c",
            Self::Sha256 => "sha256",
        }
    }
}

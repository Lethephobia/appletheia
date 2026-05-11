use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectChecksumAlgorithm {
    Md5,
    Crc32c,
    Sha256,
}

impl ObjectChecksumAlgorithm {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Md5 => "md5",
            Self::Crc32c => "crc32c",
            Self::Sha256 => "sha256",
        }
    }
}

impl AsRef<str> for ObjectChecksumAlgorithm {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

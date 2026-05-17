use serde::{Deserialize, Serialize};

use super::{MintMetadataImageObjectName, MintMetadataUri};

/// Represents an image reference included in off-chain mint metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum MintMetadataImage {
    ObjectName(MintMetadataImageObjectName),
    Uri(MintMetadataUri),
}

impl MintMetadataImage {
    pub fn object_name(value: MintMetadataImageObjectName) -> Self {
        Self::ObjectName(value)
    }

    pub fn uri(value: MintMetadataUri) -> Self {
        Self::Uri(value)
    }
}

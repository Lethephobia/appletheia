use serde::{Deserialize, Serialize};

/// Returned after an organization picture object delete request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPictureObjectDeleteOutput;

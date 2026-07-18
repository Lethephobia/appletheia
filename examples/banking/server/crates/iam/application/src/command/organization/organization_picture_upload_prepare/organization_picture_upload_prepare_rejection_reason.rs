use serde::{Deserialize, Serialize};

/// Describes why an organization-picture upload preparation was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationPictureUploadPrepareRejectionReason {
    OrganizationRemoved,
    ContentLengthTooLarge,
    ContentTypeNotAllowed,
}

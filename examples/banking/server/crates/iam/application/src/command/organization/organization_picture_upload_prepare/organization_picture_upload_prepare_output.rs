use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use appletheia::application::object_storage::SignedObjectUpload;
use banking_iam_domain::OrganizationPictureRef;
use serde::{Deserialize, Serialize};

use super::OrganizationPictureUploadPrepareRejectionReason;

/// The output returned after preparing an organization-picture upload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationPictureUploadPrepareOutput {
    Prepared {
        picture: OrganizationPictureRef,
        signed_upload: Box<SignedObjectUpload>,
    },
    Rejected {
        reason: OrganizationPictureUploadPrepareRejectionReason,
    },
}

impl CommandOutput for OrganizationPictureUploadPrepareOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}

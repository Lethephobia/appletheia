use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use appletheia::application::object_storage::SignedObjectUpload;
use banking_iam_domain::OrganizationPictureRef;
use serde::{Deserialize, Serialize};

/// The output returned after preparing an organization-picture upload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPictureUploadPrepareOutput {
    pub picture: OrganizationPictureRef,
    pub signed_upload: Box<SignedObjectUpload>,
}

impl CommandOutput for OrganizationPictureUploadPrepareOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}

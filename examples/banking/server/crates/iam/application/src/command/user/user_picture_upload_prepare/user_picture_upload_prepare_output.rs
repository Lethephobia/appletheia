use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use appletheia::application::object_storage::SignedObjectUpload;
use banking_iam_domain::UserPictureRef;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureUploadPrepareOutput {
    pub picture: UserPictureRef,
    pub signed_upload: Box<SignedObjectUpload>,
}

impl CommandOutput for UserPictureUploadPrepareOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}

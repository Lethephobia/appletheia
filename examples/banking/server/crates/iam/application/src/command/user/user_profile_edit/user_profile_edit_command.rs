use appletheia::application::command::FieldPatch;
use appletheia::command;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};
use serde::{Deserialize, Serialize};

/// Applies a partial update to a user's profile.
#[command(name = "user_profile_edit")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileEditCommand {
    pub user_id: UserId,
    #[serde(default, skip_serializing_if = "FieldPatch::is_unchanged")]
    pub username: FieldPatch<Username>,
    #[serde(default, skip_serializing_if = "FieldPatch::is_unchanged")]
    pub display_name: FieldPatch<UserDisplayName>,
    #[serde(default, skip_serializing_if = "FieldPatch::is_unchanged")]
    pub bio: FieldPatch<Option<UserBio>>,
}

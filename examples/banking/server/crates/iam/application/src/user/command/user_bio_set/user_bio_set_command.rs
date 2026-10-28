use appletheia::command;
use banking_iam_domain::{UserBio, UserId};
use serde::{Deserialize, Serialize};

/// Changes a user's bio.
#[command(name = "user_bio_set")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBioSetCommand {
    pub user_id: UserId,
    pub bio: Option<UserBio>,
}

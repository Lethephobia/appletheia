use serde::{Deserialize, Serialize};

use banking_iam_application::UserFragment;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

use appletheia::application::read_model::ReadModelObservation;

/// User owner fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicAccountListItemOwnerUserPart {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}

impl From<UserFragment> for PublicAccountListItemOwnerUserPart {
    fn from(fragment: UserFragment) -> Self {
        Self {
            id: fragment.id,
            username: fragment.username,
            display_name: fragment.display_name,
            picture: fragment.picture,
            observation: fragment.observation,
        }
    }
}

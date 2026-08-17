use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};
use serde::{Deserialize, Serialize};

use super::{MaterializedUserStatus, UserFragment};

/// User information exposed by the private user read model.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrivateUserDetailsPart {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub status: MaterializedUserStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
    pub bio: Option<UserBio>,
}

impl From<UserFragment> for PrivateUserDetailsPart {
    fn from(fragment: UserFragment) -> Self {
        Self {
            user_id: fragment.id,
            username: fragment.username,
            display_name: fragment.display_name,
            picture: fragment.picture,
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
            bio: fragment.bio,
        }
    }
}

impl ReadModelObservationSource for PrivateUserDetailsPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelPart for PrivateUserDetailsPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("user_private_info_user");

    type SourceFragment = UserFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.user_id
    }
}

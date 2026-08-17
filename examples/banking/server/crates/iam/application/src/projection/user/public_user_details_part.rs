use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};
use serde::{Deserialize, Serialize};

use super::{MaterializedUserStatus, UserFragment};

/// Detailed public user data.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicUserDetailsPart {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub status: MaterializedUserStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
    pub bio: Option<UserBio>,
}

impl From<UserFragment> for PublicUserDetailsPart {
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

impl ReadModelObservationSource for PublicUserDetailsPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelPart for PublicUserDetailsPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("public_user_details");

    type SourceFragment = UserFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.user_id
    }
}

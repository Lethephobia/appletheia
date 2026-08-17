use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use serde::{Deserialize, Serialize};

use super::UserFragment;

/// Basic user summary shared by authenticated IAM read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InternalUserSummaryPart {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}

impl From<UserFragment> for InternalUserSummaryPart {
    fn from(fragment: UserFragment) -> Self {
        Self {
            user_id: fragment.id,
            username: fragment.username,
            display_name: fragment.display_name,
            picture: fragment.picture,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for InternalUserSummaryPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelPart for InternalUserSummaryPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("internal_user_summary");

    type SourceFragment = UserFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.user_id
    }
}

use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use serde::{Deserialize, Serialize};

use super::{MaterializedUserStatus, UserFragment};

/// One row in the public user list delivery contract.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicUserListItemPart {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub status: MaterializedUserStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<UserFragment> for PublicUserListItemPart {
    fn from(fragment: UserFragment) -> Self {
        Self {
            user_id: fragment.id,
            username: fragment.username,
            display_name: fragment.display_name,
            picture: fragment.picture,
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for PublicUserListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelPart for PublicUserListItemPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("public_user_list_item");

    type SourceFragment = UserFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.user_id
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::{
        ReadModelFragment, ReadModelObservation, ReadModelPart,
    };
    use appletheia::domain::{EventId, EventOccurredAt};
    use banking_iam_domain::UserId;

    use crate::projection::MaterializedUserStatus;

    use super::*;

    #[test]
    fn serializes_a_flat_list_item() {
        let event_id = EventId::new();
        let item = PublicUserListItemPart {
            user_id: UserId::new(),
            username: None,
            display_name: None,
            picture: None,
            status: MaterializedUserStatus::Active,
            created_at: EventOccurredAt::now(),
            observation: ReadModelObservation::new(event_id, event_id),
        };

        let serialized = serde_json::to_value(&item).expect("list item should serialize");
        let partition = item
            .partition()
            .try_into_serialized::<UserFragment>()
            .expect("partition should serialize");
        assert!(serialized.get("user").is_none());
        assert!(serialized.get("user_id").is_some());
        assert_eq!(
            partition.value()["fragment_name"],
            UserFragment::NAME.value()
        );
        assert!(partition.value().get("part_name").is_none());
    }
}

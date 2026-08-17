use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};
use serde::{Deserialize, Serialize};

use super::OrganizationFragment;

/// Read model for one public organization list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicOrganizationListItemPart {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<OrganizationFragment> for PublicOrganizationListItemPart {
    fn from(fragment: OrganizationFragment) -> Self {
        Self {
            organization_id: fragment.id,
            handle: fragment.handle,
            display_name: fragment.display_name,
            picture: fragment.picture,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for PublicOrganizationListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelPart for PublicOrganizationListItemPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("public_organization_list_item");

    type SourceFragment = OrganizationFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.organization_id
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::{
        ReadModelFragment, ReadModelObservation, ReadModelPart,
    };
    use appletheia::domain::{EventId, EventOccurredAt};
    use banking_iam_domain::{OrganizationDisplayName, OrganizationHandle, OrganizationId};

    use super::*;

    #[test]
    fn serializes_a_flat_list_item() {
        let event_id = EventId::new();
        let item = PublicOrganizationListItemPart {
            organization_id: OrganizationId::new(),
            handle: OrganizationHandle::try_from("test_organization")
                .expect("handle should be valid"),
            display_name: OrganizationDisplayName::try_from("Test Organization")
                .expect("display name should be valid"),
            picture: None,
            created_at: EventOccurredAt::now(),
            observation: ReadModelObservation::new(event_id, event_id),
        };

        let serialized = serde_json::to_value(&item).expect("list item should serialize");
        let partition = item
            .partition()
            .try_into_serialized::<OrganizationFragment>()
            .expect("partition should serialize");
        assert!(serialized.get("organization").is_none());
        assert!(serialized.get("organization_id").is_some());
        assert_eq!(
            partition.value()["fragment_name"],
            OrganizationFragment::NAME.value()
        );
        assert!(partition.value().get("part_name").is_none());
    }
}

use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};

use crate::projection::{
    InternalOrganizationSummaryPart, OrganizationFragment, OrganizationMemberListItemPart,
    OrganizationMembershipFragment, UserFragment,
};

mod organization_member_list_criteria;
mod organization_member_list_cursor;
mod organization_member_list_reader;
mod organization_member_list_reader_error;
mod organization_member_list_sort_key;

pub use organization_member_list_criteria::OrganizationMemberListCriteria;
pub use organization_member_list_cursor::OrganizationMemberListCursor;
pub use organization_member_list_reader::OrganizationMemberListReader;
pub use organization_member_list_reader_error::OrganizationMemberListReaderError;
pub use organization_member_list_sort_key::OrganizationMemberListSortKey;

/// Read model for organization member list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMemberList {
    pub organization: InternalOrganizationSummaryPart,
    pub items: Vec<OrganizationMemberListItemPart>,
    pub next_cursor: Option<OrganizationMemberListCursor>,
}

impl ReadModelObservationSource for OrganizationMemberList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.organization
            .observations()
            .into_iter()
            .chain(
                self.items
                    .iter()
                    .flat_map(ReadModelObservationSource::observations),
            )
            .collect()
    }
}

impl ReadModel for OrganizationMemberList {
    const NAME: ReadModelName = ReadModelName::new("organization_member_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] =
        &[ReadModelPartChangeRoute::from_fragment::<
            OrganizationMembershipFragment,
        >(map_membership_to_member_list)];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![
            ReadModelPartTree::field::<InternalOrganizationSummaryPart>(
                "organization",
                read_model.map(|read_model| &read_model.organization),
            ),
            ReadModelPartTree::collection_with_explicit_route::<OrganizationMemberListItemPart>(
                "items",
                read_model.map(|read_model| read_model.items.as_slice()),
            ),
        ]
    }
}
fn map_membership_to_member_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<OrganizationMembershipFragment, OrganizationMemberListItemPart>(
        change,
        path_resolver,
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&fragment.organization.id)?])
        },
        |key| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&key.organization_id)?])
        },
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                UserFragment,
            >(&fragment.user.id)?])
        },
    )
}

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::ReadModelFragmentChange;
    use banking_iam_domain::{OrganizationId, UserId};

    use crate::projection::OrganizationMembershipFragmentKey;

    use super::*;

    #[test]
    fn explicit_member_route_preserves_the_complete_membership_key() {
        let key = OrganizationMembershipFragmentKey {
            user_id: UserId::new(),
            organization_id: OrganizationId::new(),
        };
        let fragment_change =
            ReadModelFragmentChange::<OrganizationMembershipFragment>::try_removed(&key)
                .expect("membership fragment change should be valid")
                .try_into_serialized()
                .expect("membership fragment should serialize");

        let path_resolver = ReadModelPartPathResolver::new(OrganizationMemberList::parts(None));
        let changes = map_membership_to_member_list(&fragment_change, path_resolver)
            .expect("membership route should succeed");
        let [
            ReadModelPartChange::Removed {
                source_partition,
                path,
                ..
            },
        ] = changes.as_slice()
        else {
            panic!("route should produce one removed part change");
        };

        let expected_key = serde_json::to_value(key).expect("membership key should serialize");
        assert_eq!(source_partition.value()["key"], expected_key);
        assert_eq!(
            serde_json::to_value(path).expect("path should serialize")[1]["value"],
            expected_key
        );
    }
}

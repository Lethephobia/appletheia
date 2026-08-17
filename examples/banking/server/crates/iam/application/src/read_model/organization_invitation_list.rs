use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};

use crate::projection::{
    InternalOrganizationSummaryPart, OrganizationFragment, OrganizationInvitationFragment,
    OrganizationInvitationListItemPart, UserFragment,
};

mod organization_invitation_list_criteria;
mod organization_invitation_list_cursor;
mod organization_invitation_list_reader;
mod organization_invitation_list_reader_error;
mod organization_invitation_list_sort_key;

pub use organization_invitation_list_criteria::OrganizationInvitationListCriteria;
pub use organization_invitation_list_cursor::OrganizationInvitationListCursor;
pub use organization_invitation_list_reader::OrganizationInvitationListReader;
pub use organization_invitation_list_reader_error::OrganizationInvitationListReaderError;
pub use organization_invitation_list_sort_key::OrganizationInvitationListSortKey;

/// Read model for organization-scoped invitation list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationList {
    pub organization: InternalOrganizationSummaryPart,
    pub items: Vec<OrganizationInvitationListItemPart>,
    pub next_cursor: Option<OrganizationInvitationListCursor>,
}

impl ReadModelObservationSource for OrganizationInvitationList {
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

impl ReadModel for OrganizationInvitationList {
    const NAME: ReadModelName = ReadModelName::new("organization_invitation_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] =
        &[ReadModelPartChangeRoute::from_fragment::<
            OrganizationInvitationFragment,
        >(map_invitation_to_organization_list)];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![
            ReadModelPartTree::field::<InternalOrganizationSummaryPart>(
                "organization",
                read_model.map(|read_model| &read_model.organization),
            ),
            ReadModelPartTree::collection_with_explicit_route::<OrganizationInvitationListItemPart>(
                "items",
                read_model.map(|read_model| read_model.items.as_slice()),
            ),
        ]
    }
}
fn map_invitation_to_organization_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<OrganizationInvitationFragment, OrganizationInvitationListItemPart>(
        change,
        path_resolver,
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&fragment.organization.id)?])
        },
        |_| Ok(Vec::new()),
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                UserFragment,
            >(&fragment.invitee.id)?])
        },
    )
}

use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};

use crate::projection::{
    InternalUserSummaryPart, OrganizationFragment, OrganizationInvitationFragment, UserFragment,
    UserOrganizationInvitationListItemPart,
};

mod user_organization_invitation_list_criteria;
mod user_organization_invitation_list_cursor;
mod user_organization_invitation_list_reader;
mod user_organization_invitation_list_reader_error;
mod user_organization_invitation_list_sort_key;

pub use user_organization_invitation_list_criteria::UserOrganizationInvitationListCriteria;
pub use user_organization_invitation_list_cursor::UserOrganizationInvitationListCursor;
pub use user_organization_invitation_list_reader::UserOrganizationInvitationListReader;
pub use user_organization_invitation_list_reader_error::UserOrganizationInvitationListReaderError;
pub use user_organization_invitation_list_sort_key::UserOrganizationInvitationListSortKey;

/// Read model for user-scoped organization invitation list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationInvitationList {
    pub user: InternalUserSummaryPart,
    pub items: Vec<UserOrganizationInvitationListItemPart>,
    pub next_cursor: Option<UserOrganizationInvitationListCursor>,
}

impl ReadModelObservationSource for UserOrganizationInvitationList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.user
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

impl ReadModel for UserOrganizationInvitationList {
    const NAME: ReadModelName = ReadModelName::new("user_organization_invitation_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] =
        &[ReadModelPartChangeRoute::from_fragment::<
            OrganizationInvitationFragment,
        >(map_invitation_to_user_list)];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![
            ReadModelPartTree::field::<InternalUserSummaryPart>(
                "user",
                read_model.map(|read_model| &read_model.user),
            ),
            ReadModelPartTree::collection_with_explicit_route::<
                UserOrganizationInvitationListItemPart,
            >(
                "items",
                read_model.map(|read_model| read_model.items.as_slice()),
            ),
        ]
    }
}
fn map_invitation_to_user_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<
        OrganizationInvitationFragment,
        UserOrganizationInvitationListItemPart,
    >(
        change,
        path_resolver,
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                UserFragment,
            >(&fragment.invitee.id)?])
        },
        |_| Ok(Vec::new()),
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&fragment.organization.id)?])
        },
    )
}

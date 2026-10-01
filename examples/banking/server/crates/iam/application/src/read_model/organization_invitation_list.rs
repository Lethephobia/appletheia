use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

mod organization_invitation_list_criteria;
mod organization_invitation_list_cursor;
mod organization_invitation_list_invitee;
mod organization_invitation_list_issuer;
mod organization_invitation_list_item;
mod organization_invitation_list_item_status;
mod organization_invitation_list_organization;
mod organization_invitation_list_reader;
mod organization_invitation_list_reader_error;
mod organization_invitation_list_sort_key;

pub use organization_invitation_list_criteria::OrganizationInvitationListCriteria;
pub use organization_invitation_list_cursor::OrganizationInvitationListCursor;
pub use organization_invitation_list_invitee::OrganizationInvitationListInvitee;
pub use organization_invitation_list_issuer::OrganizationInvitationListIssuer;
pub use organization_invitation_list_item::OrganizationInvitationListItem;
pub use organization_invitation_list_item_status::OrganizationInvitationListItemStatus;
pub use organization_invitation_list_organization::OrganizationInvitationListOrganization;
pub use organization_invitation_list_reader::OrganizationInvitationListReader;
pub use organization_invitation_list_reader_error::OrganizationInvitationListReaderError;
pub use organization_invitation_list_sort_key::OrganizationInvitationListSortKey;

/// Read model for organization-scoped invitation list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationInvitationList {
    pub organization: OrganizationInvitationListOrganization,
    pub items: Vec<OrganizationInvitationListItem>,
    pub start_cursor: Option<OrganizationInvitationListCursor>,
    pub end_cursor: Option<OrganizationInvitationListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModel for OrganizationInvitationList {
    const NAME: ReadModelName = ReadModelName::new("organization_invitation_list");
}

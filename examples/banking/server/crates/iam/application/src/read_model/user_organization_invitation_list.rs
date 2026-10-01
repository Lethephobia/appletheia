use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

mod user_organization_invitation_list_criteria;
mod user_organization_invitation_list_cursor;
mod user_organization_invitation_list_issuer;
mod user_organization_invitation_list_item;
mod user_organization_invitation_list_item_status;
mod user_organization_invitation_list_organization;
mod user_organization_invitation_list_reader;
mod user_organization_invitation_list_reader_error;
mod user_organization_invitation_list_sort_key;
mod user_organization_invitation_list_user;

pub use user_organization_invitation_list_criteria::UserOrganizationInvitationListCriteria;
pub use user_organization_invitation_list_cursor::UserOrganizationInvitationListCursor;
pub use user_organization_invitation_list_issuer::UserOrganizationInvitationListIssuer;
pub use user_organization_invitation_list_item::UserOrganizationInvitationListItem;
pub use user_organization_invitation_list_item_status::UserOrganizationInvitationListItemStatus;
pub use user_organization_invitation_list_organization::UserOrganizationInvitationListOrganization;
pub use user_organization_invitation_list_reader::UserOrganizationInvitationListReader;
pub use user_organization_invitation_list_reader_error::UserOrganizationInvitationListReaderError;
pub use user_organization_invitation_list_sort_key::UserOrganizationInvitationListSortKey;
pub use user_organization_invitation_list_user::UserOrganizationInvitationListUser;

/// Read model for user-scoped organization invitation list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationInvitationList {
    pub user: UserOrganizationInvitationListUser,
    pub items: Vec<UserOrganizationInvitationListItem>,
    pub start_cursor: Option<UserOrganizationInvitationListCursor>,
    pub end_cursor: Option<UserOrganizationInvitationListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModel for UserOrganizationInvitationList {
    const NAME: ReadModelName = ReadModelName::new("user_organization_invitation_list");
}

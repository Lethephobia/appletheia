use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

mod user_organization_membership_list_cursor;
mod user_organization_membership_list_item;
mod user_organization_membership_list_organization;
mod user_organization_membership_list_reader;
mod user_organization_membership_list_reader_error;
mod user_organization_membership_list_sort_key;
mod user_organization_membership_list_user;

pub use user_organization_membership_list_cursor::UserOrganizationMembershipListCursor;
pub use user_organization_membership_list_item::UserOrganizationMembershipListItem;
pub use user_organization_membership_list_organization::UserOrganizationMembershipListOrganization;
pub use user_organization_membership_list_reader::UserOrganizationMembershipListReader;
pub use user_organization_membership_list_reader_error::UserOrganizationMembershipListReaderError;
pub use user_organization_membership_list_sort_key::UserOrganizationMembershipListSortKey;
pub use user_organization_membership_list_user::UserOrganizationMembershipListUser;

/// Read model listing the organizations a user belongs to.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationMembershipList {
    pub user: UserOrganizationMembershipListUser,
    pub items: Vec<UserOrganizationMembershipListItem>,
    pub start_cursor: Option<UserOrganizationMembershipListCursor>,
    pub end_cursor: Option<UserOrganizationMembershipListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModel for UserOrganizationMembershipList {
    const NAME: ReadModelName = ReadModelName::new("user_organization_membership_list");
}

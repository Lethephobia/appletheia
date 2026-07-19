use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// Describes an organization-management owner snapshot upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationManagementInfoOwnerUpsert {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}

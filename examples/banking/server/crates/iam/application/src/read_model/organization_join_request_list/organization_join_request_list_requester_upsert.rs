use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// Describes an organization join request list requester snapshot upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestListRequesterUpsert {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}

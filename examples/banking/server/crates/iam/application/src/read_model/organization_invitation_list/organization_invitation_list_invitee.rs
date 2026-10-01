use serde::Serialize;

use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// Invitee profile embedded in an organization invitation list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationInvitationListInvitee {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}

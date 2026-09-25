use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// Member profile embedded in an organization member list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationMemberListMember {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}

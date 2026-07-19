use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Member profile embedded in an organization member list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMemberListMember {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}

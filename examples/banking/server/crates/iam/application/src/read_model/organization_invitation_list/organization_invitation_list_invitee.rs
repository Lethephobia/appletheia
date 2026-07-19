use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Invitee profile embedded in an organization invitation list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationListInvitee {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}

use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Requester profile embedded in an organization join request list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestListRequester {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}

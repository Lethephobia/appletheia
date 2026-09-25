use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// Requester profile embedded in an organization join request list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationJoinRequestListRequester {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}

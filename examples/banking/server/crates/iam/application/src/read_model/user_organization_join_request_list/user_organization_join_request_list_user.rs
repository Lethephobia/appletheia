use appletheia::domain::EventId;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use banking_shared_kernel_application::read_model::ReadModelObservation;

/// User profile owning a user organization join request list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationJoinRequestListUser {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}

impl UserOrganizationJoinRequestListUser {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        self.observation.event_ids().collect()
    }
}

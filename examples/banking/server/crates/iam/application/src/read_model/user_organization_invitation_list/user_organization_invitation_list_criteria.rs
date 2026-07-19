use super::UserOrganizationInvitationListItemStatus;

/// Search criteria for user organization invitation list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserOrganizationInvitationListCriteria {
    pub statuses: Option<Vec<UserOrganizationInvitationListItemStatus>>,
}

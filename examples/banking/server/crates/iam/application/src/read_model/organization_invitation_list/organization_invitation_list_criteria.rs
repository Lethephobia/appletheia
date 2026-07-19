use super::OrganizationInvitationListItemStatus;

/// Search criteria for organization invitation list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OrganizationInvitationListCriteria {
    pub statuses: Option<Vec<OrganizationInvitationListItemStatus>>,
}

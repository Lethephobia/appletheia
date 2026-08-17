use banking_iam_domain::OrganizationInvitationStatus;

/// Search criteria for organization invitation list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OrganizationInvitationListCriteria {
    pub status_in: Option<Vec<OrganizationInvitationStatus>>,
}

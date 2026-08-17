use banking_iam_domain::OrganizationInvitationStatus;

/// Search criteria for user organization invitation list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserOrganizationInvitationListCriteria {
    pub status_in: Option<Vec<OrganizationInvitationStatus>>,
}

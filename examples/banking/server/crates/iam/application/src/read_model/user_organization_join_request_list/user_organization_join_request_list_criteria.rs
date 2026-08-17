use banking_iam_domain::OrganizationJoinRequestStatus;

/// Search criteria for user organization join request list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserOrganizationJoinRequestListCriteria {
    pub status_in: Option<Vec<OrganizationJoinRequestStatus>>,
}

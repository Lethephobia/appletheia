use banking_iam_domain::OrganizationJoinRequestStatus;

/// Search criteria for organization join request list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OrganizationJoinRequestListCriteria {
    pub status_in: Option<Vec<OrganizationJoinRequestStatus>>,
}

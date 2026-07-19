use super::OrganizationJoinRequestListItemStatus;

/// Search criteria for organization join request list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OrganizationJoinRequestListCriteria {
    pub statuses: Option<Vec<OrganizationJoinRequestListItemStatus>>,
}

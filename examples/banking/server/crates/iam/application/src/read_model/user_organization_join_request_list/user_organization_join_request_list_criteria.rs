use super::UserOrganizationJoinRequestListItemStatus;

/// Search criteria for user organization join request list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserOrganizationJoinRequestListCriteria {
    pub statuses: Option<Vec<UserOrganizationJoinRequestListItemStatus>>,
}

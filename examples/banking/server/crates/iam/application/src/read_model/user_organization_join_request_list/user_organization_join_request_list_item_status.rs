/// Lifecycle status shown in a user organization join request list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserOrganizationJoinRequestListItemStatus {
    Pending,
    Approved,
    Rejected,
    Canceled,
}

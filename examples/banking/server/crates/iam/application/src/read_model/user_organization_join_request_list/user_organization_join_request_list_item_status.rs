use serde::Serialize;

/// Lifecycle status shown in a user organization join request list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum UserOrganizationJoinRequestListItemStatus {
    Pending,
    Approved,
    Rejected,
    Canceled,
}

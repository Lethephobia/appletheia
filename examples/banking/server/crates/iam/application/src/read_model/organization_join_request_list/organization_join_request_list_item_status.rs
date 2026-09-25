use serde::Serialize;

/// Lifecycle status shown in an organization join request list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum OrganizationJoinRequestListItemStatus {
    Pending,
    Approved,
    Rejected,
    Canceled,
}

use serde::Serialize;

/// Lifecycle status tracked by public user list projections.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum PublicUserListItemStatus {
    Active,
    Inactive,
}

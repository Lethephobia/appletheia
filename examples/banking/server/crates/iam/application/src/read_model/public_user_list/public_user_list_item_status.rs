/// Lifecycle status tracked by public user list projections.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PublicUserListItemStatus {
    Active,
    Inactive,
}

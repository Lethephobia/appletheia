/// Sort key for public user list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PublicUserListSortKey {
    CreatedAt,
    UserId,
}

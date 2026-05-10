/// Sort key for public account list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PublicAccountListSortKey {
    CreatedAt,
    AccountId,
}

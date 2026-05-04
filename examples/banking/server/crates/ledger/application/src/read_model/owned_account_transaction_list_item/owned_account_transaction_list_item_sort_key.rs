/// Sort key for owned account transaction list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OwnedAccountTransactionListItemSortKey {
    OccurredAt,
    Id,
}

/// Status of a transaction list item.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OwnedAccountTransactionListItemStatus {
    Pending,
    Completed,
    Failed,
    RequiresReview,
}

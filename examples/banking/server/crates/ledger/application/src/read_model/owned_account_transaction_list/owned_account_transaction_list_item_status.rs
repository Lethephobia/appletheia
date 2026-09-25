use serde::Serialize;

/// Status of a transaction list item.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum OwnedAccountTransactionListItemStatus {
    Pending,
    Completed,
    Failed,
    RequiresReview,
}

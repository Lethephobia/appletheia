/// Sort key for transfer recipient list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TransferRecipientListItemSortKey {
    CreatedAt,
    UserId,
}

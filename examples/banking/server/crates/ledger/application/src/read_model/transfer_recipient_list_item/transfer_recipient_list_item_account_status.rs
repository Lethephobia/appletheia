/// Account status tracked by transfer recipient list item projections.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TransferRecipientListItemAccountStatus {
    Active,
    Frozen,
}

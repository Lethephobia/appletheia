/// Status shown in an owned account list item.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OwnedAccountListItemStatus {
    Active,
    Frozen,
}

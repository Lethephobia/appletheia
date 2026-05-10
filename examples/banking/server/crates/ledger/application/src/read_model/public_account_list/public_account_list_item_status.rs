/// Account status tracked by public account list item projections.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PublicAccountListItemStatus {
    Active,
    Frozen,
}

/// Sort key for currency list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CurrencyListSortKey {
    CreatedAt,
    CurrencyId,
}

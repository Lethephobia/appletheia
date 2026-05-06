use super::CurrencyListItemStatus;

/// Search criteria for currency list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CurrencyListItemCriteria {
    pub status: Option<CurrencyListItemStatus>,
}

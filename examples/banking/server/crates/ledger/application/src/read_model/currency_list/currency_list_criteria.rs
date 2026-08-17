use crate::projection::MaterializedCurrencyStatus;

/// Search criteria for currency list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CurrencyListCriteria {
    pub status_in: Option<Vec<MaterializedCurrencyStatus>>,
}

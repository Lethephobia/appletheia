use super::{CurrencyListItemOwnerOrganization, CurrencyListItemOwnerUser};

/// Owner shown in a currency list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurrencyListItemOwner {
    User(CurrencyListItemOwnerUser),
    Organization(CurrencyListItemOwnerOrganization),
}

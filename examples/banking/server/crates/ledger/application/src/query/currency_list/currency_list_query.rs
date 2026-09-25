use appletheia::query;

#[query(name = "currency_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListQuery {
    pub include_inactive: bool,
}

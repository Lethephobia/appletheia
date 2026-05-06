#![allow(dead_code, unused_imports)]

use appletheia_application::query::{Query as AppQuery, QueryName};
use appletheia_macros::{Query, query};

#[query(name = "account_list")]
#[derive(Clone, Debug, PartialEq, Eq)]
struct AccountListQuery {
    owner_id: Option<String>,
}

#[derive(Query)]
#[query_derive(name = "account_detail")]
struct AccountDetailQuery {
    account_id: String,
}

fn assert_query<T: AppQuery>() {}

fn main() {
    assert_query::<AccountListQuery>();
    assert_query::<AccountDetailQuery>();
    let _: QueryName = AccountListQuery::NAME;
    let _: QueryName = AccountDetailQuery::NAME;
}

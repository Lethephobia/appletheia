mod currency_list;
mod owned_account_list;
mod owned_account_transaction_list;
mod public_account_list;
mod wallet_bookmark_list;

pub use currency_list::{
    CurrencyListQuery, CurrencyListQueryHandler, CurrencyListQueryHandlerError,
};
pub use owned_account_list::{
    OwnedAccountListQuery, OwnedAccountListQueryHandler, OwnedAccountListQueryHandlerError,
};
pub use owned_account_transaction_list::{
    OwnedAccountTransactionListQuery, OwnedAccountTransactionListQueryHandler,
    OwnedAccountTransactionListQueryHandlerError,
};
pub use public_account_list::{
    PublicAccountListQuery, PublicAccountListQueryHandler, PublicAccountListQueryHandlerError,
};
pub use wallet_bookmark_list::{
    WalletBookmarkListQuery, WalletBookmarkListQueryHandler, WalletBookmarkListQueryHandlerError,
};

use appletheia::application::query::{QueryHandler, WatchableQueryHandler};
use appletheia::application::read_model::{ReadModelDependency, ReadModelDependencyTopic};
use banking_iam_application::{OrganizationFragment, UserFragment};

use crate::projection::{
    AccountFragment, AccountTransactionFragment, CurrencyFragment, WalletBookmarkFragment,
};
use crate::read_model::{
    CurrencyListReader, OwnedAccountListReader, OwnedAccountTransactionListReader,
    PublicAccountListReader, WalletBookmarkListReader,
};

macro_rules! impl_watchable_query_handler {
    ($handler:ident, $reader:path, [$($fragment:ty),+ $(,)?]) => {
        impl<T> WatchableQueryHandler for $handler<T>
        where
            T: $reader,
            $handler<T>: QueryHandler,
        {
            fn watch_dependencies(
                &self,
                _query: &Self::Query,
            ) -> Result<Vec<ReadModelDependency>, Self::Error> {
                Ok(vec![$(
                    ReadModelDependency::Topic(ReadModelDependencyTopic::all::<$fragment>()),
                )+])
            }
        }
    };
}

impl_watchable_query_handler!(
    CurrencyListQueryHandler,
    CurrencyListReader,
    [CurrencyFragment, UserFragment, OrganizationFragment]
);
impl_watchable_query_handler!(
    OwnedAccountListQueryHandler,
    OwnedAccountListReader,
    [
        AccountFragment,
        CurrencyFragment,
        UserFragment,
        OrganizationFragment,
    ]
);
impl_watchable_query_handler!(
    OwnedAccountTransactionListQueryHandler,
    OwnedAccountTransactionListReader,
    [
        AccountTransactionFragment,
        AccountFragment,
        CurrencyFragment,
        UserFragment,
        OrganizationFragment,
    ]
);
impl_watchable_query_handler!(
    PublicAccountListQueryHandler,
    PublicAccountListReader,
    [
        AccountFragment,
        CurrencyFragment,
        UserFragment,
        OrganizationFragment,
    ]
);
impl_watchable_query_handler!(
    WalletBookmarkListQueryHandler,
    WalletBookmarkListReader,
    [WalletBookmarkFragment, UserFragment, OrganizationFragment]
);

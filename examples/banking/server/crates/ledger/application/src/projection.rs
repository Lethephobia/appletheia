macro_rules! impl_observation_part {
    ($type:ty, $name:literal, $source_fragment:ty, $key:expr) => {
        impl ReadModelObservationSource for $type {
            fn observations(&self) -> Vec<ReadModelObservation> {
                vec![self.observation]
            }
        }

        impl ReadModelPart for $type {
            const NAME: ReadModelPartName = ReadModelPartName::new($name);

            type SourceFragment = $source_fragment;

            fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
                $key(self)
            }
        }
    };
}

macro_rules! impl_part {
    ($type:ty, $name:literal, $source_fragment:ty, $key:expr) => {
        impl ReadModelPart for $type {
            const NAME: ReadModelPartName = ReadModelPartName::new($name);

            type SourceFragment = $source_fragment;

            fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
                $key(self)
            }
        }
    };
}

macro_rules! impl_composite_part {
    ($type:ty, $name:literal, $source_fragment:ty, $key:expr, $parts:expr) => {
        impl ReadModelPart for $type {
            const NAME: ReadModelPartName = ReadModelPartName::new($name);

            type SourceFragment = $source_fragment;

            fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
                $key(self)
            }

            fn parts(part: Option<&Self>) -> Vec<ReadModelPartTree> {
                $parts(part)
            }
        }
    };
}

mod account;
mod account_transaction;
mod currency;
mod fragment_owner;
mod organization;
mod user;
mod wallet_bookmark;

pub use account::{
    AccountFragment, AccountFragmentProjector, AccountFragmentProjectorError,
    AccountFragmentProjectorSpec, AccountFragmentUpsert, AccountFragmentWriter,
    AccountFragmentWriterError, MaterializedAccountStatus, MaterializedAccountStatusError,
    OwnedAccountListItemPart, OwnedAccountTransactionListItemCounterpartyAccountPart,
    PublicAccountListItemPart,
};
pub use account_transaction::{
    AccountTransactionCurrencyIssuanceIssuedRecord, AccountTransactionDirection,
    AccountTransactionFragment, AccountTransactionFragmentInsert, AccountTransactionFragmentKind,
    AccountTransactionFragmentProjector, AccountTransactionFragmentProjectorError,
    AccountTransactionFragmentProjectorSpec, AccountTransactionFragmentWriter,
    AccountTransactionFragmentWriterError, AccountTransactionId, AccountTransactionStatus,
    AccountTransactionTransferRequestedRecord, OwnedAccountTransactionListItemPart,
};
pub use currency::{
    CurrencyFragment, CurrencyFragmentProjector, CurrencyFragmentProjectorError,
    CurrencyFragmentProjectorSpec, CurrencyFragmentUpsert, CurrencyFragmentWriter,
    CurrencyFragmentWriterError, CurrencyListItemPart, MaterializedCurrencyStatus,
    MaterializedCurrencyStatusError, OwnedAccountListItemCurrencyPart,
    OwnedAccountTransactionListItemCurrencyPart, PublicAccountListItemCurrencyPart,
};
pub use fragment_owner::FragmentOwner;
pub use organization::{
    CurrencyListItemOwnerOrganizationPart, OwnedAccountListOwnerOrganizationPart,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart,
    OwnedAccountTransactionListOwnerOrganizationPart, PublicAccountListItemOwnerOrganizationPart,
};
pub use user::{
    CurrencyListItemOwnerUserPart, OwnedAccountListOwnerUserPart,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart,
    OwnedAccountTransactionListOwnerUserPart, PublicAccountListItemOwnerUserPart,
};
pub use wallet_bookmark::{
    WalletBookmarkFragment, WalletBookmarkFragmentProjector, WalletBookmarkFragmentProjectorError,
    WalletBookmarkFragmentProjectorSpec, WalletBookmarkFragmentUpsert,
    WalletBookmarkFragmentWriter, WalletBookmarkFragmentWriterError, WalletBookmarkListItemPart,
};

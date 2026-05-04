use super::{
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganization,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUser,
};

/// Counterparty account owner shown in a transfer transaction list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnedAccountTransactionListItemCounterpartyAccountOwner {
    User(OwnedAccountTransactionListItemCounterpartyAccountOwnerUser),
    Organization(OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganization),
}

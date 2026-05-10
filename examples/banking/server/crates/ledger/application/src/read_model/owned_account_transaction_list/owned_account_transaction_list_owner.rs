use super::{OwnedAccountTransactionListOwnerOrganization, OwnedAccountTransactionListOwnerUser};

/// Owner shown in an owned account transaction list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnedAccountTransactionListOwner {
    User(OwnedAccountTransactionListOwnerUser),
    Organization(OwnedAccountTransactionListOwnerOrganization),
}

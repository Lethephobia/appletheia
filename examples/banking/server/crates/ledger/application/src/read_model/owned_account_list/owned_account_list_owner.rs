use super::{OwnedAccountListOwnerOrganization, OwnedAccountListOwnerUser};

/// Owner shown in an owned account list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnedAccountListOwner {
    User(OwnedAccountListOwnerUser),
    Organization(OwnedAccountListOwnerOrganization),
}

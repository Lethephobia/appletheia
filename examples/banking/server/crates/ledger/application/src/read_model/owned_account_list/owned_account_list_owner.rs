use serde::Serialize;

use super::{OwnedAccountListOwnerOrganization, OwnedAccountListOwnerUser};

/// Owner shown in an owned account list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum OwnedAccountListOwner {
    User(OwnedAccountListOwnerUser),
    Organization(OwnedAccountListOwnerOrganization),
}

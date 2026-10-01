use serde::Serialize;

use super::{PublicAccountListItemOwnerOrganization, PublicAccountListItemOwnerUser};

/// Owner fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum PublicAccountListItemOwner {
    User(PublicAccountListItemOwnerUser),
    Organization(PublicAccountListItemOwnerOrganization),
}

use super::{PublicAccountListItemOwnerOrganization, PublicAccountListItemOwnerUser};

/// Owner fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicAccountListItemOwner {
    User(PublicAccountListItemOwnerUser),
    Organization(PublicAccountListItemOwnerOrganization),
}

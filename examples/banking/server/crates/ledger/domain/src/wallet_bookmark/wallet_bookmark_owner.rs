use banking_iam_domain::{OrganizationId, UserId};
use serde::{Deserialize, Serialize};

use crate::account::AccountOwner;

/// Identifies the owner of a `WalletBookmark`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum WalletBookmarkOwner {
    User(UserId),
    Organization(OrganizationId),
}

impl WalletBookmarkOwner {
    /// Creates a user-owned wallet bookmark owner.
    pub fn user(user_id: UserId) -> Self {
        Self::User(user_id)
    }

    /// Creates an organization-owned wallet bookmark owner.
    pub fn organization(organization_id: OrganizationId) -> Self {
        Self::Organization(organization_id)
    }

    /// Returns the user id when this owner is a user.
    pub fn user_id(&self) -> Option<&UserId> {
        match self {
            Self::User(user_id) => Some(user_id),
            Self::Organization(_) => None,
        }
    }

    /// Returns the organization id when this owner is an organization.
    pub fn organization_id(&self) -> Option<&OrganizationId> {
        match self {
            Self::User(_) => None,
            Self::Organization(organization_id) => Some(organization_id),
        }
    }
}

impl From<UserId> for WalletBookmarkOwner {
    fn from(value: UserId) -> Self {
        Self::User(value)
    }
}

impl From<OrganizationId> for WalletBookmarkOwner {
    fn from(value: OrganizationId) -> Self {
        Self::Organization(value)
    }
}

impl From<AccountOwner> for WalletBookmarkOwner {
    fn from(value: AccountOwner) -> Self {
        match value {
            AccountOwner::User(user_id) => Self::User(user_id),
            AccountOwner::Organization(organization_id) => Self::Organization(organization_id),
        }
    }
}

use banking_iam_domain::{OrganizationId, UserId};
use serde::{Deserialize, Serialize};

/// Identifies the owner of a `Currency`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum CurrencyOwner {
    User(UserId),
    Organization(OrganizationId),
}

impl CurrencyOwner {
    /// Creates a user-owned currency owner.
    pub fn user(user_id: UserId) -> Self {
        Self::User(user_id)
    }

    /// Creates an organization-owned currency owner.
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

impl From<UserId> for CurrencyOwner {
    fn from(value: UserId) -> Self {
        Self::User(value)
    }
}

impl From<OrganizationId> for CurrencyOwner {
    fn from(value: OrganizationId) -> Self {
        Self::Organization(value)
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::CurrencyOwner;
    use banking_iam_domain::{OrganizationId, UserId};

    #[test]
    fn serializes_to_json() {
        let owner = CurrencyOwner::User(
            UserId::try_from_uuid(Uuid::now_v7()).expect("user id should be accepted"),
        );
        let value = serde_json::to_value(&owner).expect("owner should serialize");

        assert_eq!(value["type"], serde_json::json!("user"));
    }

    #[test]
    fn organization_variant_preserves_id() {
        let organization_id = OrganizationId::try_from_uuid(Uuid::now_v7())
            .expect("organization id should be accepted");
        let owner = CurrencyOwner::Organization(organization_id);

        assert_eq!(owner.organization_id(), Some(&organization_id));
        assert_eq!(owner.user_id(), None);
    }
}

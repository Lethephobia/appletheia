use std::collections::BTreeSet;
use std::collections::btree_set;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use super::OrganizationRole;

/// Declares the elevated roles granted through an organization membership.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrganizationMembershipRoles(BTreeSet<OrganizationRole>);

impl OrganizationMembershipRoles {
    /// Creates a normalized role set.
    pub fn new<I>(roles: I) -> Self
    where
        I: IntoIterator<Item = OrganizationRole>,
    {
        roles.into_iter().collect()
    }

    /// Returns whether the membership contains the given role.
    pub fn contains(&self, role: &OrganizationRole) -> bool {
        self.0.contains(role)
    }

    /// Returns an iterator over roles in stable sort order.
    pub fn iter(&self) -> btree_set::Iter<'_, OrganizationRole> {
        self.0.iter()
    }

    /// Returns the number of roles in the set.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the set contains no roles.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromIterator<OrganizationRole> for OrganizationMembershipRoles {
    fn from_iter<T: IntoIterator<Item = OrganizationRole>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for OrganizationMembershipRoles {
    type Item = OrganizationRole;
    type IntoIter = std::collections::btree_set::IntoIter<OrganizationRole>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a OrganizationMembershipRoles {
    type Item = &'a OrganizationRole;
    type IntoIter = btree_set::Iter<'a, OrganizationRole>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Hash for OrganizationMembershipRoles {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for role in &self.0 {
            role.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OrganizationMembershipRoles, OrganizationRole};

    #[test]
    fn normalizes_duplicate_roles_into_sorted_set() {
        let roles = OrganizationMembershipRoles::new([
            OrganizationRole::Treasurer,
            OrganizationRole::Admin,
            OrganizationRole::Treasurer,
        ]);

        assert_eq!(
            roles.into_iter().collect::<Vec<_>>(),
            vec![OrganizationRole::Admin, OrganizationRole::Treasurer]
        );
    }

    #[test]
    fn serializes_to_json_array() {
        let roles = OrganizationMembershipRoles::new([
            OrganizationRole::Admin,
            OrganizationRole::FinanceManager,
        ]);

        let value = serde_json::to_value(roles).expect("serialize works");

        assert_eq!(
            value,
            serde_json::json!([
                { "type": "admin" },
                { "type": "finance_manager" }
            ])
        );
    }
}

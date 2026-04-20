use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::OrganizationRole;

/// Stores the elevated roles granted through an organization membership.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrganizationRoles(BTreeSet<OrganizationRole>);

impl OrganizationRoles {
    pub fn contains(&self, role: OrganizationRole) -> bool {
        self.0.contains(&role)
    }

    pub fn granted(&self, role: OrganizationRole) -> Self {
        let mut roles = self.0.clone();
        roles.insert(role);
        Self(roles)
    }

    pub fn revoked(&self, role: OrganizationRole) -> Self {
        let mut roles = self.0.clone();
        roles.remove(&role);
        Self(roles)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &OrganizationRole> {
        self.0.iter()
    }
}

impl<const N: usize> From<[OrganizationRole; N]> for OrganizationRoles {
    fn from(value: [OrganizationRole; N]) -> Self {
        Self(value.into_iter().collect())
    }
}

impl FromIterator<OrganizationRole> for OrganizationRoles {
    fn from_iter<T: IntoIterator<Item = OrganizationRole>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::{OrganizationRole, OrganizationRoles};

    #[test]
    fn deduplicates_roles() {
        let roles = OrganizationRoles::from_iter([
            OrganizationRole::FinanceManager,
            OrganizationRole::FinanceManager,
        ]);

        assert_eq!(roles.iter().count(), 1);
        assert!(roles.contains(OrganizationRole::FinanceManager));
    }

    #[test]
    fn granted_returns_new_roles_with_added_role() {
        let roles = OrganizationRoles::default();

        let updated = roles.granted(OrganizationRole::Admin);

        assert!(updated.contains(OrganizationRole::Admin));
        assert!(!roles.contains(OrganizationRole::Admin));
    }

    #[test]
    fn revoked_returns_new_roles_without_removed_role() {
        let roles = OrganizationRoles::from([OrganizationRole::Treasurer]);

        let updated = roles.revoked(OrganizationRole::Treasurer);

        assert!(updated.is_empty());
        assert!(roles.contains(OrganizationRole::Treasurer));
    }
}

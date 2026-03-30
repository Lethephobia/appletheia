use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::{Uuid, Version};

use super::{RoleIdError, RoleName};

/// Identifies a `Role` aggregate.
#[aggregate_id(error = RoleIdError, validate = validate_role_id)]
pub struct RoleId(Uuid);

impl RoleId {
    /// Returns the predefined administrator role ID.
    pub fn admin() -> Self {
        Self::from_name(&RoleName::admin())
    }

    /// Creates a deterministic role ID from a role name.
    pub fn from_name(name: &RoleName) -> Self {
        let uuid = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!("banking:iam:role:{}", name.value()).as_bytes(),
        );

        Self::try_from_uuid(uuid).expect("generated role id should satisfy validation")
    }
}

fn validate_role_id(value: Uuid) -> Result<(), RoleIdError> {
    if value.get_version() != Some(Version::Sha1) {
        return Err(RoleIdError::NotUuidV5);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::{RoleId, RoleIdError, RoleName};

    #[test]
    fn from_name_is_deterministic() {
        let name = RoleName::admin();

        assert_eq!(RoleId::from_name(&name), RoleId::from_name(&name));
    }

    #[test]
    fn admin_is_stable() {
        assert_eq!(RoleId::admin(), RoleId::from_name(&RoleName::admin()));
    }

    #[test]
    fn rejects_non_uuid_v5() {
        let error = RoleId::try_from_uuid(Uuid::now_v7()).expect_err("uuid v7 should be rejected");

        assert!(matches!(error, RoleIdError::NotUuidV5));
    }
}

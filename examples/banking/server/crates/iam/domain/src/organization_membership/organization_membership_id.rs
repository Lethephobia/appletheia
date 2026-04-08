use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies an `OrganizationMembership` aggregate.
#[aggregate_id]
pub struct OrganizationMembershipId(Uuid);

impl OrganizationMembershipId {
    /// Creates a new organization membership ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OrganizationMembershipId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::OrganizationMembershipId;

    #[test]
    fn new_creates_valid_organization_membership_id() {
        let membership_id = OrganizationMembershipId::new();

        assert!(!membership_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let membership_id =
            OrganizationMembershipId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(membership_id.value(), Uuid::nil());
    }
}

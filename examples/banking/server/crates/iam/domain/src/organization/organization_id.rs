use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies an `Organization` aggregate.
#[aggregate_id]
pub struct OrganizationId(Uuid);

impl OrganizationId {
    /// Creates a new organization ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OrganizationId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::OrganizationId;

    #[test]
    fn new_creates_valid_organization_id() {
        let organization_id = OrganizationId::new();

        assert!(!organization_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let organization_id =
            OrganizationId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(organization_id.value(), Uuid::nil());
    }
}

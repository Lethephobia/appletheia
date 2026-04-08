use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies an `OrganizationJoinRequest` aggregate.
#[aggregate_id]
pub struct OrganizationJoinRequestId(Uuid);

impl OrganizationJoinRequestId {
    /// Creates a new organization join request ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OrganizationJoinRequestId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::OrganizationJoinRequestId;

    #[test]
    fn new_creates_valid_organization_join_request_id() {
        let join_request_id = OrganizationJoinRequestId::new();

        assert!(!join_request_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let join_request_id =
            OrganizationJoinRequestId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(join_request_id.value(), Uuid::nil());
    }
}

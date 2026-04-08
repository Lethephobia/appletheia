use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies an `OrganizationInvitation` aggregate.
#[aggregate_id]
pub struct OrganizationInvitationId(Uuid);

impl OrganizationInvitationId {
    /// Creates a new organization invitation ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OrganizationInvitationId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::OrganizationInvitationId;

    #[test]
    fn new_creates_valid_organization_invitation_id() {
        let invitation_id = OrganizationInvitationId::new();

        assert!(!invitation_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let invitation_id =
            OrganizationInvitationId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(invitation_id.value(), Uuid::nil());
    }
}

use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

#[aggregate_id]
pub struct TeamMembershipId(Uuid);

impl TeamMembershipId {
    /// Creates a new team membership ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for TeamMembershipId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::TeamMembershipId;

    #[test]
    fn new_creates_valid_team_membership_id() {
        let membership_id = TeamMembershipId::new();

        assert!(!membership_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let membership_id =
            TeamMembershipId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(membership_id.value(), Uuid::nil());
    }
}

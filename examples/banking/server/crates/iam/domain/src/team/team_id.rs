use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `Team` aggregate.
#[aggregate_id]
pub struct TeamId(Uuid);

impl TeamId {
    /// Creates a new team ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for TeamId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::TeamId;

    #[test]
    fn new_creates_valid_team_id() {
        let team_id = TeamId::new();

        assert!(!team_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let team_id = TeamId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(team_id.value(), Uuid::nil());
    }
}

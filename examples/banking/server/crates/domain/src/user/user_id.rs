use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `User` aggregate.
#[aggregate_id]
pub struct UserId(Uuid);

impl UserId {
    /// Creates a new user ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::UserId;

    #[test]
    fn new_creates_valid_user_id() {
        let user_id = UserId::new();

        assert!(!user_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let user_id = UserId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(user_id.value(), Uuid::nil());
    }
}

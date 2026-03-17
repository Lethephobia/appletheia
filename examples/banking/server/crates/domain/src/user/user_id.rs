use std::fmt::{self, Display};

use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

use super::UserIdError;

fn validate_user_id(value: Uuid) -> Result<(), UserIdError> {
    if value.is_nil() {
        return Err(UserIdError::NilUuid);
    }

    Ok(())
}

/// Identifies a `User` aggregate.
#[aggregate_id(error = UserIdError, validate = validate_user_id)]
pub struct UserId(Uuid);

impl UserId {
    /// Creates a new user ID.
    pub fn new() -> Self {
        Self::try_from_uuid(Uuid::now_v7()).expect("generated uuid v7 should be valid")
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::{UserId, UserIdError};

    #[test]
    fn new_creates_valid_user_id() {
        let user_id = UserId::new();

        assert!(!user_id.value().is_nil());
    }

    #[test]
    fn rejects_nil_uuid() {
        let error = UserId::try_from_uuid(Uuid::nil()).expect_err("nil uuid should fail");

        assert!(matches!(error, UserIdError::NilUuid));
    }
}

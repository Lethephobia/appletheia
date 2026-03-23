use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies an `Account` aggregate.
#[aggregate_id]
pub struct AccountId(Uuid);

impl AccountId {
    /// Creates a new account ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for AccountId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::AccountId;

    #[test]
    fn new_creates_valid_account_id() {
        let account_id = AccountId::new();

        assert!(!account_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let account_id = AccountId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(account_id.value(), Uuid::nil());
    }
}

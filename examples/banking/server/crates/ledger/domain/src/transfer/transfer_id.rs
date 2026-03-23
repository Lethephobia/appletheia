use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `Transfer` aggregate.
#[aggregate_id]
pub struct TransferId(Uuid);

impl TransferId {
    /// Creates a new transfer ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for TransferId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::TransferId;

    #[test]
    fn new_creates_valid_transfer_id() {
        let transfer_id = TransferId::new();

        assert!(!transfer_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let transfer_id = TransferId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(transfer_id.value(), Uuid::nil());
    }
}

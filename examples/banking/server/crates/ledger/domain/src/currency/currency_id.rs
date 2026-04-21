use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `Currency` aggregate.
#[aggregate_id]
pub struct CurrencyId(Uuid);

impl CurrencyId {
    /// Creates a new currency ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CurrencyId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::CurrencyId;

    #[test]
    fn new_creates_valid_currency_id() {
        let currency_id = CurrencyId::new();

        assert!(!currency_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let currency_id = CurrencyId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(currency_id.value(), Uuid::nil());
    }
}

use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `CurrencyDefinition` aggregate.
#[aggregate_id]
pub struct CurrencyDefinitionId(Uuid);

impl CurrencyDefinitionId {
    /// Creates a new currency-definition ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CurrencyDefinitionId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::CurrencyDefinitionId;

    #[test]
    fn new_creates_valid_currency_definition_id() {
        let currency_definition_id = CurrencyDefinitionId::new();

        assert!(!currency_definition_id.value().is_nil());
    }

    #[test]
    fn accepts_uuid_without_validation() {
        let currency_definition_id =
            CurrencyDefinitionId::try_from_uuid(Uuid::nil()).expect("uuid should be accepted");

        assert_eq!(currency_definition_id.value(), Uuid::nil());
    }
}

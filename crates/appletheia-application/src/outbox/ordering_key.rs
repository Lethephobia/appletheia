use std::{fmt, fmt::Display, str::FromStr};

use crate::event::{AggregateIdOwned, AggregateTypeOwned};

use super::OrderingKeyError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderingKey {
    aggregate_type: AggregateTypeOwned,
    aggregate_id: AggregateIdOwned,
}

impl OrderingKey {
    pub fn new(aggregate_type: AggregateTypeOwned, aggregate_id: AggregateIdOwned) -> Self {
        Self {
            aggregate_type,
            aggregate_id,
        }
    }

    pub fn aggregate_type(&self) -> &AggregateTypeOwned {
        &self.aggregate_type
    }

    pub fn aggregate_id(&self) -> AggregateIdOwned {
        self.aggregate_id
    }
}

impl Display for OrderingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.aggregate_type, self.aggregate_id)
    }
}

impl FromStr for OrderingKey {
    type Err = OrderingKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.matches(':').count() != 1 {
            return Err(OrderingKeyError::MissingSeparator);
        }
        let (aggregate_type_raw, aggregate_id_raw) = s
            .split_once(':')
            .ok_or(OrderingKeyError::MissingSeparator)?;

        let aggregate_type = AggregateTypeOwned::from_str(aggregate_type_raw)
            .map_err(OrderingKeyError::InvalidAggregateType)?;
        let aggregate_id = AggregateIdOwned::from_str(aggregate_id_raw)
            .map_err(OrderingKeyError::InvalidAggregateId)?;

        Ok(Self::new(aggregate_type, aggregate_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn formats_and_parses_round_trip() {
        let agg_type = AggregateTypeOwned::from_str("order").unwrap();
        let agg_id = AggregateIdOwned::from_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
        let key = OrderingKey::new(agg_type.clone(), agg_id);

        let text = key.to_string();
        let parsed = OrderingKey::from_str(&text).unwrap();

        assert_eq!(parsed.aggregate_type(), &agg_type);
        assert_eq!(parsed.aggregate_id(), agg_id);
    }

    #[test]
    fn fails_without_separator() {
        let err = OrderingKey::from_str("noseparator").expect_err("should fail");
        assert!(matches!(err, OrderingKeyError::MissingSeparator));
    }

    #[test]
    fn fails_on_invalid_uuid() {
        let err = OrderingKey::from_str("order:not-a-uuid").expect_err("should fail");
        assert!(matches!(err, OrderingKeyError::InvalidAggregateId(_)));
    }

    #[test]
    fn uses_uuid_round_trip() {
        let uuid = Uuid::nil();
        let key_str = format!("user:{}", uuid);
        let key = OrderingKey::from_str(&key_str).unwrap();
        assert_eq!(key.aggregate_id(), AggregateIdOwned::from(uuid));
    }
}

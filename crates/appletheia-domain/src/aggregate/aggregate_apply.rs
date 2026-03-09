use std::error::Error;

use crate::event::EventPayload;

/// Applies an event payload to a mutable aggregate instance.
///
/// Implementations update the aggregate state to reflect the domain effect of
/// a previously validated event payload.
pub trait AggregateApply<P, E>
where
    P: EventPayload,
    E: Error + Send + Sync + 'static,
{
    /// Applies the payload to the aggregate.
    fn apply(&mut self, payload: &P) -> Result<(), E>;
}

#[cfg(test)]
mod tests {
    use appletheia_macros::event_payload;
    use thiserror::Error;

    use super::AggregateApply;

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[event_payload(error = CounterEventPayloadError)]
    enum CounterEventPayload {
        Incremented { amount: i32 },
        Decremented { amount: i32 },
    }

    #[derive(Debug, Error, Eq, PartialEq)]
    enum CounterError {
        #[error("counter cannot become negative")]
        NegativeValue,
    }

    #[derive(Default)]
    struct Counter {
        value: i32,
    }

    impl AggregateApply<CounterEventPayload, CounterError> for Counter {
        fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
            match payload {
                CounterEventPayload::Incremented { amount } => {
                    self.value += amount;
                    Ok(())
                }
                CounterEventPayload::Decremented { amount } => {
                    if self.value < *amount {
                        return Err(CounterError::NegativeValue);
                    }

                    self.value -= amount;
                    Ok(())
                }
            }
        }
    }

    #[test]
    fn apply_updates_aggregate_state() {
        let mut counter = Counter::default();

        counter
            .apply(&CounterEventPayload::Incremented { amount: 3 })
            .expect("increment should succeed");
        counter
            .apply(&CounterEventPayload::Decremented { amount: 1 })
            .expect("decrement should succeed");

        assert_eq!(counter.value, 2);
    }

    #[test]
    fn apply_propagates_domain_errors() {
        let mut counter = Counter::default();

        let error = counter
            .apply(&CounterEventPayload::Decremented { amount: 1 })
            .expect_err("decrement should fail when it would go negative");

        assert_eq!(error, CounterError::NegativeValue);
        assert_eq!(counter.value, 0);
    }
}

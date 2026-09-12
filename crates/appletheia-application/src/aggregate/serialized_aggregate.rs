use appletheia_domain::{Aggregate, AggregateId, AggregateVersion};

use super::{
    AggregateIdValue, AggregateTypeOwned, SerializedAggregateError, SerializedAggregateState,
};

/// Current aggregate state at a serialization boundary, excluding uncommitted events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedAggregate {
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: AggregateIdValue,
    pub aggregate_version: AggregateVersion,
    pub state: Option<SerializedAggregateState>,
}

impl SerializedAggregate {
    pub fn try_from_aggregate<A>(aggregate: &A) -> Result<Self, SerializedAggregateError>
    where
        A: Aggregate,
    {
        Ok(Self {
            aggregate_type: AggregateTypeOwned::from(A::TYPE),
            aggregate_id: AggregateIdValue::from(aggregate.aggregate_id().value()),
            aggregate_version: aggregate.version(),
            state: aggregate
                .state()
                .map(SerializedAggregateState::try_from_state)
                .transpose()?,
        })
    }

    /// Restores identity, version and optional state; no pending events are reconstructed.
    pub fn try_into_aggregate<A>(&self) -> Result<A, SerializedAggregateError>
    where
        A: Aggregate,
    {
        if self.aggregate_type.value() != A::TYPE.value() {
            return Err(SerializedAggregateError::AggregateTypeMismatch {
                expected: A::TYPE.value(),
                actual: self.aggregate_type.value().to_owned(),
            });
        }
        let aggregate_id = A::Id::try_from_uuid(self.aggregate_id.value())
            .map_err(|error| SerializedAggregateError::AggregateId(Box::new(error)))?;
        let state = self
            .state
            .as_ref()
            .map(SerializedAggregateState::try_into_state::<A::State>)
            .transpose()?;
        let mut aggregate = A::from_id(aggregate_id);
        aggregate.set_state(state);
        aggregate.core_mut().set_version(self.aggregate_version);
        aggregate.core_mut().clear_uncommitted_events();
        Ok(aggregate)
    }
}

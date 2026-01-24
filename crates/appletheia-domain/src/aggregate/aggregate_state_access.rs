use super::{AggregateId, AggregateState};

pub trait AggregateStateAccess<I: AggregateId, S: AggregateState<Id = I>> {
    fn state(&self) -> Option<&S>;

    fn set_state(&mut self, state: Option<S>);
}

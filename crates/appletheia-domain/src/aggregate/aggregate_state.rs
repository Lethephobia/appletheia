use std::{fmt::Debug, hash::Hash};

use super::AggregateId;

pub trait AggregateState: Clone + Debug + Eq + Hash + Send + Sync + 'static {
    type Id: AggregateId;

    fn id(&self) -> Self::Id;
}

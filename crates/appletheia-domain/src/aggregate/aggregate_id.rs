use std::{fmt::Debug, hash::Hash};

use crate::core::Id;

pub trait AggregateId: Copy + Debug + Eq + Hash + Ord + Send + Sync + 'static {
    fn value(self) -> Id;
}

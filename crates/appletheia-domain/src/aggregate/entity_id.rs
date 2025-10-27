use crate::core::Id;
use std::{fmt::Debug, hash::Hash};

pub trait EntityId: Copy + Debug + Eq + Hash + Send + Sync + 'static {
    fn value(self) -> Id;
}

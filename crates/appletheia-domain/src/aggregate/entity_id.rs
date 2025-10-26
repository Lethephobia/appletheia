use crate::core::{Id, ValueObject};

pub trait EntityId: Copy + ValueObject {
    fn value(self) -> Id;
}

impl<T: EntityId> ValueObject for T {}

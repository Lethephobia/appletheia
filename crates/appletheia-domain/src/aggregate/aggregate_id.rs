use crate::aggregate::EntityId;

use crate::core::Id;

pub trait AggregateId: EntityId {
    fn value(self) -> Id;
}

impl<A: AggregateId> EntityId for A {
    fn value(self) -> Id {
        AggregateId::value(self)
    }
}

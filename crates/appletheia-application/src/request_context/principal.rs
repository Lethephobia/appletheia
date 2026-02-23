use crate::authorization::AggregateRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Principal {
    Unavailable,
    Anonymous,
    System,
    Authenticated { subject: AggregateRef },
}

impl Default for Principal {
    fn default() -> Self {
        Self::Unavailable
    }
}

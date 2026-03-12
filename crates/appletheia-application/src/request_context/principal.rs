use crate::authorization::AggregateRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum Principal {
    #[default]
    Unavailable,
    Anonymous,
    System,
    Authenticated {
        subject: AggregateRef,
    },
}

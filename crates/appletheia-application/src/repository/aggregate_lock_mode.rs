#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AggregateLockMode {
    Shared,
    Exclusive,
}

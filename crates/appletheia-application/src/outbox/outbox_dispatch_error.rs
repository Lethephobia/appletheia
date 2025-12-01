#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutboxDispatchError {
    Transient { message: String },
    Permanent { message: String },
}

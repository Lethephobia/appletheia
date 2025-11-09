pub mod correlation_id;
pub mod message_id;

pub use correlation_id::CorrelationId;
pub use message_id::MessageId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestContext {
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
}

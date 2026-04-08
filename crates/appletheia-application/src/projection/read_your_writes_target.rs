use crate::request_context::{CorrelationId, MessageId};
use serde::{Deserialize, Serialize};

/// Identifies the write scope that read-your-writes consistency should observe.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReadYourWritesTarget {
    /// Waits for effects caused by a specific message.
    Message(MessageId),
    /// Waits for effects produced within a workflow correlation.
    Correlation(CorrelationId),
}

use appletheia_domain::EventId;

use crate::projection::{ProjectionConsistencyPollInterval, ProjectionConsistencyTimeout};
use crate::request_context::MessageId;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum QueryConsistency {
    #[default]
    Eventual,
    AfterMessage {
        message_id: MessageId,
        timeout: ProjectionConsistencyTimeout,
        poll_interval: ProjectionConsistencyPollInterval,
    },
    AfterEvents {
        event_ids: Vec<EventId>,
        timeout: ProjectionConsistencyTimeout,
        poll_interval: ProjectionConsistencyPollInterval,
    },
}

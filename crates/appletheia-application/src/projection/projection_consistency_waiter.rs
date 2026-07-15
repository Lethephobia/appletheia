use appletheia_domain::EventId;

use crate::request_context::MessageId;

use super::{
    ProjectionConsistencyPollInterval, ProjectionConsistencyTimeout,
    ProjectionConsistencyWaitError, ProjectorDependencies,
};

#[allow(async_fn_in_trait)]
pub trait ProjectionConsistencyWaiter: Send + Sync {
    async fn wait_for_message(
        &self,
        message_id: MessageId,
        timeout: ProjectionConsistencyTimeout,
        poll_interval: ProjectionConsistencyPollInterval,
        projector_dependencies: ProjectorDependencies<'_>,
    ) -> Result<(), ProjectionConsistencyWaitError>;

    async fn wait_for_events(
        &self,
        event_ids: &[EventId],
        timeout: ProjectionConsistencyTimeout,
        poll_interval: ProjectionConsistencyPollInterval,
        projector_dependencies: ProjectorDependencies<'_>,
    ) -> Result<(), ProjectionConsistencyWaitError>;
}

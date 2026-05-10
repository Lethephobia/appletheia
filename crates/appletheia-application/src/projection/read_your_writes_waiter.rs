use super::{
    ProjectorDependencies, ReadYourWritesPollInterval, ReadYourWritesTimeout,
    ReadYourWritesWaitError,
};
use crate::request_context::MessageId;

#[allow(async_fn_in_trait)]
pub trait ReadYourWritesWaiter: Send + Sync {
    async fn wait(
        &self,
        message_id: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
        projector_dependencies: ProjectorDependencies<'_>,
    ) -> Result<(), ReadYourWritesWaitError>;
}

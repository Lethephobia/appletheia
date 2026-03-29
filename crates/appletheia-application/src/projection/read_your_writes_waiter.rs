use crate::request_context::MessageId;

use super::{
    ProjectorDependencies, ReadYourWritesPollInterval, ReadYourWritesTimeout,
    ReadYourWritesWaitError,
};

#[allow(async_fn_in_trait)]
pub trait ReadYourWritesWaiter: Send + Sync {
    async fn wait(
        &self,
        after: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
        projector_dependencies: ProjectorDependencies<'_>,
    ) -> Result<(), ReadYourWritesWaitError>;
}

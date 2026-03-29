use super::{
    ProjectorDependencies, ReadYourWritesPollInterval, ReadYourWritesTarget, ReadYourWritesTimeout,
    ReadYourWritesWaitError,
};
use crate::saga::SagaDependencies;

#[allow(async_fn_in_trait)]
pub trait ReadYourWritesWaiter: Send + Sync {
    async fn wait(
        &self,
        target: ReadYourWritesTarget,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
        projector_dependencies: ProjectorDependencies<'_>,
        saga_dependencies: SagaDependencies<'_>,
    ) -> Result<(), ReadYourWritesWaitError>;
}

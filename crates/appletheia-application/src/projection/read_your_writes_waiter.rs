use super::{
    ProjectorDependencies, ReadYourWritesPollInterval, ReadYourWritesTarget, ReadYourWritesTimeout,
    ReadYourWritesWaitError,
};

#[allow(async_fn_in_trait)]
pub trait ReadYourWritesWaiter: Send + Sync {
    async fn wait(
        &self,
        target: ReadYourWritesTarget,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
        projector_dependencies: ProjectorDependencies<'_>,
    ) -> Result<(), ReadYourWritesWaitError>;
}

use super::{ProjectorDefinition, ProjectorWorkerError};

#[allow(async_fn_in_trait)]
pub trait ProjectorWorker: Send {
    type Projector: ProjectorDefinition;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&mut self) -> Result<(), ProjectorWorkerError>;
}

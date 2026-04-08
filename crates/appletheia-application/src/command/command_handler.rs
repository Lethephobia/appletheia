use std::error::Error;

use crate::authorization::AuthorizationPlan;
use crate::command::{Command, CommandHandled};
use crate::projection::ProjectorDependencies;
use crate::request_context::RequestContext;
use crate::saga::SagaDependencies;
use crate::unit_of_work::UnitOfWork;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Handles a command within the application command pipeline.
///
/// A handler returns an immediate `Output` for the current execution and a replay-safe
/// `ReplayOutput` that can be persisted for idempotent replays.
#[allow(async_fn_in_trait)]
pub trait CommandHandler: Send + Sync {
    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::None;
    const SAGA_DEPENDENCIES: SagaDependencies<'static> = SagaDependencies::None;

    type Command: Command;
    type Output: Send + 'static;
    type ReplayOutput: Serialize + DeserializeOwned + Send + 'static;
    type Error: Error + Send + Sync + 'static;
    type Uow: UnitOfWork;

    /// Builds the authorization requirements for the incoming command.
    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::default())
    }

    /// Executes the command and returns both the immediate output and replay-safe output.
    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error>;
}

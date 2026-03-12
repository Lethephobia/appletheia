use std::error::Error;

use crate::authorization::AuthorizationPlan;
use crate::command::{Command, CommandHandled};
use crate::projection::ProjectorDependencies;
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[allow(async_fn_in_trait)]
pub trait CommandHandler: Send + Sync {
    type Command: Command;
    type Output: Serialize + DeserializeOwned + Send + 'static;
    type ReplayOutput: Serialize + DeserializeOwned + Send + 'static;
    type Error: Error + Send + Sync + 'static;
    type Uow: UnitOfWork;

    const DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::None;

    fn authorization_plan(&self, _command: &Self::Command) -> AuthorizationPlan {
        AuthorizationPlan::default()
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error>;
}

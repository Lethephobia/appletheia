use crate::Retryability;
use crate::authorization::AuthorizationPlan;
use crate::command::{Command, CommandOutput};
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

/// Handles a command within the application command pipeline.
///
/// The output owns its conversion to the replay-safe representation persisted for idempotency.
#[allow(async_fn_in_trait)]
pub trait CommandHandler: Send + Sync {
    type Command: Command;
    type Output: CommandOutput;
    type Error: Retryability;
    type Uow: UnitOfWork;

    /// Builds the authorization requirements for the incoming command.
    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::default())
    }

    /// Executes the command and returns its immediate output.
    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error>;
}

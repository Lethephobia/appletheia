use crate::command::CommandName;
use crate::outbox::command::SerializedCommand;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SagaCommand {
    pub command_name: CommandName,
    pub command: SerializedCommand,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SagaOutcome {
    InProgress { commands: Vec<SagaCommand> },
    Succeeded,
    Failed { error: serde_json::Value },
}

impl SagaOutcome {
    pub fn commands(&self) -> &[SagaCommand] {
        match self {
            SagaOutcome::InProgress { commands } => commands,
            SagaOutcome::Succeeded | SagaOutcome::Failed { .. } => &[],
        }
    }

    pub fn into_commands(self) -> Vec<SagaCommand> {
        match self {
            SagaOutcome::InProgress { commands } => commands,
            SagaOutcome::Succeeded | SagaOutcome::Failed { .. } => Vec::new(),
        }
    }
}

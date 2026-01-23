use crate::command::CommandName;
use crate::outbox::command::CommandPayload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SagaCommand<CN: CommandName> {
    pub command_name: CN,
    pub payload: CommandPayload,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SagaOutcome<CN: CommandName> {
    InProgress { commands: Vec<SagaCommand<CN>> },
    Succeeded,
    Failed { error: serde_json::Value },
}

impl<CN: CommandName> SagaOutcome<CN> {
    pub fn commands(&self) -> &[SagaCommand<CN>] {
        match self {
            SagaOutcome::InProgress { commands } => commands,
            SagaOutcome::Succeeded | SagaOutcome::Failed { .. } => &[],
        }
    }

    pub fn into_commands(self) -> Vec<SagaCommand<CN>> {
        match self {
            SagaOutcome::InProgress { commands } => commands,
            SagaOutcome::Succeeded | SagaOutcome::Failed { .. } => Vec::new(),
        }
    }
}

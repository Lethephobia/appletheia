use crate::command::CommandNameOwned;
use crate::outbox::command::CommandPayload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SagaCommand {
    pub command_name: CommandNameOwned,
    pub payload: CommandPayload,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SagaOutcome {
    InProgress {
        commands: Vec<SagaCommand>,
    },
    Succeeded {
        commands: Vec<SagaCommand>,
    },
    Failed {
        commands: Vec<SagaCommand>,
        error: serde_json::Value,
    },
}

impl SagaOutcome {
    pub fn commands(&self) -> &[SagaCommand] {
        match self {
            SagaOutcome::InProgress { commands } => commands,
            SagaOutcome::Succeeded { commands } => commands,
            SagaOutcome::Failed { commands, .. } => commands,
        }
    }

    pub fn into_commands(self) -> Vec<SagaCommand> {
        match self {
            SagaOutcome::InProgress { commands } => commands,
            SagaOutcome::Succeeded { commands } => commands,
            SagaOutcome::Failed { commands, .. } => commands,
        }
    }
}

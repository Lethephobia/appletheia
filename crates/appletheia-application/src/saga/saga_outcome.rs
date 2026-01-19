use crate::command::CommandNameOwned;
use crate::outbox::command::CommandPayload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SagaCommand {
    pub command_name: CommandNameOwned,
    pub payload: CommandPayload,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SagaOutcome {
    pub commands: Vec<SagaCommand>,
    pub completion: SagaCompletion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SagaCompletion {
    InProgress,
    Completed,
    Failed { error: serde_json::Value },
}

impl SagaOutcome {
    pub fn in_progress(commands: Vec<SagaCommand>) -> Self {
        Self {
            commands,
            completion: SagaCompletion::InProgress,
        }
    }

    pub fn completed(commands: Vec<SagaCommand>) -> Self {
        Self {
            commands,
            completion: SagaCompletion::Completed,
        }
    }

    pub fn failed(commands: Vec<SagaCommand>, error: serde_json::Value) -> Self {
        Self {
            commands,
            completion: SagaCompletion::Failed { error },
        }
    }
}
